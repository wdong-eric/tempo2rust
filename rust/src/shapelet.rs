//! shapelet.C: source Hermite recurrences and scalar shapelet evaluation.
//! Explicit mul_add follows AB's GCC16 arm64 O2 contraction of the recurrence.
use crate::{TimingError, clock_diagnostics::boundary};
pub fn iter_factorial(n: u32) -> f64 {
    let mut out = 1.0;
    for i in 1..=n {
        out *= i as f64;
    }
    out
}
pub fn tnothpl(n: i32, x: f64, out: &mut [f64]) -> Result<(), TimingError> {
    *out.get_mut(0)
        .ok_or_else(|| boundary("TNothpl_output_zero"))? = 1.0;
    *out.get_mut(1)
        .ok_or_else(|| boundary("TNothpl_output_one"))? = 2.0 * x;
    let (mut y0, mut y1) = (1.0, 2.0 * x);
    for k in 2..n {
        let c = 2.0 * (k as f64 - 1.0);
        y0 /= (k as f64).sqrt();
        y1 /= (k as f64).sqrt();
        let yn = (2.0 * x + 0.0).mul_add(y1, -c * y0);
        *out.get_mut(k as usize)
            .ok_or_else(|| boundary("TNothpl_output_capacity"))? = yn;
        y0 = y1;
        y1 = yn;
    }
    Ok(())
}
pub fn iterative_tnothpl(n: i32, x: f64, y0: &mut f64, y1: &mut f64) -> f64 {
    if n == 0 {
        *y0 = 1.0;
        *y0
    } else if n == 1 {
        *y1 = 2.0 * x;
        *y1
    } else {
        let r = 1.0 / (n as f64).sqrt();
        let c = 2.0 * (n as f64 - 1.0);
        *y0 *= r;
        *y1 *= r;
        let yn = (2.0 * x + 0.0).mul_add(*y1, -c * *y0);
        *y0 = *y1;
        *y1 = yn;
        yn
    }
}
pub fn evaluate_shapelet(
    n: i32,
    pos: f64,
    width: f64,
    coef: &[f64],
    bat: f64,
) -> Result<f64, TimingError> {
    let mut result = 0.0;
    let h = (bat - pos) / (2.0f64.sqrt() * width);
    let (mut y0, mut y1) = (0.0, 0.0);
    for c in 0..n {
        let norm = 1.0
            / ((2.0 * std::f64::consts::PI).sqrt()
                * 2.0f64.powf(c as f64)
                * iter_factorial(c as u32))
            .sqrt();
        result += norm
            * *coef
                .get(c as usize)
                .ok_or_else(|| boundary("shapelet_coefficient_capacity"))?
            * iterative_tnothpl(c, h, &mut y0, &mut y1);
    }
    Ok(result * (-0.5 * ((bat - pos) / width).powi(2)).exp())
}
