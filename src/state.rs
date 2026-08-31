use std::cell::RefCell;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, File, HtmlAnchorElement, HtmlInputElement, HtmlSelectElement, Window};

use crate::canvas::{self, GrayscaleCache};
use crate::editor;
// `Document` collides with `web_sys::Document`; alias the model type.
use crate::model::Document as Document_;
use crate::model::*;
use crate::render;

const DRAFT_KEY: &str = "baptism-program-document-v2";

struct AppState {
    document: Document_,
    selected_page: PageId,
    focus_mode: bool,
    preview_zoom: f64,
}

thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState {
        document: restore_draft().unwrap_or_else(default_document),
        selected_page: PageId::Front,
        focus_mode: false,
        preview_zoom: 1.0,
    });
    static GRAYSCALE: GrayscaleCache = GrayscaleCache::default();
    static SAVE_TIMER: RefCell<Option<(i32, Closure<dyn FnMut()>)>> = RefCell::new(None);
    static STATUS_TIMER: RefCell<Option<(i32, Closure<dyn FnMut()>)>> = RefCell::new(None);
    static HIGHLIGHTED_BLOCK_ID: RefCell<String> = RefCell::new(String::new());
}

fn selected_page() -> PageId {
    STATE.with(|s| s.borrow().selected_page)
}

/// Toggles `.is-linked-highlight` on the block-card and preview block that
/// share `data-block-id == highlighted`. Mirrors `applyLinkedHighlight`.
pub fn apply_linked_highlight() {
    let highlighted = HIGHLIGHTED_BLOCK_ID.with(|h| h.borrow().clone());
    let Ok(nodes) = document().query_selector_all(".block-card[data-block-id], .page-preview-shell [data-block-id]") else { return };
    for i in 0..nodes.length() {
        let Some(node) = nodes.get(i).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
        let matches = !highlighted.is_empty() && node.get_attribute("data-block-id").as_deref() == Some(highlighted.as_str());
        node.class_list().toggle_with_force("is-linked-highlight", matches).ok();
    }
}

/// Mirrors `setLinkedHighlight`.
pub fn set_linked_highlight(block_id: &str) {
    HIGHLIGHTED_BLOCK_ID.with(|h| *h.borrow_mut() = block_id.to_string());
    apply_linked_highlight();
}

/// Whether the document's currently focused element is inside `container`.
/// Used so a pointerleave while a field inside the card still has focus
/// doesn't clear the highlight.
pub fn active_element_within(container: &Element) -> bool {
    document().active_element().is_some_and(|active| container.contains(Some(active.unchecked_ref())))
}

fn window() -> Window {
    web_sys::window().expect("window exists")
}

fn document() -> web_sys::Document {
    window().document().expect("document exists")
}

fn el(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

fn input(id: &str) -> Option<HtmlInputElement> {
    el(id).and_then(|e| e.dyn_into().ok())
}

fn select(id: &str) -> Option<HtmlSelectElement> {
    el(id).and_then(|e| e.dyn_into().ok())
}

// ---------------------------------------------------------------------------
// Draft persistence (mirrors storeDraft / restoreDraft / scheduleSave)
// ---------------------------------------------------------------------------

fn session_storage() -> Option<web_sys::Storage> {
    window().session_storage().ok().flatten()
}

fn store_draft() {
    let json = STATE.with(|s| document_to_json(&s.borrow().document).to_string());
    match session_storage() {
        Some(storage) => {
            if storage.set_item(DRAFT_KEY, &json).is_err() {
                show_status("This design is too large for browser draft storage. Download a save to keep it.", true);
            }
        }
        None => show_status("This design is too large for browser draft storage. Download a save to keep it.", true),
    }
}

fn restore_draft() -> Option<Document_> {
    let storage = session_storage()?;
    let raw = storage.get_item(DRAFT_KEY).ok().flatten()?;
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(value) => Some(sanitize_document(&value)),
        Err(_) => {
            storage.remove_item(DRAFT_KEY).ok();
            None
        }
    }
}

