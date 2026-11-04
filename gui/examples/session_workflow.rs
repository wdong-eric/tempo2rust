//! Fresh-process comparison of GUI session ownership with direct engine calls.
use std::{fs, io::Write, path::PathBuf};
use tempo2_rust::{RuntimeContext, bats, core_output, execution, general_fitting, timing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 5 {
        return Err("session_workflow gui|engine PAR TIM RUNTIME NEW_OUTPUT_DIRECTORY".into());
    }
    let mode = &args[0];
    let (par, tim, runtime, out) = (
        PathBuf::from(&args[1]),
        PathBuf::from(&args[2]),
        PathBuf::from(&args[3]),
        PathBuf::from(&args[4]),
    );
    let history = execution::with_workflow(|| -> Result<_, Box<dyn std::error::Error>> {
        let mut history = vec![];
        if mode == "gui" {
            let mut session = tempo2_gui::load(&par, &tim, &runtime)?;
            history.push(serde_json::to_value(session.timing())?);
            session.set_excluded(3, true)?;
            history.push(serde_json::to_value(session.timing())?);
            session.refit()?;
            history.push(serde_json::to_value(session.fit_result())?);
            session.refit()?;
            history.push(serde_json::to_value(session.fit_result())?);
            let before = serde_json::to_vec(session.timing())?;
            session.save(&out)?;
            assert_eq!(before, serde_json::to_vec(session.timing())?);
            assert!(session.save(&out).unwrap_err().contains("Cannot create"));
            session.refit()?;
            history.push(serde_json::to_value(session.fit_result())?);
            session.set_excluded(3, false)?;
            assert!(!session.can_save());
            history.push(serde_json::to_value(session.timing())?);
        } else if mode == "engine" {
            let mut resources = RuntimeContext::from_root(&runtime)?;
            let mut input = bats::load_pairs(
                &[bats::InputPair {
                    par: par.clone(),
                    tim: tim.clone(),
                }],
                &runtime,
            )?;
            bats::preprocess(&mut input, &[], &runtime)?;
            history.push(serde_json::to_value(timing::evaluate(
                &mut input,
                &mut resources,
                true,
            )?)?);
            input
                .context
                .pulsar_observations_mut(0)?
                .1
                .iter_mut()
                .find(|r| r.input_index == Some(3))
                .unwrap()
                .deleted = Some(1);
            history.push(serde_json::to_value(timing::evaluate(
                &mut input,
                &mut resources,
                true,
            )?)?);
            let options = general_fitting::FitOptions::default();
            history.push(serde_json::to_value(general_fitting::fit(
                &mut input,
                &mut resources,
                &options,
            )?)?);
            let fit = general_fitting::fit(&mut input, &mut resources, &options)?;
            history.push(serde_json::to_value(&fit)?);
            fs::create_dir(&out)?;
            let mut snapshot = input.context.clone();
            let (mut stdout, mut stderr) = (vec![], vec![]);
            core_output::text_output(
                &mut snapshot,
                &core_output::OutputOptions {
                    new_par: Some("session.par".into()),
                    ..Default::default()
                },
                &mut core_output::OutputIo {
                    stdout: &mut stdout,
                    stderr: &mut stderr,
                    stdin: &mut std::io::empty(),
                    directory: &out,
                },
            )?;
            fs::write(out.join("engine.stdout"), stdout)?;
            fs::write(out.join("engine.stderr"), stderr)?;
            tempo2_rust::source_tim::write_tim(
                &out.join("session.tim"),
                &mut snapshot,
                0,
                b"tempo2",
                &runtime,
            )?;
            let mut bytes = serde_json::to_vec_pretty(
                &serde_json::json!({"timing": &fit.cycles.last().unwrap().postfit, "fit": &fit}),
            )?;
            bytes.push(b'\n');
            fs::write(out.join("session.json"), bytes)?;
            history.push(serde_json::to_value(general_fitting::fit(
                &mut input,
                &mut resources,
                &options,
            )?)?);
            input
                .context
                .pulsar_observations_mut(0)?
                .1
                .iter_mut()
                .find(|r| r.input_index == Some(3))
                .unwrap()
                .deleted = Some(0);
            history.push(serde_json::to_value(timing::evaluate(
                &mut input,
                &mut resources,
                true,
            )?)?);
        } else {
            return Err("mode must be gui or engine".into());
        }
        // Native writeTim preserves excluded rows as C comments; the reader skips them.
        let loaded =
            tempo2_gui::load(&out.join("session.par"), &out.join("session.tim"), &runtime)?;
        history.push(
            serde_json::json!({"reloaded_toas": loaded.timing().pulsars[0].observations.len()}),
        );
        Ok(history)
    })?;
    let mut bytes = serde_json::to_vec_pretty(&history)?;
    bytes.push(b'\n');
    std::io::stdout().write_all(&bytes)?;
    Ok(())
}
