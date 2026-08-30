use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

use crate::canvas::GrayscaleCache;
use crate::model::*;

pub fn make(doc: &Document, tag: &str, class: &str) -> HtmlElement {
    let el = doc.create_element(tag).expect("create element").dyn_into::<HtmlElement>().expect("html element");
    if !class.is_empty() {
        el.set_class_name(class);
    }
    el
}

pub fn make_text(doc: &Document, tag: &str, class: &str, text: &str) -> HtmlElement {
    let el = make(doc, tag, class);
    el.set_text_content(Some(text));
    el
}

fn make_svg(doc: &Document, tag: &str) -> Element {
    doc.create_element_ns(Some("http://www.w3.org/2000/svg"), tag).expect("svg element")
}

// ---------------------------------------------------------------------------
// Decorations
// ---------------------------------------------------------------------------

pub fn decoration_svg(doc: &Document, style: DecorationStyle) -> Element {
    let svg = make_svg(doc, "svg");
    svg.class_list().add_1("decoration-symbol").ok();
    svg.set_attribute("viewBox", "0 0 120 64").ok();
    svg.set_attribute("aria-hidden", "true").ok();
    svg.set_attribute("focusable", "false").ok();

    let shape = |tag: &str, attrs: &[(&str, &str)]| {
        let node = make_svg(doc, tag);
        for (key, value) in attrs {
            node.set_attribute(key, value).ok();
        }
        svg.append_child(&node).ok();
    };

    match style {
        DecorationStyle::Water => {
            shape("path", &[("d", "M60 7C52 19 48 25 48 32a12 12 0 0 0 24 0c0-7-4-13-12-25z"), ("fill", "none")]);
            shape("path", &[("d", "M31 43c8-5 18-7 29-7s21 2 29 7M19 51c11-6 25-9 41-9s30 3 41 9M9 58c14-7 31-10 51-10s37 3 51 10"), ("fill", "none")]);
        }
        DecorationStyle::Jordan => {
            shape("path", &[("d", "M13 58c12-7 20-14 25-23 5-8 8-17 9-27M107 58c-12-7-20-14-25-23-5-8-8-17-9-27"), ("fill", "none")]);
            shape("path", &[("d", "M47 8c5 5 8 10 8 16 0 8-5 14-13 20-5 4-9 9-12 14M73 8c-5 5-8 10-8 16 0 8 5 14 13 20 5 4 9 9 12 14"), ("fill", "none")]);
            shape("path", &[("d", "M49 52c7-3 15-3 22 0M52 43c5-2 11-2 16 0M55 34c3-1 7-1 10 0"), ("fill", "none")]);
        }
        DecorationStyle::Dove => {
            shape("path", &[("d", "M12 40c19 2 34-2 46-13 9-8 22-10 49-5-14 5-23 12-29 22-7 11-19 17-34 14-11-2-20-8-27-14z"), ("fill", "none")]);
            shape("path", &[("d", "M54 30C42 24 35 15 35 5c10 4 19 11 25 20"), ("fill", "none")]);
            shape("path", &[("d", "M27 50 14 58m22-4-8 8"), ("fill", "none")]);
            shape("circle", &[("cx", "88"), ("cy", "27"), ("r", "1.6"), ("fill", "currentColor"), ("stroke", "none")]);
        }
        DecorationStyle::Scriptures => {
            shape("path", &[("d", "M10 17c20-5 36-1 50 9v30c-14-10-30-14-50-9zm100 0c-20-5-36-1-50 9v30c14-10 30-14 50-9z"), ("fill", "none")]);
            shape("path", &[("d", "M60 26v30M18 25c13-2 24 0 34 5M18 34c13-2 24 0 34 5m50-14c-13-2-24 0-34 5m34 4c-13-2-24 0-34 5"), ("fill", "none")]);
        }
        DecorationStyle::Temple => {
            shape("path", &[("d", "M8 58h104M16 58V45h19V34h13V22h8V11h8v11h8v12h13v11h19v13"), ("fill", "none")]);
            shape("path", &[("d", "M56 11 60 3l4 8M51 58V43h18v15M57 43V31h6v12"), ("fill", "none")]);
            shape("path", &[("d", "M23 50h5m8-9h6m36 0h6m8 9h5"), ("fill", "none")]);
        }
        DecorationStyle::TreeLife => {
            shape("path", &[("d", "M60 58V34m0 8L45 27m15 9 16-14M52 35 36 38m32-8 17 4M60 34 58 17"), ("fill", "none")]);
            for (cx, cy, r) in [(58, 11, 7), (42, 18, 8), (76, 16, 8), (29, 29, 7), (91, 29, 7), (47, 30, 8), (72, 29, 8), (38, 42, 6), (84, 41, 6)] {
                shape("circle", &[("cx", &cx.to_string()), ("cy", &cy.to_string()), ("r", &r.to_string()), ("fill", "none")]);
            }
            shape("path", &[("d", "M45 59c5-5 10-7 15-7s10 2 15 7"), ("fill", "none")]);
        }
        DecorationStyle::Rays => {
            shape("path", &[("d", "M60 5v16M25 17l12 12M8 47h18m69 0h17M95 17 83 29"), ("fill", "none")]);
            shape("path", &[("d", "M37 55c3-11 11-18 23-18s20 7 23 18"), ("fill", "none")]);
        }
        DecorationStyle::Olive | DecorationStyle::Line | DecorationStyle::Custom => {}
    }
    svg
}

