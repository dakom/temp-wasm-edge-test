use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
pub fn main_js() {
    let root_element:HtmlElement = 
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("web-sys-elem")
            .unwrap()
            .dyn_into()
            .unwrap();

    root_element.set_inner_text("Hello World!");

}