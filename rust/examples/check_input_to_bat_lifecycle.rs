//! Public library histories: repeat evaluation and resource failures followed by recovery.
use rug::Float;
use serde_json::{Value, json};
use std::{error::Error, fs, path::Path};
use tempo2_rust::{
    PRECISION_BITS, RuntimeContext,
    bats::{self, InputPair},
};
fn moments(result: &Value) -> Vec<Float> {
    result["pulsars"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| {
            p["observations"]
                .as_array()
                .unwrap()
                .iter()
                .chain(p.get("reference_observation").filter(|v| !v.is_null()))
        })
        .flat_map(|o| {
            ["sat_mjd", "bat_mjd", "bbat_mjd"].map(|k| {
                Float::with_val(
                    PRECISION_BITS,
                    Float::parse(o[k].as_str().unwrap()).unwrap(),
                )
            })
        })
        .collect()
}
fn canonical_moments(input: &bats::BatInput) -> Vec<Float> {
    input
        .context
        .pulsars()
        .iter()
        .flat_map(|p| {
            let rows = input
                .context
                .observations(p.obsn.unwrap().unwrap())
                .unwrap();
            rows[..p.nobs.unwrap() as usize]
                .iter()
                .chain((p.refphs == Some(1)).then_some(&p.tzrobs))
                .flat_map(|o| {
                    [
                        o.sat.clone().unwrap(),
                        o.bat.clone().unwrap(),
                        o.bbat.clone().unwrap(),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
fn check_history(expected: &Value, actual: &Value) -> Result<Vec<Float>, String> {
    let identity = |result: &Value| -> Vec<Value> {
        result["pulsars"].as_array().unwrap().iter().map(|p|{let row=|o:&Value|json!([o["canonical_index"],o["filename"],o["site"]]);json!({"name":p["name"],"rows":p["observations"].as_array().unwrap().iter().map(row).collect::<Vec<_>>(),"reference":p.get("reference_observation").filter(|v|!v.is_null()).map(row)})}).collect()
    };
    if identity(expected) != identity(actual) {
        return Err("ordered pulsar/TOA/reference membership differs".into());
    }
    let (a, b) = (moments(expected), moments(actual));
    if a.len() != b.len() {
        return Err("MJD membership differs".into());
    }
    let delta: Vec<_> = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| ((a - b) * 86400i32).abs())
        .collect();
    if delta.iter().any(|d| d > &1e-12) {
        return Err(format!("MJD difference above 1e-12 seconds: {delta:?}"));
    }
    Ok(delta)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: check_input_to_bat_lifecycle RUNTIME OUTPUT_DIRECTORY".into());
    }
    let root = Path::new(&args[0]);
    let out = Path::new(&args[1]);
    fs::create_dir_all(out)?;
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/input_to_bat/fixtures");
    let pairs = [
        InputPair {
            par: fixture.join("ordinary.par"),
            tim: fixture.join("ordinary.tim"),
        },
        InputPair {
            par: fixture.join("ecliptic.par"),
            tim: fixture.join("ordinary.tim"),
        },
    ];
    let mut runtime = RuntimeContext::from_root(root)?;
    let mut input = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut input, &[], root)?;
    let first = serde_json::to_value(bats::evaluate(&mut input, &mut runtime)?)?;
    let repeated = serde_json::to_value(bats::evaluate(&mut input, &mut runtime)?)?;
    let delta = check_history(&first, &repeated)?;
    let mut controls = vec![];
    for kind in [
        "drop_all_pulsars",
        "drop_last_pulsar",
        "drop_toa",
        "drop_reference",
        "reverse_pulsars",
    ] {
        let mut changed = first.clone();
        match kind {
            "drop_all_pulsars" => changed["pulsars"].as_array_mut().unwrap().clear(),
            "drop_last_pulsar" => {
                changed["pulsars"].as_array_mut().unwrap().pop();
            }
            "drop_toa" => {
                changed["pulsars"][0]["observations"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            }
            "drop_reference" => changed["pulsars"][1]["reference_observation"] = Value::Null,
            _ => changed["pulsars"].as_array_mut().unwrap().reverse(),
        }
        assert!(
            check_history(&first, &changed).is_err(),
            "false pass: {kind}"
        );
        controls.push(kind);
    }
    // updateBatsAll uses retained clocks and ephemerides; changing only the
    // selected resource path must not make this narrower driver open it.
    let retained = canonical_moments(&input);
    let mut source_text = tempo2_rust::source_text::SourceText::new(
        Path::new("update-history.par"),
        b"EPHEM_FILE AC_UPDATE_MUST_NOT_OPEN\n".to_vec(),
    );
    tempo2_rust::source_par::read_simple_parfile(&mut input.context, 0, &mut source_text, root)?;
    let resource_count = runtime.resources().len();
    bats::update_bats_all(&mut input.context, &mut runtime)?;
    assert_eq!(resource_count, runtime.resources().len());
    let updated = canonical_moments(&input);
    assert_eq!(retained.len(), updated.len());
    let update_delta: Vec<_> = retained
        .into_iter()
        .zip(updated)
        .map(|(a, b)| ((a - b) * 86400i32).abs())
        .collect();
    assert!(update_delta.iter().all(|x| x <= &1e-12));
    let update_error = bats::form_bats_all(&mut input.context, &mut runtime)
        .unwrap_err()
        .to_string();
    assert!(
        update_error.contains("AC_UPDATE_MUST_NOT_OPEN"),
        "{update_error}"
    );
    let mut empty = tempo2_rust::pulsar_lifecycle::PulsarContext::unwritten(0);
    let empty_error = bats::form_bats_all(&mut empty, &mut runtime)
        .unwrap_err()
        .to_string();
    assert!(
        empty_error.contains("global_empty_pulsar_context"),
        "{empty_error}"
    );
    let source = fs::read_to_string(&pairs[0].par)?;
    let missing = out.join("missing-ephemeris.par");
    fs::write(
        &missing,
        source.replace("EPHEM DE405", "EPHEM AC_MISSING_EPHEMERIS"),
    )?;
    let mut bad = bats::load_pairs(
        &[InputPair {
            par: missing,
            tim: pairs[0].tim.clone(),
        }],
        root,
    )?;
    bats::preprocess(&mut bad, &[], root)?;
    let error = bats::evaluate(&mut bad, &mut runtime)
        .unwrap_err()
        .to_string();
    assert!(error.contains("AC_MISSING_EPHEMERIS"), "{error}");
    let mut recovered = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut recovered, &[], root)?;
    let recovered = serde_json::to_value(bats::evaluate(&mut recovered, &mut runtime)?)?;
    check_history(&first, &recovered)?;
    let corrupt_data = out.join("corrupt-jpl.bin");
    fs::write(&corrupt_data, b"deliberately incomplete JPL header")?;
    let corrupt_par = out.join("corrupt-ephemeris.par");
    fs::write(
        &corrupt_par,
        format!("{source}EPHEM_FILE {}\n", corrupt_data.display()),
    )?;
    let mut corrupt = bats::load_pairs(
        &[InputPair {
            par: corrupt_par,
            tim: pairs[0].tim.clone(),
        }],
        root,
    )?;
    bats::preprocess(&mut corrupt, &[], root)?;
    let corrupt_error = bats::evaluate(&mut corrupt, &mut runtime)
        .unwrap_err()
        .to_string();
    assert!(corrupt_error.contains("corrupt-jpl.bin"), "{corrupt_error}");
    let mut after_corrupt = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut after_corrupt, &[], root)?;
    let after_corrupt = serde_json::to_value(bats::evaluate(&mut after_corrupt, &mut runtime)?)?;
    check_history(&first, &after_corrupt)?;
    let calceph = out.join("calceph.par");
    fs::write(&calceph, source + "EPH_FILE unavailable.bsp\n")?;
    let mut requested = bats::load_pairs(
        &[InputPair {
            par: calceph,
            tim: pairs[0].tim.clone(),
        }],
        root,
    )?;
    bats::preprocess(&mut requested, &[], root)?;
    let unavailable = bats::evaluate(&mut requested, &mut runtime)
        .unwrap_err()
        .to_string();
    assert!(
        unavailable.contains("CALCEPH_library_unavailable"),
        "{unavailable}"
    );
    let mut final_input = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut final_input, &[], root)?;
    let final_result = serde_json::to_value(bats::evaluate(&mut final_input, &mut runtime)?)?;
    check_history(&first, &final_result)?;
    fs::write(
        out.join("lifecycle.json"),
        serde_json::to_vec_pretty(
            &json!({"repeat_difference_seconds":delta.iter().map(|d|d.to_string_radix(10,None)).collect::<Vec<_>>(),"missing_ephemeris_error":error,"corrupt_ephemeris_error":corrupt_error,"after_corrupt":after_corrupt,"narrow_update_difference_seconds":update_delta.iter().map(|x|x.to_string_radix(10,None)).collect::<Vec<_>>(),"narrow_update_resource_count":resource_count,"full_after_update_error":update_error,"empty_context_error":empty_error,"calceph_error":unavailable,"recovery":"passed","rejected_membership_mutations":controls,"first":first,"repeated":repeated,"recovered":recovered,"final":final_result}),
        )?,
    )?;
    println!(
        "repeat, narrow update, empty context, and all three resource-error recovery histories passed"
    );
    Ok(())
}
