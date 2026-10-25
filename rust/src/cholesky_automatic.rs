//! choleskyAutomatic.C automatic covariance, with source diagnostic files.
use crate::{
    TimingError, cholesky_routines as r, cholesky_spectrum as s,
    clock_diagnostics::boundary,
    clock_text::{NumberFormat, format_number},
    pulsar_lifecycle::PulsarContext,
};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub struct AutomaticCovariance {
    pub white_corner_per_day: f64,
    pub red_corner_per_year: f64,
    pub alpha: f64,
    pub model_value: f64,
    pub white_noise_level: f64,
    pub covariance_function: Vec<f64>,
    pub lower: Vec<Vec<f64>>,
    /// Source globals EXPSMOOTH, UPW, NFIT, FCFINAL, FCALPHA and WNLEVEL.
    pub smoothing_days: f64,
    pub prewhitening: i32,
    pub first_nfit: usize,
    pub first_corner_per_year: f64,
    pub first_alpha: f64,
    pub first_white_noise_level: f64,
    pub files: Vec<PathBuf>,
}

fn write_rows(
    path: &Path,
    rows: impl Iterator<Item = Vec<f64>>,
    format: NumberFormat,
    trailing_space: bool,
) -> Result<(), TimingError> {
    let mut file = std::fs::File::create(path)
        .map_err(|e| TimingError::new(path, None, "covariance output", e.to_string()))?;
    for row in rows {
        let mut bytes = Vec::new();
        for (i, value) in row.into_iter().enumerate() {
            if i > 0 {
                bytes.push(b' ');
            }
            bytes.extend(format_number(value, format));
        }
        if trailing_space {
            bytes.push(b' ');
        }
        bytes.push(b'\n');
        file.write_all(&bytes)
            .map_err(|e| TimingError::new(path, None, "covariance output", e.to_string()))?;
    }
    Ok(())
}

