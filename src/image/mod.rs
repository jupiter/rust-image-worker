mod transform;

pub use transform::{PixelSize, Transform, TransformMode};

use image::{load_from_memory, DynamicImage, FilterType, GenericImageView, ImageOutputFormat};

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

    let output_image = image.resize_exact(
        output_dimensions.size.width,
        output_dimensions.size.height,
        FilterType::Gaussian,
    );

    let mut output: Vec<u8> = Vec::new();
    output_image
        .write_to(&mut output, ImageOutputFormat::PNG)
        .map(|_| output)
        .map_err(|_| failure::format_err!("um"))
}