pub fn render_decoration(doc: &Document, target: &Element, style: DecorationStyle, size: Size, data: &str, for_print: bool) {
    target.set_class_name(&format!("program-decoration decoration-{} size-{}", style.as_str(), size.as_str()));
    target.set_attribute("role", "img").ok();
    target.set_attribute("aria-label", style.label()).ok();
    if style == DecorationStyle::Olive {
        target.append_child(&make(doc, "span", "decoration-art")).ok();
    } else if style == DecorationStyle::Custom {
        if is_image_data_url(data) {
            let image = make(doc, "img", "decoration-custom-image");
            image.set_attribute("src", data).ok();
            image.set_attribute("alt", "").ok();
            target.append_child(&image).ok();
        } else if !for_print {
            target.append_child(&make_text(doc, "span", "decoration-placeholder", "Upload an image")).ok();
        }
    } else if style != DecorationStyle::Line {
        target.append_child(&decoration_svg(doc, style)).ok();
    }
}

// ---------------------------------------------------------------------------
// Markdown
// ---------------------------------------------------------------------------

pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// Replaces the leftmost, non-overlapping `marker ... marker` pairs (for
/// whichever marker in `markers` occurs first at each step) with `<tag>...</tag>`.
/// Mirrors a global regex alternation like `/\*\*(.+?)\*\*|__(.+?)__/g`.
fn apply_markup_alternation(input: &str, markers: &[&str], tag: &str) -> String {
    let mut result = String::new();
    let mut rest = input;
    loop {
        let mut best: Option<(usize, &str)> = None;
        for &marker in markers {
            if let Some(idx) = rest.find(marker) {
                if best.map_or(true, |(best_idx, _)| idx < best_idx) {
                    best = Some((idx, marker));
                }
            }
        }
        let Some((start, marker)) = best else {
            result.push_str(rest);
            break;
        };
        let after = &rest[start + marker.len()..];
        if let Some(end) = after.find(marker) {
            if end > 0 {
                result.push_str(&rest[..start]);
                result.push('<');
                result.push_str(tag);
                result.push('>');
                result.push_str(&after[..end]);
                result.push_str("</");
                result.push_str(tag);
                result.push('>');
                rest = &after[end + marker.len()..];
                continue;
            }
        }
        result.push_str(&rest[..start + marker.len()]);
        rest = &rest[start + marker.len()..];
    }
    result
}

pub fn render_inline_markdown(line: &str) -> String {
    let escaped = escape_html(line);
    let bold = apply_markup_alternation(&escaped, &["**", "__"], "strong");
    apply_markup_alternation(&bold, &["*", "_"], "em")
}

/// Splits on runs of two or more consecutive newlines, mirroring `split(/\n{2,}/)`.
fn split_blank_lines(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            let run_start = i;
            let mut j = i;
            while j < bytes.len() && bytes[j] == b'\n' {
                j += 1;
            }
            if j - run_start >= 2 {
                result.push(&text[start..run_start]);
                start = j;
                i = j;
                continue;
            }
        }
        i += 1;
    }
    result.push(&text[start..]);
    result
}

fn is_bullet_line(line: &str) -> bool {
    let mut chars = line.chars();
    matches!(chars.next(), Some('-') | Some('*')) && chars.next().is_some_and(|c| c.is_whitespace())
}

fn strip_bullet(line: &str) -> &str {
    line[1..].trim_start()
}

fn is_numbered_line(line: &str) -> bool {
    let digits_end = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    digits_end > 0
        && line[digits_end..].starts_with(['.', ')'])
        && line[digits_end + 1..].starts_with(|c: char| c.is_whitespace())
}

fn strip_numbered(line: &str) -> &str {
    let digits_end = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    line[digits_end + 1..].trim_start()
}

