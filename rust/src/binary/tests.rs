//! AD component checks reuse immutable G state and defined return records.
use super::*;
use serde_json::Value;
use std::{fs, path::PathBuf};
fn ld(v: &Value) -> Float {
    let s = v.as_str().unwrap();
    Float::with_val(
        PRECISION_BITS,
        Float::parse(match s {
            "NaN" => "nan",
            "+Infinity" => "inf",
            "-Infinity" => "-inf",
            s => s,
        })
        .unwrap(),
    )
}
fn label(s: &str) -> usize {
    crate::tempo2::PARAMETER_NAMES
        .iter()
        .position(|v| *v == s)
        .unwrap()
}
#[test]
#[ignore = "uses accepted G local corpus and runtime"]
fn g_components() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let base = root.join("validation/binary");
    let runtime = root.join("validation/reference/build/source/T2runtime");
    let filter = std::env::var("AD_CASE").ok();
    let mut failures = vec![];
    let mut checked = 0;
    let mut withheld = 0;
    let corpus = std::env::var_os("AD_CORRECTED_CORPUS")
        .map(PathBuf::from)
        .unwrap_or_else(|| base.join("outputs"));
    for file in fs::read_dir(corpus).unwrap() {
        let file = file.unwrap().path();
        if file.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let name = file.file_stem().unwrap().to_str().unwrap();
        if filter.as_ref().is_some_and(|s| !name.contains(s)) {
            continue;
        }
        let doc: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
        let case = &doc["case"];
        let Some(records) = doc["records"].as_array() else {
            continue;
        };
        let Some(parsed) = records.iter().find(|r| r["record"] == "parsed") else {
            continue;
        };
        let Some(prepared) = records.iter().find(|r| r["record"] == "prepared") else {
            continue;
        };
        let pair = crate::bats::InputPair {
            par: base.join("fixtures").join(case["par"].as_str().unwrap()),
            tim: base.join("fixtures").join(case["tim"].as_str().unwrap()),
        };
        let mut input = crate::bats::load_pairs(&[pair], &runtime).unwrap();
        // Direct component comparison consumes G's recorded physical state below.
        // Independently executed file-to-timing comparisons belong to the public suite.
        for cell in parsed["parameters"].as_array().unwrap() {
            let l = label(cell["label"].as_str().unwrap());
            let k = cell["k"].as_u64().unwrap() as usize;
            let c = &mut input.context.pulsars[0].param.slots[l].cells[k];
            let st = &cell["state"];
            c.val = ld(&st["value"]);
            c.err = ld(&st["error"]);
            c.param_set = st["set"].as_i64().unwrap() as i32;
            c.fit_flag = st["fit"].as_i64().unwrap() as i32;
        }
        let (_, rows) = input.context.pulsar_observations_mut(0).unwrap();
        for (o, n) in rows
            .iter_mut()
            .zip(prepared["observations"].as_array().unwrap())
        {
            o.sat = Some(ld(&n["sat_mjd"]));
            o.bat = Some(ld(&n["bat_mjd"]));
            o.bbat = Some(ld(&n["bbat_mjd"]));
            for j in 0..3 {
                o.earth_ssb[j] = Some(ld(&n["earth_ssb_light_s"][j]).to_f64());
                o.psrPos[j] = Some(ld(&n["psr_position_unit"][j]).to_f64());
                if let Some(a) = n["observatory_earth_light_s"].as_array() {
                    o.observatory_earth[j] = Some(ld(&a[j]).to_f64());
                }
            }
        }
        if doc["process"]["outcome"] == "source_exit" {
            let start = records.last().unwrap();
            assert_eq!(start["record"], "start");
            let err = model(
                &mut input.context,
                0,
                start["obs"].as_u64().unwrap() as usize,
                -1,
                start["k"].as_u64().unwrap() as usize,
            )
            .unwrap_err();
            assert!(err.field.contains("source exit"), "{name}: {err}");
            checked += 1;
            continue;
        }
        let requests = fs::read_to_string(
            base.join("fixtures")
                .join(case["requests"].as_str().unwrap()),
        )
        .unwrap();
        let req: Vec<Vec<&str>> = requests
            .lines()
            .map(|s| s.split_whitespace().collect())
            .collect();
        let mut current = input.context.clone();
        let mut last = None;
        let mut previous_residual = false;
        for pair in records.windows(2) {
            let start = &pair[0];
            let expected = &pair[1];
            if start["record"] != "start" || expected["record"] != "result" {
                continue;
            }
            let id = start["id"].as_u64().unwrap() as usize;
            if last != Some(id) {
                // Binary routines borrow observations immutably and write only param.
                // Residual calls additionally own row/geometry/TZR lifecycle state.
                let residual = start["op"] == "residual";
                if residual || previous_residual {
                    current = input.context.clone();
                } else {
                    current.pulsars[0].param = input.context.pulsars[0].param.clone();
                }
                previous_residual = residual;
                last = Some(id);
            }
            let op = start["op"].as_str().unwrap();
            if op == "residual" {
                if let Err(e) =
                    crate::timing::form_residuals(&mut current, start["k"].as_i64().unwrap() == 1)
                {
                    failures.push(format!("{name}/{id} residual: {e}"));
                    continue;
                }
                let (_, rows) = current.pulsar_observations_mut(0).unwrap();
                for (o, n) in rows
                    .iter()
                    .zip(expected["observations"].as_array().unwrap())
                {
                    for (key, v, scale, gate) in [
                        ("pet_mjd", o.pet.as_ref().unwrap(), 86400.0f64, 1e-12f64),
                        ("torb_s", o.torb.as_ref().unwrap(), 1.0, 1e-12),
                        ("phase_turn", o.phase.as_ref().unwrap(), 1.0, 1e-10),
                        ("nphase_turn", o.nphase.as_ref().unwrap(), 1.0, 0.0),
                        ("residual_s", o.residual.as_ref().unwrap(), 1.0, 1e-12),
                    ] {
                        let e = ld(&n[key]);
                        if ((v.clone() - e) * scale).abs() > gate {
                            failures.push(format!("{name}/{id} integrated {key}"));
                        }
                    }
                    if o.pulseN.unwrap().to_string() != n["pulse_number"].as_str().unwrap() {
                        failures.push(format!("{name}/{id} integrated pulse count"));
                    }
                }
                checked += 1;
                continue;
            }
            if expected["source_semantics"]
                .as_str()
                .is_some_and(|s| s != "defined")
            {
                withheld += 1;
                continue;
            }
            let l = start["label"].as_str().unwrap();
            let sel = match l {
                "delay" => -1,
                "derived" => -2,
                s => label(s) as i32,
            };
            let k = start["k"].as_u64().unwrap() as usize;
            let obs = start["obs"].as_u64().unwrap() as usize;
            let before = current.pulsars[0].param.clone();
            let actual = (|| -> Result<Float, TimingError> {
                if op.starts_with("update") {
                    update(
                        &mut current,
                        0,
                        crate::tempo2::PARAMETER_LABELS[sel as usize],
                        k,
                        req[id][5].parse().unwrap(),
                        req[id][6].parse().unwrap(),
                    )?;
                }
                if op == "update_native" || op == "update_wrapper" {
                    return Ok(Float::with_val(PRECISION_BITS, 0));
                }
                if op == "wrapper" {
                    return Ok(Float::with_val(
                        PRECISION_BITS,
                        fit_function(
                            &mut current,
                            0,
                            obs,
                            crate::tempo2::PARAMETER_LABELS[sel as usize],
                            k,
                        )?,
                    ));
                }
                model(
                    &mut current,
                    0,
                    obs,
                    if op == "update_delay" { -1 } else { sel },
                    k,
                )
            })();
            let v = match actual {
                Ok(v) => v,
                Err(e) => {
                    failures.push(format!("{name}/{id}: {e}"));
                    continue;
                }
            };
            checked += 1;
            if expected.get("value").is_some() {
                let e = ld(&expected["value"]);
                let delta = (v.clone() - &e).abs();
                // Frozen per-column reference maximum, independent of Rust values.
                let max = records
                    .windows(2)
                    .filter(|p| {
                        p[0]["record"] == "start"
                            && p[0]["label"] == start["label"]
                            && p[0]["k"] == start["k"]
                            && p[1].get("value").is_some()
                    })
                    .map(|p| ld(&p[1]["value"]).abs().to_f64())
                    .fold(0.0f64, f64::max);
                let limit = if sel < 0 || op == "update_delay" {
                    1e-12
                } else {
                    5e-14 * max
                };
                let ok = if e.is_nan() {
                    v.is_nan() && v.is_sign_negative() == e.is_sign_negative()
                } else if e.is_infinite() {
                    v == e
                } else {
                    delta <= limit
                };
                if !ok {
                    failures.push(format!(
                        "{name}/{id}/{l}: got {v}, native {e}, delta {delta}, gate {limit}"
                    ));
                }
            }
            if op.starts_with("update") {
                for (field, idx) in [("increment_binary64", 5), ("error_binary64", 6)] {
                    assert_eq!(
                        ld(&expected[field]).to_f64().to_bits(),
                        req[id][idx].parse::<f64>().unwrap().to_bits(),
                        "supplied binary64 coordinate"
                    );
                }
                if expected.get("target_before").is_some() {
                    let li = sel as usize;
                    let a = &current.pulsars[0].param.slots[li].cells[k];
                    let b = &before.slots[li].cells[k];
                    let na = ld(&expected["target_after"]["value"]);
                    let nb = ld(&expected["target_before"]["value"]);
                    if na.is_finite() && nb.is_finite() && a.val.is_finite() && b.val.is_finite() {
                        let inc = a.val.clone() - &b.val;
                        let native_inc = na.clone() - &nb;
                        // Native before/after are binary128, recorded with 36 digits.
                        // One relative ulp bounds both endpoint half-ulp roundings;
                        // add decimal endpoint error and the fixed increment gate.
                        let scale = na.abs().max(&nb.abs());
                        let gate = scale
                            * (Float::with_val(PRECISION_BITS, 2).pow(-112)
                                + Float::with_val(PRECISION_BITS, 1e-35))
                            + native_inc.clone().abs() * 1e-30;
                        if (inc.clone() - &native_inc).abs() > gate {
                            failures.push(format!(
                                "{name}/{id} applied increment {inc} != {native_inc}"
                            ));
                        }
                    }
                }
            }
            if let Some(narrowed) = expected.get("narrowed_binary64") {
                let a = v.to_f64();
                let b = ld(narrowed).to_f64();
                if a.is_nan() != b.is_nan()
                    || a.is_infinite() != b.is_infinite()
                    || (!b.is_finite() && a.is_sign_negative() != b.is_sign_negative())
                {
                    failures.push(format!("{name}/{id} narrowed return class"));
                }
            }
            let mutations = expected["mutations"].as_array().unwrap();
            for (li, par) in current.pulsars[0]
                .param
                .slots
                .iter()
                .enumerate()
                .take(param_LAST as usize)
            {
                for (ki, c) in par.cells.iter().enumerate() {
                    let old = &before.slots[li].cells[ki];
                    let expected = mutations.iter().find(|m| {
                        label(m["label"].as_str().unwrap()) == li
                            && m["k"].as_u64().unwrap() as usize == ki
                    });
                    if let Some(m) = expected {
                        for (field, a) in [("value", &c.val), ("error", &c.err)] {
                            let e = ld(&m["after"][field]);
                            let diff = (a.clone() - &e).abs();
                            let limit = e.clone().abs() * 1e-30;
                            if !(if e.is_nan() {
                                a.is_nan()
                            } else if e.is_infinite() {
                                a == &e
                            } else {
                                diff <= limit
                            }) {
                                failures.push(format!(
                                    "{name}/{id} mutation {li}/{ki}/{field}: {a} != {e}"
                                ));
                            }
                        }
                        if c.param_set != m["after"]["set"].as_i64().unwrap() as i32
                            || c.fit_flag != m["after"]["fit"].as_i64().unwrap() as i32
                        {
                            failures.push(format!("{name}/{id} mutation flags {li}/{ki}"));
                        }
                    } else if c.val != old.val
                        || c.err != old.err
                        || c.param_set != old.param_set
                        || c.fit_flag != old.fit_flag
                    {
                        failures.push(format!("{name}/{id} unexpected mutation {li}/{ki}"));
                    }
                }
            }
        }
    }
    println!(
        "G checked {checked}, undefined withheld {withheld}, failures {}",
        failures.len()
    );
    for f in &failures {
        println!("{f}");
    }
    assert!(failures.is_empty());
}

