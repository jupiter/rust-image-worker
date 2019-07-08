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

fn positive_int_value(value: u32) -> Option<u32> {
    if value > 0 {
        Some(value)
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
struct ProcessImageParams {
    bg: Vec<u8>,
    dx: f32,
    dy: f32,
    format: String,
    height: u32,
    mode: String,
    quality: u8,
    scale: f32,
    width: u32,
}

fn error_to_js_value(e: failure::Error) -> JsValue {
    JsValue::from_str(&e.to_string())
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

    let output_format = match string_to_output_format(&params.format, params.quality) {
        None => {
            let input_format = image::input_format(buffer).map_err(error_to_js_value)?;
            image::input_to_output_format(input_format, params.quality)
                .map_err(error_to_js_value)?
        }
        Some(output_format) => output_format,
    };

    let mut image = image::load(buffer).map_err(|e| JsValue::from(e.to_string()))?;
    let image_size = image::size(&image);

    let mut transform = image::Transform::new(&image_size, transform_mode);
    transform.relative_center_offset.dx = params.dx;
    transform.relative_center_offset.dy = params.dy;
    transform.scale = params.scale;

    let color_option = if params.bg.is_empty() {
        None
    } else {
        Some([params.bg[0], params.bg[1], params.bg[2]])
    };

    let mut output = image::process(&mut image, &transform, output_format.clone(), color_option)
        .map_err(error_to_js_value)?;

    output.push(output_format_to_key(output_format));

    Ok(output)
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

fn string_to_output_format(format_string: &str, quality: u8) -> Option<image::ImageOutputFormat> {
    match format_string {
        "png" => Some(image::ImageOutputFormat::PNG),
        "jpg" => Some(image::ImageOutputFormat::JPEG(quality)),
        _ => None,
    }
}

fn output_format_to_key(output_format: image::ImageOutputFormat) -> u8 {
    match output_format {
        image::ImageOutputFormat::PNG => 0,
        image::ImageOutputFormat::JPEG(_) => 1,
        _ => unimplemented!(),
    }
}