pub fn render_markdown(source: &str) -> String {
    let text = source.replace("\r\n", "\n");
    split_blank_lines(&text)
        .into_iter()
        .map(|block| {
            let trimmed = block.trim();
            if trimmed.is_empty() {
                return String::new();
            }
            let lines: Vec<&str> = trimmed.split('\n').map(|l| l.trim()).collect();
            if lines.iter().all(|l| is_bullet_line(l)) {
                let items: String = lines.iter().map(|l| format!("<li>{}</li>", render_inline_markdown(strip_bullet(l)))).collect();
                format!("<ul>{items}</ul>")
            } else if lines.iter().all(|l| is_numbered_line(l)) {
                let items: String = lines.iter().map(|l| format!("<li>{}</li>", render_inline_markdown(strip_numbered(l)))).collect();
                format!("<ol>{items}</ol>")
            } else {
                let joined = lines.iter().map(|l| render_inline_markdown(l)).collect::<Vec<_>>().join("<br>");
                format!("<p>{joined}</p>")
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Typography
// ---------------------------------------------------------------------------

pub fn apply_typography(el: &HtmlElement, typography: Option<&Typography>) {
    let Some(t) = typography else { return };
    let style = el.style();
    match t.weight {
        Weight::Bold => { style.set_property("font-weight", "700").ok(); }
        Weight::Regular => { style.set_property("font-weight", "400").ok(); }
        Weight::Default => {}
    }
    match t.slant {
        Slant::Italic => { style.set_property("font-style", "italic").ok(); }
        Slant::Regular => { style.set_property("font-style", "normal").ok(); }
        Slant::Default => {}
    }
    if !t.color.is_empty() {
        style.set_property("color", &t.color).ok();
    }
    if let Some(stack) = t.font.css_stack() {
        style.set_property("font-family", stack).ok();
    }
}

// ---------------------------------------------------------------------------
// Program blocks / pages
// ---------------------------------------------------------------------------

pub fn render_program_block(doc: &Document, block: &Block, monochrome: bool, for_print: bool, grayscale: &GrayscaleCache) -> HtmlElement {
    match block {
        Block::Heading { typography, data, .. } => {
            let node = make_text(doc, "h2", &format!("program-heading size-{} color-{}", data.size.as_str(), data.color.as_str()), &data.text);
            node.style().set_property("text-align", data.align.as_str()).ok();
            apply_typography(&node, Some(typography));
            node
        }
        Block::Text { typography, data, .. } => {
            let node = make_text(doc, "p", &format!("program-text style-{}", data.style.as_str()), &data.text);
            node.style().set_property("text-align", data.align.as_str()).ok();
            apply_typography(&node, Some(typography));
            node
        }
        Block::Item { typography, data, .. } => {
            let node = make(doc, "div", &format!("program-item size-{} style-{} align-{}", data.size.as_str(), data.style.as_str(), data.align.as_str()));
            let text_span = make_text(doc, "span", "program-item-text", &data.text);
            apply_typography(&text_span, Some(typography));
            node.append_child(&make_text(doc, "span", "program-item-label", &data.label)).ok();
            node.append_child(&text_span).ok();
            node
        }
        Block::Callout { typography, data, .. } => {
            let node = make(doc, "div", &format!("program-callout size-{} align-{}", data.size.as_str(), data.align.as_str()));
            let title_node = make_text(doc, "h3", "program-callout-title", &data.title);
            apply_typography(&title_node, Some(typography));
            node.append_child(&title_node).ok();
            if !data.subtitle.is_empty() {
                node.append_child(&make_text(doc, "p", "program-callout-subtitle", &data.subtitle)).ok();
            }
            node
        }
        Block::Hymn { typography, data, .. } => {
            let node = make(doc, "div", &format!("program-hymn size-{} align-{} columns-{}", data.size.as_str(), data.align.as_str(), data.columns.as_str()));
            let lyrics_node = make(doc, "div", &format!("program-hymn-lyrics lyrics-align-{}", data.lyrics_align.as_str()));
            apply_typography(&lyrics_node, Some(typography));
            let normalized = data.lyrics.replace("\r\n", "\n");
            for verse in split_blank_lines(&normalized) {
                if verse.trim().is_empty() {
                    continue;
                }
                lyrics_node.append_child(&make_text(doc, "p", "program-hymn-verse", verse)).ok();
            }
            node.append_child(&make_text(doc, "h3", "program-hymn-title", &data.title)).ok();
            node.append_child(&lyrics_node).ok();
            node
        }
        Block::Quote { typography, data, .. } => {
            let node = make(doc, "figure", &format!("program-quote kind-{} size-{} align-{}", data.kind.as_str(), data.size.as_str(), data.align.as_str()));
            let quote_node = make_text(doc, "blockquote", "program-quote-text", &data.text);
            apply_typography(&quote_node, Some(typography));
            node.append_child(&quote_node).ok();
            if !data.citation.is_empty() {
                node.append_child(&make_text(doc, "figcaption", "program-quote-citation", &data.citation)).ok();
            }
            node
        }
        Block::Markdown { typography, data, .. } => {
            let node = make(doc, "div", "program-markdown");
            node.style().set_property("text-align", data.align.as_str()).ok();
            node.set_inner_html(&render_markdown(&data.text));
            apply_typography(&node, Some(typography));
            node
        }
        Block::Image { data, .. } => {
            let artwork = find_art(&data.art);
            let source: String = match artwork {
                Some(art) => art.src.to_string(),
                None if is_image_data_url(&data.data) => data.data.clone(),
                None => String::new(),
            };
            if source.is_empty() && for_print {
                return make(doc, "span", "empty-print-block");
            }
            let mut class = format!("program-image size-{} shape-{}", data.size.as_str(), data.shape.as_str());
            if artwork.is_some() {
                class.push_str(" built-in-art");
            }
            let figure = make(doc, "figure", &class);
            if !source.is_empty() {
                let img = make(doc, "img", "");
                let alt = if !data.caption.is_empty() { data.caption.as_str() } else { artwork.map(|a| a.alt).unwrap_or("Program image") };
                img.set_attribute("alt", alt).ok();
                if monochrome {
                    if let Some(cached) = grayscale.get(&source) {
                        img.set_attribute("src", &cached).ok();
                    } else {
                        img.set_attribute("src", &source).ok();
                        let img_handle = img.clone();
                        grayscale.ensure(source.clone(), move |data_url| {
                            img_handle.set_attribute("src", &data_url).ok();
                        });
                    }
                } else {
                    img.set_attribute("src", &source).ok();
                }
                figure.append_child(&img).ok();
            } else if !for_print {
                figure.append_child(&make_text(doc, "div", "image-placeholder", "Add an image")).ok();
            }
            if !data.caption.is_empty() && !source.is_empty() {
                figure.append_child(&make_text(doc, "figcaption", "", &data.caption)).ok();
            }
            figure
        }
        Block::Decoration { data, .. } => {
            if data.style == DecorationStyle::Custom && !is_image_data_url(&data.data) && for_print {
                return make(doc, "span", "empty-print-block");
            }
            let node = make(doc, "div", "");
            render_decoration(doc, &node, data.style, data.size, &data.data, for_print);
            node
        }
        Block::Spacer { data, .. } => make(doc, "div", &format!("program-spacer size-{}", data.size.as_str())),
    }
}

pub fn to_grayscale_hex(hex: &str) -> String {
    let bytes = hex.as_bytes();
    if bytes.len() != 7 || bytes[0] != b'#' {
        return hex.to_string();
    }
    let component = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    let (Some(r), Some(g), Some(b)) = (component(1), component(3), component(5)) else {
        return hex.to_string();
    };
    let gray = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64).round() as u8;
    format!("#{gray:02x}{gray:02x}{gray:02x}")
}

pub fn build_program_page(doc: &Document, page: &Page, theme: &Theme, for_print: bool, grayscale: &GrayscaleCache) -> HtmlElement {
    let mut class = format!("program-page font-{}", theme.font.as_str());
    if theme.monochrome {
        class.push_str(" is-monochrome");
    }
    let outer = make(doc, "article", &class);
    let style = outer.style();
    let (paper, text, accent) = if theme.monochrome {
        (to_grayscale_hex(&theme.paper), to_grayscale_hex(&theme.text), to_grayscale_hex(&theme.accent))
    } else {
        (theme.paper.clone(), theme.text.clone(), theme.accent.clone())
    };
    style.set_property("--page-paper", &paper).ok();
    style.set_property("--page-text", &text).ok();
    style.set_property("--page-accent", &accent).ok();

    let inner = make(doc, "div", "program-page-inner");
    for (index, block) in page.blocks.iter().enumerate() {
        let rendered = render_program_block(doc, block, theme.monochrome, for_print, grayscale);
        rendered.class_list().add_1("program-block").ok();
        rendered.set_attribute("data-block-id", block.id()).ok();
        rendered.set_attribute("data-block-number", &(index + 1).to_string()).ok();
        inner.append_child(&rendered).ok();
    }
    outer.append_child(&inner).ok();
    outer
}