fn schedule_save() {
    SAVE_TIMER.with(|cell| {
        let mut slot = cell.borrow_mut();
        if let Some((handle, _)) = slot.take() {
            window().clear_timeout_with_handle(handle);
        }
        let closure = Closure::<dyn FnMut()>::new(store_draft);
        let handle = window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 250).expect("setTimeout");
        *slot = Some((handle, closure));
    });
}

// ---------------------------------------------------------------------------
// Status line (mirrors showStatus)
// ---------------------------------------------------------------------------

pub fn show_status(message: &str, is_error: bool) {
    STATUS_TIMER.with(|cell| {
        let mut slot = cell.borrow_mut();
        if let Some((handle, _)) = slot.take() {
            window().clear_timeout_with_handle(handle);
        }
        if let Some(status) = el("save-status") {
            status.set_text_content(Some(message));
            status.class_list().toggle_with_force("error", is_error).ok();
        }
        let closure = Closure::<dyn FnMut()>::new(|| {
            if let Some(status) = el("save-status") {
                status.set_text_content(Some(""));
                status.class_list().remove_1("error").ok();
            }
        });
        let handle = window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 5000).expect("setTimeout");
        *slot = Some((handle, closure));
    });
}

// ---------------------------------------------------------------------------
// Rendering orchestration
// ---------------------------------------------------------------------------

fn render_pages() {
    let doc = document();
    let Some(container) = el("live-preview") else { return };
    container.set_inner_html("");

    STATE.with(|s| {
        let state = s.borrow();
        let grayscale = GRAYSCALE.with(|g| g.clone());
        for page in &state.document.pages {
            let selected = page.id == state.selected_page;
            let shell = render::make(&doc, "button", if selected { "page-preview-shell selected" } else { "page-preview-shell" });
            shell.set_attribute("type", "button").ok();
            shell.set_attribute("data-page", page.id.as_str()).ok();
            shell.set_attribute("aria-label", &format!("Edit {}", page.id.label())).ok();
            shell.append_child(&render::make_text(&doc, "span", "preview-page-label", page.id.label())).ok();

            let viewport = render::make(&doc, "span", "page-preview-viewport");
            let scaled = render::make(&doc, "span", "page-preview-scale");
            scaled.append_child(&render::build_program_page(&doc, page, &state.document.theme, false, &grayscale)).ok();
            wire_preview_block_links(&scaled, page.id);
            viewport.append_child(&scaled).ok();
            shell.append_child(&viewport).ok();

            let page_id = page.id;
            let onclick = Closure::<dyn FnMut()>::new(move || select_page(page_id));
            shell.set_onclick(Some(onclick.as_ref().unchecked_ref()));
            onclick.forget();

            container.append_child(&shell).ok();

            for target in doc.query_selector_all(&format!("[data-print-page=\"{}\"]", page.id.as_str())).ok().into_iter().flat_map(|list| (0..list.length()).filter_map(move |i| list.get(i))) {
                if let Some(target) = target.dyn_ref::<Element>() {
                    target.set_inner_html("");
                    target.append_child(&render::build_program_page(&doc, page, &state.document.theme, true, &grayscale)).ok();
                }
            }
        }
    });

    let overflow_check = Closure::<dyn FnMut()>::new(move || {
        let doc = document();
        if let Some(shells) = doc.query_selector_all(".page-preview-shell").ok() {
            for i in 0..shells.length() {
                let Some(shell) = shells.get(i).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
                let Some(inner) = shell.query_selector(".program-page-inner").ok().flatten() else { continue };
                let overflowing = inner.scroll_height() > inner.client_height() + 1;
                shell.class_list().toggle_with_force("has-overflow", overflowing).ok();
                if overflowing {
                    shell.append_child(&render::make_text(&doc, "span", "overflow-warning", "Too much content")).ok();
                }
            }
        }
        apply_linked_highlight();
    });
    window().request_animation_frame(overflow_check.as_ref().unchecked_ref()).ok();
    overflow_check.forget();
}

