//! Explicit covariance-spectrum residual diagnostic destination.
mod analysis_support;
use analysis_support::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    let x: Vec<_> = (0..8).map(|i| i as f64 * 1.25).collect();
    let y: Vec<_> = (0..8)
        .map(|i| ((i * i + 3 * i) % 11) as f64 * 1e-6)
        .collect();
    let mut lower = vec![vec![0.; 8]; 8];
    for (i, r) in lower.iter_mut().enumerate() {
        r[i] = 1e-6;
    }
    let d = tempo2_rust::tkfit::FitDiagnostics {
        directory: required(&a, "--output")?.into(),
        write_residuals: 7,
        debug: false,
    };
    let s =
        tempo2_rust::cholesky_spectrum::calculate_with_diagnostics(&lower, &x, &y, 2, Some(&d))?;
    vector(
        "diagnostic.frequency",
        "frequency_day",
        &s.frequency_per_day,
    );
    vector("diagnostic.power", "yr^3", &s.density_year3);
    Ok(())
}
