extern crate image;

pub mod permutations;
pub mod smoothing;
pub mod perlin;

use permutations::PERMUTATIONS;

fn lattice_to_simplex_space(x: f32, y: f32) -> (f32, f32) {
    let a = 15.0f64.to_radians().cos() as f32;
    let b = -15.0f64.to_radians().sin() as f32;
    (a * x + b * y, b * x + a * y)
}

fn simplex_to_lattice_space(x: f32, y: f32) -> (f32, f32) {
    let a = ((3.0f64.sqrt() + 1.0) / 6.0f64.sqrt()) as f32;
    let b = ((3.0f64.sqrt() - 1.0) / 6.0f64.sqrt()) as f32;
    (a * x + b * y, b * x + a * y)
}

const ALMOST_ONE_F32: f32 = 0.999_999_94;

const A: f32 = -1.0;
const B: f32 = 0.0;
const C: f32 = ALMOST_ONE_F32;

static GRADIENTS_2D: [(f32, f32); 8] = [
    // (A, A), no flat pieces.
    (A, B),
    (A, C),
    (B, A),
    (B, B),
    (B, C),
    (C, A),
    (C, B),
    (C, C),
    // (A, A),
    // (A, C),
    // (C, A),
    // (C, C),
];

fn dot_2d(a: (f32, f32), b: (f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

fn main() {
    let width = 512;
    let height = 512;
    let mut data = Vec::new();
    data.resize(width * height, 0u8);

    for row in 0..height {
        let y = row as f32 as f32 / height as f32;
        for col in 0..width {
            let x = col as f32 as f32 / width as f32;

            let (x_sim, y_sim) = (8.0*x, 8.0*y);

            let (x_lat, y_lat) = simplex_to_lattice_space(x_sim, y_sim);

            let (x0_lat, y0_lat) = (x_lat.floor(), y_lat.floor());

            let (x0i_lat, y0i_lat) = (x0_lat as usize, y0_lat as usize);

            fn g(xi: usize, yi: usize) -> (f32, f32) {
                GRADIENTS_2D[(PERMUTATIONS[(xi + PERMUTATIONS[yi as usize] as usize) & 255] as usize) % GRADIENTS_2D.len() ]
            }

            // FIXME: only doing the lower left corner.

            let g00 = g(x0i_lat + 0, y0i_lat + 0);
            let g10 = g(x0i_lat + 1, y0i_lat + 0);
            let g01 = g(x0i_lat + 0, y0i_lat + 1);
            let g11 = g(x0i_lat + 1, y0i_lat + 1);

            let (x_g00_sim, y_g00_sim) = lattice_to_simplex_space(x0_lat + 0.0, y0_lat + 0.0);
            let (x_g10_sim, y_g10_sim) = lattice_to_simplex_space(x0_lat + 1.0, y0_lat + 0.0);
            let (x_g01_sim, y_g01_sim) = lattice_to_simplex_space(x0_lat + 0.0, y0_lat + 1.0);
            let (x_g11_sim, y_g11_sim) = lattice_to_simplex_space(x0_lat + 1.0, y0_lat + 1.0);

            let (dx_g00_sim, dy_g00_sim) = (x_sim - x_g00_sim, y_sim - y_g00_sim);
            let (dx_g10_sim, dy_g10_sim) = (x_sim - x_g10_sim, y_sim - y_g10_sim);
            let (dx_g01_sim, dy_g01_sim) = (x_sim - x_g01_sim, y_sim - y_g01_sim);
            let (dx_g11_sim, dy_g11_sim) = (x_sim - x_g11_sim, y_sim - y_g11_sim);

            fn b(x: f32) -> f32 {
                x*x*x*(x*(x*6.0 - 15.0) + 10.0)
            }

            let h00 = dot_2d(g00, (dx_g00_sim, dy_g00_sim));
            let d_sq = dx_g00_sim.powi(2) + dy_g00_sim.powi(2);
            let w00 = if d_sq < 3.0/4.0 {
                b(1.0 - (d_sq/(3.0/4.0)).sqrt())
            } else {
                0.0
            };

            let h10 = dot_2d(g10, (dx_g10_sim, dy_g10_sim));
            let d_sq = dx_g10_sim.powi(2) + dy_g10_sim.powi(2);
            let w10 = if d_sq < 3.0/4.0 {
                b(1.0 - (d_sq/(3.0/4.0)).sqrt())
            } else {
                0.0
            };

            let h01 = dot_2d(g01, (dx_g01_sim, dy_g01_sim));
            let d_sq = dx_g01_sim.powi(2) + dy_g01_sim.powi(2);
            let w01 = if d_sq < 3.0/4.0 {
                b(1.0 - (d_sq/(3.0/4.0)).sqrt())
            } else {
                0.0
            };

            let h11 = dot_2d(g11, (dx_g11_sim, dy_g11_sim));
            let d_sq = dx_g11_sim.powi(2) + dy_g11_sim.powi(2);
            let w11 = if d_sq < 3.0/4.0 {
                b(1.0 - (d_sq/(3.0/4.0)).sqrt())
            } else {
                0.0
            };

            let h = if x_lat - x0_lat > y_lat - y0_lat {
                // Lower half: (0,0); (1,0); (1,1)
                w00*h00 + w10*h10 + w11*h11
            } else {
                // Upper half: (0,0); (0,1); (1,1)
                w00*h00 + w01*h01 + w11*h11
            };

            let gray: u8 = h.mul_add(128.0, 128.0) as u8;

            data[(width - 1 - row) * width + col] = gray;
        }
    }

    let file = std::fs::File::create("simplex.png").unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lat_sim_lat() {
        let p_l = (4.2, 1.3);
        let p_s = lattice_to_simplex_space(p_l.0, p_l.1);
        let p_l = simplex_to_lattice_space(p_s.0, p_s.1);
        assert!((p_l.0 - 4.2).abs() < 5.0 * std::f32::EPSILON);
        assert!((p_l.1 - 1.3).abs() < 5.0 * std::f32::EPSILON);
    }

    #[test]
    fn sim_lat_sim() {
        let p_s = (4.2, 1.3);
        let p_l = simplex_to_lattice_space(p_s.0, p_s.1);
        let p_s = lattice_to_simplex_space(p_l.0, p_l.1);
        assert!((p_s.0 - 4.2).abs() < 5.0 * std::f32::EPSILON);
        assert!((p_s.1 - 1.3).abs() < 5.0 * std::f32::EPSILON);
    }
}