/// Mirrors the pointerenter/pointerleave/click wiring on each rendered
/// `[data-block-id]` block inside a preview page, linking it to the matching
/// numbered card in the editor sidebar.
fn wire_preview_block_links(scaled: &Element, page_id: PageId) {
    let Ok(blocks) = scaled.query_selector_all("[data-block-id]") else { return };
    for i in 0..blocks.length() {
        let Some(block_el) = blocks.get(i).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
        let Some(block_id) = block_el.get_attribute("data-block-id") else { continue };

        let id_for_enter = block_id.clone();
        let onenter = Closure::<dyn FnMut()>::new(move || set_linked_highlight(&id_for_enter));
        block_el.add_event_listener_with_callback("pointerenter", onenter.as_ref().unchecked_ref()).ok();
        onenter.forget();

        let onleave = Closure::<dyn FnMut()>::new(move || set_linked_highlight(""));
        block_el.add_event_listener_with_callback("pointerleave", onleave.as_ref().unchecked_ref()).ok();
        onleave.forget();

        let id_for_click = block_id.clone();
        let onclick = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            event.stop_propagation();
            if page_id != selected_page() {
                select_page(page_id);
            }
            let id_for_raf = id_for_click.clone();
            let raf = Closure::<dyn FnMut()>::new(move || {
                if let Some(list) = el("block-list") {
                    if let Ok(cards) = list.query_selector_all(".block-card") {
                        for j in 0..cards.length() {
                            let Some(card) = cards.get(j).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
                            if card.get_attribute("data-block-id").as_deref() == Some(id_for_raf.as_str()) {
                                let options = web_sys::ScrollIntoViewOptions::new();
                                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                                options.set_block(web_sys::ScrollLogicalPosition::Center);
                                card.scroll_into_view_with_scroll_into_view_options(&options);
                                break;
                            }
                        }
                    }
                }
                set_linked_highlight(&id_for_raf);
            });
            window().request_animation_frame(raf.as_ref().unchecked_ref()).ok();
            raf.forget();
        });
        block_el.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref()).ok();
        onclick.forget();
    }
}

fn render_tabs() {
    STATE.with(|s| editor::render_tabs(&document(), s.borrow().selected_page));
}

fn render_block_editor() {
    STATE.with(|s| {
        let state = s.borrow();
        let page = state.document.page(state.selected_page);
        editor::render_block_editor(&document(), state.selected_page, page);
    });
}

fn render_all() {
    render_tabs();
    render_block_editor();
    render_pages();
}

fn update_theme_inputs() {
    let theme = STATE.with(|s| s.borrow().document.theme.clone());
    if let Some(i) = input("paper-color") { i.set_value(&theme.paper); }
    if let Some(i) = input("text-color") { i.set_value(&theme.text); }
    if let Some(i) = input("accent-color") { i.set_value(&theme.accent); }
    if let Some(s) = select("font-family") { s.set_value(theme.font.as_str()); }
    if let Some(i) = input("monochrome-images") { i.set_checked(theme.monochrome); }
    if let Some(s) = select("theme-preset") { s.set_value(matching_theme_preset(&theme).unwrap_or("custom")); }
}

fn update_focus_mode() {
    let (focus_mode, zoom) = STATE.with(|s| { let s = s.borrow(); (s.focus_mode, s.preview_zoom) });
    if let Some(preview) = el("live-preview") {
        preview.class_list().toggle_with_force("is-focused", focus_mode).ok();
        let style = preview.dyn_ref::<web_sys::HtmlElement>().map(|e| e.style());
        if let Some(style) = style {
            style.set_property("--focus-width", &format!("{}px", (528.0 * zoom).round())).ok();
            style.set_property("--focus-height", &format!("{}px", (816.0 * zoom).round())).ok();
            style.set_property("--focus-transform", &format!("scale({zoom})")).ok();
        }
    }
    if let Some(button) = el("toggle-focus") {
        button.set_attribute("aria-pressed", if focus_mode { "true" } else { "false" }).ok();
        button.set_text_content(Some(if focus_mode { "Show all pages" } else { "Expand page" }));
    }
    let zoom_percent = format!("{}%", (zoom * 100.0).round());
    if let Some(zoom_value) = el("zoom-value") {
        zoom_value.set_text_content(Some(&zoom_percent));
    }
    if let Some(zoom_out) = input_or_button("zoom-out") {
        zoom_out.set_disabled(zoom <= 0.6);
    }
    if let Some(zoom_in) = input_or_button("zoom-in") {
        zoom_in.set_disabled(zoom >= 1.2);
    }
}

