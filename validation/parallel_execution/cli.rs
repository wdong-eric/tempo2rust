//! External native-grammar CLI with explicitly selected AK execution controls.
use tempo2_rust::{core_cli, core_output::OutputIo, execution};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workers = std::env::var("AK_WORKERS").unwrap_or_else(|_| "1".into()).parse()?;
    let args = std::env::args_os().collect::<Vec<_>>();
    let environment = core_cli::Environment::from_process();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let mut stdin = std::io::stdin().lock();
    let mut io = OutputIo { stdout: &mut stdout, stderr: &mut stderr, stdin: &mut stdin,
        directory: std::path::Path::new(".") };
    let status = execution::with_workflow(|| execution::with_observation_workers(workers, ||
        core_cli::run(&args, &environment, &mut io)))?;
    std::process::exit(status);
}
