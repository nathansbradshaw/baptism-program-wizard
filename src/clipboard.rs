//! Cross-window clipboard support for both whole pages and single elements.
//! Mirrors `pageClipboardPayload`/`copyCurrentPage`/`pasteCurrentPage` in
//! app.js (the page half), extended with an analogous element-level clipboard
//! so a design can donate individual blocks to another design. Both layer the
//! same four redundant transports so a copy in one browser window can be
//! pasted in another: the system clipboard, an IndexedDB record shared by
//! same-origin windows, and localStorage/sessionStorage as last resorts.

use std::cell::RefCell;
use std::thread::LocalKey;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{IdbDatabase, IdbTransactionMode};

use crate::model::{block_to_json, sanitize_block, Block, Page};

const DB_NAME: &str = "baptism-program-shared-clipboard";
const STORE_NAME: &str = "clipboard";

const PAGE_FORMAT: &str = "baptism-program-page";
const PAGE_STORAGE_KEY: &str = "baptism-program-page-clipboard-v1";
const PAGE_RECORD_KEY: &str = "current-page";

const BLOCK_FORMAT: &str = "baptism-program-block";
const BLOCK_STORAGE_KEY: &str = "baptism-program-block-clipboard-v1";
const BLOCK_RECORD_KEY: &str = "current-block";

thread_local! {
    static IN_MEMORY_PAGE: RefCell<Option<serde_json::Value>> = RefCell::new(None);
    static IN_MEMORY_BLOCK: RefCell<Option<serde_json::Value>> = RefCell::new(None);
}

// ---------------------------------------------------------------------------
// Payload shapes
// ---------------------------------------------------------------------------

pub fn payload_for_page(page: &Page) -> serde_json::Value {
    serde_json::json!({
        "format": PAGE_FORMAT,
        "version": 1,
        "copiedFrom": page.id.as_str(),
        "blocks": page.blocks.iter().map(block_to_json).collect::<Vec<_>>(),
    })
}

/// Sanitizes and re-ids the blocks from a page clipboard payload for pasting.
pub fn blocks_from_page_payload(payload: &serde_json::Value) -> Vec<Block> {
    payload
        .get("blocks")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(100).filter_map(sanitize_block).map(|b| b.duplicated()).collect())
        .unwrap_or_default()
}

pub fn payload_for_block(block: &Block) -> serde_json::Value {
    serde_json::json!({
        "format": BLOCK_FORMAT,
        "version": 1,
        "block": block_to_json(block),
    })
}

/// Sanitizes and re-ids the block from an element clipboard payload for pasting.
pub fn block_from_payload(payload: &serde_json::Value) -> Option<Block> {
    sanitize_block(payload.get("block")?).map(|b| b.duplicated())
}

fn parse_clipboard_value(expected_format: &str, value: &serde_json::Value) -> Option<serde_json::Value> {
    let obj = value.as_object()?;
    if obj.get("format").and_then(|v| v.as_str()) != Some(expected_format) {
        return None;
    }
    if obj.get("version").and_then(|v| v.as_i64()) != Some(1) {
        return None;
    }
    Some(value.clone())
}

fn parse_clipboard_str(expected_format: &str, s: &str) -> Option<serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(s).ok().and_then(|v| parse_clipboard_value(expected_format, &v))
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
// IndexedDB (shared across windows/tabs of this origin; generic over which
// record — page clipboard vs. element clipboard — is being read/written)
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

async fn store_shared(record_key: &str, payload: &serde_json::Value) -> Result<(), JsValue> {
    let database = open_database().await?;
    let js_payload = json_to_js_value(payload)?;
    let transaction = database.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)?;
    let store = transaction.object_store(STORE_NAME)?;
    store.put_with_key(&js_payload, &JsValue::from_str(record_key))?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let oncomplete = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).ok();
        });
        transaction.set_oncomplete(Some(oncomplete.as_ref().unchecked_ref()));
        oncomplete.forget();

        let reject_for_error = reject.clone();
        let onerror = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject_for_error.call1(&JsValue::NULL, &JsValue::from_str("Could not store the copied item.")).ok();
        });
        transaction.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        let onabort = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            reject.call1(&JsValue::NULL, &JsValue::from_str("Could not store the copied item.")).ok();
        });
        transaction.set_onabort(Some(onabort.as_ref().unchecked_ref()));
        onabort.forget();
    });

    JsFuture::from(promise).await?;
    database.close();
    Ok(())
}