fn input_or_button(id: &str) -> Option<web_sys::HtmlButtonElement> {
    el(id).and_then(|e| e.dyn_into().ok())
}

pub fn zoom_out() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let rounded = ((state.preview_zoom - 0.1) * 10.0).round() / 10.0;
        state.preview_zoom = rounded.max(0.6);
        state.focus_mode = true;
    });
    update_focus_mode();
}

pub fn zoom_in() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let rounded = ((state.preview_zoom + 0.1) * 10.0).round() / 10.0;
        state.preview_zoom = rounded.min(1.2);
        state.focus_mode = true;
    });
    update_focus_mode();
}

// ---------------------------------------------------------------------------
// Mutation API used by editor.rs
// ---------------------------------------------------------------------------

pub fn update_block_field(page_id: PageId, block_id: &str, f: impl FnOnce(&mut Block)) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(block) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            f(block);
        }
    });
    render_pages();
    schedule_save();
}

pub fn update_typography_field(page_id: PageId, block_id: &str, f: impl FnOnce(&mut Typography)) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(block) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            if let Some(typography) = block.typography_mut() {
                f(typography);
            }
        }
    });
    render_pages();
    schedule_save();
}

pub fn select_page(page_id: PageId) {
    STATE.with(|s| s.borrow_mut().selected_page = page_id);
    render_all();
}

