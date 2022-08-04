//! Jerboa graphics library.
use jerboa::core::image::PixelBufferRgb32f;
use std::path::PathBuf;
use jerboa::rtc::scene::Scene;
use minifb::{Window, WindowOptions, Key, InputCallback, KeyRepeat};

fn main() {
    let mut window = Window::new(
        "Jerboa",
        512,
        512,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap();

    let mut image = PixelBufferRgb32f::new(512, 512);

    for ((x, y), pixel) in image.pixels_mut() {
        pixel[0] = x as f32 / 512.0;
        pixel[1] = y as f32 / 512.0;
        pixel[2] = (x + y) as f32 / 1024.0;
    }

    let buffer: Vec<u32> = image.pixels().map(|(_, pixel)| {
        let (r, g, b) = (pixel[0] * 255.0, pixel[1] * 255.0, pixel[2] * 255.0);
        ((r as u32) << 16) | ((g as u32) << 8) | b as u32
    }).collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            let now = chrono::Utc::now();
            let filepath = PathBuf::from(format!("output_{}", now.format("%Y-%m-%d_%H:%M:%S")));
            image.write_as_pfm(&filepath).unwrap();
            println!("Saved to {}", filepath.display());
        }

        window.update_with_buffer(&buffer, 512, 512).unwrap();
    }
}
