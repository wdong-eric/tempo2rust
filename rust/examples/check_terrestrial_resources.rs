//! Exercise public-library resource errors and successful recovery in one process.
use serde_json::json;
use std::{error::Error, fs, path::Path, process::Command};
use tempo2_rust::{RuntimeContext, evaluate_terrestrial, load};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err(
            "usage: check_terrestrial_resources RUNTIME SCRATCH_DIRECTORY CLI_BINARY".into(),
        );
    }
    let source = Path::new(&args[0]);
    let scratch = Path::new(&args[1]);
    // Scratch must be new: tests never replace the user's runtime or existing files.
    fs::create_dir(scratch)?;
    let runtime_dir = scratch.join("runtime");
    fs::create_dir(&runtime_dir)?;
    let original = RuntimeContext::new(source)?;
    for r in original.resources() {
        let path = runtime_dir.join(&r.path);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::copy(source.join(&r.path), &path)?;
    }
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/terrestrial/fixtures");
    let par = fixtures.join("test1.par");
    let tim = fixtures.join("test2.tim");
    let input = load(&par, &tim)?;
    let mut runtime = RuntimeContext::new(&runtime_dir)?;
    let baseline = evaluate_terrestrial(&input, &mut runtime)?;
    let expected = serde_json::to_value(&baseline.timing.observations)?;
    let expected_result = serde_json::to_value(&baseline)?;
    let run_cli = || {
        Command::new(&args[2])
            .args(["residuals", "--par"])
            .arg(&par)
            .arg("--tim")
            .arg(&tim)
            .arg("--runtime")
            .arg(&runtime_dir)
            .output()
    };
    let baseline_cli = run_cli()?;
    assert!(baseline_cli.status.success());
    assert_eq!(
        expected_result,
        serde_json::from_slice::<serde_json::Value>(&baseline_cli.stdout)?
    );
    // Decision 0005 reproducers use disposable copies of the exact pinned files.
    // No panic/signal catching: the public constructor itself must return Err.
    let mut grid_checks = Vec::new();
    for (name, offset, field) in [
        ("ephemeris/DE405.1950.2050", 2668, "JPL"),
        ("ephemeris/TIMEEPH_short.te405", 280, "IF99"),
    ] {
        let path = runtime_dir.join(name);
        let saved = fs::read(&path)?;
        assert_eq!(&saved[offset..offset + 8], &32.0f64.to_be_bytes());
        let mut malformed = saved.clone();
        malformed[offset..offset + 8].copy_from_slice(&1e-300f64.to_be_bytes());
        fs::write(&path, &malformed)?;
        let error = RuntimeContext::new(&runtime_dir)
            .err()
            .expect("impossible record grid accepted");
        assert_eq!(error.path, path);
        assert_eq!(error.field, field);
        assert!(error.message.contains("record grid"));
        let failed_cli = run_cli()?;
        assert_eq!(failed_cli.status.code(), Some(1));
        assert!(failed_cli.stdout.is_empty());
        let stderr = String::from_utf8(failed_cli.stderr)?;
        assert!(stderr.contains(&error.to_string()));
        fs::write(&path, &saved)?;
        // Same host process, new valid context after constructor rejection.
        let restored = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
        assert_eq!(expected_result, serde_json::to_value(&restored)?);
        let restored_cli = run_cli()?;
        assert!(restored_cli.status.success());
        assert_eq!(baseline_cli.stdout, restored_cli.stdout);
        grid_checks.push(
            json!({"resource":name,"step_offset":offset,"step_days":"1e-300",
            "library_error":error.to_string(),"same_process_library_recovery":"PASS",
            "cli_exit":1,"cli_stdout_bytes":0,"cli_stderr":stderr,
            "restored_cli_byte_identical":true}),
        );
    }
    let mut checks = Vec::new();
    for name in [
        "observatory/observatories.dat",
        "ephemeris/DE405.1950.2050",
        "ephemeris/TIMEEPH_short.te405",
        "earth/eopc04_IAU2000.62-now",
        "clock/utc2tai.clk",
    ] {
        let path = runtime_dir.join(name);
        let saved = fs::read(&path)?;
        fs::remove_file(&path)?;
        let missing = match RuntimeContext::new(&runtime_dir) {
            Err(e) => e.to_string(),
            Ok(mut ctx) => evaluate_terrestrial(&input, &mut ctx)
                .unwrap_err()
                .to_string(),
        };
        fs::write(&path, &saved)?;
        fs::write(&path, b"malformed resource\n")?;
        let malformed = match RuntimeContext::new(&runtime_dir) {
            Err(e) => e.to_string(),
            Ok(mut ctx) => evaluate_terrestrial(&input, &mut ctx)
                .unwrap_err()
                .to_string(),
        };
        fs::write(&path, &saved)?;
        let mut restored = RuntimeContext::new(&runtime_dir)?;
        assert_eq!(
            expected,
            serde_json::to_value(
                &evaluate_terrestrial(&input, &mut restored)?
                    .timing
                    .observations
            )?
        );
        checks.push(json!({"resource":name,"missing_error":missing,"malformed_error":malformed,"recovery":"PASS"}));
    }
    let obs_path = runtime_dir.join("observatory/observatories.dat");
    let saved_obs = fs::read(&obs_path)?;
    fs::write(&obs_path, "0 0 0 Parkes PKS\n")?;
    let geometry_error = match RuntimeContext::new(&runtime_dir) {
        Err(error) => error.to_string(),
        Ok(_) => panic!("invalid geometry was accepted"),
    };
    assert!(geometry_error.contains("invalid GRS80 geometry"));
    fs::write(&obs_path, &saved_obs)?;
    let huge_dm = scratch.join("overflow-dm.par");
    fs::write(
        &huge_dm,
        fs::read_to_string(&par)?.replace("DM             100.0", "DM             1e1000"),
    )?;
    let nonfinite_input = load(&huge_dm, &fixtures.join("test2.tim"))?;
    assert!(
        evaluate_terrestrial(&nonfinite_input, &mut runtime)
            .unwrap_err()
            .to_string()
            .contains("nonfinite correction")
    );
    assert_eq!(
        expected,
        serde_json::to_value(
            &evaluate_terrestrial(&input, &mut runtime)?
                .timing
                .observations
        )?
    );
    let sap = runtime_dir.join("atmosphere/pks.sap");
    let zwd = runtime_dir.join("atmosphere/pks.zwd");
    let sap_data = fs::read(&sap)?;
    let zwd_data = fs::read(&zwd)?;
    fs::remove_file(&sap)?;
    fs::remove_file(&zwd)?;
    let missing_weather = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
    assert_eq!(
        expected,
        serde_json::to_value(&missing_weather.timing.observations)?
    );
    assert!(
        missing_weather
            .corrections
            .iter()
            .all(|c| c.weather.pressure.reason == "no weather table for site")
    );
    // Valid zero rows use the same source fallback but must report the true reason.
    fs::write(&sap, "# PKS\n49000 0\n57000 0\n")?;
    fs::write(&zwd, "# PKS\n49000 0\n57000 0\n")?;
    let zero_weather = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
    assert_eq!(
        expected,
        serde_json::to_value(&zero_weather.timing.observations)?
    );
    assert!(
        zero_weather
            .corrections
            .iter()
            .all(|c| c.weather.pressure.reason == "zero table value"
                && c.weather.zenith_wet_delay.reason == "zero table value")
    );
    fs::write(&sap, "# PKS\n49000 90\n57000 110\n")?;
    fs::write(&zwd, "# PKS\n49000 1e-9\n57000 3e-9\n")?;
    let measured = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
    assert!(measured.warnings.is_empty());
    assert_eq!(
        measured.corrections[0]
            .weather
            .pressure
            .effective_value
            .as_float(),
        &92.5
    );
    assert_ne!(
        measured.corrections[0].troposphere_s,
        baseline.corrections[0].troposphere_s
    );
    fs::write(&sap, &sap_data)?;
    fs::write(&zwd, &zwd_data)?;
    // Failure after one complete TOA must not publish a prefix or mutate input.
    let path = scratch.join("late-error.tim");
    fs::write(
        &path,
        "FORMAT 1\nfirst 1400 50000 10 pks\nbad 1400 80000 10 pks\n",
    )?;
    let bad = load(&par, &path)?;
    let before = serde_json::to_vec(&bad)?;
    assert!(evaluate_terrestrial(&bad, &mut runtime).is_err());
    assert_eq!(before, serde_json::to_vec(&bad)?);
    assert_eq!(
        expected,
        serde_json::to_value(
            &evaluate_terrestrial(&input, &mut runtime)?
                .timing
                .observations
        )?
    );
    let output = json!({"record_grid_errors":grid_checks,
        "resource_errors":checks,"invalid_geometry_error":"PASS",
        "nonfinite_propagation_error_then_recovery":"PASS",
        "weather_absent_success":"PASS","weather_zero_success":"PASS",
        "weather_valid_interpolation_success":"PASS","late_failure_then_valid_same_context":"PASS","input_unchanged":"PASS"});
    fs::write(
        scratch.join("checks.json"),
        serde_json::to_vec_pretty(&output)?,
    )?;
    Ok(())
}