pub fn move_page(index: usize, direction: i32) {
    let other = index as i64 + direction as i64;
    if other < 0 || other as usize >= PageId::ALL.len() {
        return;
    }
    let a = PageId::ALL[index];
    let b = PageId::ALL[other as usize];
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let blocks_a = std::mem::take(&mut state.document.page_mut(a).blocks);
        let blocks_b = std::mem::take(&mut state.document.page_mut(b).blocks);
        state.document.page_mut(a).blocks = blocks_b;
        state.document.page_mut(b).blocks = blocks_a;
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn move_block(page_id: PageId, index: usize, direction: i32) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let blocks = &mut state.document.page_mut(page_id).blocks;
        let target = index as i64 + direction as i64;
        if target < 0 || target as usize >= blocks.len() {
            return;
        }
        blocks.swap(index, target as usize);
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn duplicate_block(page_id: PageId, index: usize) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let blocks = &mut state.document.page_mut(page_id).blocks;
        if let Some(block) = blocks.get(index) {
            let copy = block.duplicated();
            blocks.insert(index + 1, copy);
        }
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn remove_block(page_id: PageId, index: usize) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let blocks = &mut state.document.page_mut(page_id).blocks;
        if index < blocks.len() {
            blocks.remove(index);
        }
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn set_block_art(page_id: PageId, block_id: &str, art_id: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(Block::Image { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            data.art = art_id.to_string();
            data.data.clear();
            data.shape = Shape::Square;
        }
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn clear_block_image(page_id: PageId, block_id: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(Block::Image { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            data.data.clear();
            data.art.clear();
        }
    });
    render_block_editor();
    render_pages();
    store_draft();
}

pub fn upload_block_image(page_id: PageId, block_id: String, file: File) {
    let content_type = file.type_();
    let ok_type = content_type == "image/jpeg" || content_type == "image/png" || content_type == "image/webp";
    if !ok_type || file.size() > 12.0 * 1024.0 * 1024.0 {
        show_status("Choose a JPEG, PNG, or WebP image smaller than 12 MB.", true);
        return;
    }
    show_status("Preparing image\u{2026}", false);
    wasm_bindgen_futures::spawn_local(async move {
        match canvas::resize_image_to_data_url(&file).await {
            Ok(data_url) => {
                STATE.with(|s| {
                    let mut state = s.borrow_mut();
                    if let Some(Block::Image { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
                        data.data = data_url;
                        data.art.clear();
                    }
                });
                render_block_editor();
                render_pages();
                store_draft();
                show_status("Image added.", false);
            }
            Err(_) => show_status("That image could not be read.", true),
        }
    });
}

pub fn set_decoration_style(page_id: PageId, block_id: &str, style: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(Block::Decoration { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            data.style = DecorationStyle::parse_or(style, data.style);
        }
    });
    render_block_editor();
    render_pages();
    schedule_save();
}

pub fn clear_decoration_image(page_id: PageId, block_id: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(Block::Decoration { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
            data.data.clear();
        }
    });
    render_block_editor();
    render_pages();
    store_draft();
}

pub fn upload_decoration_image(page_id: PageId, block_id: String, file: File) {
    let content_type = file.type_();
    let ok_type = content_type == "image/jpeg" || content_type == "image/png" || content_type == "image/webp";
    if !ok_type || file.size() > 12.0 * 1024.0 * 1024.0 {
        show_status("Choose a JPEG, PNG, or WebP image smaller than 12 MB.", true);
        return;
    }
    show_status("Preparing image\u{2026}", false);
    wasm_bindgen_futures::spawn_local(async move {
        match canvas::resize_decoration_image_to_data_url(&file).await {
            Ok(data_url) => {
                STATE.with(|s| {
                    let mut state = s.borrow_mut();
                    if let Some(Block::Decoration { data, .. }) = state.document.page_mut(page_id).blocks.iter_mut().find(|b| b.id() == block_id) {
                        data.data = data_url;
                    }
                });
                render_block_editor();
                render_pages();
                store_draft();
                show_status("Decoration added.", false);
            }
            Err(_) => show_status("That image could not be read.", true),
        }
    });
}

// ---------------------------------------------------------------------------
// Top-level actions (mirrors the bottom event-listener block in app.js)
// ---------------------------------------------------------------------------

pub fn add_block(kind: BlockKind) {
    let page_id = STATE.with(|s| {
        let mut state = s.borrow_mut();
        let page_id = state.selected_page;
        state.document.page_mut(page_id).blocks.push(kind.new_block());
        page_id
    });
    let _ = page_id;
    render_block_editor();
    render_pages();
    schedule_save();
    if let Some(list) = el("block-list") {
        if let Some(last) = list.last_element_child() {
            last.scroll_into_view();
        }
    }
}

pub fn apply_template(key: &str) {
    let Some(pages) = content_template(key) else { return };
    let confirmed = window()
        .confirm_with_message(&format!("Replace the content on all four pages with the \"{}\" template? Your current text and images will be lost.", template_label(key)))
        .unwrap_or(false);
    if !confirmed {
        return;
    }
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.document.pages = pages;
        state.selected_page = PageId::Front;
    });
    render_all();
    schedule_save();
    show_status("Template applied.", false);
}

fn template_label(key: &str) -> &'static str {
    match key {
        "child-same-day" => "Child baptism + confirmation",
        "child-later" => "Child baptism only (confirmed later)",
        "convert" => "Convert baptism",
        "multiple" => "Multiple candidates",
        _ => "",
    }
}

pub fn apply_theme_preset(name: &str) {
    if name == "custom" {
        return;
    }
    let Some(theme) = theme_preset(name) else { return };
    STATE.with(|s| s.borrow_mut().document.theme = theme);
    update_theme_inputs();
    render_pages();
    schedule_save();
}

pub fn set_theme_paper(value: String) {
    STATE.with(|s| s.borrow_mut().document.theme.paper = value);
    mark_custom_and_rerender();
}

pub fn set_theme_text(value: String) {
    STATE.with(|s| s.borrow_mut().document.theme.text = value);
    mark_custom_and_rerender();
}

pub fn set_theme_accent(value: String) {
    STATE.with(|s| s.borrow_mut().document.theme.accent = value);
    mark_custom_and_rerender();
}

pub fn set_theme_font(value: String) {
    STATE.with(|s| s.borrow_mut().document.theme.font = FontFamily::parse_or(&value, FontFamily::Serif));
    mark_custom_and_rerender();
}

