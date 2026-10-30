// Frozen 4accd26e2d51b3dc805d9a1ae0a2be7195e171ea cholesky_spectrum.rs::lomb.
// Diagnostic identity only; production uses the qualified source profile.
pub fn lomb(
    x: &[f64],
    y: &[f64],
    ofac: f64,
    hifac: f64,
) -> Result<(Vec<f64>, Vec<f64>, f64), TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n {
        return Err(boundary("TKlomb_d dimensions"));
    }
    let mean = crate::toolkit::mean_d(y);
    let variance = crate::toolkit::variance_d(y);
    let min = crate::toolkit::find_min_d(x)?;
    let max = crate::toolkit::find_max_d(x)?;
    let frequency = 1.0 / ((max - min) * ofac * (n as f64 / (n - 1) as f64));
    let center = (min + max) / 2.0;
    let count = 0.5 * ofac * hifac * n as f64;
    if !count.is_finite() || count < 0.0 || count > i32::MAX as f64 {
        return Err(boundary("TKlomb_d output count"));
    }
    let (mut alpha, mut beta, mut sine, mut cosine) = (vec![], vec![], vec![], vec![]);
    for &t in x {
        let theta = 2.0 * PI * frequency * (t - center);
        alpha.push(-2.0 * (theta / 2.0).sin() * (theta / 2.0).sin());
        beta.push(theta.sin());
        sine.push(theta.sin());
        cosine.push(theta.cos());
    }
    let (mut frequencies, mut power) = (vec![], vec![]);
    for j in 0..count as usize {
        let (mut s1, mut s2) = (0.0, 0.0);
        for i in 0..n {
            let (st, ct) = (sine[i], cosine[i]);
            s1 = st.mul_add(ct, s1);
            s2 = (ct - st).mul_add(ct + st, s2);
        }
        let tau = 0.5 * (2.0 * s1).atan2(s2);
        let (stau, ctau) = tau.sin_cos();
        let (mut s1, mut s2, mut s3, mut s4) = (0.0, 0.0, 0.0, 0.0);
        for i in 0..n {
            let (st, ct, value) = (sine[i], cosine[i], y[i] - mean);
            // GCC16/arm64 pinned native contracts these multiply/add pairs;
            // the Nyquist recurrence is sensitive to their rounding (AG witness).
            let c = ctau.mul_add(ct, st * stau);
            let s = ctau.mul_add(st, -(ct * stau));
            s1 = c.mul_add(value, s1);
            s2 = s.mul_add(value, s2);
            s3 += c.powf(2.0);
            s4 += (-ctau).mul_add(st, ct * stau).powf(2.0);
            cosine[i] += alpha[i].mul_add(ct, -(beta[i] * st));
            sine[i] += alpha[i].mul_add(st, beta[i] * ct);
        }
        frequencies.push(frequency * (j + 1) as f64);
        power.push(0.5 / variance * (s1 * s1 / s3 + s2 * s2 / s4));
    }
    Ok((frequencies, power, variance))
}
