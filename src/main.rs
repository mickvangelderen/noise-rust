extern crate image;

pub mod permutations;
pub mod smoothing;
pub mod perlin;

use perlin::*;

fn main() {
    let width = 512;
    let height = 512;
    let mut data = Vec::new();
    data.resize(width * height, 0u8);

    for row in 0..height {
        let y = row as f32 as f32 / height as f32;
        for col in 0..width {
            let x = col as f32 as f32 / width as f32;
            let gray = perlin_2d(8.0 * x, 8.0 * y) * 128.0 + 128.0;
            data[(width - 1 - row) * width + col] = gray as u8;
        }
    }

    let file = std::fs::File::create("perlin.png").unwrap();
    let encoder = image::png::PNGEncoder::new(file);
    encoder
        .encode(
            &data[..],
            width as u32,
            height as u32,
            image::ColorType::Gray(8),
        )
        .unwrap();
}
