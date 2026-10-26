//! Actual canonical prefit timing -> TKspectrum/covariance/sigma-z, no hidden fit.
mod analysis_support;
use analysis_support::*;
use tempo2_rust::{
    sigma_z::SigmaZ,
    spectrum::{self, Options, SpectralSession},
    tempo2::Label,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    let (mut input, _runtime, _timing) = load(&a)?;
    let (psr, rows) = input.context.pulsar_observations_mut(0)?;
    let n = psr.nobs.unwrap() as usize;
    let epoch = psr.param.slots()[Label::param_pepoch as usize]
        .cell(0)?
        .value()
        .clone();
    let mut x = vec![];
    let mut y = vec![];
    let mut e = vec![];
    for (i, o) in rows[..n].iter().enumerate() {
        x.push((o.sat.clone().unwrap() - &epoch).to_f64());
        y.push(o.residual.as_ref().unwrap().to_f64());
        e.push(o.toaErr.unwrap() * 1e-6);
        emit(&format!("row.{i}.deleted"), "exact", o.deleted.unwrap());
    }
    vector("input.x", "days", &x);
    vector("input.y", "seconds", &y);
    vector("input.e", "seconds", &e);
    let options = Options {
        estimator: value(&a, "--estimator", 2)?,
        output_kind: value(&a, "--output-kind", 1)?,
        prewhite: value(&a, "--prewhite", 0)?,
        average_time: value(&a, "--average-time", 0)?,
        fit_spline: value(&a, "--spline", 0)?,
        smooth_type: value(&a, "--smooth-type", 0)?,
        smooth_width: value(&a, "--smooth-width", 1)?,
        ..Default::default()
    };
    let mut tx = x.clone();
    let mut ty = y.clone();
    let mut te = e.clone();
    let result = spectrum::analyze(&mut tx, &mut ty, &mut te, &options)?;
    emit("spectrum.reported", "exact", result.reported_count);
    emit("spectrum.units", "text", result.units);
    vector(
        "spectrum.frequency",
        "frequency_day",
        &result.frequency_per_day,
    );
    vector("spectrum.values", result.units, &result.values);
    vector("spectrum.real", "coefficient_s", &result.real);
    vector("spectrum.imaginary", "coefficient_s", &result.imaginary);
    for w in result.warnings {
        eprintln!("{w}");
    }
    let mut lower = vec![vec![0.0; n]; n];
    for i in 0..n {
        lower[i][i] = e[i];
    }
    let mut session = SpectralSession::default();
    let out = session.calc_spectra_ri(&lower, &x, &y, 3)?;
    vector("cov.frequency", "frequency_day", &out.frequency_per_day);
    vector("cov.real", "sqrt_year3", &out.real);
    vector("cov.imaginary", "sqrt_year3", &out.imaginary);
    vector("cov.error", "yr^3", &out.error);
    for mode in *b"NTI" {
        let out = session.calc_spectra_ri_t(
            &mut input.context,
            &lower,
            &x,
            &y,
            3,
            x[n - 1] - x[0],
            mode,
        )?;
        vector(
            &format!("cov{}.real", mode as char),
            "sqrt_year3",
            &out.real,
        );
        vector(
            &format!("cov{}.imaginary", mode as char),
            "sqrt_year3",
            &out.imaginary,
        );
    }
    let mut sigma = SigmaZ::default();
    let out = sigma.calculate(
        &mut input.context,
        0,
        value(&a, "--weights", -1)?,
        value(&a, "--mintau", 0.01)?,
    )?;
    emit("sigma.count", "exact", out.len());
    for (i, p) in out.iter().enumerate() {
        for (k, v) in [
            ("tau", p.log10_tau_year),
            ("sigma", p.log10_sigma_bias),
            ("plus", p.error_plus),
            ("minus", p.error_minus),
        ] {
            num(&format!("sigma.{i}.{k}"), "log_sigma", v);
        }
    }
    Ok(())
}
