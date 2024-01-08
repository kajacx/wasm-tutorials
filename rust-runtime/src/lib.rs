use wasm_bindgen::prelude::*;

mod program;

#[wasm_bindgen]
pub async fn get_text_js() -> String {
    program::get_text().await
}