#[test]
#[ignore = "diagnose the public dependency separately from G component state"]
fn ac_fixture_preparation() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let base = root.join("validation/binary");
    let runtime = root.join("validation/reference/build/source/T2runtime");
    for file in fs::read_dir(base.join("outputs")).unwrap() {
        let file = file.unwrap().path();
        if file.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let doc: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
        if !doc["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["record"] == "prepared")
        {
            continue;
        }
        let pair = crate::bats::InputPair {
            par: base
                .join("fixtures")
                .join(doc["case"]["par"].as_str().unwrap()),
            tim: base
                .join("fixtures")
                .join(doc["case"]["tim"].as_str().unwrap()),
        };
        let mut input = crate::bats::load_pairs(std::slice::from_ref(&pair), &runtime).unwrap();
        if let Err(e) = crate::bats::form_bats_all(
            &mut input.context,
            &mut crate::RuntimeContext::from_root(&runtime).unwrap(),
        ) {
            println!("NO_PREPROCESS {}: {e}", file.display());
            let mut input = crate::bats::load_pairs(&[pair], &runtime).unwrap();
            crate::bats::preprocess(&mut input, &[], &runtime).unwrap();
            let result = crate::bats::form_bats_all(
                &mut input.context,
                &mut crate::RuntimeContext::from_root(&runtime).unwrap(),
            );
            println!("PUBLIC_PREPROCESS {}: {result:?}", file.display());
        }
    }
}
