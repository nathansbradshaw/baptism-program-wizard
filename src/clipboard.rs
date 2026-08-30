//! Cross-window "copy a whole page, paste it elsewhere" support. Mirrors
//! `pageClipboardPayload`/`copyCurrentPage`/`pasteCurrentPage` in app.js,
//! which layer four redundant transports so a copy in one browser window can
//! be pasted in another: the system clipboard, an IndexedDB record shared by
//! same-origin windows, and localStorage/sessionStorage as last resorts.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{IdbDatabase, IdbTransactionMode};

use crate::model::{block_to_json, sanitize_block, Block, Page};

const STORAGE_KEY: &str = "baptism-program-page-clipboard-v1";
const DB_NAME: &str = "baptism-program-shared-clipboard";
const STORE_NAME: &str = "pages";
const RECORD_KEY: &str = "current-page";

thread_local! {
    static IN_MEMORY: std::cell::RefCell<Option<serde_json::Value>> = std::cell::RefCell::new(None);
}

pub fn payload_for(page: &Page) -> serde_json::Value {
    serde_json::json!({
        "format": "baptism-program-page",
        "version": 1,
        "copiedFrom": page.id.as_str(),
        "blocks": page.blocks.iter().map(block_to_json).collect::<Vec<_>>(),
    })
}

fn parse_clipboard_value(value: &serde_json::Value) -> Option<serde_json::Value> {
    let obj = value.as_object()?;
    if obj.get("format").and_then(|v| v.as_str()) != Some("baptism-program-page") {
        return None;
    }
    if obj.get("version").and_then(|v| v.as_i64()) != Some(1) {
        return None;
    }
    obj.get("blocks")?.as_array()?;
    Some(value.clone())
}

fn parse_clipboard_str(s: &str) -> Option<serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(s).ok().and_then(|v| parse_clipboard_value(&v))
}

/// Sanitizes and re-ids the blocks from a clipboard payload for pasting.
pub fn blocks_from_payload(payload: &serde_json::Value) -> Vec<Block> {
    payload
        .get("blocks")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(100).filter_map(sanitize_block).map(|b| b.duplicated()).collect())
        .unwrap_or_default()
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window exists")
}

fn js_value_to_json(value: &JsValue) -> Option<serde_json::Value> {
    let text = js_sys::JSON::stringify(value).ok()?.as_string()?;
    serde_json::from_str(&text).ok()
}

fn json_to_js_value(value: &serde_json::Value) -> Result<JsValue, JsValue> {
    js_sys::JSON::parse(&value.to_string())
}

// ---------------------------------------------------------------------------
// IndexedDB (shared across windows/tabs of this origin)
// ---------------------------------------------------------------------------

async fn open_database() -> Result<IdbDatabase, JsValue> {
    let factory = window()
        .indexed_db()?
        .ok_or_else(|| JsValue::from_str("Shared browser storage is unavailable."))?;
    let open_request = factory.open_with_u32(DB_NAME, 1)?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let req_for_upgrade = open_request.clone();
        let onupgradeneeded = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            if let Ok(result) = req_for_upgrade.result() {
                if let Ok(db) = result.dyn_into::<IdbDatabase>() {
                    let names = db.object_store_names();
                    let mut has_store = false;
                    for i in 0..names.length() {
                        if names.get(i).as_deref() == Some(STORE_NAME) {
                            has_store = true;
                            break;
                        }
                    }
                    if !has_store {
                        db.create_object_store(STORE_NAME).ok();
                    }
                }
            }
        });
        open_request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        onupgradeneeded.forget();

        let req_for_success = open_request.clone();
        let onsuccess = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            if let Ok(result) = req_for_success.result() {
                resolve.call1(&JsValue::NULL, &result).ok();
            }
        });
        open_request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        onsuccess.forget();

        let reject_for_error = reject.clone();
        let onerror = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject_for_error.call1(&JsValue::NULL, &JsValue::from_str("Could not open shared browser storage.")).ok();
        });
        open_request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        let onblocked = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject.call1(&JsValue::NULL, &JsValue::from_str("Shared browser storage is blocked.")).ok();
        });
        open_request.set_onblocked(Some(onblocked.as_ref().unchecked_ref()));
        onblocked.forget();
    });

    JsFuture::from(promise).await?.dyn_into::<IdbDatabase>()
}

