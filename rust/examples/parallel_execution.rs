//! Full public timing/fitting work under explicit observation and workflow scopes.
//! parallel_execution MODE WORKERS REPEATS RUNTIME OUTPUT COVARIANCE PAR TIM [...]
//! MODE is bats, timing, fit or raw-bats (engine timing without output projection).
//! COVARIANCE is '-' or a supplied model file.
use std::{num::NonZeroUsize, path::Path, time::Instant};
use tempo2_rust::{RuntimeContext, bats, execution, general_fitting, timing};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() < 8 || (a.len() - 6) % 2 != 0 {
        return Err("MODE WORKERS REPEATS RUNTIME OUTPUT COVARIANCE PAR TIM [...]".into());
    }
    let workers = a[1].parse::<NonZeroUsize>()?;
    let repeats = a[2].parse::<NonZeroUsize>()?.get();
    execution::with_workflow(|| {
        execution::with_observation_workers(workers, || {
            let begin = Instant::now();
            let pairs: Vec<_> = a[6..]
                .chunks_exact(2)
                .map(|p| bats::InputPair {
                    par: p[0].clone().into(),
                    tim: p[1].clone().into(),
                })
                .collect();
            let root = Path::new(&a[3]);
            let mut input = bats::load_pairs(&pairs, root)?;
            bats::preprocess(&mut input, &[], root)?;
            let mut runtime = RuntimeContext::from_root(root)?;
            let loaded = begin.elapsed().as_secs_f64();
            let options = general_fitting::FitOptions {
                covariance: (a[5] != "-").then(|| a[5].clone()),
                ..Default::default()
            };
            let mut samples = Vec::new();
            let mut output = Vec::new();
            for _ in 0..repeats {
                let start = Instant::now();
                let value = match a[0].as_str() {
                    "raw-bats" => {
                        tempo2_rust::global::form_bats_all(&mut input.context, &mut runtime)?;
                        serde_json::Value::Null
                    }
                    "bats" => serde_json::to_value(bats::evaluate(&mut input, &mut runtime)?)?,
                    "timing" => {
                        serde_json::to_value(timing::evaluate(&mut input, &mut runtime, true)?)?
                    }
                    "fit" => serde_json::to_value(general_fitting::fit(
                        &mut input,
                        &mut runtime,
                        &options,
                    )?)?,
                    _ => return Err("unknown workflow".into()),
                };
                output.push(value);
                samples.push(start.elapsed().as_secs_f64());
            }
            let evaluated = begin.elapsed().as_secs_f64();
            std::fs::write(&a[4], serde_json::to_vec(&output)?)?;
            println!(
                "{}",
                serde_json::json!({"workers_requested": workers.get(), "load_s": loaded,
            "evaluate_and_project_s": samples, "before_file_output_s": evaluated,
            "total_s": begin.elapsed().as_secs_f64(), "precision_bits": tempo2_rust::PRECISION_BITS})
            );
            Ok(())
        })
    })
}
