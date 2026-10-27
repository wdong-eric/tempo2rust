//! AK: exact observation state, source effects and complete concurrent workflows.
use crate::{
    RuntimeContext,
    bats::{self, InputPair},
    clkcorr::ClockSession,
    execution::{with_observation_workers, with_workflow},
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
};
use sha2::{Digest, Sha256};
use std::{num::NonZeroUsize, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tempo2/T2runtime")
}
fn pairs() -> Vec<InputPair> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting/inputs");
    vec![
        InputPair {
            par: base.join("pks_all.par"),
            tim: base.join("pks.tim")
        };
        2
    ]
}
fn prepared() -> PulsarContext {
    let mut input = bats::load_pairs(&pairs(), &root()).unwrap();
    bats::preprocess(&mut input, &[], &root()).unwrap();
    let mut runtime = RuntimeContext::from_root(&root()).unwrap();
    crate::global::form_bats_all(&mut input.context, &mut runtime).unwrap();
    for (p, n) in [259, 17].into_iter().enumerate() {
        let originals = input.context.pulsar_observations_mut(p).unwrap().1[..16].to_vec();
        input.context.ensure_observation_capacity(p, n).unwrap();
        let (pulsar, rows) = input.context.pulsar_observations_mut(p).unwrap();
        pulsar.nobs = Some(n as i32);
        pulsar.correctTroposphere = Some(0);
        for (i, row) in rows[..n].iter_mut().enumerate() {
            *row = originals[i % originals.len()].clone();
            // Repeated source-validated epochs; distinct source row identities.
            row.input_index = Some(i);
            row.deleted = Some(i32::from(i % 11 == 0));
            if i % 13 == 0 {
                row.delayCorr = Some(0);
            }
        }
    }
    input.context
}
fn row_hash(c: &mut PulsarContext) -> String {
    let mut h = Sha256::new();
    for i in 0..c.pulsars.len() {
        let (p, rows) = c.pulsar_observations_mut(i).unwrap();
        for row in &rows[..p.nobs.unwrap() as usize] {
            // Debug prints Rug's complete stored value, native signed zeros and
            // every Option/backing field; no binary128 wire rounding is applied.
            h.update(format!("{row:?}"));
        }
    }
    format!("{:x}", h.finalize())
}
fn run_case(
    base: &PulsarContext,
    workers: usize,
    very_fast: i32,
    root: &std::path::Path,
) -> serde_json::Value {
    let mut c = base.clone();
    let mut session = ClockSession {
        root: Some(root.into()),
        ..Default::default()
    };
    session.source.very_fast = very_fast;
    session.diagnostics.debug = true;
    session.diagnostics.display_version = true;
    session.diagnostics.log_outfile_is_terminal = true;
    let mut resources = vec![];
    let mut passes = vec![];
    for pass in 0..2 {
        *crate::propagation::WORKER_VISITS.lock().unwrap() = Some(vec![]);
        let result = with_observation_workers(NonZeroUsize::new(workers).unwrap(), || {
            crate::propagation::calculate(&mut session, &mut c, &mut resources)
        });
        let visits = crate::propagation::WORKER_VISITS
            .lock()
            .unwrap()
            .take()
            .unwrap();
        for p in 0..c.pulsars.len() {
            let selected: Vec<_> = visits.iter().filter(|v| v.0 == p).collect();
            let ids: std::collections::HashSet<_> = selected.iter().map(|v| v.2).collect();
            if result.is_ok() {
                let n = c.pulsars[p].nobs.unwrap() as usize;
                let actual = if workers == 1 {
                    1
                } else {
                    n.div_ceil(n.div_ceil(workers.min(n)))
                };
                assert_eq!(ids.len(), actual);
                let mut indices: Vec<_> = selected.iter().map(|v| v.1).collect();
                indices.sort();
                assert_eq!(indices, (0..n).collect::<Vec<_>>());
            }
            println!(
                "AK pass={pass} pulsar={p} requested={workers} actual={} observations={}",
                ids.len(),
                selected.len()
            );
        }
        passes.push(serde_json::json!({
            "rows": row_hash(&mut c), "error": result.err().map(|e| (e.field,e.message)),
            "events": session.diagnostics.events, "warnings": session.diagnostics.warnings,
            "errors": session.diagnostics.errors, "warning_log": session.diagnostics.warning_log,
            "error_log": session.diagnostics.error_log, "mask": session.source.dmmodel_warning,
            "resources": resources,
        }));
    }
    serde_json::json!(passes)
}

