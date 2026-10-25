//! Standalone fitting helpers on naturally loaded and timed canonical state.
use serde_json::json;
use std::path::PathBuf;
use tempo2_rust::{RuntimeContext, bats, constraints, timing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 4 {
        return Err("expected MODE RUNTIME PAR TIM [DMSTEP CMSTEP START END FIXCM]".into());
    }
    let runtime = PathBuf::from(&args[1]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: args[2].clone().into(),
            tim: args[3].clone().into(),
        }],
        &runtime,
    )?;
    bats::preprocess(&mut input, &[], &runtime)?;
    let mut resources = RuntimeContext::from_root(&runtime)?;
    timing::evaluate(&mut input, &mut resources, true)?;
    match args[0].as_str() {
        "grid-weights" => constraints::matrix_dm_weights(&mut input.context, 0)?,
        "grid-auto" => {
            if args.len() != 9 {
                return Err("grid-auto needs DMSTEP CMSTEP START END FIXCM".into());
            }
            constraints::auto_set_dm_cm(
                &mut input.context,
                0,
                args[4].parse()?,
                args[5].parse()?,
                args[6].parse()?,
                args[7].parse()?,
                args[8] == "1",
            )?;
        }
        _ => return Err("unknown fitting component".into()),
    }
    let p = &input.context.pulsars()[0];
    let mut rows = Vec::new();
    for (family, n, epochs, values, weights) in [
        (
            "DM",
            p.dmoffsDMnum,
            &p.dmoffsDM_mjd[..],
            &p.dmoffsDM[..],
            &p.dmoffsDM_weight[..],
        ),
        (
            "CM",
            p.dmoffsCMnum,
            &p.dmoffsCM_mjd[..],
            &p.dmoffsCM[..],
            &p.dmoffsCM_weight[..],
        ),
    ] {
        for i in 0..n.ok_or("unwritten grid count")? as usize {
            rows.push(json!({"family":family,"index":i,"epoch":epochs[i].ok_or("unwritten epoch")?.to_string(),"value":values[i].ok_or("unwritten grid value")?.to_string(),"weight":weights[i].ok_or("unwritten weight")?.to_string()}));
        }
    }
    println!("{}", serde_json::to_string_pretty(&json!({"rows":rows}))?);
    Ok(())
}
