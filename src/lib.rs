extern crate cfg_if;
extern crate wasm_bindgen;

mod image;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn convert_image(buffer: &[u8]) -> Result<Vec<u8>, JsValue> {
    let output = image::process(buffer).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output)
}
