use std::fs::File;

use image::{codecs::png::PngEncoder, RgbaImage};

pub fn write_to_stdout(image: RgbaImage) {
    let stdout = std::io::stdout();
    let encoder = PngEncoder::new(&stdout);
    image.write_with_encoder(encoder).unwrap();
}

pub fn write_to_file(image: RgbaImage, name: String) {
    let file = File::create(name).unwrap();
    let encoder = PngEncoder::new(&file);
    image.write_with_encoder(encoder).unwrap();
}
