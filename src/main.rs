#![feature(fn_must_use)]
#![feature(inclusive_range_syntax)]

extern crate image;

mod permutations;

use permutations::PERMUTATIONS;

// fn smooth_p3(t: f32) -> f32 {
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
fn smooth_p5(x: f32) -> f32 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

// fn smooth_p7(t: f32) -> f32 {
//     let t2 = t * t;
//     t2 * t2 * (t * (t * (t * -20.0 + 70.0) - 84.0) + 35.0)
// }

// fn smooth_sin(x: f32) -> f32 {
//     0.5*(1.0 + f32::sin(std::f32::consts::PI*(x - 0.5)))
// }

fn dot2(a: (f32, f32), b: (f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

fn rem_pos(a: isize, b: isize) -> usize {
    let r = a % b;
    (if r < 0 { r + b } else { r }) as usize
}

fn almost_one_f32() -> f32 {
    unsafe { std::mem::transmute(0x3F7FFFFFu32) }
}

const ALMOST_ONE_F32: f32 = 0.999_999_94;

// static GRADIENTS_1D: [f32; 2] = [-1.0, ALMOST_ONE_F32];

// fn perlin_1d(x: f32) -> f32 {
//     let x0 = x.floor();

//     let x0i = rem_pos(x0 as isize + 0, PERMUTATIONS.len() as isize);
//     let x1i = rem_pos(x0 as isize + 1, PERMUTATIONS.len() as isize);

//     let gi0 = PERMUTATIONS[x0i % PERMUTATIONS.len()] as usize & 1;
//     let gi1 = PERMUTATIONS[x1i % PERMUTATIONS.len()] as usize & 1;

//     assert!(gi0 <= 1);
//     assert!(gi1 <= 1);

//     // Derived the computation for each possible combination of gradients.
//     let gi = (gi0 << 1) | gi1;

//     let xd = x - x0;
//     let fxd = smooth_p5(xd);

//     match gi {
//         0 => {
//             fxd - xd
//         },
//         1 => {
//             -fxd - xd + 2.0*xd*fxd
//         },
//         2 => {
//             fxd + xd - 2.0*xd*fxd
//         },
//         _ => {
//             -fxd + xd
//         },
//     }
// }

static GRADIENTS_2D: [(f32, f32); 4] = [
    (ALMOST_ONE_F32, 0.0),
    (-1.0, 0.0),
    (0.0, ALMOST_ONE_F32),
    (0.0, -1.0),
];

fn perlin_2d(x: f32, y: f32) -> f32 {
    let y0 = y.floor();
    let y1 = y0 + 1.0;

    let x0 = x.floor();
    let x1 = x0 + 1.0;

    let y0i = rem_pos(y0 as isize + 0, PERMUTATIONS.len() as isize);
    let y1i = rem_pos(y0 as isize + 1, PERMUTATIONS.len() as isize);
    let x0i = rem_pos(x0 as isize + 0, PERMUTATIONS.len() as isize);
    let x1i = rem_pos(x0 as isize + 1, PERMUTATIONS.len() as isize);

    let gi00 = PERMUTATIONS[(x0i + PERMUTATIONS[y0i] as usize) % PERMUTATIONS.len()] as usize
        % GRADIENTS_2D.len();
    let gi10 = PERMUTATIONS[(x1i + PERMUTATIONS[y0i] as usize) % PERMUTATIONS.len()] as usize
        % GRADIENTS_2D.len();
    let gi01 = PERMUTATIONS[(x0i + PERMUTATIONS[y1i] as usize) % PERMUTATIONS.len()] as usize
        % GRADIENTS_2D.len();
    let gi11 = PERMUTATIONS[(x1i + PERMUTATIONS[y1i] as usize) % PERMUTATIONS.len()] as usize
        % GRADIENTS_2D.len();

    let g00 = GRADIENTS_2D[gi00];
    let g10 = GRADIENTS_2D[gi10];
    let g01 = GRADIENTS_2D[gi01];
    let g11 = GRADIENTS_2D[gi11];

    let n00 = dot2(g00, (x - x0, y - y0));
    let n10 = dot2(g10, (x - x1, y - y0));
    let n01 = dot2(g01, (x - x0, y - y1));
    let n11 = dot2(g11, (x - x1, y - y1));

    let nx0 = smooth_p5(x1 - x) * n00 + smooth_p5(x - x0) * n10;
    let nx1 = smooth_p5(x1 - x) * n01 + smooth_p5(x - x0) * n11;

    let nxy = smooth_p5(y1 - y) * nx0 + smooth_p5(y - y0) * nx1;

    nxy
}

fn main() {
    assert_eq!(almost_one_f32(), ALMOST_ONE_F32);

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
