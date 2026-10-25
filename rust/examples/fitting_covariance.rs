//! Automatic covariance from naturally loaded, timed and fitted observations.
use tempo2_rust::{RuntimeContext, bats, cholesky_automatic, general_fitting, tkfit::FitNumber};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("fft") && args.len() == 3 {
        let direction = args[1].parse()?;
        let n: usize = args[2].parse()?;
        let mut real: Vec<_> = (0..n).map(|i| ((i * 7 % 19) as f64 - 9.0) / 8.0).collect();
        let mut imaginary: Vec<_> = (0..n).map(|i| ((i * 3 % 17) as f64 - 8.0) / 16.0).collect();
        tempo2_rust::cholesky::fft(direction, &mut real, &mut imaginary)?;
        println!(
            "{}",
            serde_json::json!({"real":real.into_iter().map(FitNumber::from).collect::<Vec<_>>(), "imaginary":imaginary.into_iter().map(FitNumber::from).collect::<Vec<_>>() })
        );
        return Ok(());
    }
    if args.len() != 4 {
        return Err("expected RUNTIME PAR TIM OUTPUT_DIRECTORY".into());
    }
    let root = std::path::Path::new(&args[0]);
    let directory = std::path::Path::new(&args[3]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: args[1].clone().into(),
            tim: args[2].clone().into(),
        }],
        root,
    )?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    general_fitting::fit(&mut input, &mut runtime, &Default::default())?;
    let result = cholesky_automatic::automatic(
        &mut input.context,
        0,
        10.0,
        &directory.join("automatic.dcf"),
        directory,
        0,
        0,
    )?;
    let numbers = |v: &[f64]| v.iter().copied().map(FitNumber::from).collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "controls": numbers(&[result.white_corner_per_day, result.red_corner_per_year, result.alpha,
                result.model_value, result.white_noise_level, result.smoothing_days, result.prewhitening as f64,
                result.first_nfit as f64, result.first_corner_per_year, result.first_alpha, result.first_white_noise_level]),
            "covariance_function": numbers(&result.covariance_function),
            "lower": result.lower.iter().map(|r| numbers(r)).collect::<Vec<_>>(),
            "files": result.files,
        }))?
    );
    Ok(())
}
