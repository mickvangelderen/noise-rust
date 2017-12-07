#![feature(fn_must_use)]

extern crate image;
extern crate rand;

use rand::Rng;

// fn blend_p3(t: f32) -> f32 {
//     t * t * (-2.0 * t + 3.0)
// }

/// Fifth order polynomial f(x) = 6x^5 - 14x^4 + 10x^3 for which:
///  * f(0) = 0, f(1) = 1,
///  * f'(0) = 0, f'(1) = 0,
///  * f''(0) = 0, f''(1) = 0.
///
/// It can be proven that, f(x) = 1 - f(1 - x). This can be utilized
/// when computing y1 = f(x), y2 = f(1 - x) = 1 - f(x) = 1 - y1, requiring
/// only one call to f.
///
/// The same polynomial can be centered around (0, 0) as
/// g(x) = (3/8)x^5 - (10/8)x^3 + (15/8)x.
fn blend_p5(x: f32) -> f32 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

// fn blend_p7(t: f32) -> f32 {
//     let t2 = t * t;
//     t2 * t2 * (t * (t * (t * -20.0 + 70.0) - 84.0) + 35.0)
// }

fn mix(x0: f32, x1: f32, t: f32) -> f32 {
    x0 * (1.0 - t) + x1 * t
}

fn mod_pos(a: isize, b: usize) -> usize {
    if a < 0 {
        ((a % b as isize) + b as isize) as usize
    } else {
        (a % b as isize) as usize
    }
}

fn index_2d(row: isize, col: isize, width: usize, height: usize) -> usize {
    mod_pos(row, height) * width + mod_pos(col, width)
}

fn dot2(a: (f32, f32), b: (f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

fn main() {
    let mut rng = rand::Isaac64Rng::new_unseeded();

    const GW: usize = 8;
    const GH: usize = 8;
    let gradients: Vec<(f32, f32)> = vec![(0, 0); GW * GH]
        .iter()
        .map(|_| (rng.next_f32()*2.0 - 1.0, rng.next_f32()*2.0 - 1.0))
        .collect();

    let width = 512;
    let height = 512;
    let mut data = Vec::new();
    data.resize(width * height, 0u8);

    // Gradients per pixel (resolution).
    let gpp_x = GW as f32 / width as f32;
    let gpp_y = GH as f32 / height as f32;

    let gpp_x_recip = 1.0 / gpp_x;
    let gpp_y_recip = 1.0 / gpp_y;

    for y in 0..height {
        let yf = y as f32;
        // y index of gradient: floor(yf * GH / height)
        let yi = (yf * gpp_y) as isize;
        let y0 = yi as f32 * gpp_y_recip;
        // fraction: (y - y0)/(y1 - y0)
        let yt = (yf - y0) * gpp_y;
        let yb = blend_p5(yt);

        for x in 0..width {
            let xf = x as f32;
            // x index of gradient: floor(xf * GW / width)
            let xi = (xf * gpp_x) as isize;
            let x0 = xi as f32 * gpp_x_recip;
            // fraction: (x - x0)/(x1 - x0)
            let xt = (xf - x0) * gpp_x;
            let xb = blend_p5(xt);

            let g00 = gradients[index_2d(yi + 0, xi + 0, GW, GH)];
            let g10 = gradients[index_2d(yi + 0, xi + 1, GW, GH)];
            let g01 = gradients[index_2d(yi + 1, xi + 0, GW, GH)];
            let g11 = gradients[index_2d(yi + 1, xi + 1, GW, GH)];

            let n00 = dot2(g00, (xt + 0.0, yt + 0.0));
            let n10 = dot2(g10, (xt - 1.0, yt + 0.0));
            let n01 = dot2(g01, (xt + 0.0, yt - 1.0));
            let n11 = dot2(g11, (xt - 1.0, yt - 1.0));

            let nx0 = mix(n00, n10, xb);
            let nx1 = mix(n01, n11, xb);

            let nxy = mix(nx0, nx1, yb);

            data[y * width + x] = ((nxy + 1.0) * 128.0) as u8;
            // data[(width - 1 - y) * width + x] = ((yb + xb) * 128.0) as u8;
            // data[(width - 1 - y) * width + x] = (g01.0 * 256.0) as u8;
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

// use std::ops::{Sub, Add, Mul, Div};

// fn map<T>(x: T, x_range: (T, T), y_range: (T, T)) -> T
// where
//     T: Copy + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
// {
//     let (x0, x1) = x_range;
//     let (y0, y1) = y_range;
//     let xr = (x - x0) / (x1 - x0);
//     y0 + xr * y1
// }

// fn map_stable<T>(x: T, x_range: (T, T), y_range: (T, T)) -> T
// where
//     T: Copy + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
// {
//     let (x0, x1) = x_range;
//     let (y0, y1) = y_range;
//     // y0 + (x - x0)/(x1 - x0) * (y1 - y0);
//     // y0 + (x - x0)*(y1 - y0)/(x1 - x0);
//     // (y0(x1 - x0) + (x - x0)*(y1 - y0))/(x1 - x0);
//     // (x1*y0 - x0*y0 + x*y1 - x*y0 - x0*y1 + x0*y0)/(x1 - x0);
//     // (x1*y0 + x*y1 - x*y0 - x0*y1)/(x1 - x0);
//     ((x1 - x) * y0 + (x - x0) * y1) / (x1 - x0)
// }

// struct Domain(f64, f64, f64);

// struct Weights(f64, f64, f64);

// struct Range(f64, f64);

// impl Domain {
//     fn new(x0: f64, x1: f64) -> Self {
//         Domain(x0, x1, 1.0/(x1 - x0))
//     }

//     fn weights(&self, x: f64) -> Weights {
//         Weights(self.1 - x, x - self.0, self.2)
//     }
// }

// impl Weights {
//     fn apply(&self, range: &Range) -> f64 {
//         (self.0*range.0 + self.1*range.1)*self.2
//     }

//     fn blend_and_apply(&self, blend: fn(f64) -> f64, range: &Range) -> f64 {
//         blend(self.0*self.2)*range.0 + blend(self.1*self.2)*range.1
//     }
// }
