mod transform;

use image::{
    load_from_memory, DynamicImage, FilterType, GenericImage, GenericImageView, ImageOutputFormat,
};

pub use transform::{PixelCoords, PixelSize, Transform, TransformMode};

pub fn load(buffer: &[u8]) -> Result<DynamicImage, failure::Error> {
    load_from_memory(buffer).map_err(|e| failure::format_err!("could not load image {}", e))
}

pub fn size(image: &DynamicImage) -> PixelSize {
    PixelSize {
        width: image.width(),
        height: image.height(),
    }
}

pub fn process(image: &mut DynamicImage, transform: &Transform) -> Result<Vec<u8>, failure::Error> {
    let output_dimensions = transform.get_output_pixel_dimensions();
    let canvas_size = output_dimensions.canvas;
    let output_size = output_dimensions.size;
    let output_origin = output_dimensions.origin;

    let mut resized_image =
        image.resize_exact(output_size.width, output_size.height, FilterType::Gaussian);

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

    let mut output: Vec<u8> = Vec::new();
    output_canvas
        .write_to(&mut output, ImageOutputFormat::PNG)
        .map(|_| output)
        .map_err(|_| failure::format_err!("um"))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn process_an_image() {
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

        let output = process(&mut image, &transform);

        let mut file =
            std::fs::File::create("tests/output/test_pattern_fill_top_right.png").unwrap();
        let result = file.write_all(&output.unwrap());
        result.unwrap();
    }
}