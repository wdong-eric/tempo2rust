//! Test-only terrestrial-v1 projection; never invokes the reference executable.
use rug::Float;
use serde_json::{Value, json};
use std::{error::Error, fs, path::Path};
use tempo2_rust::{
    PRECISION_BITS, RuntimeContext, TerrestrialResult, TimingInput, evaluate_terrestrial, load,
};

fn rows(input: &TimingInput, result: &TerrestrialResult, stage: &str) -> Vec<Value> {
    input.observations().iter().zip(&result.timing.observations).zip(&result.corrections).map(|((obs,value),c)| {
        let mut row=json!({
            "fname":obs.original.name,"site":obs.original.site,"deleted":"0","clockCorr":"1","delayCorr":"1",
            "sat_mjd":value.sat_mjd.to_string(),
            "frequency_mhz":Float::with_val(PRECISION_BITS,obs.frequency_mhz).to_string_radix(10,None),
            "toa_error_us":Float::with_val(PRECISION_BITS,obs.toa_error_us).to_string_radix(10,None)
        });
        if stage!="parsed" {
            for (key,v) in [("bat_mjd",&value.bat_mjd),("bbat_mjd",&value.bbat_mjd),
                ("bat_correction_day",&c.bat_correction_day),("clock_tt_s",&c.clock_tt_s),("tt_tb_s",&c.tt_tb_s),
                ("tt_teph_s",&c.tt_teph_s),("roemer_s",&c.roemer_s),("troposphere_s",&c.troposphere_s),
                ("dispersion_s",&c.dispersion_s),("solar_dispersion_s",&c.solar_dispersion_s),("frequency_ssb_hz",&c.frequency_ssb_hz)] {
                row[key]=json!(v.to_string());
            }
            row["shklovskii_s"]=json!("0");
            for (key,vector) in [("earth_ssb_light_s",&c.earth_ssb_light_s),("observatory_earth_light_s",&c.observatory_earth_light_s)] {
                for (i,v) in vector.iter().enumerate() {row[format!("{key}_{i}")]=json!(v.to_string());}
            }
            for (key,v) in ["sun","venus","jupiter","saturn","uranus","neptune"].iter().zip(&c.shapiro_s) {
                row[format!("shapiro_{key}_s")]=json!(v.to_string());
            }
            for (i,step) in c.clock_steps.iter().enumerate() {
                row[format!("clock_to_{i}")]=json!(step.corrects_to);
                row[format!("clock_step_s_{i}")]=json!(step.correction_s.to_string());
            }
        }
        if stage=="residuals" {
            row["binary_delay_s"]=json!("0");
            row["phase_turn"]=json!(value.phase_turn.to_string());
            row["nphase_turn"]=json!(value.nphase_turn.to_string());
            row["pulse_number"]=json!(value.pulse_number.to_string());
            row["residual_s"]=json!(value.residual_s.to_string());
        }
        row
    }).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: compare_terrestrial RUNTIME OUTPUT_DIRECTORY".into());
    }
    let root = Path::new(&args[0]);
    let out = Path::new(&args[1]);
    fs::create_dir_all(out)?;
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/terrestrial/fixtures");
    let par = fixture.join("test1.par");
    let input = load(&par, &fixture.join("test2.tim"))?;
    let mut runtime = RuntimeContext::new(root)?;
    let result = evaluate_terrestrial(&input, &mut runtime)?;
    fs::write(
        out.join("cli-equivalent.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    let text = |key: &str| input.text_parameters()[key].tokens[0].clone();
    let configuration = json!({
        "binary_model":"NONE","calc_shapiro":"1","clock":text("CLK"),"correct_troposphere":"1",
        "dilate_frequency":"1","ephemeris":text("EPHEM"),"fit_performed":"false","ipm":"1","planet_shapiro":"1",
        "refphs":"0","remove_mean":"0","solar_wind_model":"0","solar_wind_ne_cm3":"4","t2c_method":"1",
        "time_ephemeris":"IF99","tzrsite":text("TZRSITE"),"units":"TCB"
    });
    let parameters: serde_json::Map<String, Value> = input
        .parameters()
        .iter()
        .map(|(key, p)| {
            (
                key.clone(),
                json!({"value":p.value.to_string(),"set":u8::from(p.set),"fit_flag":p.fit_flag}),
            )
        })
        .collect();
    let mut stages = json!({"parsed":rows(&input,&result,"parsed"),"bats":rows(&input,&result,"bats"),"residuals":rows(&input,&result,"residuals")});
    for name in ["terrestrial", "terrestrial_4ns"] {
        if name == "terrestrial_4ns" {
            for (stage, step) in [
                ("offset_4ns", "4e-9"),
                ("slope_plus", "0.001"),
                ("slope_minus", "-0.001"),
            ] {
                let step = Float::with_val(PRECISION_BITS, Float::parse(step)?) / 86400i32;
                let mut tim = String::from("FORMAT 1\n");
                for obs in input.observations() {
                    let epoch = obs.sat_mjd.as_float().clone() + &step;
                    tim += &format!(
                        "{} {} {} {} {}\n",
                        obs.original.name,
                        obs.original.frequency_mhz_text,
                        epoch.to_string_radix(10, None),
                        obs.original.toa_error_us_text,
                        obs.original.site
                    );
                }
                let path = out.join(format!("{stage}.tim"));
                fs::write(&path, tim)?;
                let shifted = load(&par, &path)?;
                let shifted_result = evaluate_terrestrial(&shifted, &mut runtime)?;
                // The adapter preserves original identity while evaluating new SATs.
                // Diagnostic stages have no residual lifecycle in A2; omit those fields.
                stages[stage] = json!(rows(&input, &shifted_result, stage));
            }
        }
        let document = json!({
            "schema_version":1,"comparison_profile":"terrestrial-v1","status":"completed","fixture":name,
            "configuration":configuration,"parameters":parameters,
            "original_observations":input.observations().iter().map(|o|&o.original).collect::<Vec<_>>(),
            "stages":stages,"resources":result.resources,
            "engine_provenance":result.timing.engine,"warnings":result.warnings
        });
        fs::write(
            out.join(format!("{name}.json")),
            serde_json::to_vec_pretty(&document)?,
        )?;
    }
    // Failure followed by the same valid input in one live resource context.
    let before = serde_json::to_value(&result.timing.observations)?;
    let input_before = serde_json::to_vec(&input)?;
    let bad = out.join("outside-coverage.tim");
    fs::write(&bad, "FORMAT 1\noutside 1400 80000 10 pks\n")?;
    let failure = evaluate_terrestrial(&load(&par, &bad)?, &mut runtime).unwrap_err();
    assert_eq!(failure.field, "source_exit");
    assert!(failure.message.contains("IFTE_short_record"));
    assert!(failure.path.ends_with("ephemeris/TIMEEPH_short.te405"));
    let after = evaluate_terrestrial(&input, &mut runtime)?;
    assert_eq!(before, serde_json::to_value(&after.timing.observations)?);
    assert_eq!(input_before, serde_json::to_vec(&input)?);
    fs::write(
        out.join("recovery.json"),
        serde_json::to_vec_pretty(&json!({
            "failed_then_valid_same_context":"PASS","input_unchanged":"PASS","error":failure.to_string()
        }))?,
    )?;
    Ok(())
}
