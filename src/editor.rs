use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use crate::model::*;
use crate::render::{make, make_text};
use crate::state;

// ---------------------------------------------------------------------------
// Generic field controls (mirrors labelControl/inputControl/selectControl/...)
// ---------------------------------------------------------------------------

pub fn label_control(doc: &Document, text: &str, control: &HtmlElement) -> HtmlElement {
    let label = make(doc, "label", "field-control");
    label.append_child(&make_text(doc, "span", "", text)).ok();
    label.append_child(control).ok();
    label
}

pub fn make_row(doc: &Document, children: Vec<HtmlElement>) -> HtmlElement {
    let row = make(doc, "div", "control-row");
    for child in children {
        row.append_child(&child).ok();
    }
    row
}

pub fn input_control(doc: &Document, value: &str, on_input: impl Fn(String) + 'static) -> HtmlElement {
    let input: HtmlInputElement = doc.create_element("input").unwrap().dyn_into().unwrap();
    input.set_value(value);
    let handle = input.clone();
    let closure = Closure::<dyn FnMut()>::new(move || on_input(handle.value()));
    input.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    input.dyn_into().unwrap()
}

pub fn textarea_control(doc: &Document, value: &str, rows: u32, on_input: impl Fn(String) + 'static) -> HtmlElement {
    let textarea: HtmlTextAreaElement = doc.create_element("textarea").unwrap().dyn_into().unwrap();
    textarea.set_rows(rows);
    textarea.set_value(value);
    let handle = textarea.clone();
    let closure = Closure::<dyn FnMut()>::new(move || on_input(handle.value()));
    textarea.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    textarea.dyn_into().unwrap()
}

pub fn select_control(doc: &Document, value: &str, options: &[(&str, &str)], on_change: impl Fn(String) + 'static) -> HtmlElement {
    let select: HtmlSelectElement = doc.create_element("select").unwrap().dyn_into().unwrap();
    for (option_value, label) in options {
        let option = doc.create_element("option").unwrap();
        option.set_text_content(Some(label));
        option.set_attribute("value", option_value).ok();
        select.append_child(&option).ok();
    }
    select.set_value(value);
    let handle = select.clone();
    let closure = Closure::<dyn FnMut()>::new(move || on_change(handle.value()));
    select.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    select.dyn_into().unwrap()
}

pub fn color_override_control(doc: &Document, value: &str, on_change: impl Fn(String) + 'static, on_clear: impl Fn() + 'static) -> HtmlElement {
    let wrap = make(doc, "div", "typography-color-control");

    let input: HtmlInputElement = doc.create_element("input").unwrap().dyn_into().unwrap();
    input.set_attribute("type", "color").ok();
    input.set_value(if value.is_empty() { "#000000" } else { value });
    let input_for_input = input.clone();
    let oninput = Closure::<dyn FnMut()>::new(move || on_change(input_for_input.value()));
    input.set_oninput(Some(oninput.as_ref().unchecked_ref()));
    oninput.forget();

    let clear: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
    clear.set_class_name("quiet-button");
    clear.set_text_content(Some("Use theme color"));
    clear.set_attribute("type", "button").ok();
    let input_for_clear = input.clone();
    let onclick = Closure::<dyn FnMut()>::new(move || {
        input_for_clear.set_value("#000000");
        on_clear();
    });
    clear.set_onclick(Some(onclick.as_ref().unchecked_ref()));
    onclick.forget();

    wrap.append_child(&input).ok();
    wrap.append_child(&clear).ok();
    wrap
}

