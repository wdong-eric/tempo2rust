//! Covariance selectors on loaded, timed PAR/TIM rows, retaining input hashes.
use tempo2_rust::{
    RuntimeContext, bats, cholesky, general_fitting, timing,
    tkfit::{FitDiagnostics, FitNumber},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 5 {
        return Err("expected RUNTIME PAR TIM SELECTOR OUTPUT_DIRECTORY_OR_DASH".into());
    }
    let root = std::path::Path::new(&a[0]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: (&a[1]).into(),
            tim: (&a[2]).into(),
        }],
        root,
    )?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    timing::evaluate(&mut input, &mut runtime, true)?;
    let rows = general_fitting::get_fit_data(&mut input.context, 0)?;
    let x: Vec<_> = rows.iter().map(|r| r.x_days.as_float().to_f64()).collect();
    let sigma: Vec<_> = rows.iter().map(|r| r.sigma_s.as_float().to_f64()).collect();
    let indices: Vec<_> = rows.iter().map(|r| r.observation).collect();
    let io = (a[4] != "-").then(|| FitDiagnostics {
        directory: (&a[4]).into(),
        write_residuals: 0,
        debug: true,
    });
    let mut inputs = vec![];
    let mut history = vec![];
    if a[3] == "recovery" {
        for bad in [
            "missing.model",
            "MODEL Unknown",
            "MODEL T2SqExp missing 3",
            "cycle.model",
        ] {
            let error = cholesky::get_cholesky_matrix(
                &mut input.context,
                0,
                bad,
                &indices,
                &x,
                &sigma,
                0,
                &mut inputs,
                None,
            )
            .err()
            .ok_or("invalid model succeeded")?;
            history.push(error.to_string());
        }
    }
    let selector = if a[3] == "recovery" {
        "MODEL T2SqExp 1e-12 30"
    } else {
        &a[3]
    };
    let result = cholesky::get_cholesky_matrix(
        &mut input.context,
        0,
        selector,
        &indices,
        &x,
        &sigma,
        0,
        &mut inputs,
        io.as_ref(),
    )?;
    let matrix = |m: &[Vec<f64>]| {
        m.iter()
            .map(|r| r.iter().copied().map(FitNumber::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    println!(
        "{}",
        serde_json::json!({"covariance":matrix(&result.covariance),"lower":matrix(&result.lower),
        "determinant":FitNumber::from(input.context.pulsars()[0].detL.unwrap()),"files":result.diagnostic_files,
        "inputs":inputs,"errors":history,"indices":indices})
    );
    Ok(())
}