pub fn set_monochrome(checked: bool) {
    STATE.with(|s| s.borrow_mut().document.theme.monochrome = checked);
    mark_custom_and_rerender();
}

fn mark_custom_and_rerender() {
    if let Some(preset) = select("theme-preset") {
        preset.set_value("custom");
    }
    render_pages();
    schedule_save();
}

pub fn reset_theme() {
    let classic = theme_preset("classic").unwrap();
    STATE.with(|s| s.borrow_mut().document.theme = classic);
    update_theme_inputs();
    render_pages();
    schedule_save();
}

pub fn toggle_focus() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.focus_mode = !state.focus_mode;
    });
    update_focus_mode();
}

pub fn copy_current_page() {
    let (page, label) = STATE.with(|s| {
        let state = s.borrow();
        (state.document.page(state.selected_page).clone(), state.selected_page.label())
    });
    wasm_bindgen_futures::spawn_local(async move {
        crate::clipboard::copy_page(&page).await;
        show_status(&format!("{label} copied. You can paste it into another program window."), false);
    });
}

pub fn paste_current_page() {
    wasm_bindgen_futures::spawn_local(async move {
        let Some(payload) = crate::clipboard::read_page_clipboard().await else {
            show_status("Copy a program page first, then paste it here.", true);
            return;
        };
        let page_id = selected_page();
        let label = page_id.label();
        let has_blocks = STATE.with(|s| !s.borrow().document.page(page_id).blocks.is_empty());
        if has_blocks {
            let confirmed = window()
                .confirm_with_message(&format!("Replace every element on {label} with the copied page?"))
                .unwrap_or(false);
            if !confirmed {
                return;
            }
        }
        let blocks = crate::clipboard::blocks_from_page_payload(&payload);
        STATE.with(|s| {
            s.borrow_mut().document.page_mut(page_id).blocks = blocks;
        });
        render_block_editor();
        render_pages();
        schedule_save();
        show_status(&format!("Copied page pasted onto {label}."), false);
    });
}

pub fn copy_block_element(page_id: PageId, block_id: &str) {
    let block = STATE.with(|s| {
        let state = s.borrow();
        state.document.page(page_id).blocks.iter().find(|b| b.id() == block_id).cloned()
    });
    let Some(block) = block else { return };
    let label = block.kind().label();
    wasm_bindgen_futures::spawn_local(async move {
        crate::clipboard::copy_block(&block).await;
        show_status(&format!("{label} copied. You can paste it into another element list."), false);
    });
}

pub fn paste_block_element(page_id: PageId, block_id: &str) {
    let block_id = block_id.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        let Some(payload) = crate::clipboard::read_block_clipboard().await else {
            show_status("Copy an element first, then paste it here.", true);
            return;
        };
        let Some(block) = crate::clipboard::block_from_payload(&payload) else {
            show_status("Clipboard does not contain a supported element.", true);
            return;
        };
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            let blocks = &mut state.document.page_mut(page_id).blocks;
            let insert_at = blocks.iter().position(|b| b.id() == block_id).map(|i| i + 1).unwrap_or(blocks.len());
            blocks.insert(insert_at, block);
        });
        render_block_editor();
        render_pages();
        schedule_save();
        show_status("Element pasted.", false);
    });
}

fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_dash = false;
    for c in text.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() { "baptism-program".to_string() } else { trimmed.to_string() }
}

pub fn download_save() {
    let (json, filename) = STATE.with(|s| {
        let state = s.borrow();
        let saved_at = String::from(js_sys::Date::new_0().to_iso_string());
        let save = serde_json::json!({
            "format": "baptism-program",
            "version": 2,
            "savedAt": saved_at,
            "document": document_to_json(&state.document),
        });
        let heading_text = state
            .document
            .page(PageId::Front)
            .blocks
            .iter()
            .find_map(|b| if let Block::Heading { data, .. } = b { Some(data.text.clone()) } else { None })
            .unwrap_or_else(|| "baptism-program".to_string());
        (save.to_string(), format!("{}.baptism.json", slugify(&heading_text)))
    });

    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(&json));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/json");
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&array, &options) else { return };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };

    if let Ok(link) = document().create_element("a") {
        if let Ok(link) = link.dyn_into::<HtmlAnchorElement>() {
            link.set_href(&url);
            link.set_download(&filename);
            link.click();
        }
    }
    let url_for_cleanup = url;
    let closure = Closure::<dyn FnMut()>::new(move || {
        web_sys::Url::revoke_object_url(&url_for_cleanup).ok();
    });
    window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 0).ok();
    closure.forget();
    show_status("Save downloaded.", false);
}

