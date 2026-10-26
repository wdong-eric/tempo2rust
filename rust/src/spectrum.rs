//! TKspectrum.C preprocessing/estimators and t2spectrum.C callback lifecycle.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards. GPL-3.0-or-later.
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
pub use crate::cholesky::fft;
pub use crate::cholesky_spectrum::{calculate as calc_spectra, lomb, weighted_lomb};
use crate::{TimingError, clock_diagnostics::boundary, toolkit as tk};
use std::f64::consts::PI;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Options {
    pub average_time: i32,
    pub smooth_width: i32,
    pub smooth_type: i32,
    pub fit_spline: i32,
    pub prewhite: i32,
    pub estimator: i32,
    pub ofac: f64,
    pub hifac: f64,
    pub output_kind: i32,
    pub calc_white: i32,
    pub diagnostic: i32,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            average_time: 0,
            smooth_width: 1,
            smooth_type: 0,
            fit_spline: 0,
            prewhite: 0,
            estimator: 2,
            ofac: 1.0,
            hifac: 1.0,
            output_kind: 1,
            calc_white: 0,
            diagnostic: 0,
        }
    }
}
#[derive(Debug, serde::Serialize)]
pub struct Analysis {
    pub scalar: crate::FitNumber,
    pub frequency_per_day: Vec<f64>,
    pub values: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub reported_count: i32,
    pub reported_count_written: bool,
    pub units: &'static str,
    pub warnings: Vec<String>,
    pub diagnostic: Option<(&'static str, Vec<f64>, Vec<f64>)>,
    pub source_exit_status: Option<i32>,
}
impl Default for Analysis {
    fn default() -> Self {
        Self {
            scalar: 0.0.into(),
            frequency_per_day: vec![],
            values: vec![],
            real: vec![],
            imaginary: vec![],
            reported_count: 0,
            reported_count_written: false,
            units: "unwritten",
            warnings: vec![],
            diagnostic: None,
            source_exit_status: None,
        }
    }
}
fn dimensions(x: &[f64], y: &[f64]) -> Result<(), TimingError> {
    if x.len() != y.len() || x.is_empty() {
        Err(boundary("spectrum empty or mismatched backing"))
    } else {
        Ok(())
    }
}
pub fn sortit(x: &mut [f64], y: &mut [f64]) -> Result<(), TimingError> {
    if x.len() != y.len() {
        return Err(boundary("TKsortit dimensions"));
    }
    while let Some(i) = x.windows(2).position(|p| p[0] > p[1]) {
        x.swap(i, i + 1);
        y.swap(i, i + 1);
    }
    Ok(())
}
pub fn first_difference(y: &mut [f64]) {
    let sy: Vec<f32> = y.iter().map(|&v| v as f32).collect();
    for i in 0..y.len() {
        y[i] = if i == 0 {
            0.0
        } else {
            (sy[i] - sy[i - 1]) as f64
        };
    }
}
/// TKaveragePts recursively splits the largest gap, preserving first ties.
pub fn average_pts(x: &[f64], y: &[f64], width: i32) -> Result<(Vec<f64>, Vec<f64>), TimingError> {
    dimensions(x, y)?;
    if tk::range_d(x)? < width as f64 {
        return Ok((vec![tk::mean_d(x)], vec![tk::mean_d(y)]));
    }
    let (mut gap, mut split) = (-1.0, None);
    for i in 1..x.len() {
        if gap < (x[i] - x[i - 1]).abs() {
            gap = (x[i] - x[i - 1]).abs();
            split = Some(i);
        }
    }
    let i = split.ok_or_else(|| boundary("TKaveragePts source error: no largest gap"))?;
    let (mut a, mut b) = average_pts(&x[..i], &y[..i], width)?;
    let (c, d) = average_pts(&x[i..], &y[i..], width)?;
    a.extend(c);
    b.extend(d);
    Ok((a, b))
}
/// TKboxcar and TKhann retain the source's exclusive right endpoint.
fn smooth(
    x: &[f64],
    y: &[f64],
    width: i32,
    hann: bool,
) -> Result<(Vec<f64>, Vec<f64>), TimingError> {
    dimensions(x, y)?;
    if width < 0 {
        return Err(boundary("smoother negative backing index"));
    }
    let half = ((width - 1) / 2) as usize;
    let (mut ox, mut oy) = (vec![], vec![]);
    if half >= x.len() || half > x.len() - half {
        return Ok((ox, oy));
    }
    for i in half..x.len() - half {
        let mut sum = 0.0;
        let mut count = 0;
        for j in i - half..i + half {
            sum += if hann {
                0.5 * (1.0 - (2.0 * PI * (j - (i - half)) as f64 / width as f64).cos()) * y[j]
            } else {
                y[j]
            };
            count += 1;
        }
        ox.push(x[i]);
        oy.push(
            sum / if hann {
                width as f64 / 2.0
            } else {
                count as f64
            },
        );
    }
    Ok((ox, oy))
}
pub fn boxcar(x: &[f64], y: &[f64], width: i32) -> Result<(Vec<f64>, Vec<f64>), TimingError> {
    if width % 2 == 0 {
        crate::clkcorr::with_process(|s| {
            s.diagnostics
                .stdout(format!("Should use odd number for the width. Used {width}\n").as_bytes());
            Ok(())
        })?;
    }
    smooth(x, y, width, false)
}
pub fn hann(x: &[f64], y: &[f64], width: i32) -> Result<(Vec<f64>, Vec<f64>), TimingError> {
    smooth(x, y, width, true)
}
/// TKcmonot: Fritsch-Carlson / Dave Hale 1989–1991 source coefficients.
pub fn cmonot(x: &[f64], y: &[f64]) -> Result<Vec<[f64; 4]>, TimingError> {
    dimensions(x, y)?;
    let n = x.len();
    let mut yd: Vec<_> = y.iter().map(|&v| [v, 0.0, 0.0, 0.0]).collect();
    if n == 1 {
        return Ok(yd);
    }
    if n == 2 {
        yd[0][1] = (y[1] - y[0]) / (x[1] - x[0]);
        yd[1][1] = yd[0][1];
        return Ok(yd);
    }
    let (mut h1, mut h2) = (x[1] - x[0], x[2] - x[1]);
    let mut hsum = h1 + h2;
    let (mut del1, mut del2) = ((y[1] - y[0]) / h1, (y[2] - y[1]) / h2);
    yd[0][1] = (h1 + hsum) / hsum * del1 + (-h1 / hsum) * del2;
    if yd[0][1] * del1 <= 0.0 {
        yd[0][1] = 0.0;
    } else if del1 * del2 < 0.0 && yd[0][1].abs() > (3.0 * del1).abs() {
        yd[0][1] = 3.0 * del1;
    }
    for i in 1..n - 1 {
        h1 = x[i] - x[i - 1];
        h2 = x[i + 1] - x[i];
        hsum = h1 + h2;
        del1 = (y[i] - y[i - 1]) / h1;
        del2 = (y[i + 1] - y[i]) / h2;
        if del1 * del2 <= 0.0 {
            yd[i][1] = 0.0;
        } else {
            let hsum3 = hsum + hsum + hsum;
            let w1 = (hsum + h1) / hsum3;
            let w2 = (hsum + h2) / hsum3;
            let dmin = tk::ret_min_d(del1.abs(), del2.abs());
            let dmax = tk::ret_max_d(del1.abs(), del2.abs());
            yd[i][1] = dmin / (w1 * (del1 / dmax) + w2 * (del2 / dmax));
        }
    }
    yd[n - 1][1] = (-h2 / hsum) * del1 + (h2 + hsum) / hsum * del2;
    if yd[n - 1][1] * del2 <= 0.0 {
        yd[n - 1][1] = 0.0;
    } else if del1 * del2 < 0.0 && yd[n - 1][1].abs() > (3.0 * del2).abs() {
        yd[n - 1][1] = 3.0 * del2;
    }
    for i in 0..n - 1 {
        h2 = x[i + 1] - x[i];
        del2 = (y[i + 1] - y[i]) / h2;
        let divdf3 = yd[i][1] + yd[i + 1][1] - 2.0 * del2;
        yd[i][2] = 2.0 * (del2 - yd[i][1] - divdf3) / h2;
        yd[i][3] = (divdf3 / h2) * (6.0 / h2);
    }
    yd[n - 1][2] = yd[n - 2][2] + (x[n - 1] - x[n - 2]) * yd[n - 2][3];
    yd[n - 1][3] = yd[n - 2][3];
    Ok(yd)
}
pub fn spline_interpolate(
    x: &[f64],
    yd: &[[f64; 4]],
    interp_x: &[f64],
) -> Result<Vec<f64>, TimingError> {
    if yd.len() != x.len() {
        return Err(boundary("TKspline_interpolate dimensions"));
    }
    Ok(interp_x
        .iter()
        .map(|&v| {
            if let Some(j) = x.windows(2).position(|p| v >= p[0] && v < p[1]) {
                let h = v - x[j];
                yd[j][0] + h * (yd[j][1] + h * (yd[j][2] / 2.0 + h * yd[j][3] / 6.0))
            } else {
                0.0
            }
        })
        .collect())
}
pub fn interpolate_spline_smooth_fixed_x_pts(
    x: &[f64],
    y: &[f64],
    interp_x: &[f64],
) -> Result<Vec<f64>, TimingError> {
    spline_interpolate(x, &cmonot(x, y)?, interp_x)
}
/// TK_dft writes n coefficients, but reports n/2.
pub fn dft(x: &[f64], y: &[f64]) -> Result<Analysis, TimingError> {
    dimensions(x, y)?;
    let n = x.len();
    let range = tk::range_d(x)? * n as f64 / (n as f64 - 1.0);
    let mut out = Analysis {
        reported_count: (n / 2) as i32,
        reported_count_written: true,
        units: "s^2",
        ..Default::default()
    };
    for i in 0..n {
        let f = 1.0 / range * (i + 1) as f64;
        let (mut re, mut im) = (0.0, 0.0);
        for j in 0..n {
            let t = x[j] * 86400.0;
            let hz = f / 86400.0;
            re += y[j] * (2.0 * PI * hz * t).cos();
            im += y[j] * (2.0 * PI * hz * t).sin();
        }
        out.frequency_per_day.push(f);
        out.real.push(re);
        out.imaginary.push(im);
        out.values.push(re.powf(2.0) + im.powf(2.0));
    }
    Ok(out)
}
/// TK_fitSinusoids, with valid diagonal covariance backing.
pub fn fit_sinusoids(
    x: &[f64],
    y: &[f64],
    sig: &[f64],
) -> Result<crate::cholesky_spectrum::Spectrum, TimingError> {
    if x.len() != sig.len() {
        return Err(boundary("TK_fitSinusoids dimensions"));
    }
    // 0053: TKspectrum.C:453 passes a 1xN inverse-sigma allocation to
    // calcSpectraErr, whose TKforwardSub requires a full lower L. Build
    // diagonal sigma so L^-1 is the intended 1/sigma weighting; no invalid
    // original memory is executed or used as a numerical oracle.
    let mut lower = vec![vec![0.0; sig.len()]; sig.len()];
    for i in 0..sig.len() {
        lower[i][i] = sig[i];
    }
    crate::cholesky_spectrum::calculate(&lower, x, y, (x.len() / 2) as i32)
}
pub fn analyze(
    x: &mut [f64],
    y: &mut [f64],
    errors: &mut [f64],
    o: &Options,
) -> Result<Analysis, TimingError> {
    dimensions(x, y)?;
    let n = x.len();
    // 0053, TKspectrum.C:104 at fda6ad4a: TKsortit moved x/y without
    // corresponding e, corrupting weighted row identity. Sort all three.
    let mut order: Vec<_> = (0..n).map(|i| i as f64).collect();
    if errors.len() >= n {
        tk::sort_3d(x, y, &mut errors[..n])?;
    } else {
        // Earlier diagnostic/white-noise exits never consume weights (0090).
        tk::sort_3d(x, y, &mut order)?;
    }
    let mut out = Analysis::default();
    macro_rules! diagnostic {
        ($stage:expr,$label:expr,$x:expr,$y:expr) => {
            if o.diagnostic == $stage {
                out.diagnostic = Some(($label, $x.to_vec(), $y.to_vec()));
                out.source_exit_status = Some(1);
                return Ok(out);
            }
        };
    }
    diagnostic!(1, "sortdata", x, y);
    let (mut sx, mut sy) = if o.average_time == 0 {
        (x.to_vec(), y.to_vec())
    } else {
        average_pts(x, y, o.average_time)?
    };
    diagnostic!(2, "averagePoints", sx, sy);
    tk::zero_mean_d(&mut sy);
    diagnostic!(3, "meanRemoved", sx, sy);
    if o.fit_spline == 1 {
        // 0053, TKspectrum.C:91,165: n-sized scratch was written through
        // 4096 cells. Allocate the actual fixed grid; grid/normalization unchanged.
        let ix: Vec<_> = (0..4096)
            .map(|i| sx[0] + i as f64 * (sx[sx.len() - 1] - sx[0]) / 4096.0)
            .collect();
        sy = interpolate_spline_smooth_fixed_x_pts(&sx, &sy, &ix)?;
        sx = ix;
        diagnostic!(4, "spline", sx, sy);
    }
    match o.smooth_type {
        0 => {}
        1 => {
            out.warnings
                .push(format!("SmoothWidth = {}", o.smooth_width));
            (sx, sy) = hann(&sx, &sy, o.smooth_width)?;
        }
        _ => return Err(boundary("TKspectrum unknown SmoothType; source exit(1)")),
    }
    diagnostic!(5, "smooth", sx, sy);
    if o.calc_white == 1 {
        let white_width = if o.smooth_type == 1 {
            o.smooth_width.max(1)
        } else {
            o.smooth_width
        };
        let interp = interpolate_spline_smooth_fixed_x_pts(&sx, &sy, x)?;
        let (mut xv, mut xvsq, mut c) = (0.0, 0.0, 0);
        for i in 0..n {
            y[i] -= interp[i];
            if x[i] > x[0] + white_width as f64 / 2.0 && x[i] < x[n - 1] - white_width as f64 / 2.0
            {
                xv += y[i];
                xvsq += y[i] * y[i];
                c += 1;
            }
        }
        out.scalar = (xvsq / c as f64 - (xv / c as f64).powf(2.0)).sqrt().into();
        out.units = "s";
        return Ok(out);
    }
    if sy.len() < 2 {
        return Err(boundary("TKspectrum insufficient transformed rows"));
    }
    if o.prewhite == 1 || o.prewhite == 2 {
        for _ in 0..o.prewhite {
            first_difference(&mut sy);
        }
        if o.prewhite == 2 {
            sy[1] = 0.0;
        }
    }
    let span = tk::range_d(&sx)?;
    let nsmooth = sx.len();
    let mut variance = 1.0;
    if matches!(o.estimator, 4 | 5) {
        if errors.len() < nsmooth {
            return Err(boundary(
                "TKspectrum transformed rows exceed supplied positional-error backing",
            ));
        }
        if errors.len() < n {
            let sorted = order[..nsmooth]
                .iter()
                .map(|&i| {
                    errors.get(i as usize).copied().ok_or_else(|| {
                        boundary("TKspectrum sorted row lacks supplied associated error")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            errors[..nsmooth].copy_from_slice(&sorted);
        }
        // 0090: TKspectrum.C:113–392 at fda6ad4a retains supplied errors
        // positionally after transforms; no transformed covariance is inferred.
        let mut transforms = Vec::new();
        if o.average_time != 0 {
            transforms.push("averaging");
        }
        if o.fit_spline == 1 {
            transforms.push("spline interpolation");
        }
        if o.smooth_type == 1 {
            transforms.push("Hann smoothing");
        }
        match o.prewhite {
            1 => transforms.push("first differencing"),
            2 => transforms.push("second differencing"),
            _ => {}
        }
        if !transforms.is_empty() {
            out.warnings.push(format!("TKspectrum: {} retains original positional errors without propagated uncertainty/covariance", transforms.join(", ")));
        }
    }
    let nf;
    match o.estimator {
        1 => {
            let d = dft(&sx, &sy)?;
            nf = d.reported_count as usize;
            out.frequency_per_day = d.frequency_per_day[..nf].to_vec();
            out.values = d.values[..nf].to_vec();
            out.real = d.real;
            out.imaginary = d.imaginary;
        }
        2 => {
            (out.frequency_per_day, out.values, variance) = lomb(&sx, &sy, o.ofac, o.hifac)?;
            nf = out.values.len();
        }
        3 => {
            let mut im = vec![0.0; nsmooth];
            fft(1, &mut sy, &mut im)?;
            nf = nsmooth / 2 - 1;
            for i in 1..nsmooth / 2 {
                out.frequency_per_day.push(i as f64 / span);
                out.values.push(sy[i].powf(2.0) + im[i].powf(2.0));
            }
        }
        4 => {
            (out.frequency_per_day, out.values, out.real, out.imaginary) =
                weighted_lomb(&sx, &sy, &errors[..nsmooth])?;
            nf = out.values.len();
        }
        5 => {
            let s = fit_sinusoids(&sx, &sy, &errors[..nsmooth])?;
            out.frequency_per_day = s.frequency_per_day;
            out.values = s.density_year3;
            nf = out.values.len();
        }
        _ => return Err(boundary("TKspectrum undefined estimator output")),
    }
    for v in &mut out.values {
        match (o.estimator, o.output_kind) {
            (1, 1) => {
                *v = (*v / (365.25_f64 * 86400.0).powf(2.0)) * 2.0 * (span / 365.25)
                    / nsmooth as f64
                    / nsmooth as f64
            }
            (1, 2) => *v = v.sqrt() / nf as f64,
            (1, 3) => *v = *v / nf as f64 / nf as f64,
            (2, 1) => {
                *v *= 2.0 * variance * nf as f64 * o.ofac / o.hifac;
                *v = (*v / (365.25_f64 * 86400.0).powf(2.0)) * 2.0 * (span / 365.25)
                    / nsmooth as f64
                    / nsmooth as f64;
            }
            (2, 2) => *v = (*v * 2.0 * variance / nf as f64).sqrt(),
            (2, 3) => *v = *v * 2.0 * variance / nf as f64 * o.ofac * o.hifac,
            (2, 4) => *v = *v * 2.0 / nf as f64 * o.ofac * o.hifac,
            (3, 1) => *v = (*v / (365.25_f64 * 86400.0).powf(2.0)) * 2.0 * (span / 365.25),
            (3, 2) => *v = (*v * 4.0).sqrt(),
            (3, 3) => *v *= 4.0,
            (4 | 5, 1) => *v = (*v / (365.25_f64 * 86400.0).powf(2.0)) * (span / 365.25) / 2.0,
            (4 | 5, 2) => *v = v.sqrt(),
            _ => {}
        }
    }
    // Estimator 5 already returns yr^3 before the driver's additional source normalization.
    out.units = match (o.estimator, o.output_kind) {
        (5, 1) => "yr^3 * yr / s^2",
        (5, 2) => "yr^(3/2)",
        (5, _) => "yr^3",
        (_, 1) => "yr^3",
        (_, 2) => "s",
        (2, 4) => "dimensionless",
        (_, 3) => "s^2",
        (2, _) => "dimensionless",
        _ => "s^2",
    };
    if o.prewhite == 1 || o.prewhite == 2 {
        for (f, v) in out.frequency_per_day.iter().zip(&mut out.values) {
            if *f != 0.0 {
                let filter = 4.0 * (PI * f * (sx[1] - sx[0])).sin().powf(2.0);
                *v /= if o.prewhite == 1 {
                    filter
                } else {
                    filter.powf(2.0)
                };
            }
        }
    }
    diagnostic!(6, "spectrum", out.frequency_per_day, out.values);
    out.reported_count = nf as i32 - 1;
    out.reported_count_written = true;
    Ok(out)
}

/// t2spectrum.C::GLOBAL_OMEGA, explicit reusable callback state.
#[derive(Debug, Default)]
pub struct SpectralSession {
    pub omega: f64,
}
impl SpectralSession {
    pub fn fit_mean_sine_func(&self, x: f64) -> [f64; 3] {
        [1.0, (self.omega * x).cos(), (self.omega * x).sin()]
    }
    pub fn fit_cos_sine_func(&self, x: f64) -> [f64; 2] {
        [(self.omega * x).cos(), (self.omega * x).sin()]
    }
    pub fn fit_mean_sine_func_ifunc(
        &self,
        context: &mut crate::pulsar_lifecycle::PulsarContext,
        p: usize,
        x: f64,
        ival: usize,
    ) -> Result<[f64; 3], TimingError> {
        use crate::observation::read;
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        let n = *read(&psr.nobs, "nobs")? as usize;
        if ival >= n {
            return Err(boundary("IFUNC spectrum observation index"));
        }
        let mut v = [0.0; 3];
        let mut area = 0.0;
        let w = self.omega;
        for right in [true, false] {
            if (right && ival + 1 < n) || (!right && ival > 0) {
                let j = if right { ival + 1 } else { ival - 1 };
                let x1 = (crate::tksvd::ld(x)
                    + (read(&rows[j].sat, "SAT")?.clone() - read(&rows[ival].sat, "SAT")?))
                .to_f64();
                let m = if right { -1.0 } else { 1.0 } / (x - x1).powf(2.0);
                let c = if right { x1 } else { -x1 } / (x - x1).powf(2.0);
                let integral = |t: f64| {
                    [
                        m * t * t / 2.0 + c * t,
                        (m * (w * t).cos() - w * (c + m * t) * (w * t).sin()) / w / w,
                        (m * (w * t).sin() - w * (c + m * t) * (w * t).cos()) / w / w,
                    ]
                };
                let (a, b) = (integral(x), integral(x1));
                for k in 0..3 {
                    v[k] += if right { b[k] - a[k] } else { a[k] - b[k] };
                }
                area += 0.5;
            }
        }
        for a in &mut v {
            *a /= area;
        }
        Ok(v)
    }
    /// Current calcSpectra_ri delegates without changing GLOBAL_OMEGA.
    pub fn calc_spectra_ri(
        &mut self,
        lower: &[Vec<f64>],
        x: &[f64],
        y: &[f64],
        nfit: i32,
    ) -> Result<crate::cholesky_spectrum::Spectrum, TimingError> {
        crate::cholesky_spectrum::calculate(lower, x, y, nfit)
    }
    pub fn calc_spectra_ri_t(
        &mut self,
        context: &mut crate::pulsar_lifecycle::PulsarContext,
        lower: &[Vec<f64>],
        x: &[f64],
        y: &[f64],
        nfit: i32,
        t: f64,
        mode: u8,
    ) -> Result<crate::cholesky_spectrum::Spectrum, TimingError> {
        dimensions(x, y)?;
        let n = x.len();
        if n < 2 {
            return Err(boundary("calcSpectra_ri_T dimensions"));
        }
        let nf = if nfit < 0 { n / 2 - 1 } else { nfit as usize };
        let binfactor = if mode == b'T' {
            1.0
        } else {
            n as f64 / (n - 1) as f64
        };
        let mut out = crate::cholesky_spectrum::Spectrum {
            frequency_per_day: vec![],
            density_year3: vec![],
            real: vec![],
            imaginary: vec![],
            error: vec![],
        };
        for k in 0..nf {
            self.omega = 2.0 * PI / (t * binfactor) * (k + 1) as f64;
            // TKrobust.C:149–152 at fda6ad4a dereferences optional e/Ocvm
            // from t2spectrum's NULL outputs. Owned solver results keep these
            // outputs valid; the corrected native witness guards those writes.
            let fit = crate::legacy_fitting::single_pulsar(
                context,
                x,
                y,
                &(0..n).collect::<Vec<_>>(),
                3,
                lower,
                |ctx, p, x, i, v| {
                    let a = if mode == b'I' {
                        self.fit_mean_sine_func_ifunc(ctx, p, x, i)?
                    } else {
                        self.fit_mean_sine_func(x)
                    };
                    v.copy_from_slice(&a);
                    Ok(())
                },
                1e-40,
                true,
            )?;
            let v = (x[n - 1] - x[0]) / 365.25 / 2.0 / (365.25_f64 * 86400.0).powf(2.0);
            out.frequency_per_day.push(self.omega / 2.0 / PI);
            let re = v.sqrt() * fit.increments[1].as_float().to_f64();
            let im = v.sqrt() * fit.increments[2].as_float().to_f64();
            out.real.push(re);
            out.imaginary.push(im);
            out.density_year3.push(re.powf(2.0) + im.powf(2.0));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn weighted_elimination_recovers_noiseless_coefficients() {
        let x = [0.0, 0.3, 1.1, 2.7, 4.2, 6.0, 7.4, 10.0];
        let e = [1.0, 2.0, 1.5, 0.8, 1.2, 2.1, 0.9, 1.4];
        let w = 2.0 * PI / (10.0 * 8.0 / 7.0);
        let y: Vec<_> = x
            .iter()
            .map(|v| 2.0 * (w * v).sin() + 3.0 * (w * v).cos())
            .collect();
        let (_, _, re, im) = weighted_lomb(&x, &y, &e).unwrap();
        assert!((re[0] - 3.0).abs() < 1e-14);
        assert!((im[0] - 2.0).abs() < 1e-14);
    }
    #[test]
    fn corrected_sort_and_fixed_grid() {
        let mut x = vec![2.0, 0.0, 1.0, 3.0];
        let mut y = vec![2.0, 0.0, 1.0, 3.0];
        let mut e = vec![20.0, 1.0, 10.0, 30.0];
        let out = analyze(
            &mut x,
            &mut y,
            &mut e,
            &Options {
                fit_spline: 1,
                diagnostic: 4,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(e, [1.0, 10.0, 20.0, 30.0]);
        assert_eq!(out.diagnostic.unwrap().1.len(), 4096);
        assert!(
            analyze(
                &mut x,
                &mut y,
                &mut e,
                &Options {
                    fit_spline: 1,
                    estimator: 4,
                    ..Default::default()
                }
            )
            .is_err()
        );
        let d = dft(&x, &y).unwrap();
        assert_eq!(d.reported_count, 2);
        assert_eq!(d.real.len(), 4);
    }
}
