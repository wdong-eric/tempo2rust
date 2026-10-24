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
    // Clock indexing is lazy and process-wide. Copy the declared clock inputs
    // explicitly; constructing a context must not index them for this fixture.
    fs::create_dir_all(runtime_dir.join("clock"))?;
    for entry in fs::read_dir(source.join("clock"))? {
        let entry = entry?;
        if entry.path().extension().is_some_and(|x| x == "clk") {
            fs::copy(
                entry.path(),
                runtime_dir.join("clock").join(entry.file_name()),
            )?;
        }
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
    let cli_result: serde_json::Value = serde_json::from_slice(&baseline_cli.stdout)?;
    assert_eq!(expected, cli_result["observations"]);
    assert_eq!(expected_result["corrections"], cli_result["corrections"]);
    // Cached source-root observatory provenance correctly differs from the
    // fresh CLI process's scratch-root resources. The numerical projection above
    // and the direct clock provenance controls test these distinct contracts.
    assert!(baseline.resources.iter().any(|r| {
        r.path
            == source
                .join("observatory/observatories.dat")
                .to_string_lossy()
    }));
    // Decision 0005 reproducers use disposable copies of the exact pinned files.
    // Both readers reject undefined record conversion at use. IFTE no longer
    // imposes the old immutable-image header-grid restriction (0058).
    let mut grid_checks = Vec::new();
    for (name, offset, field) in [
        ("ephemeris/DE405.1950.2050", 2668, "JPL"),
        ("ephemeris/TIMEEPH_short.te405", 280, "IFTE_record_int"),
    ] {
        let path = runtime_dir.join(name);
        let saved = fs::read(&path)?;
        assert_eq!(&saved[offset..offset + 8], &32.0f64.to_be_bytes());
        let mut malformed = saved.clone();
        malformed[offset..offset + 8].copy_from_slice(&1e-300f64.to_be_bytes());
        fs::write(&path, &malformed)?;
        let error = match RuntimeContext::new(&runtime_dir) {
            Err(e) => e,
            Ok(mut ctx) => evaluate_terrestrial(&input, &mut ctx).unwrap_err(),
        };
        assert_eq!(error.path, path);
        assert_eq!(error.field, field);
        if field == "JPL" {
            assert!(error.message.contains("record_conversion"));
        } else {
            assert!(error.message.contains("invalid or unwritten storage"));
        }
        let failed_cli = run_cli()?;
        assert_eq!(failed_cli.status.code(), Some(1));
        assert!(failed_cli.stdout.is_empty());
        let stderr = String::from_utf8(failed_cli.stderr)?;
        assert!(stderr.contains(&error.to_string()));
        fs::write(&path, &saved)?;
        // Same host process, new valid context after the external-file error.
        let restored = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
        // W: source displayMsg keys persist. Numerical/resource output is stable;
        // a repeated evaluation need not repeat its first-use weather diagnostics.
        let mut repeated = serde_json::to_value(&restored)?;
        let mut first = expected_result.clone();
        first["warnings"] = json!([]);
        repeated["warnings"] = json!([]);
        assert_eq!(first, repeated);
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
    for name in ["ephemeris/DE405.1950.2050", "ephemeris/TIMEEPH_short.te405"] {
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
    // Loaded process resources survive file removal/replacement and new contexts.
    for name in [
        "observatory/observatories.dat",
        "clock/utc2tai.clk",
        "earth/eopc04_IAU2000.62-now",
    ] {
        let path = runtime_dir.join(name);
        let saved = fs::read(&path)?;
        fs::remove_file(&path)?;
        let mut next = RuntimeContext::new(&runtime_dir)?;
        assert_eq!(
            expected,
            serde_json::to_value(&evaluate_terrestrial(&input, &mut next)?.timing.observations)?
        );
        fs::write(&path, b"malformed resource\n")?;
        let mut next = RuntimeContext::new(&runtime_dir)?;
        assert_eq!(
            expected,
            serde_json::to_value(&evaluate_terrestrial(&input, &mut next)?.timing.observations)?
        );
        fs::write(&path, &saved)?;
        checks.push(json!({"resource":name,"loaded_process_cache_survives_remove_replace_and_new_context":"PASS"}));
    }
    let obs_path = runtime_dir.join("observatory/observatories.dat");
    let saved_obs = fs::read(&obs_path)?;
    fs::write(&obs_path, "0 0 0 Parkes PKS\n")?;
    // A fresh process still exercises the unchanged high-level geometry guard.
    let geometry_error = run_cli()?;
    assert_eq!(geometry_error.status.code(), Some(1));
    assert!(geometry_error.stdout.is_empty());
    assert!(String::from_utf8(geometry_error.stderr)?.contains("invalid GRS80 geometry"));
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
    // W: weather and EOP now have source process lifetimes. A new context
    // retains the actual original tables and provenance; fresh CLI processes
    // exercise the changed-file branches without pretending to reset a cache.
    let retained = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
    assert_eq!(
        expected,
        serde_json::to_value(&retained.timing.observations)?
    );
    assert_eq!(
        expected_result["corrections"],
        serde_json::to_value(&retained)?["corrections"]
    );
    let absent_cli = run_cli()?;
    assert!(absent_cli.status.success());
    let missing_weather: serde_json::Value = serde_json::from_slice(&absent_cli.stdout)?;
    assert_eq!(expected, missing_weather["observations"]);
    assert!(
        missing_weather["corrections"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["weather"]["pressure"]["reason"] == "no weather table for site")
    );
    fs::write(&sap, "# PKS\n49000 0\n57000 0\n")?;
    fs::write(&zwd, "# PKS\n49000 0\n57000 0\n")?;
    let zero_cli = run_cli()?;
    assert!(zero_cli.status.success());
    let zero_weather: serde_json::Value = serde_json::from_slice(&zero_cli.stdout)?;
    assert_eq!(expected, zero_weather["observations"]);
    assert!(
        zero_weather["corrections"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["weather"]["pressure"]["reason"] == "zero table value"
                && c["weather"]["zenith_wet_delay"]["reason"] == "zero table value")
    );
    fs::write(&sap, "# PKS\n49000 90\n57000 110\n")?;
    fs::write(&zwd, "# PKS\n49000 1e-9\n57000 3e-9\n")?;
    let measured_cli = run_cli()?;
    assert!(measured_cli.status.success());
    let measured: serde_json::Value = serde_json::from_slice(&measured_cli.stdout)?;
    assert!(
        measured["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|w| !w.as_str().unwrap().contains("TROP"))
    );
    assert_eq!(
        measured["corrections"][0]["weather"]["pressure"]["effective_value"]
            .as_str()
            .unwrap()
            .parse::<f64>()?,
        92.5
    );
    assert_ne!(
        measured["corrections"][0]["troposphere_s"],
        expected_result["corrections"][0]["troposphere_s"]
    );
    let cached = evaluate_terrestrial(&input, &mut RuntimeContext::new(&runtime_dir)?)?;
    assert_eq!(
        expected_result["corrections"],
        serde_json::to_value(&cached)?["corrections"]
    );
    for name in [
        "earth/eopc04_IAU2000.62-now",
        "atmosphere/pks.sap",
        "atmosphere/pks.zwd",
    ] {
        let actual = cached
            .resources
            .iter()
            .find(|r| r.path == source.join(name).to_string_lossy())
            .unwrap();
        let original = baseline
            .resources
            .iter()
            .find(|r| r.path == actual.path)
            .unwrap();
        assert_eq!(actual.sha256, original.sha256);
    }
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
