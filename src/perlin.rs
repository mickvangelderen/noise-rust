use permutations::PERMUTATIONS;
use smoothing::smooth_p5;

const ALMOST_ONE_F32: f32 = 0.999_999_94;

pub fn perlin_1d(x: f32) -> f32 {
    let x0 = x.floor();

    let x0i = rem_pos(x0 as isize + 0, PERMUTATIONS.len() as isize);
    let x1i = rem_pos(x0 as isize + 1, PERMUTATIONS.len() as isize);

    let gi0 = PERMUTATIONS[x0i % PERMUTATIONS.len()] as usize & 1;
    let gi1 = PERMUTATIONS[x1i % PERMUTATIONS.len()] as usize & 1;

    // Derived the computation for each possible combination of gradients.
    let gi = (gi0 << 1) | gi1;

    let xd = x - x0;
    let fxd = smooth_p5(xd);

    // Range of these equations is [-0.5, 0.5), assuming a proper blend
    // function.
    2.0 * (match gi {
        0 => fxd - xd,
        1 => -fxd - xd + 2.0 * xd * fxd,
        2 => fxd + xd - 2.0 * xd * fxd,
        _ => -fxd + xd,
    })
}

static GRADIENTS_2D: [(f32, f32); 4] = [
    (ALMOST_ONE_F32, 0.0),
    (-1.0, 0.0),
    (0.0, ALMOST_ONE_F32),
    (0.0, -1.0),
];

fn dot_2d(a: (f32, f32), b: (f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

fn rem_pos(a: isize, b: isize) -> usize {
    let r = a % b;
    (if r < 0 { r + b } else { r }) as usize
}

pub fn perlin_2d(x: f32, y: f32) -> f32 {
    let y0 = y.floor();
    let y1 = y0 + 1.0;

    let x0 = x.floor();
    let x1 = x0 + 1.0;

    let y0i = rem_pos(y0 as isize + 0, PERMUTATIONS.len() as isize);
    let y1i = rem_pos(y0 as isize + 1, PERMUTATIONS.len() as isize);
    let x0i = rem_pos(x0 as isize + 0, PERMUTATIONS.len() as isize);
    let x1i = rem_pos(x0 as isize + 1, PERMUTATIONS.len() as isize);

    #[inline]
    fn gi(x0i: usize, y0i: usize) -> usize {
        PERMUTATIONS[(x0i + PERMUTATIONS[y0i as usize] as usize) % PERMUTATIONS.len()] as usize
            % GRADIENTS_2D.len()
    }

    let g00 = GRADIENTS_2D[gi(x0i, y0i)];
    let g10 = GRADIENTS_2D[gi(x1i, y0i)];
    let g01 = GRADIENTS_2D[gi(x0i, y1i)];
    let g11 = GRADIENTS_2D[gi(x1i, y1i)];

    let n00 = dot_2d(g00, (x - x0, y - y0));
    let n10 = dot_2d(g10, (x - x1, y - y0));
    let n01 = dot_2d(g01, (x - x0, y - y1));
    let n11 = dot_2d(g11, (x - x1, y - y1));

    let nx0 = smooth_p5(x1 - x) * n00 + smooth_p5(x - x0) * n10;
    let nx1 = smooth_p5(x1 - x) * n01 + smooth_p5(x - x0) * n11;

    let nxy = smooth_p5(y1 - y) * nx0 + smooth_p5(y - y0) * nx1;

    nxy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_table_length_is_256() {
        assert_eq!(PERMUTATIONS.len(), 256);
    }

    #[test]
    fn almost_one_is_almost_one() {
        let almost_one = unsafe { ::std::mem::transmute::<u32, f32>(0x3F7FFFFF) };
        assert_eq!(almost_one, ALMOST_ONE_F32);
        assert!(ALMOST_ONE_F32 < 1.0);
        assert_eq!(1.0, ALMOST_ONE_F32 + ::std::f32::EPSILON);
    }
}