#[test]
fn observation_workers_preserve_full_rows_effects_and_failed_prefix() {
    if !crate::source_par_tests::in_isolated_process(
        "execution_tests::observation_workers_preserve_full_rows_effects_and_failed_prefix",
    ) {
        return;
    }
    let _workflow = crate::execution::enter();
    let base = prepared();
    for fast in [0, 1] {
        let serial = run_case(&base, 1, fast, &root());
        for workers in [2, 4] {
            assert_eq!(serial, run_case(&base, workers, fast, &root()));
        }
    }
    let mut dm = base.clone();
    for p in &mut dm.pulsars {
        p.param.slots[param_dmmodel as usize].cells[0].param_set = 1;
        p.param.slots[param_dmmodel as usize].cells[0]
            .val
            .assign(10);
        p.dmoffsDMnum = Some(2);
        p.dmoffsDM_mjd[0] = Some(53000.0);
        p.dmoffsDM_mjd[1] = Some(55000.0);
        p.dmoffsDM[0] = Some(0.1);
        p.dmoffsDM[1] = Some(0.2);
    }
    let serial = run_case(&dm, 1, 0, &root());
    assert_eq!(serial[0]["errors"], 2);
    assert_eq!(serial[1]["errors"], 2); // process warn-once history persists
    for workers in [2, 4] {
        assert_eq!(serial, run_case(&dm, workers, 0, &root()));
    }
    // Missing real input at row 71: later scratch work must not publish any
    // mutation, diagnostics or resources, including after repeated failure.
    dm.pulsar_observations_mut(0).unwrap().1[71].earth_ssb[0] = None;
    let serial = run_case(&dm, 1, 0, &root());
    assert!(serial[0]["error"].is_array());
    for workers in [2, 4] {
        assert_eq!(serial, run_case(&dm, workers, 0, &root()));
    }
    // Recovery on valid state remains exactly the original serial result.
    assert_eq!(
        run_case(&base, 1, 0, &root()),
        run_case(&base, 4, 0, &root())
    );
}
use rug::Assign;

