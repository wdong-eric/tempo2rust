use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};
use tempo2_rust::{RuntimeContext, evaluate_barycentric, evaluate_terrestrial, load};

const USAGE: &str = "Usage: tempo2-rust residuals --par PATH --tim PATH [--runtime PATH]";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("{USAGE}");
        return Ok(());
    }
    if ![5, 7].contains(&args.len()) || args[0] != "residuals" {
        return Err(USAGE.into());
    }
    let mut par = None;
    let mut tim = None;
    let mut runtime = None;
    for pair in args[1..].chunks_exact(2) {
        if pair[0] == "--par" && par.is_none() {
            par = Some(PathBuf::from(&pair[1]));
        } else if pair[0] == "--tim" && tim.is_none() {
            tim = Some(PathBuf::from(&pair[1]));
        } else if pair[0] == "--runtime" && runtime.is_none() {
            runtime = Some(PathBuf::from(&pair[1]));
        } else {
            return Err(USAGE.into());
        }
    }
    let (par, tim) = par.zip(tim).ok_or(USAGE)?;
    let input = load(&par, &tim)?;
    // Calculation and serialization finish before any successful JSON is published.
    let mut json = if input.observations()[0].original.site == "@" {
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
            eprintln!("tempo2-rust: {error}");
            ExitCode::FAILURE
        }
    }
}
