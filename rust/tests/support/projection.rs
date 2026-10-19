//! Test-only adapter for the unchanged A2 barycentric-v1 contract.
use rug::Float;
use serde_json::{Value, json};
use std::{error::Error, fs, path::Path};
use tempo2_rust::{PRECISION_BITS, TimingInput, TimingResult, evaluate_barycentric, load};

fn rows(input: &TimingInput, result: &TimingResult, stage: &str) -> Vec<Value> {
    input.observations().iter().zip(&result.observations).map(|(obs, value)| {
        let mut row = json!({
            "fname": obs.original.name, "site": obs.original.site,
            "deleted": "0", "clockCorr": "0", "delayCorr": "0",
            "sat_mjd": value.sat_mjd.to_string(),
            "frequency_mhz": Float::with_val(PRECISION_BITS, obs.frequency_mhz).to_string_radix(10, None),
            "toa_error_us": Float::with_val(PRECISION_BITS, obs.toa_error_us).to_string_radix(10, None),
        });
        if stage != "parsed" {
            row["bat_mjd"] = json!(value.bat_mjd.to_string());
            row["bbat_mjd"] = json!(value.bbat_mjd.to_string());
        }
        if stage != "parsed" && stage != "bats" {
            row["phase_turn"] = json!(value.phase_turn.to_string());
            row["nphase_turn"] = json!(value.nphase_turn.to_string());
            row["pulse_number"] = json!(value.pulse_number.to_string());
            row["residual_s"] = json!(value.residual_s.to_string());
        }
        row
    }).collect()
}

fn evaluate_epochs(
    input: &TimingInput,
    par: &Path,
    epochs: &[Float],
    stage_path: &Path,
) -> Result<TimingResult, Box<dyn Error>> {
    let mut text = String::from("FORMAT 1\n");
    for (obs, epoch) in input.observations().iter().zip(epochs) {
        text.push_str(&format!(
            "{} {} {} {} @\n",
            obs.original.name,
            obs.original.frequency_mhz_text,
            epoch.to_string_radix(10, None),
            obs.original.toa_error_us_text
        ));
    }
    fs::write(stage_path, text)?;
    Ok(evaluate_barycentric(&load(par, stage_path)?)?)
}

pub fn produce(reference: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(output)?;
    for fixture in ["barycentric", "barycentric_4ns", "long_span"] {
        let stem = if fixture == "long_span" {
            "fixtures/long_span"
        } else {
            "fixtures/original/test1"
        };
        let par = reference.join(format!("{stem}.par"));
        let input = load(&par, &reference.join(format!("{stem}.tim")))?;
        let result = evaluate_barycentric(&input)?;
        let text_value = |key: &str| input.text_parameters()[key].tokens[0].clone();
        // These are the source initializer's inactive settings for the fixture subset,
        // not evidence of Rust applying terrestrial corrections or reading resources.
        let config = json!({
            "binary_model": "NONE", "calc_shapiro": "1", "clock": text_value("CLK"),
            "correct_troposphere": "1", "dilate_frequency": "1", "ephemeris": text_value("EPHEM"),
            "fit_performed": "false", "ipm": "1", "planet_shapiro": "1", "refphs": "0", "remove_mean": "0",
            "solar_wind_model": "0", "solar_wind_ne_cm3": "4", "t2c_method": "1", "time_ephemeris": "IF99",
            "tzrsite": text_value("TZRSITE"), "units": input.conventions().time_scale,
        });
        let parameters: serde_json::Map<String, Value> = input.parameters().iter().map(|(key, p)|
            (key.clone(), json!({"value": p.value.to_string(), "set": u8::from(p.set), "fit_flag": p.fit_flag}))).collect();
        let mut stages = json!({"parsed": rows(&input, &result, "parsed")});
        if fixture == "barycentric_4ns" {
            stages["direct_initial"] = json!(rows(&input, &result, "direct_initial"));
            let aligned_epochs: Vec<_> = result
                .observations
                .iter()
                .map(|row| {
                    row.sat_mjd.as_float().clone() - row.residual_s.as_float().clone() / 86400i32
                })
                .collect();
            let aligned =
                evaluate_epochs(&input, &par, &aligned_epochs, &output.join("aligned.tim"))?;
            stages["aligned"] = json!(rows(&input, &aligned, "aligned"));
            let mut perturbed_epochs = aligned_epochs;
            let four_ns = Float::with_val(PRECISION_BITS, Float::parse("4e-9")?) / 86400i32;
            for epoch in perturbed_epochs.iter_mut().skip(1) {
                *epoch += &four_ns;
            }
            let perturbed = evaluate_epochs(
                &input,
                &par,
                &perturbed_epochs,
                &output.join("perturbed.tim"),
            )?;
            stages["perturbed"] = json!(rows(&input, &perturbed, "perturbed"));
        } else {
            stages["bats"] = json!(rows(&input, &result, "bats"));
            stages["residuals"] = json!(rows(&input, &result, "residuals"));
        }
        let projection = json!({
            "schema_version": 2, "status": "completed", "availability": "available",
            "comparison_profile": "barycentric-v1", "fixture": fixture,
            "configuration": config, "parameters": parameters, "stages": stages,
            "original_observations": input.observations().iter().map(|o| &o.original).collect::<Vec<_>>(),
            "engine_provenance": result.engine,
            "input_files": {"par_sha256": input.par_source().sha256, "tim_sha256": input.tim_source().sha256},
        });
        fs::write(
            output.join(format!("{fixture}.json")),
            serde_json::to_vec_pretty(&projection)?,
        )?;
    }
    Ok(())
}
