//! Headless verification entry point for the exact workflow used by the viewer.
use std::{io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if args.len() != 3 {
        return Err("usage: export_timing PAR TIM RUNTIME".into());
    }
    let session = tempo2_gui::load(&args[0], &args[1], &args[2])?;
    tempo2_gui::plot_toas(&session.timing().pulsars[0])?;
    let mut bytes = serde_json::to_vec_pretty(session.timing())?;
    bytes.push(b'\n');
    std::io::stdout().lock().write_all(&bytes)?;
    Ok(())
}
