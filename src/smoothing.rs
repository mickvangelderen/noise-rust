pub fn smooth_p3(x: f32) -> f32 {
    x * x * (-2.0 * x + 3.0)
}

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
pub fn smooth_p5(x: f32) -> f32 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

pub fn smooth_p7(x: f32) -> f32 {
    let x2 = x * x;
    x2 * x2 * (x * (x * (x * -20.0 + 70.0) - 84.0) + 35.0)
}

pub fn smooth_sin(x: f32) -> f32 {
    use std::f32::consts::PI;
    (f32::sin(PI * (x - 0.5)) + 1.0) / 2.0
}
