//! Rust-only CLI/JSON orchestration for tempo2.C::main; argument and output
//! roles correspond to getInputs.C::getInputs and textOutput.C::textOutput.
//! No standalone counterpart or complete upstream CLI/output contract is claimed.
use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};
use tempo2_rust::{
    FitOptions, RuntimeContext, evaluate_barycentric, evaluate_terrestrial, fit_barycentric,
    fit_terrestrial, load,
};

const USAGE: &str = "Usage: tempo2-rust timing --par PATH --tim PATH [--par PATH --tim PATH ...] --runtime PATH [--no-mean] [-- PREPROCESS_ARGS]\n       tempo2-rust bats --par PATH --tim PATH [--par PATH --tim PATH ...] --runtime PATH [-- PREPROCESS_ARGS]\n        tempo2-rust residuals --par PATH --tim PATH [--runtime PATH]\n       tempo2-rust fit --par PATH --tim PATH [--runtime PATH] [--norescale]";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("{USAGE}");
        return Ok(());
    }
    if args.first().is_some_and(|s| s == "bats" || s == "timing") {
        return run_bats(&args[1..], args[0] == "timing");
    }
    if args.is_empty() || (args[0] != "residuals" && args[0] != "fit") {
        return Err(USAGE.into());
    }
    let mut par = None;
    let mut tim = None;
    let mut runtime = None;
    let fit = args[0] == "fit";
    let mut options = FitOptions::default();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--norescale" && fit && options.rescale_err_chisq {
            options.rescale_err_chisq = false;
            i += 1;
            continue;
        }
        let value = args.get(i + 1).ok_or(USAGE)?;
        if args[i] == "--par" && par.is_none() {
            par = Some(PathBuf::from(value));
        } else if args[i] == "--tim" && tim.is_none() {
            tim = Some(PathBuf::from(value));
        } else if args[i] == "--runtime" && runtime.is_none() {
            runtime = Some(PathBuf::from(value));
        } else {
            return Err(USAGE.into());
        }
        i += 2;
    }
    let (par, tim) = par.zip(tim).ok_or(USAGE)?;
    let input = load(&par, &tim)?;
    // Calculation and serialization finish before any successful JSON is published.
    let mut json = if fit {
        let result = if input.observation_site(0)? == b"@" {
            fit_barycentric(&input, &options)?
        } else {
            let root = runtime.ok_or("PKS observations require --runtime PATH")?;
            fit_terrestrial(&input, &mut RuntimeContext::new(&root)?, &options)?
        };
        let json = serde_json::to_vec_pretty(&result)?;
        for cycle in &result.cycles {
            for warning in cycle.warnings.iter().chain(&cycle.postfit.warnings) {
                eprintln!("tempo2-rust: cycle {}: {warning}", cycle.iteration);
            }
        }
        json
    } else if input.observation_site(0)? == b"@" {
        serde_json::to_vec_pretty(&evaluate_barycentric(&input)?)?
    } else {
        let root = runtime.ok_or("PKS observations require --runtime PATH")?;
        let result = evaluate_terrestrial(&input, &mut RuntimeContext::new(&root)?)?;
        let json = serde_json::to_vec_pretty(&result)?;
        for warning in &result.warnings {
            eprintln!("tempo2-rust: {warning}");
        }
        json
    };
    json.push(b'\n');
    io::stdout().lock().write_all(&json)?;
    Ok(())
}

fn run_bats(args: &[OsString], timing: bool) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::ffi::OsStrExt;
    let (mut pars, mut tims, mut root, mut preprocess) = (vec![], vec![], None, vec![]);
    let mut remove_mean = true;
    let mut i = 0;
    while i < args.len() {
        if timing && args[i] == "--no-mean" {
            remove_mean = false;
            i += 1;
            continue;
        }
        if args[i] == "--" {
            preprocess = args[i + 1..]
                .iter()
                .map(|s| s.as_bytes().to_vec())
                .collect();
            break;
        }
        let v = args.get(i + 1).ok_or(USAGE)?;
        if args[i] == "--par" {
            pars.push(PathBuf::from(v));
        } else if args[i] == "--tim" {
            tims.push(PathBuf::from(v));
        } else if args[i] == "--runtime" && root.is_none() {
            root = Some(PathBuf::from(v));
        } else {
            return Err(USAGE.into());
        }
        i += 2;
    }
    if pars.is_empty() || pars.len() != tims.len() {
        return Err("bats requires matching nonempty ordered PAR/TIM lists".into());
    }
    let root = root.ok_or("bats requires --runtime PATH")?;
    let pairs = pars
        .into_iter()
        .zip(tims)
        .map(|(par, tim)| tempo2_rust::bats::InputPair { par, tim })
        .collect::<Vec<_>>();
    let mut runtime = RuntimeContext::from_root(&root)?;
    let mut input = tempo2_rust::bats::load_pairs(&pairs, &root)?;
    tempo2_rust::bats::preprocess(&mut input, &preprocess, &root)?;
    let mut bytes = if timing {
        serde_json::to_vec_pretty(&tempo2_rust::timing::evaluate(
            &mut input,
            &mut runtime,
            remove_mean,
        )?)?
    } else {
        serde_json::to_vec_pretty(&tempo2_rust::bats::evaluate(&mut input, &mut runtime)?)?
    };
    bytes.push(b'\n');
    io::stdout().lock().write_all(&bytes)?;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            // Source stdout/stderr diagnostics both go to CLI stderr; a failed
            // timing operation never publishes a partial JSON document.
            for event in tempo2_rust::take_clock_diagnostics() {
                let _ = io::stderr().lock().write_all(&event.bytes);
            }
            eprintln!("tempo2-rust: {error}");
            ExitCode::FAILURE
        }
    }
}