#[test]
fn solar_workers_preserve_resource_history_and_failure_cleanup() {
    if !crate::source_par_tests::in_isolated_process(
        "execution_tests::solar_workers_preserve_resource_history_and_failure_cleanup",
    ) {
        return;
    }
    let _workflow = crate::execution::enter();
    let mut base = prepared();
    for p in &mut base.pulsars {
        p.nobs = Some(5);
        p.swm = Some(1);
    }
    let dir = std::env::temp_dir().join(format!("tempo2-ak-solar-{}", std::process::id()));
    let sheets = dir.join("solarWindModel");
    std::fs::create_dir_all(&sheets).unwrap();
    // AB's defined current-sheet format, covering all rotations of these
    // validated 53910..54150 epochs. Deliberately identical supplied geometry.
    for rotation in 1800..2400 {
        std::fs::write(
            sheets.join(format!("CS{rotation}.txt")),
            b"0 0\n90 10\n180 0\n270 -10\n360 0\n",
        )
        .unwrap();
    }
    let serial = run_case(&base, 1, 0, &dir);
    assert!(serial[0]["error"].is_null(), "{}", serial[0]["error"]);
    assert!(!serial[0]["resources"].as_array().unwrap().is_empty());
    for workers in [2, 4] {
        assert_eq!(serial, run_case(&base, workers, 0, &dir));
    }
    std::fs::rename(&sheets, dir.join("retained-sheets")).unwrap();
    let serial = run_case(&base, 1, 0, &dir);
    assert!(serial[0]["error"].is_array());
    assert_eq!(serial, run_case(&base, 4, 0, &dir));
    std::fs::rename(dir.join("retained-sheets"), &sheets).unwrap();
    assert_eq!(run_case(&base, 1, 0, &dir), run_case(&base, 4, 0, &dir));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn concurrent_complete_workflows_own_controls_and_diagnostic_collection() {
    if !crate::source_par_tests::in_isolated_process(
        "execution_tests::concurrent_complete_workflows_own_controls_and_diagnostic_collection",
    ) {
        return;
    }
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
    let (tx, rx) = std::sync::mpsc::channel();
    for caller in 0..4 {
        let barrier = barrier.clone();
        let tx = tx.clone();
        std::thread::spawn(move || {
            barrier.wait();
            let values = with_workflow(|| {
                with_observation_workers(NonZeroUsize::new(caller + 1).unwrap(), || {
                    let mut input = bats::load_pairs(&pairs(), &root()).unwrap();
                    bats::preprocess(&mut input, &[], &root()).unwrap();
                    let mut runtime = RuntimeContext::from_root(&root()).unwrap();
                    crate::take_clock_diagnostics(); // explicit boundary: retain no prior workflow events
                    let mut outputs = vec![];
                    for pass in 0..3 {
                        let marker = format!("AK caller {caller} pass {pass}");
                        crate::clkcorr::with_process(|s| {
                            s.diagnostics.debug = caller % 2 == 0;
                            s.diagnostics.log(b'w', "AK", 1, marker.as_bytes());
                            Ok(())
                        })
                        .unwrap();
                        let result =
                            crate::timing::evaluate(&mut input, &mut runtime, true).unwrap();
                        let messages: Vec<_> = result
                            .diagnostics
                            .iter()
                            .map(|d| String::from_utf8_lossy(&d.bytes))
                            .collect();
                        let markers: Vec<_> = messages
                            .iter()
                            .filter(|m| m.contains("AK caller"))
                            .collect();
                        assert_eq!(markers.len(), 1);
                        assert!(markers[0].contains(&marker));
                        assert!(crate::take_clock_diagnostics().is_empty());
                        outputs.push(serde_json::to_value(result.pulsars).unwrap());
                    }
                    outputs
                })
            });
            tx.send(values).unwrap();
        });
    }
    drop(tx);
    let mut values = vec![];
    for _ in 0..4 {
        values.push(
            rx.recv_timeout(std::time::Duration::from_secs(45))
                .expect("bounded complete-workflow deadlock detection"),
        );
    }
    for caller in &values {
        assert_eq!(&values[0], caller);
    }
}

#[test]
fn execution_controls_restore_on_unwind() {
    let before = crate::execution::observation_workers();
    let result = std::panic::catch_unwind(|| {
        with_workflow(|| {
            with_observation_workers(NonZeroUsize::new(4).unwrap(), || panic!("scope test"));
        })
    });
    assert!(result.is_err());
    assert_eq!(crate::execution::observation_workers(), before);
    with_workflow(|| with_workflow(|| assert_eq!(2 + 2, 4)));
}

#[test]
fn observation_scratch_retains_unwritten_and_float_classes() {
    use rug::{Float, float::Special};
    let row = crate::tempo2::SourceObservation {
        residual: Some(-Float::with_val(128, Special::Nan)),
        prefitResidual: Some(Float::with_val(128, Special::NegZero)),
        residualtn: Some(Float::with_val(128, Special::NegInfinity)),
        ..Default::default()
    };
    let copy = row.execution_snapshot();
    assert!(copy.residual.as_ref().unwrap().is_nan());
    assert!(copy.residual.as_ref().unwrap().is_sign_negative());
    assert!(copy.prefitResidual.as_ref().unwrap().is_zero());
    assert!(copy.prefitResidual.as_ref().unwrap().is_sign_negative());
    assert!(copy.residualtn.as_ref().unwrap().is_infinite());
    assert!(copy.residualtn.as_ref().unwrap().is_sign_negative());
    assert!(copy.bat.is_none());
}