async fn store_shared_page_clipboard(payload: &serde_json::Value) -> Result<(), JsValue> {
    let database = open_database().await?;
    let js_payload = json_to_js_value(payload)?;
    let transaction = database.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)?;
    let store = transaction.object_store(STORE_NAME)?;
    store.put_with_key(&js_payload, &JsValue::from_str(RECORD_KEY))?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let oncomplete = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).ok();
        });
        transaction.set_oncomplete(Some(oncomplete.as_ref().unchecked_ref()));
        oncomplete.forget();

        let reject_for_error = reject.clone();
        let onerror = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject_for_error.call1(&JsValue::NULL, &JsValue::from_str("Could not store the copied page.")).ok();
        });
        transaction.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        let onabort = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject.call1(&JsValue::NULL, &JsValue::from_str("Could not store the copied page.")).ok();
        });
        transaction.set_onabort(Some(onabort.as_ref().unchecked_ref()));
        onabort.forget();
    });

    JsFuture::from(promise).await?;
    database.close();
    Ok(())
}

async fn read_shared_page_clipboard() -> Result<Option<serde_json::Value>, JsValue> {
    let database = open_database().await?;
    let transaction = database.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)?;
    let store = transaction.object_store(STORE_NAME)?;
    let request = store.get(&JsValue::from_str(RECORD_KEY))?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let req_for_success = request.clone();
        let onsuccess = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            if let Ok(result) = req_for_success.result() {
                resolve.call1(&JsValue::NULL, &result).ok();
            }
        });
        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        onsuccess.forget();

        let onerror = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject.call1(&JsValue::NULL, &JsValue::from_str("Could not read the copied page.")).ok();
        });
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    });

    let value = JsFuture::from(promise).await?;
    database.close();
    Ok(js_value_to_json(&value).and_then(|v| parse_clipboard_value(&v)))
}

// ---------------------------------------------------------------------------
// System clipboard
// ---------------------------------------------------------------------------

pub(crate) async fn write_system_clipboard(text: &str) -> Result<(), JsValue> {
    let clipboard = window().navigator().clipboard();
    JsFuture::from(clipboard.write_text(text)).await?;
    Ok(())
}

pub(crate) async fn read_system_clipboard() -> Result<String, JsValue> {
    let clipboard = window().navigator().clipboard();
    let value = JsFuture::from(clipboard.read_text()).await?;
    value.as_string().ok_or_else(|| JsValue::from_str("empty clipboard"))
}

// ---------------------------------------------------------------------------
// Public API (mirrors copyCurrentPage / pasteCurrentPage)
// ---------------------------------------------------------------------------

pub async fn copy_page(page: &Page) {
    let payload = payload_for(page);
    let serialized = payload.to_string();

    IN_MEMORY.with(|cell| *cell.borrow_mut() = Some(payload.clone()));

    if let Some(storage) = window().local_storage().ok().flatten() {
        storage.set_item(STORAGE_KEY, &serialized).ok();
    }
    if let Some(storage) = window().session_storage().ok().flatten() {
        storage.set_item(STORAGE_KEY, &serialized).ok();
    }

    // Best-effort: neither of these being unavailable should block the copy.
    let _ = store_shared_page_clipboard(&payload).await;
    let _ = write_system_clipboard(&serialized).await;
}

pub async fn read_clipboard() -> Option<serde_json::Value> {
    if let Ok(text) = read_system_clipboard().await {
        if let Some(value) = parse_clipboard_str(&text) {
            return Some(value);
        }
    }

    if let Some(value) = IN_MEMORY.with(|cell| cell.borrow().clone()) {
        return Some(value);
    }

    if let Ok(Some(value)) = read_shared_page_clipboard().await {
        return Some(value);
    }

    if let Some(storage) = window().local_storage().ok().flatten() {
        if let Some(raw) = storage.get_item(STORAGE_KEY).ok().flatten() {
            if let Some(value) = parse_clipboard_str(&raw) {
                return Some(value);
            }
        }
    }
    if let Some(storage) = window().session_storage().ok().flatten() {
        if let Some(raw) = storage.get_item(STORAGE_KEY).ok().flatten() {
            if let Some(value) = parse_clipboard_str(&raw) {
                return Some(value);
            }
        }
    }

    None
}
