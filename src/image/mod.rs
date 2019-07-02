mod transform;

use image::{load_from_memory, ImageOutputFormat};

pub fn process(buffer: &[u8]) -> Result<Vec<u8>, String> {
        let image = load_from_memory(buffer).map_err(|e| e.to_string())?;

        let mut output: Vec<u8> = Vec::new();
        image.write_to(&mut output, ImageOutputFormat::PNG)
                .map(|_| output)
                .map_err(|_| String::from("An error"))
}
