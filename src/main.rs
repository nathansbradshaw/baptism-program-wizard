mod canvas;
mod clipboard;
mod editor;
mod model;
mod render;
mod state;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    state::init();
}

fn main() {}
