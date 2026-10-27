//! Rust-only fixture harness for TEMPO2 fitting counterparts; no production code.
use crate::{
    FitNumber, load,
    std_freq::{spin_errors, spin_values, std_freq, update_std_freq},
    tkfit::tk_least_squares,
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting")
}
fn numbers(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().parse().unwrap())
        .collect()
}
fn matrix(v: &Value) -> Vec<Vec<f64>> {
    v.as_array().unwrap().iter().map(numbers).collect()
}

#[test]
fn independent_source_solver_inputs_and_callbacks() {
    assert_eq!(crate::tksvd::ld(1.0).prec(), 128);
    let root = root();
    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/m2");
    fs::create_dir_all(&out).unwrap();
    let cases: Value =
        serde_json::from_str(&fs::read_to_string(root.join("cases.json")).unwrap()).unwrap();
    // Shared-input solves include every timing iteration, so forward timing error cannot mask a solver error.
    for family in ["solver", "timing"] {
        for case in cases[family].as_array().unwrap() {
            if case["expected"] == "boundary" {
                continue;
            }
            let name = case["name"].as_str().unwrap();
            let reference: Value = serde_json::from_str(
                &fs::read_to_string(root.join("reference").join(format!("{name}.json"))).unwrap(),
            )
            .unwrap();
            let inputs = if family == "solver" {
                vec![&reference]
            } else {
                reference["cycles"].as_array().unwrap().iter().collect()
            };
            let mut results = Vec::new();
            for c in inputs {
                let mode = reference["mode"].as_u64().unwrap();
                let rescale = if family == "solver" {
                    reference["rescale_effective"].as_bool().unwrap()
                } else {
                    mode == 0 || case["rescale"].as_bool().unwrap()
                };
                let result = tk_least_squares(
                    &numbers(&c["data_s"]),
                    &numbers(&c["white_data"]),
                    &matrix(&c["matrix"]),
                    &matrix(&c["white_matrix"]),
                    rescale,
                )
                .unwrap();
                let mut projection = c.clone();
                projection["solver"] = serde_json::to_value(result).unwrap();
                results.push(projection);
            }
            fs::write(
                out.join(format!("shared_{name}.json")),
                serde_json::to_vec_pretty(&results).unwrap(),
            )
            .unwrap();
        }
    }
    let callback: Value =
        serde_json::from_str(&fs::read_to_string(root.join("reference/callbacks.json")).unwrap())
            .unwrap();
    let mut input = load(&root.join("inputs/all.par"), &root.join("inputs/bary.tim")).unwrap();
    for (k, key) in ["F0", "F1", "F2", "F3"].iter().enumerate() {
        input.parameter_cell_mut(key).val = callback["before"][k]
            .as_str()
            .unwrap()
            .parse::<crate::Scalar>()
            .unwrap()
            .0;
    }
    let x = callback["x_days"].as_str().unwrap().parse().unwrap();
    let columns: Vec<FitNumber> = std::iter::once(1.0)
        .chain((0..4).map(|k| std_freq(x, input.parameter_cell("F0").val.to_f64(), k)))
        .map(FitNumber::from)
        .collect();
    for k in 0..4 {
        update_std_freq(
            &mut input,
            k,
            numbers(&callback["increments"])[k],
            numbers(&callback["errors"])[k],
        );
    }
    let result = json!({"columns":columns,"after":spin_values(&input).map(FitNumber),"physical_errors":spin_errors(&input)});
    for k in 0..4 {
        update_std_freq(&mut input, k, 0.0, f64::NAN);
    }
    assert_eq!(
        result["physical_errors"],
        serde_json::to_value(spin_errors(&input)).unwrap()
    );
    for k in 0..4 {
        update_std_freq(&mut input, k, 0.0, f64::INFINITY);
    }
    assert_eq!(
        serde_json::to_value(spin_errors(&input)).unwrap(),
        callback["after_positive_infinity_errors"]
    );
    for k in 0..4 {
        update_std_freq(&mut input, k, 0.0, f64::NEG_INFINITY);
    }
    assert_eq!(
        serde_json::to_value(spin_errors(&input)).unwrap(),
        callback["after_negative_infinity_errors"]
    );
    fs::write(
        out.join("callbacks.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
}

#[test]
fn source_failure_boundaries_and_nonfinite_transport() {
    let root = root();
    let options = crate::FitOptions::default();
    let good = load(
        &root.join("inputs/square.par"),
        &root.join("inputs/square.tim"),
    )
    .unwrap();
    let baseline = serde_json::to_value(crate::fit_barycentric(&good, &options).unwrap()).unwrap();
    let mut evidence = Vec::new();
    for name in ["zero_error", "tiny_error"] {
        let input = load(
            &root.join("inputs/square.par"),
            &root.join(format!("inputs/{name}.tim")),
        )
        .unwrap();
        let before = serde_json::to_vec(&input).unwrap();
        let error = crate::fit_barycentric(&input, &options).unwrap_err();
        assert!(
            error.message.contains("30 iterations") || error.message.contains("w[-1]"),
            "{error}"
        );
        assert_eq!(before, serde_json::to_vec(&input).unwrap());
        assert_eq!(
            baseline,
            serde_json::to_value(crate::fit_barycentric(&good, &options).unwrap()).unwrap()
        );
        evidence.push(json!({"case":name,"error":error.to_string(),"caller_unchanged":true,"valid_recovery_equal":true}));
        // MODE 0 ignores even a zero measurement uncertainty, exactly as source whitening does.
        let mut unweighted = input.clone();
        unweighted.source.pulsars[0].fitMode = Some(0);
        assert!(crate::fit_barycentric(&unweighted, &options).is_ok());
    }
    let mut too_few = good.clone();
    too_few.source.pulsars[0].nobs = Some(1);
    assert!(
        crate::fit_barycentric(&too_few, &options)
            .unwrap_err()
            .message
            .contains("n < p")
    );
    let singular = tk_least_squares(
        &[0.0, 0.0],
        &[0.0, 0.0],
        &[vec![1.0, 0.0], vec![0.0, 1.0]],
        &[vec![1.0, 0.0], vec![0.0, 1.0]],
        true,
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&singular.errors).unwrap(),
        json!(["NaN", "NaN"])
    );
    let encoded = serde_json::to_value([
        FitNumber::from(f64::NAN),
        FitNumber::from(f64::INFINITY),
        FitNumber::from(f64::NEG_INFINITY),
    ])
    .unwrap();
    assert_eq!(encoded, json!(["NaN", "+Infinity", "-Infinity"]));
    assert!("NaN".parse::<crate::Scalar>().is_err());
    fs::create_dir_all(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/m2")).unwrap();
    fs::write(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/m2/recovery.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
}

#[test]
fn fit_controls_and_restrictions_leave_fixed_evaluation_intact() {
    use crate::SourceFile;
    let base = "PSRJ T\nF0 1 1\nPEPOCH 54000\nUNITS TCB\n";
    for control in [
        "MODE 2",
        "WEIGHT -1",
        "NITS 0",
        "NITS -1",
        "NITS 1.5",
        "NITS 2147483648",
        "MODE 1 extra",
    ] {
        let source = SourceFile {
            path: "controls.par".into(),
            sha256: String::new(),
            text: format!("{base}{control}\n"),
        };
        let error = crate::read_parfile::read_parfile(&source).err().unwrap();
        assert_eq!(error.line, Some(5));
    }
    let source = SourceFile {
        path: "controls.par".into(),
        sha256: String::new(),
        text: format!("{base}MODE 1\nWEIGHT 0\nMODE 1\nNITS 2\n"),
    };
    let parsed = crate::read_parfile::read_parfile(&source).unwrap();
    assert_eq!(parsed.fit_controls.mode, 1);
    assert_eq!(parsed.fit_controls.nits, 2);
    assert_eq!(parsed.fit_controls.declarations.len(), 4);
    let root = root();
    let valid = load(
        &root.join("inputs/square.par"),
        &root.join("inputs/square.tim"),
    )
    .unwrap();
    let default = crate::FitOptions::default();
    for (key, flag) in [("F0", 2), ("PEPOCH", 1)] {
        let mut bad = valid.clone();
        bad.parameter_cell_mut(key).fit_flag = flag;
        assert!(crate::evaluate_barycentric(&bad).is_ok());
        let before = serde_json::to_vec(&bad).unwrap();
        assert!(crate::fit_barycentric(&bad, &default).is_err());
        assert_eq!(before, serde_json::to_vec(&bad).unwrap());
        assert!(crate::fit_barycentric(&valid, &default).is_ok());
    }
    let result = crate::fit_barycentric(&valid, &default).unwrap();
    assert_eq!(result.status, "completed_with_source_nonfinite");
    assert_eq!(result.cycles[0].after_errors[0].as_float(), &0);
    assert!(result.cycles[0].offset_error_s.is_none());
    assert_eq!(
        valid.parameters()["F0"]
            .uncertainty
            .as_ref()
            .unwrap()
            .as_float(),
        &0.25
    );
    let unscaled = crate::fit_barycentric(
        &valid,
        &crate::FitOptions {
            rescale_err_chisq: false,
        },
    )
    .unwrap();
    assert_eq!(unscaled.status, "completed");
    assert_eq!(
        serde_json::to_value(&result.cycles[0].solver.covariance).unwrap(),
        serde_json::to_value(&unscaled.cycles[0].solver.covariance).unwrap()
    );
}

#[test]
fn terrestrial_fit_failure_recovers_with_the_same_runtime() {
    // This is one logical failure/recovery workflow, including diagnostic
    // collection. Other callers may run concurrently but cannot change its
    // process controls or drain its warnings between the three evaluations.
    let _workflow = crate::execution::enter();
    let root = root();
    let valid = load(
        &root.join("inputs/pks_all.par"),
        &root.join("inputs/pks.tim"),
    )
    .unwrap();
    let mut runtime = crate::RuntimeContext::new(&root.join("../../tempo2/T2runtime")).unwrap();
    let options = crate::FitOptions::default();
    let mut baseline =
        serde_json::to_value(crate::fit_terrestrial(&valid, &mut runtime, &options).unwrap())
            .unwrap();
    let mut invalid = valid.clone();
    invalid.source_observations_mut()[0].toaErr = Some(0.0);
    let before = serde_json::to_vec(&invalid).unwrap();
    let error = crate::fit_terrestrial(&invalid, &mut runtime, &options).unwrap_err();
    assert!(error.message.contains("source boundary"), "{error}");
    assert_eq!(before, serde_json::to_vec(&invalid).unwrap());
    let mut recovered =
        serde_json::to_value(crate::fit_terrestrial(&valid, &mut runtime, &options).unwrap())
            .unwrap();
    // Source weather initialization and displayMsg suppression have process
    // lifetime. Only the first prefit warning batch can differ on recovery;
    // every numerical result, resource identity and later stage remains exact.
    for result in [&mut baseline, &mut recovered] {
        result["cycles"][0]["prefit"]
            .as_object_mut()
            .unwrap()
            .remove("warnings");
    }
    assert_eq!(baseline, recovered);
}
