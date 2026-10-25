//! ifunc.C: linear/sinc interpolation and the source basis-mask overload.
use crate::{TimingError, clock_diagnostics::boundary};
pub fn ifunc(mjd: &[f64], offset: &[f64], t: f64, n: i32) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| interpolate(mjd, offset, t, n, &mut s.diagnostics))
}
pub(crate) fn interpolate(
    mjd: &[f64],
    offset: &[f64],
    t: f64,
    n: i32,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<f64, TimingError> {
    // ifunc.C:38 returns the first offset before inspecting N. This remains
    // defined for a nonpositive count with initialized first-cell backing.
    let first = *mjd.first().ok_or_else(|| boundary("ifunc_first_mjd"))?;
    if t < first {
        return offset
            .first()
            .copied()
            .ok_or_else(|| boundary("ifunc_first_offset"));
    }
    if n <= 0 {
        return Err(boundary("ifunc_empty_knots"));
    }
    let n = n as usize;
    let x = mjd.get(..n).ok_or_else(|| boundary("ifunc_mjd_capacity"))?;
    let y = offset
        .get(..n)
        .ok_or_else(|| boundary("ifunc_offset_capacity"))?;
    if t < x[0] {
        return Ok(y[0]);
    }
    if t >= x[n - 1] {
        return Ok(y[n - 1]);
    }
    for i in 0..n {
        if t >= x[i]
            && t < *x
                .get(i + 1)
                .ok_or_else(|| boundary("ifunc_unbracketed_time"))?
        {
            let next = x[i + 1];
            let fraction = (t - x[i]) / (next - x[i]);
            return Ok((y[i + 1] - y[i]) * fraction + y[i]);
        }
    }
    d.log(b'e', "ifunc.C", 53, b"Shouldn't get here");
    d.stderr(b"Assertion failed: (0), function ifunc, file ifunc.C, line 54.\n");
    Err(boundary("ifunc_unbracketed_time"))
}
pub fn ifunc_basis(mjd: &[f64], t: f64, n: i32, k: i32) -> Result<f64, TimingError> {
    if k >= n {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stderr(b"Assertion failed: (k<N), function ifunc, file ifunc.C, line 8.\n")
        })?;
        return Err(boundary("ifunc_basis_storage"));
    }
    if n <= 0 || n as usize > crate::pulsar::MAX_IFUNC {
        return Err(boundary("ifunc_basis_storage"));
    }
    crate::clkcorr::lifecycle_diagnostics(|d| {
        let g = |v| {
            String::from_utf8(crate::clock_text::format_number(
                v,
                crate::clock_text::NumberFormat::GeneralSix,
            ))
            .unwrap()
        };
        d.log(
            b'd',
            "ifunc.C",
            7,
            format!(
                "ifunc {} => {} : {} {n} {k}",
                g(mjd[0]),
                g(mjd[n as usize - 1]),
                g(t)
            )
            .as_bytes(),
        );
    })?;
    let mut y = vec![0.0; n as usize];
    if k >= 0 {
        y[k as usize] = 1.0;
    }
    ifunc(mjd, &y, t, n)
}
pub fn sinfunc(t: &[f64], bat: f64, k: usize) -> Result<f64, TimingError> {
    let dt = bat - *t.get(k).ok_or_else(|| boundary("sinfunc_index"))?;
    let angle =
        std::f64::consts::PI / (*t.get(1).ok_or_else(|| boundary("sinfunc_spacing"))? - t[0]) * dt;
    Ok(if angle == 0.0 {
        1.0
    } else {
        angle.sin() / angle
    })
}