enum SaveParseError {
    InvalidJson,
    Unsupported,
}

/// Parses a `{format, version, ...}` save payload (from a file or the
/// clipboard) into a document, mirroring the `sanitizeDocument`/
/// `legacyDocument` dispatch in the load-save handler.
fn parse_save_text(text: &str) -> Result<(Document_, bool), SaveParseError> {
    let parsed: serde_json::Value = serde_json::from_str(text).map_err(|_| SaveParseError::InvalidJson)?;
    let format_ok = parsed.get("format").and_then(|v| v.as_str()) == Some("baptism-program");
    let version = parsed.get("version").and_then(|v| v.as_i64());
    let loaded = match (format_ok, version) {
        (true, Some(2)) => parsed.get("document").map(sanitize_document),
        (true, Some(1)) => Some(legacy_document(parsed.get("fields").unwrap_or(&serde_json::Value::Null))),
        _ => None,
    };
    loaded.map(|doc| (doc, version == Some(1))).ok_or(SaveParseError::Unsupported)
}

fn apply_loaded_document(document: Document_) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.document = document;
        state.selected_page = PageId::Front;
    });
    update_theme_inputs();
    render_all();
    store_draft();
}

pub fn load_save(file: File) {
    if file.size() > 20.0 * 1024.0 * 1024.0 {
        show_status("That save is larger than the 20 MB limit.", true);
        return;
    }
    wasm_bindgen_futures::spawn_local(async move {
        let blob: &web_sys::Blob = file.unchecked_ref();
        let text = match wasm_bindgen_futures::JsFuture::from(blob.text()).await {
            Ok(value) => value.as_string().unwrap_or_default(),
            Err(_) => {
                show_status("The selected file could not be read.", true);
                return;
            }
        };
        match parse_save_text(&text) {
            Ok((document, is_legacy)) => {
                apply_loaded_document(document);
                show_status(if is_legacy { "Older save loaded and upgraded." } else { "Save loaded." }, false);
            }
            Err(SaveParseError::InvalidJson) => show_status("The selected file is not valid JSON.", true),
            Err(SaveParseError::Unsupported) => show_status("This is not a supported baptism program save.", true),
        }
    });
}

fn build_save_json() -> String {
    STATE.with(|s| {
        let state = s.borrow();
        let saved_at = String::from(js_sys::Date::new_0().to_iso_string());
        serde_json::json!({
            "format": "baptism-program",
            "version": 2,
            "savedAt": saved_at,
            "document": document_to_json(&state.document),
        })
        .to_string()
    })
}

pub fn copy_save() {
    let json = build_save_json();
    wasm_bindgen_futures::spawn_local(async move {
        match crate::clipboard::write_system_clipboard(&json).await {
            Ok(()) => show_status("Save copied to clipboard.", false),
            Err(_) => show_status("Could not copy the save to the clipboard.", true),
        }
    });
}

pub fn load_save_from_clipboard() {
    wasm_bindgen_futures::spawn_local(async move {
        let text = match crate::clipboard::read_system_clipboard().await {
            Ok(text) => text,
            Err(_) => {
                show_status("Could not read the clipboard.", true);
                return;
            }
        };
        match parse_save_text(&text) {
            Ok((document, is_legacy)) => {
                apply_loaded_document(document);
                show_status(if is_legacy { "Older save loaded and upgraded." } else { "Save loaded from clipboard." }, false);
            }
            Err(SaveParseError::InvalidJson) => show_status("Clipboard contents are not valid JSON.", true),
            Err(SaveParseError::Unsupported) => show_status("Clipboard does not contain a supported baptism program save.", true),
        }
    });
}