/// Wraps a per-block-type field mutator into a change handler that writes
/// through to global state and triggers a live-preview re-render. Mirrors
/// `changeBlock(item, property, value)`.
fn on_field(page_id: PageId, block_id: &str, mutate: impl Fn(&mut Block, String) + 'static) -> impl Fn(String) + 'static {
    let block_id = block_id.to_string();
    move |value: String| {
        state::update_block_field(page_id, &block_id, |b| mutate(b, value));
    }
}

fn on_typography_field(page_id: PageId, block_id: &str, mutate: impl Fn(&mut Typography, String) + 'static) -> impl Fn(String) + 'static {
    let block_id = block_id.to_string();
    move |value: String| {
        state::update_typography_field(page_id, &block_id, |t| mutate(t, value));
    }
}

fn render_typography_controls(doc: &Document, card: &Element, page_id: PageId, block_id: &str, typography: &Typography, note: Option<&str>) {
    let details = doc.create_element("details").unwrap();
    details.set_class_name("advanced-style");
    details.append_child(&make_text(doc, "summary", "", "More styling: bold, italic, color, font")).ok();
    if let Some(note) = note {
        details.append_child(&make_text(doc, "p", "advanced-style-note", note)).ok();
    }

    details
        .append_child(&make_row(doc, vec![
            label_control(doc, "Weight", &select_control(
                doc, typography.weight.as_str(),
                &[("default", "Default"), ("bold", "Bold"), ("regular", "Regular")],
                on_typography_field(page_id, block_id, |t, v| t.weight = Weight::parse_or(&v, Weight::Default)),
            )),
            label_control(doc, "Slant", &select_control(
                doc, typography.slant.as_str(),
                &[("default", "Default"), ("italic", "Italic"), ("regular", "Upright")],
                on_typography_field(page_id, block_id, |t, v| t.slant = Slant::parse_or(&v, Slant::Default)),
            )),
        ]))
        .ok();

    details
        .append_child(&label_control(doc, "Font", &select_control(
            doc, typography.font.as_str(),
            &[("default", "Match page font"), ("serif", "Classic serif"), ("sans", "Clean sans serif"), ("soft", "Soft rounded")],
            on_typography_field(page_id, block_id, |t, v| t.font = FontOverride::parse_or(&v, FontOverride::Default)),
        )))
        .ok();

    let color_control = color_override_control(
        doc,
        &typography.color,
        on_typography_field(page_id, block_id, |t, v| t.color = v),
        {
            let block_id = block_id.to_string();
            move || state::update_typography_field(page_id, &block_id, |t| t.color = String::new())
        },
    );
    details.append_child(&label_control(doc, "Color", &color_control)).ok();

    card.append_child(&details).ok();
}

// ---------------------------------------------------------------------------
// Per-block-type field editors (mirrors renderBlockFields)
// ---------------------------------------------------------------------------

const SIZE_OPTIONS: [(&str, &str); 3] = [("small", "Small"), ("medium", "Medium"), ("large", "Large")];
const ALIGN_OPTIONS: [(&str, &str); 3] = [("left", "Left"), ("center", "Center"), ("right", "Right")];

fn render_block_fields(doc: &Document, card: &Element, page_id: PageId, block: &Block) {
    let id = block.id();
    match block {
        Block::Heading { data, typography, .. } => {
            card.append_child(&label_control(doc, "Text", &input_control(doc, &data.text, on_field(page_id, id, |b, v| if let Block::Heading { data, .. } = b { data.text = v; })))).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Heading { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
                label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Heading { data, .. } = b { data.align = Align::parse_or(&v, data.align); }))),
            ])).ok();
            card.append_child(&label_control(doc, "Color", &select_control(doc, data.color.as_str(), &[("ink", "Ink (matches text)"), ("accent", "Accent color")], on_field(page_id, id, |b, v| if let Block::Heading { data, .. } = b { data.color = HeadingColor::parse_or(&v, data.color); })))).ok();
            render_typography_controls(doc, card, page_id, id, typography, None);
        }
        Block::Text { data, typography, .. } => {
            card.append_child(&label_control(doc, "Text", &textarea_control(doc, &data.text, 3, on_field(page_id, id, |b, v| if let Block::Text { data, .. } = b { data.text = v; })))).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Style", &select_control(doc, data.style.as_str(), &[("normal", "Normal"), ("italic", "Italic"), ("eyebrow", "Small caps")], on_field(page_id, id, |b, v| if let Block::Text { data, .. } = b { data.style = TextStyle::parse_or(&v, data.style); }))),
                label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Text { data, .. } = b { data.align = Align::parse_or(&v, data.align); }))),
            ])).ok();
            render_typography_controls(doc, card, page_id, id, typography, None);
        }
        Block::Item { data, typography, .. } => {
            card.append_child(&label_control(doc, "Label", &input_control(doc, &data.label, on_field(page_id, id, |b, v| if let Block::Item { data, .. } = b { data.label = v; })))).ok();
            card.append_child(&label_control(doc, "Name or details", &input_control(doc, &data.text, on_field(page_id, id, |b, v| if let Block::Item { data, .. } = b { data.text = v; })))).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Line style", &select_control(doc, data.style.as_str(), &[("underline", "Underline"), ("dotted", "Dotted leader"), ("plain", "Simple, no line"), ("none", "No line")], on_field(page_id, id, |b, v| if let Block::Item { data, .. } = b { data.style = ItemLineStyle::parse_or(&v, data.style); }))),
                label_control(doc, "Spacing", &select_control(doc, data.size.as_str(), &[("compact", "Compact"), ("cozy", "Cozy"), ("roomy", "Roomy")], on_field(page_id, id, |b, v| if let Block::Item { data, .. } = b { data.size = ItemSize::parse_or(&v, data.size); }))),
            ])).ok();
            card.append_child(&label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Item { data, .. } = b { data.align = Align::parse_or(&v, data.align); })))).ok();
            render_typography_controls(doc, card, page_id, id, typography, Some("Applies to the name or details text (not the label)."));
        }
        Block::Callout { data, typography, .. } => {
            card.append_child(&label_control(doc, "Title", &input_control(doc, &data.title, on_field(page_id, id, |b, v| if let Block::Callout { data, .. } = b { data.title = v; })))).ok();
            card.append_child(&label_control(doc, "Subtitle", &input_control(doc, &data.subtitle, on_field(page_id, id, |b, v| if let Block::Callout { data, .. } = b { data.subtitle = v; })))).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Callout { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
                label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Callout { data, .. } = b { data.align = Align::parse_or(&v, data.align); }))),
            ])).ok();
            render_typography_controls(doc, card, page_id, id, typography, Some("Applies to the title (not the subtitle)."));
        }
        Block::Hymn { data, typography, .. } => {
            card.append_child(&label_control(doc, "Title", &input_control(doc, &data.title, on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.title = v; })))).ok();
            card.append_child(&label_control(doc, "Lyrics", &textarea_control(doc, &data.lyrics, 10, on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.lyrics = v; })))).ok();
            card.append_child(&make_text(doc, "p", "field-hint", "Leave a blank line between verses so they can wrap or split into columns cleanly.")).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
                label_control(doc, "Title align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.align = Align::parse_or(&v, data.align); }))),
            ])).ok();
            card.append_child(&label_control(doc, "Lyrics align", &select_control(doc, data.lyrics_align.as_str(), &[("left", "Left (recommended)"), ("center", "Center"), ("right", "Right")], on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.lyrics_align = Align::parse_or(&v, data.lyrics_align); })))).ok();
            card.append_child(&make_text(doc, "p", "field-hint", "Center or right lyrics can look ragged once a line wraps onto a second line — left works best for most hymns.")).ok();
            card.append_child(&label_control(doc, "Layout", &select_control(doc, data.columns.as_str(), &[("1", "One column"), ("2", "Two columns (fits more per page)")], on_field(page_id, id, |b, v| if let Block::Hymn { data, .. } = b { data.columns = Columns::parse_or(&v, data.columns); })))).ok();
            render_typography_controls(doc, card, page_id, id, typography, Some("Applies to the lyrics (not the title)."));
        }
        Block::Quote { data, typography, .. } => {
            card.append_child(&label_control(doc, "Passage", &textarea_control(doc, &data.text, 5, on_field(page_id, id, |b, v| if let Block::Quote { data, .. } = b { data.text = v; })))).ok();
            card.append_child(&label_control(doc, "Reference or attribution", &input_control(doc, &data.citation, on_field(page_id, id, |b, v| if let Block::Quote { data, .. } = b { data.citation = v; })))).ok();
            card.append_child(&make_row(doc, vec![
                label_control(doc, "Type", &select_control(doc, data.kind.as_str(), &[("scripture", "Scripture"), ("quote", "Quotation")], on_field(page_id, id, |b, v| if let Block::Quote { data, .. } = b { data.kind = QuoteKind::parse_or(&v, data.kind); }))),
                label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Quote { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
            ])).ok();
            card.append_child(&label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Quote { data, .. } = b { data.align = Align::parse_or(&v, data.align); })))).ok();
            render_typography_controls(doc, card, page_id, id, typography, Some("Applies to the passage (not the reference)."));
        }
        Block::Markdown { data, typography, .. } => {
            card.append_child(&label_control(doc, "Text", &textarea_control(doc, &data.text, 6, on_field(page_id, id, |b, v| if let Block::Markdown { data, .. } = b { data.text = v; })))).ok();
            card.append_child(&make_text(doc, "p", "field-hint", "Supports **bold**, *italic*, and blank lines for new paragraphs. For a list, put every item on its own line starting with \"- \" (or \"1. \"), separated from other text by a blank line.")).ok();
            card.append_child(&label_control(doc, "Align", &select_control(doc, data.align.as_str(), &ALIGN_OPTIONS, on_field(page_id, id, |b, v| if let Block::Markdown { data, .. } = b { data.align = Align::parse_or(&v, data.align); })))).ok();
            render_typography_controls(doc, card, page_id, id, typography, None);
        }
        Block::Image { data, .. } => render_image_fields(doc, card, page_id, id, data),
        Block::Decoration { data, .. } => render_decoration_fields(doc, card, page_id, id, data),
        Block::Spacer { data, .. } => {
            card.append_child(&label_control(doc, "Amount", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Spacer { data, .. } = b { data.size = Size::parse_or(&v, data.size); })))).ok();
        }
    }
}

