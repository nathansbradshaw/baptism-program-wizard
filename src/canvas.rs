use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, File, HtmlCanvasElement, HtmlImageElement};

fn window() -> web_sys::Window {
    web_sys::window().expect("window exists")
}

fn document() -> web_sys::Document {
    window().document().expect("document exists")
}

/// Awaits an `<img>` element's load/error events. Mirrors the `new Image()` +
/// `onload`/`onerror` Promise wrapper used throughout app.js.
pub async fn load_image_element(src: &str) -> Result<HtmlImageElement, JsValue> {
    let img = HtmlImageElement::new()?;
    let img_onload = img.clone();
    let img_onerror = img.clone();
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onload = Closure::once(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).ok();
        });
        img_onload.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        let onerror = Closure::once(move |_event: web_sys::Event| {
            reject.call0(&JsValue::NULL).ok();
        });
        img_onerror.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    });
    img.set_src(src);
    JsFuture::from(promise).await?;
    Ok(img)
}

fn new_canvas(width: u32, height: u32) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let canvas: HtmlCanvasElement = document().create_element("canvas")?.dyn_into()?;
    canvas.set_width(width.max(1));
    canvas.set_height(height.max(1));
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d")?.ok_or("no 2d context")?.dyn_into()?;
    Ok((canvas, ctx))
}

fn canvas_to_data_url(canvas: &HtmlCanvasElement, mime: &str, quality: Option<f64>) -> Result<String, JsValue> {
    let func = js_sys::Reflect::get(canvas, &JsValue::from_str("toDataURL"))?.dyn_into::<js_sys::Function>()?;
    let result = match quality {
        Some(q) => func.call2(canvas, &JsValue::from_str(mime), &JsValue::from_f64(q))?,
        None => func.call1(canvas, &JsValue::from_str(mime))?,
    };
    result.as_string().ok_or_else(|| JsValue::from_str("toDataURL did not return a string"))
}

/// Mirrors `resizeImage`: downscales an uploaded file to at most 1400px on
/// its longest side and re-encodes it as a JPEG data URL.
pub async fn resize_image_to_data_url(file: &File) -> Result<String, JsValue> {
    let blob: &web_sys::Blob = file.unchecked_ref();
    let object_url = web_sys::Url::create_object_url_with_blob(blob)?;
    let load_result = load_image_element(&object_url).await;
    web_sys::Url::revoke_object_url(&object_url).ok();
    let img = load_result.map_err(|_| JsValue::from_str("That image could not be read."))?;

    let natural_width = img.natural_width() as f64;
    let natural_height = img.natural_height() as f64;
    let max_dimension = 1400.0_f64;
    let scale = (max_dimension / natural_width.max(natural_height)).min(1.0);
    let width = (natural_width * scale).round().max(1.0);
    let height = (natural_height * scale).round().max(1.0);

    let (canvas, ctx) = new_canvas(width as u32, height as u32)?;
    ctx.set_fill_style_str("#fff");
    ctx.fill_rect(0.0, 0.0, width, height);
    ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, width, height)?;

    canvas_to_data_url(&canvas, "image/jpeg", Some(0.84))
}

/// Mirrors `resizeDecorationImage`: downscales an uploaded decoration image to
/// at most 600px on its longest side, preserving transparency as PNG.
pub async fn resize_decoration_image_to_data_url(file: &File) -> Result<String, JsValue> {
    let blob: &web_sys::Blob = file.unchecked_ref();
    let object_url = web_sys::Url::create_object_url_with_blob(blob)?;
    let load_result = load_image_element(&object_url).await;
    web_sys::Url::revoke_object_url(&object_url).ok();
    let img = load_result.map_err(|_| JsValue::from_str("That image could not be read."))?;

    let natural_width = img.natural_width() as f64;
    let natural_height = img.natural_height() as f64;
    let max_dimension = 600.0_f64;
    let scale = (max_dimension / natural_width.max(natural_height)).min(1.0);
    let width = (natural_width * scale).round().max(1.0);
    let height = (natural_height * scale).round().max(1.0);

    let (canvas, ctx) = new_canvas(width as u32, height as u32)?;
    ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, width, height)?;

    canvas_to_data_url(&canvas, "image/png", None)
}

/// Caches grayscale conversions of built-in art / uploaded images, keyed by
/// source URL or data URL. Mirrors `grayscaleImageCache` + `getGrayscaleImageSrc`.
#[derive(Clone, Default)]
pub struct GrayscaleCache(Rc<RefCell<HashMap<String, String>>>);

impl GrayscaleCache {
    pub fn get(&self, src: &str) -> Option<String> {
        self.0.borrow().get(src).cloned()
    }

    /// Kicks off (or ignores, if already in flight/cached) an async grayscale
    /// conversion of `src`, calling `on_ready` with the resulting data URL.
    pub fn ensure(&self, src: String, on_ready: impl Fn(String) + 'static) {
        if let Some(cached) = self.get(&src) {
            on_ready(cached);
            return;
        }
        let cache = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = convert_to_grayscale(&src).await;
            match result {
                Ok(data_url) => {
                    cache.0.borrow_mut().insert(src, data_url.clone());
                    on_ready(data_url);
                }
                Err(_) => {
                    // Fall back to the original image; don't cache failures so
                    // a later retry (e.g. after the image finishes loading
                    // elsewhere) can succeed.
                }
            }
        });
    }
}

async fn convert_to_grayscale(src: &str) -> Result<String, JsValue> {
    let img = load_image_element(src).await?;
    let (canvas, ctx) = new_canvas(img.natural_width(), img.natural_height())?;
    ctx.set_filter("grayscale(1) contrast(1.12)");
    ctx.draw_image_with_html_image_element(&img, 0.0, 0.0)?;
    canvas_to_data_url(&canvas, "image/png", None)
}
