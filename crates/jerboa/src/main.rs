//! Jerboa graphics library.

use std::io::{BufWriter, Write};
use std::path::Path;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("Hello, jerboa!");

    let now = chrono::Utc::now();
    let filename = format!("output_{}.ppm", now.format("%Y-%m-%d_%H:%M:%S"));
    let filepath = Path::new(&filename);

    match std::fs::File::create(&filepath)
        .map_err(|err| err.to_string())
        .and_then(|file| {
            let mut writer = BufWriter::new(file);

            writer
                .write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
                .map_err(|err| err.to_string())?;

            for i in 0..IMAGE_HEIGHT {
                print!("\rScan lines remaining: {}", IMAGE_HEIGHT - i);
                std::io::stdout().flush().unwrap();

                for j in 0..IMAGE_WIDTH {
                    let r = ((i as f32 / IMAGE_HEIGHT as f32) * 255.0) as u8;
                    let g = ((j as f32 / IMAGE_WIDTH as f32) * 255.0) as u8;
                    let b = ((i + j) as f32 / (IMAGE_WIDTH + IMAGE_HEIGHT) as f32) as u8;
                    writer
                        .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
                        .map_err(|err| err.to_string())?;
                }
            }

            println!("\nDone");

            Ok(())
        }) {
        Ok(()) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}
