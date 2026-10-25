//! Public canonical-state examples for caller-owned controls and stored signals.
//! Run as: timing_lifecycle MODE RUNTIME PAR TIM CONTROL [--no-mean].
//! Eccentric GW consumes residuals from a real preceding timing evaluation.
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use tempo2_rust::{
    RuntimeContext,
    bats::{self, InputPair},
    source_par,
    source_text::SourceText,
    tempo2::Label,
    timing,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args_os().skip(1).collect();
    if a.len() < 5 {
        return Err("expected MODE RUNTIME PAR TIM CONTROL [--no-mean]".into());
    }
    let mode = a[0].to_str().ok_or("mode must be text")?;
    let root = PathBuf::from(&a[1]);
    let control = PathBuf::from(&a[4]);
    let bytes = fs::read(&control)?;
    let mut input = bats::load_pairs(
        &[InputPair {
            par: a[2].clone().into(),
            tim: a[3].clone().into(),
        }],
        &root,
    )?;
    input.inputs.push(bats::ConsumedInput {
        path: control.clone(),
        role: format!("caller {mode} controls"),
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        bytes: bytes.clone(),
    });
    bats::preprocess(&mut input, &[], &root)?;
    let mut runtime = RuntimeContext::from_root(&root)?;
    match mode {
        "eccentric" => {
            // The source epoch depends on o.residual. Produce it with the fixed
            // base model, then activate the supplied eccentric-source parameters.
            timing::evaluate(&mut input, &mut runtime, false)?;
            source_par::read_simple_parfile(
                &mut input.context,
                0,
                &mut SourceText::new(&control, bytes),
                &root,
            )?;
        }
        "state" => {
            let (p, _) = input.context.pulsar_observations_mut(0)?;
            for line in std::str::from_utf8(&bytes)?.lines() {
                let mut fields = line.splitn(3, ' ');
                let key = fields.next().ok_or("key")?;
                let k: usize = fields.next().ok_or("index")?.parse()?;
                let value = fields.next().ok_or("value")?;
                let label = match key {
                    "DF1" => Label::param_df1,
                    "SWITCH" => Label::param_stateSwitchT,
                    _ => return Err("unknown control".into()),
                };
                source_par::read_value(
                    p,
                    key.as_bytes(),
                    &mut SourceText::new(&control, format!("{value}\n").into_bytes()),
                    label,
                    k,
                )?;
            }
        }
        "noise" => {
            let (p, rows) = input.context.pulsar_observations_mut(0)?;
            p.AverageDMResiduals = Some(1);
            // Supplied signal seconds are caller data, not inferred by this engine.
            for line in std::str::from_utf8(&bytes)?.lines() {
                let v: Vec<_> = line.split_whitespace().collect();
                if v.len() != 6 {
                    return Err("expected index red_s dm_s chrom_s swgp_s duration_s".into());
                }
                let o = rows
                    .get_mut(v[0].parse::<usize>()?)
                    .ok_or("signal row index")?;
                o.TNRedSignal = Some(v[1].parse()?);
                o.TNDMSignal = Some(v[2].parse()?);
                o.TNChromSignal = Some(v[3].parse()?);
                o.SWGPSignal = Some(v[4].parse()?);
                o.tobs = Some(v[5].parse()?);
            }
        }
        _ => return Err("unknown scenario".into()),
    }
    let result = timing::evaluate(
        &mut input,
        &mut runtime,
        !a.iter().any(|s| s == "--no-mean"),
    )?;
    let output = serde_json::to_string_pretty(&result)?;
    println!("{output}");
    Ok(())
}
