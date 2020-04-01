use wasm_bindgen::prelude::*;
use dominator::{append_dom, body, html};

#[wasm_bindgen(start)]
pub fn main_js() {
    append_dom(&body(), html!("h1", {
        .text("Hello World!")
    }));
}