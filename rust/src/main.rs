use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};
use tempo2_rust::{evaluate_barycentric, load};

const USAGE: &str = "Usage: tempo2-rust residuals --par PATH --tim PATH";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("{USAGE}");
        return Ok(());
    }
    if args.len() != 5 || args[0] != "residuals" {
        return Err(USAGE.into());
    }
    let mut par = None;
    let mut tim = None;
    for pair in args[1..].chunks_exact(2) {
        if pair[0] == "--par" && par.is_none() {
            par = Some(PathBuf::from(&pair[1]));
        } else if pair[0] == "--tim" && tim.is_none() {
            tim = Some(PathBuf::from(&pair[1]));
        } else {
            return Err(USAGE.into());
        }
    }
    let (par, tim) = par.zip(tim).ok_or(USAGE)?;
    let result = evaluate_barycentric(&load(&par, &tim)?)?;
    // Calculation and serialization finish before any successful JSON is published.
    let mut json = serde_json::to_vec_pretty(&result)?;
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
