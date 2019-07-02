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

pub fn string_to_transform_mode(
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

#[wasm_bindgen]
pub fn convert_image(buffer: &[u8]) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();

    let mut image = image::load(buffer).map_err(|e| JsValue::from(e.to_string()))?;
    let image_size = image::size(&image);

    let transform = image::Transform::new(
        &image_size,
        image::TransformMode::Limit {
            width: image_size.height.min(image_size.width),
            height: image_size.height.min(image_size.width),
        },
    );

    let output =
        image::process(&mut image, &transform).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output)
}
