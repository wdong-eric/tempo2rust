//! Shared-input covariance kernels, interpolation, decomposition and MJD masks.
use tempo2_rust::{
    cholesky as c, cholesky_routines, pulsar_lifecycle::PulsarContext, tkfit::FitNumber,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kind = std::env::args()
        .nth(1)
        .ok_or("expected kernel name or mask")?;
    let numbers = |x: &[f64]| x.iter().copied().map(FitNumber::from).collect::<Vec<_>>();
    let matrix = |x: &[Vec<f64>]| x.iter().map(|r| numbers(r)).collect::<Vec<_>>();
    if kind == "mask" {
        let sat: Vec<_> = [0., 1., 2., 4., 6., 7., 8.]
            .into_iter()
            .map(|x| rug::Float::with_val(128, x))
            .collect();
        let mm: Vec<Vec<f64>> = (0..8)
            .map(|i| (0..8).map(|j| (10 * i + j + 1) as f64).collect())
            .collect();
        let mut masked = vec![vec![0.; 8]; 8];
        c::add_covariance(&mut masked, &mm, Some(&sat), 1, 1., 7.)?;
        let mut unmasked = vec![vec![0.; 8]; 8];
        c::add_covariance(&mut unmasked, &mm, None, 1, 1., 7.)?;
        println!(
            "{}",
            serde_json::json!({"masked":matrix(&masked),"unmasked":matrix(&unmasked)})
        );
        return Ok(());
    }
    let x = [0., 0.25, 2.75, 5.5, 12.5, 0.];
    let kernel = match kind.as_str() {
        "power" => c::powerlaw_covariance(4., 3., 1e-18, "", &[], 13)?,
        "beta" => c::powerlaw_covariance(4., 3., 1e-18, "beta", &[0.5], 13)?,
        "redpink" => c::powerlaw_covariance(4., 3., 1e-18, "redpink", &[1.5, 15.], 13)?,
        "qp" => c::powerlaw_covariance(4., 3., 1e-18, "qp", &[2., 0.25, 0.03, 1.], 13)?,
        "sqexp" => c::squared_exponential(0.25, 3., 13),
        "matern15" => c::matern15(0.25, 3., 13),
        "matern25" => c::matern25(0.25, 3., 13),
        "dmparam" => c::dm_covariance_parameters(1.5, 0.25, 3., 13),
        "dm" => c::dm_covariance(4., 2., 12.5, 13)?,
        _ => return Err("unknown kernel".into()),
    };
    let covariance = c::covar_function_to_matrix(&kernel, &x, 1)?;
    let mut positive = covariance.clone();
    for (i, row) in positive.iter_mut().enumerate() {
        row[i] += 0.125;
    }
    let lower = c::form_l(&positive)?;
    let inverse = c::form_linv(&positive)?;
    let mut decomposed = positive.clone();
    let diagonal = c::decompose(&mut decomposed)?;
    let white = cholesky_routines::white_residuals(&lower, &[0.1, 0.25, -0.25, 0.5, -0.5, 1.])?;
    let reconstructed = tempo2_rust::tk_matrix::mult_matrix_vec(&lower, &white)?;
    let mut context = PulsarContext::unwritten(1);
    context.initialise(1, 8, 0)?;
    let sigma = c::diagonals(&mut context, 0, &[0.125, 0.25, 0.5, 1., 2., 4.]);
    println!(
        "{}",
        serde_json::json!({"kernel":numbers(&kernel), "covariance":matrix(&covariance),
        "positive":matrix(&positive), "lower":matrix(&lower), "inverse":matrix(&inverse),
        "decomposed":matrix(&decomposed), "diagonal":numbers(&diagonal), "white":numbers(&white), "reconstructed":numbers(&reconstructed),
        "sigma":numbers(&sigma), "determinant":FitNumber::from(context.pulsars()[0].detL.unwrap())})
    );
    Ok(())
}