/// T2get_covFunc_automatic. expSmooth is overwritten by the source's 40-day
/// start; dcmflag is unused. Output paths are explicit, avoiding process chdir.
pub fn automatic(
    context: &mut PulsarContext,
    p: usize,
    _exp_smooth: f64,
    output: &Path,
    directory: &Path,
    realflag: i32,
    _dcmflag: i32,
) -> Result<AutomaticCovariance, TimingError> {
    if realflag != 0 {
        // The .cov branch never initializes fc_white before 0.2/fc_white and
        // never initializes modelAmp. It has no defined numerical oracle.
        return Err(boundary(
            "T2get_covFunc_automatic realflag branch uses uninitialized fc_white/modelAmp",
        ));
    }
    let residuals = r::timing_residuals(context, p)?;
    let (mut x, y, e) = (residuals.time_days, residuals.residual_s, residuals.sigma_s);
    let n = x.len();
    if n < 4 {
        return Err(boundary("automatic covariance requires cubic-fit data"));
    }
    let fit = r::cubic_fit(&x, &y)?;
    let coefficients = std::array::from_fn(|i| fit.increments[i].as_float().to_f64());
    let span = (x[n - 1] - x[0]).ceil();
    if !span.is_finite() || span < 0.0 || span > i32::MAX as f64 {
        return Err(boundary("automatic covariance span"));
    }
    let mut width = 40.0;
    let mut prewhite = 0;
    let mut first = None;
    let mut first_spectrum = None;
    let mut white = 0.0;
    let mut dprewhite = 0.0;
    let files = vec![
        directory.join("comp.dat"),
        directory.join("compchol.dat"),
        directory.join("cholSpec.dat"),
        output.to_owned(),
    ];
    for _ in 0..2 {
        let smooth = r::smooth_curve(&x, &y, &e, &coefficients, width)?;
        let high = r::high_frequency_residuals(&y, &smooth)?;
        let mut lower = vec![vec![0.0; n]; n];
        for i in 0..n {
            lower[i][i] = e[i];
        }
        let high_spectrum = s::calculate(&lower, &x, &high, -1)?;
        let (mut tx, mut ty) = r::interpolate(&x, &y, &e, &coefficients, 28, width)?;
        prewhite = 0;
        loop {
            let spectrum = s::periodogram(&mut tx, &mut ty, &e, prewhite, false)?;
            if spectrum.frequency_per_day.len() < 2 {
                return Err(boundary(
                    "automatic prewhitened spectrum has fewer than two frequencies",
                ));
            }
            dprewhite = spectrum.frequency_per_day[1] - spectrum.frequency_per_day[0];
            white = r::white_noise_level(
                &high_spectrum.density_year3,
                high_spectrum.density_year3.len() / 2,
            )?;
            let nspec = spectrum.frequency_per_day.len();
            if nspec > high_spectrum.frequency_per_day.len() {
                return Err(boundary(
                    "automatic comp.dat reads beyond high-frequency spectrum",
                ));
            }
            write_rows(
                &files[0],
                (0..nspec).map(|i| {
                    vec![
                        spectrum.frequency_per_day[i],
                        spectrum.density_year3[i],
                        high_spectrum.frequency_per_day[i],
                        high_spectrum.density_year3[i],
                    ]
                }),
                NumberFormat::ScientificThree,
                false,
            )?;
            let guess = r::guess_spectrum(
                &spectrum.frequency_per_day,
                &spectrum.density_year3,
                white,
                prewhite,
            )?;
            let done = (prewhite == 0 && guess.alpha > -1.0)
                || (prewhite == 1 && guess.alpha > -3.0)
                || prewhite == 2;
            first = Some(guess);
            first_spectrum = Some(spectrum);
            if done {
                break;
            }
            prewhite += 1;
        }
        width = 0.2 / first.as_ref().unwrap().white_corner_per_day;
    }
    let first = first.unwrap();
    let spectrum = first_spectrum.unwrap();
    let alpha = -first.alpha;
    let fc = first.corner_per_day * 365.25;
    let (scale, _) = r::fit_spectrum(
        &spectrum.frequency_per_day,
        &spectrum.density_year3,
        alpha,
        fc,
        first.nfit,
        -1.0,
        None,
    )?;
    let (_, lower) = r::calculate_cholesky(&x, &e, alpha, fc, scale, None)?;
    let mut whitened = crate::tk_matrix::forward_sub_vec(&lower, &y).map_err(|e| boundary(&e))?;
    let whitened_spectrum = s::periodogram(&mut x, &mut whitened, &e, 0, false)?;
    write_rows(
        &files[1],
        whitened_spectrum
            .frequency_per_day
            .iter()
            .zip(&whitened_spectrum.density_year3)
            .map(|(&x, &y)| vec![x, y]),
        NumberFormat::ScientificThree,
        true,
    )?;
    r::daily_covariance(&x, &whitened, &e, false)?;
    let improved = s::calculate(&lower, &x, &y, -1)?;
    write_rows(
        &files[2],
        improved
            .frequency_per_day
            .iter()
            .zip(&improved.density_year3)
            .map(|(&x, &y)| vec![x, y]),
        NumberFormat::ScientificThree,
        true,
    )?;
    let initial_white = white;
    let _intermediate_white =
        r::white_noise_level(&improved.density_year3, improved.density_year3.len() / 3)?;
    if improved.frequency_per_day.len() < 2 {
        return Err(boundary(
            "automatic Cholesky spectrum has fewer than two frequencies",
        ));
    }
    let count = first.nfit as f64 * dprewhite
        / (improved.frequency_per_day[1] - improved.frequency_per_day[0]);
    if !count.is_finite() || count < 0.0 || count > i32::MAX as f64 {
        return Err(boundary("automatic refined fit count"));
    }
    let nfit = count as usize;
    let (scale, _) = r::fit_spectrum(
        &improved.frequency_per_day,
        &improved.density_year3,
        alpha,
        fc,
        nfit,
        -1.0,
        None,
    )?;
    // The source repeats this identical aval=1 call; it has no state mutation
    // besides already-set T2fitSpectra time=2 and diagnostic text.
    let (function, lower) = r::calculate_cholesky(&x, &e, alpha, fc, scale, None)?;
    let final_spectrum = s::calculate(&lower, &x, &y, -1)?;
    white = r::white_noise_level(
        &final_spectrum.density_year3,
        final_spectrum.density_year3.len() / 3,
    )?;
    let final_guess = r::guess_spectrum(
        &final_spectrum.frequency_per_day,
        &final_spectrum.density_year3,
        white,
        prewhite,
    )?;
    r::fit_spectrum(
        &final_spectrum.frequency_per_day,
        &final_spectrum.density_year3,
        -final_guess.alpha,
        final_guess.corner_per_day * 365.25,
        final_guess.nfit,
        -1.0,
        None,
    )?;
    write_rows(
        output,
        std::iter::once(vec![1.0]).chain(function[..=span as usize].iter().map(|&v| vec![v])),
        NumberFormat::GeneralFifteen,
        false,
    )?;
    Ok(AutomaticCovariance {
        white_corner_per_day: first.white_corner_per_day,
        red_corner_per_year: final_guess.corner_per_day * 365.25,
        alpha: -final_guess.alpha,
        model_value: final_guess.amplitude,
        white_noise_level: white,
        covariance_function: function,
        lower,
        smoothing_days: width,
        prewhitening: prewhite,
        first_nfit: first.nfit,
        first_corner_per_year: fc,
        first_alpha: alpha,
        first_white_noise_level: initial_white,
        files,
    })
}

/// T2writeCovarFuncModel, preserving its MODEL 1 text and %g conversion.
pub fn write_model(
    path: &Path,
    alpha: f64,
    fc: f64,
    amplitude: f64,
    white: f64,
) -> Result<(), TimingError> {
    let mut bytes = b"MODEL 1\n".to_vec();
    for (name, value) in [
        ("ALPHA", alpha),
        ("FC", fc),
        ("AMP", amplitude),
        ("WHITENOISE", white),
    ] {
        bytes.extend(name.as_bytes());
        bytes.push(b' ');
        bytes.extend(format_number(value, NumberFormat::GeneralSix));
        bytes.push(b'\n');
    }
    std::fs::write(path, bytes)
        .map_err(|e| TimingError::new(path, None, "T2writeCovarFuncModel", e.to_string()))
}
