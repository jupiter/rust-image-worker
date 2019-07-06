extern crate cfg_if;
extern crate wasm_bindgen;

mod image;
mod utils;

use cfg_if::cfg_if;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

fn string_to_transform_mode(
    mode_string: &str,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<image::TransformMode, failure::Error> {
    if mode_string == "fit" {
        if width.and(height).is_some() {
            Ok(image::TransformMode::Fit {
                width: width.unwrap(),
                height: height.unwrap(),
            })
        } else if width.is_some() {
            Ok(image::TransformMode::FitWidth(width.unwrap()))
        } else {
            Ok(image::TransformMode::FitHeight(height.unwrap()))
        }
    } else if width.and(height).is_some() {
        match mode_string {
            "fill" => Ok(image::TransformMode::Fill {
                width: width.unwrap(),
                height: height.unwrap(),
            }),
            "limit" => Ok(image::TransformMode::Limit {
                width: width.unwrap(),
                height: height.unwrap(),
            }),
            _ => Err(failure::format_err!("unknown mode")),
        }
    } else {
        Err(failure::format_err!("mode needs width and height"))
    }
}

fn positive_int_value(value: u32) -> Option<u32> {
    if value > 0 {
        Some(value)
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
struct ProcessImageParams {
    format: String,
    width: u32,
    height: u32,
    dx: f32,
    dy: f32,
    scale: f32,
    mode: String,
}

#[wasm_bindgen]
pub fn process_image(buffer: &[u8], params_value: JsValue) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();

    let params: ProcessImageParams = from_value(params_value)?;

    let transform_mode = string_to_transform_mode(
        &params.mode,
        positive_int_value(params.width),
        positive_int_value(params.height),
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut image = image::load(buffer).map_err(|e| JsValue::from(e.to_string()))?;
    let image_size = image::size(&image);

    let transform = image::Transform::new(&image_size, transform_mode);

    let output =
        image::process(&mut image, &transform).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output)
}