async fn read_shared(record_key: &str) -> Result<Option<serde_json::Value>, JsValue> {
    let database = open_database().await?;
    let transaction = database.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)?;
    let store = transaction.object_store(STORE_NAME)?;
    let request = store.get(&JsValue::from_str(record_key))?;

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
            reject.call1(&JsValue::NULL, &JsValue::from_str("Could not read the copied item.")).ok();
        });
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    });

    let value = JsFuture::from(promise).await?;
    database.close();
    Ok(js_value_to_json(&value))
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
// Generic copy/read, shared by the page and element clipboards
// ---------------------------------------------------------------------------

async fn copy_generic(memory: &'static LocalKey<RefCell<Option<serde_json::Value>>>, storage_key: &str, record_key: &str, payload: serde_json::Value) {
    let serialized = payload.to_string();
    memory.with(|cell| *cell.borrow_mut() = Some(payload.clone()));

    if let Some(storage) = window().local_storage().ok().flatten() {
        storage.set_item(storage_key, &serialized).ok();
    }
    if let Some(storage) = window().session_storage().ok().flatten() {
        storage.set_item(storage_key, &serialized).ok();
    }

    // Best-effort: neither of these being unavailable should block the copy.
    let _ = store_shared(record_key, &payload).await;
    let _ = write_system_clipboard(&serialized).await;
}

async fn read_generic(
    memory: &'static LocalKey<RefCell<Option<serde_json::Value>>>,
    format: &str,
    storage_key: &str,
    record_key: &str,
) -> Option<serde_json::Value> {
    if let Ok(text) = read_system_clipboard().await {
        if let Some(value) = parse_clipboard_str(format, &text) {
            return Some(value);
        }
    }

    if let Some(value) = memory.with(|cell| cell.borrow().clone()) {
        return Some(value);
    }

    if let Ok(Some(raw)) = read_shared(record_key).await {
        if let Some(value) = parse_clipboard_value(format, &raw) {
            return Some(value);
        }
    }

    if let Some(storage) = window().local_storage().ok().flatten() {
        if let Some(raw) = storage.get_item(storage_key).ok().flatten() {
            if let Some(value) = parse_clipboard_str(format, &raw) {
                return Some(value);
            }
        }
    }
    if let Some(storage) = window().session_storage().ok().flatten() {
        if let Some(raw) = storage.get_item(storage_key).ok().flatten() {
            if let Some(value) = parse_clipboard_str(format, &raw) {
                return Some(value);
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Public API (mirrors copyCurrentPage / pasteCurrentPage, plus the analogous
// single-element clipboard)
// ---------------------------------------------------------------------------

pub async fn copy_page(page: &Page) {
    copy_generic(&IN_MEMORY_PAGE, PAGE_STORAGE_KEY, PAGE_RECORD_KEY, payload_for_page(page)).await;
}

pub async fn read_page_clipboard() -> Option<serde_json::Value> {
    read_generic(&IN_MEMORY_PAGE, PAGE_FORMAT, PAGE_STORAGE_KEY, PAGE_RECORD_KEY).await
}

pub async fn copy_block(block: &Block) {
    copy_generic(&IN_MEMORY_BLOCK, BLOCK_STORAGE_KEY, BLOCK_RECORD_KEY, payload_for_block(block)).await;
}

pub async fn read_block_clipboard() -> Option<serde_json::Value> {
    read_generic(&IN_MEMORY_BLOCK, BLOCK_FORMAT, BLOCK_STORAGE_KEY, BLOCK_RECORD_KEY).await
}
