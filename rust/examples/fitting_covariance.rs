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
    let recovery = args.get(4).map(String::as_str) == Some("recovery");
    if args.len() != 4 && !(args.len() == 5 && recovery) {
        return Err("expected RUNTIME PAR TIM OUTPUT_DIRECTORY [recovery]".into());
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
    let mut fresh = recovery.then(|| input.context.clone());
    if recovery {
        let missing = directory.join("missing");
        assert!(!missing.exists());
        for (realflag, destination) in [(1, directory), (0, missing.as_path())] {
            let failure = cholesky_automatic::automatic(
                &mut input.context,
                0,
                10.,
                &destination.join("automatic.dcf"),
                destination,
                realflag,
                0,
            );
            assert!(failure.is_err());
        }
    }
    let result = cholesky_automatic::automatic(
        &mut input.context,
        0,
        10.0,
        &directory.join("automatic.dcf"),
        directory,
        0,
        0,
    )?;
    if let Some(context) = &mut fresh {
        let fresh_directory = directory.join("fresh");
        std::fs::create_dir_all(&fresh_directory)?;
        let expected = cholesky_automatic::automatic(
            context,
            0,
            10.,
            &fresh_directory.join("automatic.dcf"),
            &fresh_directory,
            0,
            0,
        )?;
        assert_eq!(result.covariance_function, expected.covariance_function);
        assert_eq!(result.lower, expected.lower);
        assert_eq!(
            (
                result.white_corner_per_day,
                result.red_corner_per_year,
                result.alpha,
                result.model_value,
                result.white_noise_level,
                result.smoothing_days,
                result.prewhitening,
                result.first_nfit,
                result.first_corner_per_year,
                result.first_alpha,
                result.first_white_noise_level
            ),
            (
                expected.white_corner_per_day,
                expected.red_corner_per_year,
                expected.alpha,
                expected.model_value,
                expected.white_noise_level,
                expected.smoothing_days,
                expected.prewhitening,
                expected.first_nfit,
                expected.first_corner_per_year,
                expected.first_alpha,
                expected.first_white_noise_level
            )
        );
        for file in &result.files {
            assert_eq!(
                std::fs::read(file)?,
                std::fs::read(fresh_directory.join(file.file_name().ok_or("output name")?))?
            );
        }
    }
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
            "failure_recovery_checked": recovery,
        }))?
    );
    Ok(())
}
