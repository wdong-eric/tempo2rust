//! Rust-only bounded CLI/JSON orchestration for tempo2.C::main; argument and output
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

const USAGE: &str = "Usage: tempo2-rust residuals --par PATH --tim PATH [--runtime PATH]\n       tempo2-rust fit --par PATH --tim PATH [--runtime PATH] [--norescale]";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("{USAGE}");
        return Ok(());
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
        let result = if input.observations()[0].original.site == "@" {
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
    } else if input.observations()[0].original.site == "@" {
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