fn render_decoration_fields(doc: &Document, card: &Element, page_id: PageId, id: &str, data: &DecorationBlock) {
    let block_id = id.to_string();
    card.append_child(&make_row(doc, vec![
        label_control(doc, "Symbol", &select_control(doc, data.style.as_str(), &DecorationStyle::ALL.map(|s| (s.as_str(), s.label())), {
            let block_id = block_id.clone();
            move |v| state::set_decoration_style(page_id, &block_id, &v)
        })),
        label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Decoration { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
    ])).ok();

    if data.style != DecorationStyle::Custom {
        return;
    }

    if is_image_data_url(&data.data) {
        let thumbnail = make(doc, "img", "editor-thumbnail is-built-in");
        thumbnail.set_attribute("src", &data.data).ok();
        thumbnail.set_attribute("alt", "Custom decoration").ok();
        card.append_child(&thumbnail).ok();
    }

    let actions = make(doc, "div", "image-actions");
    let picker = make(doc, "label", "image-picker");
    picker.set_text_content(Some(if data.data.is_empty() { "Upload your own" } else { "Replace image" }));
    let file_input: HtmlInputElement = doc.create_element("input").unwrap().dyn_into().unwrap();
    file_input.set_type("file");
    file_input.set_accept("image/jpeg,image/png,image/webp");
    file_input.set_hidden(true);
    let file_input_handle = file_input.clone();
    let block_id_for_upload = block_id.clone();
    let onchange = Closure::<dyn FnMut()>::new(move || {
        if let Some(files) = file_input_handle.files() {
            if let Some(file) = files.get(0) {
                state::upload_decoration_image(page_id, block_id_for_upload.clone(), file);
            }
        }
        file_input_handle.set_value("");
    });
    file_input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    onchange.forget();
    picker.append_child(&file_input).ok();
    actions.append_child(&picker).ok();

    if !data.data.is_empty() {
        let clear: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
        clear.set_class_name("quiet-button danger");
        clear.set_text_content(Some("Clear image"));
        clear.set_attribute("type", "button").ok();
        let block_id_for_clear = block_id.clone();
        let onclick = Closure::<dyn FnMut()>::new(move || state::clear_decoration_image(page_id, &block_id_for_clear));
        clear.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();
        actions.append_child(&clear).ok();
    }
    card.append_child(&actions).ok();
    card.append_child(&make_text(doc, "p", "field-hint", "A small PNG with a transparent background works best, like a simple line drawing or symbol.")).ok();
}

fn render_image_fields(doc: &Document, card: &Element, page_id: PageId, id: &str, data: &ImageBlock) {
    let id_string = id.to_string();
    let artwork = find_art(&data.art);
    let source: String = match artwork {
        Some(art) => art.src.to_string(),
        None if is_image_data_url(&data.data) => data.data.clone(),
        None => String::new(),
    };

    if !source.is_empty() {
        let thumbnail = make(doc, "img", if artwork.is_some() { "editor-thumbnail is-built-in" } else { "editor-thumbnail" });
        thumbnail.set_attribute("src", &source).ok();
        let alt = if !data.caption.is_empty() { data.caption.as_str() } else { artwork.map(|a| a.alt).unwrap_or("Selected image") };
        thumbnail.set_attribute("alt", alt).ok();
        card.append_child(&thumbnail).ok();
    }

    card.append_child(&make_text(doc, "span", "art-library-title", "Built-in art")).ok();
    for group in ArtGroup::ALL {
        card.append_child(&make_text(doc, "span", "art-library-group-title", group.label())).ok();
        let library = make(doc, "div", "art-library");
        for art in BUILT_IN_ART.iter().filter(|a| a.group == group) {
            let selected = data.art == art.id;
            let choice: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
            choice.set_class_name(if selected { "art-choice selected" } else { "art-choice" });
            choice.set_attribute("type", "button").ok();
            choice.set_attribute("aria-pressed", if selected { "true" } else { "false" }).ok();
            let img = make(doc, "img", "");
            img.set_attribute("src", art.src).ok();
            img.set_attribute("alt", "").ok();
            choice.append_child(&img).ok();
            choice.append_child(&make_text(doc, "span", "", art.name)).ok();

            let art_id = art.id.to_string();
            let block_id = id_string.clone();
            let onclick = Closure::<dyn FnMut()>::new(move || state::set_block_art(page_id, &block_id, &art_id));
            choice.set_onclick(Some(onclick.as_ref().unchecked_ref()));
            onclick.forget();

            library.append_child(&choice).ok();
        }
        card.append_child(&library).ok();

        match group {
            ArtGroup::Lds => {
                let credit = make(doc, "p", "art-library-credit");
                credit.set_text_content(Some("Independently sourced LDS imagery \u{b7} "));
                let link = doc.create_element("a").unwrap();
                link.set_attribute("href", "static/art/ATTRIBUTION.md").ok();
                link.set_attribute("target", "_blank").ok();
                link.set_text_content(Some("licenses and sources"));
                credit.append_child(&link).ok();
                credit.append_child(&doc.create_text_node(" \u{b7} not an official Church product")).ok();
                card.append_child(&credit).ok();
            }
            ArtGroup::Human => {
                let credit = make(doc, "p", "art-library-credit");
                credit.set_text_content(Some("Drawn by Jim Padgett \u{b7} "));
                let license = doc.create_element("a").unwrap();
                license.set_attribute("href", "https://creativecommons.org/licenses/by-sa/3.0/").ok();
                license.set_attribute("target", "_blank").ok();
                license.set_attribute("rel", "noreferrer").ok();
                license.set_text_content(Some("CC BY-SA 3.0"));
                credit.append_child(&license).ok();
                credit.append_child(&doc.create_text_node(" \u{b7} ")).ok();
                let details = doc.create_element("a").unwrap();
                details.set_attribute("href", "static/art/ATTRIBUTION.md").ok();
                details.set_attribute("target", "_blank").ok();
                details.set_text_content(Some("sources"));
                credit.append_child(&details).ok();
                card.append_child(&credit).ok();
            }
            ArtGroup::Ai => {
                card.append_child(&make_text(doc, "p", "art-library-credit", "Created with AI for this project.")).ok();
            }
        }
    }

    let actions = make(doc, "div", "image-actions");
    let picker = make(doc, "label", "image-picker");
    picker.set_text_content(Some("Upload your own"));
    let file_input: HtmlInputElement = doc.create_element("input").unwrap().dyn_into().unwrap();
    file_input.set_type("file");
    file_input.set_accept("image/jpeg,image/png,image/webp");
    file_input.set_hidden(true);
    let id_for_upload = id.to_string();
    let file_input_handle = file_input.clone();
    let onchange = Closure::<dyn FnMut()>::new(move || {
        if let Some(files) = file_input_handle.files() {
            if let Some(file) = files.get(0) {
                state::upload_block_image(page_id, id_for_upload.clone(), file);
            }
        }
        file_input_handle.set_value("");
    });
    file_input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    onchange.forget();
    picker.append_child(&file_input).ok();
    actions.append_child(&picker).ok();

    if !source.is_empty() {
        let clear: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
        clear.set_class_name("quiet-button danger");
        clear.set_text_content(Some("Clear image"));
        clear.set_attribute("type", "button").ok();
        let id_for_clear = id.to_string();
        let onclick = Closure::<dyn FnMut()>::new(move || state::clear_block_image(page_id, &id_for_clear));
        clear.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();
        actions.append_child(&clear).ok();
    }
    card.append_child(&actions).ok();

    card.append_child(&make_row(doc, vec![
        label_control(doc, "Size", &select_control(doc, data.size.as_str(), &SIZE_OPTIONS, on_field(page_id, id, |b, v| if let Block::Image { data, .. } = b { data.size = Size::parse_or(&v, data.size); }))),
        label_control(doc, "Shape", &select_control(doc, data.shape.as_str(), &[("square", "Square"), ("soft", "Rounded"), ("circle", "Circle")], on_field(page_id, id, |b, v| if let Block::Image { data, .. } = b { data.shape = Shape::parse_or(&v, data.shape); }))),
    ])).ok();
    card.append_child(&label_control(doc, "Caption", &input_control(doc, &data.caption, on_field(page_id, id, |b, v| if let Block::Image { data, .. } = b { data.caption = v; })))).ok();
}

// ---------------------------------------------------------------------------
// Block list (mirrors renderBlockEditor)
// ---------------------------------------------------------------------------

fn icon_button(doc: &Document, label: &str, title: &str, disabled: bool, class: &str, on_click: impl Fn() + 'static) -> HtmlElement {
    let button: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
    let class_name = if class.is_empty() { "icon-button".to_string() } else { format!("icon-button {class}") };
    button.set_class_name(&class_name);
    button.set_text_content(Some(label));
    button.set_attribute("type", "button").ok();
    button.set_attribute("title", title).ok();
    button.set_attribute("aria-label", title).ok();
    button.set_disabled(disabled);
    let closure = Closure::<dyn FnMut()>::new(move || on_click());
    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    button.dyn_into().unwrap()
}

pub fn render_block_editor(doc: &Document, page_id: PageId, page: &Page) {
    if let Some(label_el) = doc.get_element_by_id("current-page-label") {
        label_el.set_text_content(Some(page_id.label()));
    }
    if let Some(count_el) = doc.get_element_by_id("element-count") {
        let count = page.blocks.len();
        count_el.set_text_content(Some(&format!("{count} element{}", if count == 1 { "" } else { "s" })));
    }
    let Some(list) = doc.get_element_by_id("block-list") else { return };
    list.set_inner_html("");

    if page.blocks.is_empty() {
        list.append_child(&make_text(doc, "div", "empty-page", "This page is empty. Add an element below.")).ok();
    }

    let total = page.blocks.len();
    for (index, block) in page.blocks.iter().enumerate() {
        let card = make(doc, "article", "block-card");
        card.set_attribute("data-block-id", block.id()).ok();

        let header = make(doc, "header", "block-card-header");
        let title = make(doc, "div", "block-card-title");
        title.append_child(&make_text(doc, "span", "link-number", &(index + 1).to_string())).ok();
        title.append_child(&make_text(doc, "strong", "", block.kind().label())).ok();
        header.append_child(&title).ok();

        let actions = make(doc, "div", "block-actions");
        actions.append_child(&icon_button(doc, "\u{2191}", "Move up", index == 0, "", { let id = page_id; move || state::move_block(id, index, -1) })).ok();
        actions.append_child(&icon_button(doc, "\u{2193}", "Move down", index + 1 == total, "", { let id = page_id; move || state::move_block(id, index, 1) })).ok();
        actions.append_child(&icon_button(doc, "\u{29c9}", "Duplicate", false, "", { let id = page_id; move || state::duplicate_block(id, index) })).ok();
        actions.append_child(&icon_button(doc, "\u{d7}", "Remove", false, "remove", { let id = page_id; move || state::remove_block(id, index) })).ok();
        header.append_child(&actions).ok();
        card.append_child(&header).ok();

        render_block_fields(doc, &card, page_id, block);
        wire_linked_highlight(&card, block.id());
        list.append_child(&card).ok();
    }
    state::apply_linked_highlight();
}

/// Mirrors the pointerenter/pointerleave/focusin/focusout wiring on each
/// block-card that links it to the matching numbered block in the live preview.
fn wire_linked_highlight(card: &Element, block_id: &str) {
    let id_for_enter = block_id.to_string();
    let onenter = Closure::<dyn FnMut()>::new(move || state::set_linked_highlight(&id_for_enter));
    card.add_event_listener_with_callback("pointerenter", onenter.as_ref().unchecked_ref()).ok();
    onenter.forget();

    let card_for_leave = card.clone();
    let onleave = Closure::<dyn FnMut()>::new(move || {
        let active = crate::state::active_element_within(&card_for_leave);
        if !active {
            state::set_linked_highlight("");
        }
    });
    card.add_event_listener_with_callback("pointerleave", onleave.as_ref().unchecked_ref()).ok();
    onleave.forget();

    let id_for_focusin = block_id.to_string();
    let onfocusin = Closure::<dyn FnMut()>::new(move || state::set_linked_highlight(&id_for_focusin));
    card.add_event_listener_with_callback("focusin", onfocusin.as_ref().unchecked_ref()).ok();
    onfocusin.forget();

    let card_for_focusout = card.clone();
    let onfocusout = Closure::<dyn FnMut(web_sys::FocusEvent)>::new(move |event: web_sys::FocusEvent| {
        let related = event.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        if !card_for_focusout.contains(related.as_ref()) {
            state::set_linked_highlight("");
        }
    });
    card.add_event_listener_with_callback("focusout", onfocusout.as_ref().unchecked_ref()).ok();
    onfocusout.forget();
}

// ---------------------------------------------------------------------------
// Page tabs (mirrors renderTabs)
// ---------------------------------------------------------------------------

pub fn render_tabs(doc: &Document, selected: PageId) {
    let Some(container) = doc.get_element_by_id("page-tabs") else { return };
    container.set_inner_html("");

    for (index, page_id) in PageId::ALL.into_iter().enumerate() {
        let wrapper = make(doc, "div", "page-tab");
        wrapper.set_attribute("data-selected", if page_id == selected { "true" } else { "false" }).ok();

        let select: HtmlButtonElement = doc.create_element("button").unwrap().dyn_into().unwrap();
        select.set_class_name("page-tab-select");
        select.set_attribute("type", "button").ok();
        select.set_attribute("role", "tab").ok();
        select.set_attribute("data-page", page_id.as_str()).ok();
        select.set_attribute("aria-selected", if page_id == selected { "true" } else { "false" }).ok();
        select.append_child(&make_text(doc, "span", "page-number", &(index + 1).to_string())).ok();
        select.append_child(&make_text(doc, "span", "", page_id.label())).ok();
        let onclick = Closure::<dyn FnMut()>::new(move || state::select_page(page_id));
        select.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();

        let move_wrap = make(doc, "div", "page-tab-move");
        move_wrap.append_child(&icon_button(doc, "\u{25c0}", "Swap with previous page", index == 0, "", move || state::move_page(index, -1))).ok();
        move_wrap.append_child(&icon_button(doc, "\u{25b6}", "Swap with next page", index + 1 == PageId::ALL.len(), "", move || state::move_page(index, 1))).ok();

        wrapper.append_child(&select).ok();
        wrapper.append_child(&move_wrap).ok();
        container.append_child(&wrapper).ok();
    }
}
