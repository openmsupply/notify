mod utils;

use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = renderOneOff)]
pub fn render_one_off(template: &str, params: &str) -> String {
    let params: serde_json::Value = serde_json::from_str(params).unwrap();
    let context = Context::from_serialize(params).unwrap();
    let result = Tera::one_off(template, &context, true);
    match result {
        Ok(s) => s,
        Err(e) => format!("Error: {:?}", e),
    }
}
