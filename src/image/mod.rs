mod transform;

pub use image::ImageOutputFormat;

use image::{
    guess_format, load_from_memory, DynamicImage, FilterType, GenericImage, GenericImageView,
    ImageFormat,
};

pub use transform::{PixelCoords, PixelSize, Transform, TransformMode};

pub fn input_to_output_format(
    input_format: ImageFormat,
    quality: u8,
) -> Result<ImageOutputFormat, failure::Error> {
    match input_format {
        ImageFormat::JPEG => Ok(ImageOutputFormat::JPEG(quality)),
        ImageFormat::PNG => Ok(ImageOutputFormat::PNG),
        ImageFormat::GIF => Ok(ImageOutputFormat::PNG),
        ImageFormat::WEBP => Ok(ImageOutputFormat::PNG),
        _ => Err(failure::format_err!("unsupported input format")),
    }
}

pub fn input_format(buffer: &[u8]) -> Result<ImageFormat, failure::Error> {
    guess_format(buffer).map_err(|e| failure::format_err!("could not guess image format {}", e))
}

pub fn load(buffer: &[u8]) -> Result<DynamicImage, failure::Error> {
    load_from_memory(buffer).map_err(|e| failure::format_err!("could not load image {}", e))
}

pub fn size(image: &DynamicImage) -> PixelSize {
    PixelSize {
        width: image.width(),
        height: image.height(),
    }
}

pub fn process(
    image: &mut DynamicImage,
    transform: &Transform,
    output_format: ImageOutputFormat,
    color: Option<[u8; 3]>,
) -> Result<Vec<u8>, failure::Error> {
    let output_dimensions = transform.get_output_pixel_dimensions();
    let canvas_size = output_dimensions.canvas;
    let output_size = output_dimensions.size;
    let output_origin = output_dimensions.origin;

    if color.is_some() {
        fill(image, color.unwrap());
    }

    let mut resized_image =
        image.resize_exact(output_size.width, output_size.height, FilterType::Triangle);

    let mut output_canvas = DynamicImage::new_rgba8(canvas_size.width, canvas_size.height);

    let sub_image_x: u32;
    let sub_image_y: u32;
    let copied_x: u32;
    let copied_y: u32;

    if output_origin.x < 0 {
        sub_image_x = output_origin.x.abs() as u32;
        copied_x = 0;
    } else {
        sub_image_x = 0;
        copied_x = output_origin.x as u32;
    }

    if output_origin.y < 0 {
        sub_image_y = output_origin.y.abs() as u32;
        copied_y = 0;
    } else {
        sub_image_y = 0;
        copied_y = output_origin.y as u32;
    }

    let has_copied = output_canvas.copy_from(
        &resized_image.sub_image(
            sub_image_x,
            sub_image_y,
            canvas_size.width.min(output_size.width - sub_image_x),
            canvas_size.height.min(output_size.height - sub_image_y),
        ),
        copied_x,
        copied_y,
    );

    if !has_copied {
        return Err(failure::format_err!(
            "could not place image due to sizing errors",
        ));
    }

    if color.is_some() {
        fill(&mut output_canvas, color.unwrap());
    }

    let mut output: Vec<u8> = Vec::new();
    output_canvas
        .write_to(&mut output, output_format)
        .map(|_| output)
        .map_err(|_| failure::format_err!("um"))
}

fn fill(image: &mut DynamicImage, color_data: [u8; 3]) {
    match image {
        DynamicImage::ImageRgba8(image_buffer) => {
            for mut pixel_mut in image_buffer.pixels_mut() {
                let a = pixel_mut.data[3] as f32 / 255.0;

                let r = (a * pixel_mut.data[0] as f32) + ((1.0 - a) * color_data[0] as f32);
                let g = (a * pixel_mut.data[1] as f32) + ((1.0 - a) * color_data[1] as f32);
                let b = (a * pixel_mut.data[2] as f32) + ((1.0 - a) * color_data[2] as f32);

                pixel_mut.data[0] = r as u8;
                pixel_mut.data[1] = g as u8;
                pixel_mut.data[2] = b as u8;
                pixel_mut.data[3] = 255;
            }
        }
        _ => {}
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn process_a_png_image() {
        let mut image =
            image::open(std::path::Path::new("./tests/input/test_pattern.png")).unwrap();
        let image_size = size(&image);

        let mut transform = Transform::new(
            &image_size,
            TransformMode::Fill {
                width: 600,
                height: 100,
            },
        );

        transform.scale = 0.5;
        transform.relative_center_offset.dx = 1.0;
        transform.relative_center_offset.dy = -1.0;

        let output = process(
            &mut image,
            &transform,
            ImageOutputFormat::PNG,
            Some([100, 200, 100]),
        );

        let mut file =
            std::fs::File::create("tests/output/test_pattern_fill_top_right.png").unwrap();
        let result = file.write_all(&output.unwrap());
        result.unwrap();
    }

    #[test]
    fn output_a_jpg_image() {
        let mut image =
            image::open(std::path::Path::new("./tests/input/test_pattern.png")).unwrap();
        let image_size = size(&image);

        let mut transform = Transform::new(
            &image_size,
            TransformMode::Fit {
                width: 100,
                height: 100,
            },
        );

        transform.scale = 0.5;

        let output = process(
            &mut image,
            &transform,
            ImageOutputFormat::JPEG(90),
            Some([100, 200, 100]),
        );

        let mut file = std::fs::File::create("tests/output/test_pattern_fit.jpg").unwrap();
        let result = file.write_all(&output.unwrap());
        result.unwrap();
    }

    #[test]
    fn process_a_jpg_image() {
        let mut image = image::open(std::path::Path::new(
            "./tests/input/Apollo_17_Image_Of_Earth_From_Space.jpeg",
        ))
        .unwrap();
        let image_size = size(&image);

        let transform = Transform::new(
            &image_size,
            TransformMode::Fit {
                width: 100,
                height: 200,
            },
        );

        let output = process(&mut image, &transform, ImageOutputFormat::JPEG(90), None);

        let mut file =
            std::fs::File::create("tests/output/Apollo_17_Image_Of_Earth_From_Space.jpg").unwrap();
        let result = file.write_all(&output.unwrap());
        result.unwrap();
    }
}