pub fn print_program() {
    if let Some(body) = document().body() {
        body.class_list().remove_1("print-mode-sample").ok();
    }
    if let Some(style) = el("sample-page-size-override") {
        style.remove();
    }
    window().print().ok();
}

pub fn print_sample() {
    let style = document().create_element("style").expect("style element");
    style.set_id("sample-page-size-override");
    style.set_text_content(Some("@page { size: 5.5in 8.5in; margin: 0; }"));
    if let Some(head) = document().head() {
        head.append_child(&style).ok();
    }
    let Some(body) = document().body() else { return };
    body.class_list().add_1("print-mode-sample").ok();

    let options = web_sys::AddEventListenerOptions::new();
    options.set_once(true);
    let cleanup = Closure::<dyn FnMut()>::new(move || {
        if let Some(body) = document().body() {
            body.class_list().remove_1("print-mode-sample").ok();
        }
        if let Some(style) = el("sample-page-size-override") {
            style.remove();
        }
    });
    window().add_event_listener_with_callback_and_add_event_listener_options("afterprint", cleanup.as_ref().unchecked_ref(), &options).ok();
    cleanup.forget();

    window().print().ok();
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn attach_click(id: &str, handler: impl Fn() + 'static) {
    let Some(element) = el(id) else { return };
    let closure = Closure::<dyn FnMut()>::new(handler);
    element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
    closure.forget();
}

fn attach_input(id: &str, event: &str, handler: impl Fn(String) + 'static) {
    let Some(element) = el(id) else { return };
    let target = element.clone();
    let closure = Closure::<dyn FnMut()>::new(move || {
        let value = js_sys::Reflect::get(&target, &JsValue::from_str("value")).ok().and_then(|v| v.as_string()).unwrap_or_default();
        handler(value);
    });
    element.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref()).ok();
    closure.forget();
}

fn attach_checkbox_change(id: &str, handler: impl Fn(bool) + 'static) {
    let Some(element) = el(id) else { return };
    let target = element.clone();
    let closure = Closure::<dyn FnMut()>::new(move || {
        let checked = js_sys::Reflect::get(&target, &JsValue::from_str("checked")).ok().and_then(|v| v.as_bool()).unwrap_or(false);
        handler(checked);
    });
    element.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).ok();
    closure.forget();
}

pub fn init() {
    attach_click("add-block", || {
        let kind = select("new-block-type").and_then(|s| BlockKind::parse(&s.value())).unwrap_or(BlockKind::Heading);
        add_block(kind);
    });

    attach_input("content-template", "change", |value| {
        if let Some(s) = select("content-template") {
            s.set_value("");
        }
        if !value.is_empty() {
            apply_template(&value);
        }
    });

    attach_input("theme-preset", "change", |value| apply_theme_preset(&value));
    attach_input("paper-color", "input", set_theme_paper);
    attach_input("text-color", "input", set_theme_text);
    attach_input("accent-color", "input", set_theme_accent);
    attach_input("font-family", "change", set_theme_font);
    attach_checkbox_change("monochrome-images", set_monochrome);
    attach_click("reset-theme", reset_theme);
    attach_click("download-save", download_save);
    attach_click("copy-save", copy_save);
    attach_click("load-save-clipboard", load_save_from_clipboard);

    attach_click("print-program", print_program);
    attach_click("print-sample", print_sample);
    attach_click("toggle-focus", toggle_focus);
    attach_click("zoom-out", zoom_out);
    attach_click("zoom-in", zoom_in);
    attach_click("copy-page", copy_current_page);
    attach_click("paste-page", paste_current_page);

    if let Some(file_input) = input("load-save") {
        let handle = file_input.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            if let Some(files) = handle.files() {
                if let Some(file) = files.get(0) {
                    load_save(file);
                }
            }
            handle.set_value("");
        });
        file_input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    update_theme_inputs();
    update_focus_mode();
    render_all();
}
