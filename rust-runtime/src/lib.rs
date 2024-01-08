use wasm_bindgen::prelude::*;

mod program;

#[wasm_bindgen]
pub fn get_text_js() -> String {
    program::get_text()
}
