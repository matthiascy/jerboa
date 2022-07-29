//! Jerboa graphics library.
use jerboa::core::image::PixelBufferRgb32f;
use std::path::PathBuf;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("Hello, jerboa!");

    let now = chrono::Utc::now();
    let filepath = PathBuf::from(format!("output_{}", now.format("%Y-%m-%d_%H:%M:%S")));

    let mut image = PixelBufferRgb32f::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for ((x, y), pixel) in image.pixels_mut() {
        pixel[0] = x as f32 / IMAGE_WIDTH as f32;
        pixel[1] = y as f32 / IMAGE_HEIGHT as f32;
        pixel[2] = (x + y) as f32 / (IMAGE_WIDTH + IMAGE_HEIGHT) as f32;
    }

    image.write_as_pfm(filepath).unwrap();
}
