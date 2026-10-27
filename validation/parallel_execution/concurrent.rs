//! Separately compiled complete fit/predictor callers; bounded by the runner.
use std::{num::NonZeroUsize, path::PathBuf, sync::{Arc, Barrier, mpsc}, time::Duration};
use tempo2_rust::{RuntimeContext, bats::{self, InputPair}, cheby2d_int::{self, ChebyRequest},
    clkcorr, execution, general_fitting, predictor, t2accel, take_clock_diagnostics};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).map(PathBuf::from).collect::<Vec<_>>();
    assert_eq!(args.len(), 6); // runtime, par, tim, predictor par, covariance, fresh output
    std::fs::create_dir(&args[5])?;
    let barrier = Arc::new(Barrier::new(4));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for caller in 0..4 {
        let (args, barrier, tx) = (args.clone(), barrier.clone(), tx.clone());
        handles.push(std::thread::spawn(move || {
            barrier.wait();
            let result = execution::with_workflow(|| execution::with_observation_workers(
                NonZeroUsize::new(caller + 1).unwrap(), || {
                let pairs = vec![InputPair { par: args[1].clone(), tim: args[2].clone() }; 2];
                let mut input = bats::load_pairs(&pairs, &args[0]).unwrap();
                bats::preprocess(&mut input, &[], &args[0]).unwrap();
                let mut runtime = RuntimeContext::from_root(&args[0]).unwrap();
                let mut diagnostics = take_clock_diagnostics();
                let acceleration = if cfg!(feature="lapack") { 2 } else { 0 };
                let options = general_fitting::FitOptions {
                    covariance: Some(args[4].to_string_lossy().into_owned()), acceleration,
                    ..Default::default()
                };
                let mut fits = Vec::new();
                for pass in 0..2 {
                    let marker = format!("AK fit caller {caller} pass {pass}");
                    clkcorr::with_process(|s| {
                        s.diagnostics.debug = caller % 2 == 0;
                        s.diagnostics.log(b'w', "AK", 1, marker.as_bytes()); Ok(())
                    }).unwrap();
                    let fit = general_fitting::fit(&mut input, &mut runtime, &options).unwrap();
                    let bytes = serde_json::to_vec(&fit).unwrap();
                    std::fs::write(args[5].join(format!("fit-{caller}-{pass}.json")), &bytes).unwrap();
                    let messages = &fit.cycles[0].prefit.diagnostics;
                    let found = messages.iter().filter(|d| String::from_utf8_lossy(&d.bytes).contains("AK fit caller")).collect::<Vec<_>>();
                    assert_eq!(found.len(), 1);
                    assert!(String::from_utf8_lossy(&found[0].bytes).contains(&marker));
                    // State history is retained in files; compare numerical products separately.
                    fits.push(serde_json::json!({"prefit":fit.cycles[0].prefit.pulsars,
                        "postfit":fit.cycles[0].postfit.pulsars,"systems":fit.cycles[0].systems}));
                    assert!(take_clock_diagnostics().is_empty());
                    assert!(bats::load_pairs(&[InputPair {par:args[5].join("missing.par"),tim:args[2].clone()}], &args[0]).is_err());
                    diagnostics.extend(take_clock_diagnostics());
                }
                let mut input = predictor::load_par(&args[3], &args[0], &[]).unwrap();
                predictor::cli_defaults(&mut input.context, "PKS").unwrap();
                let request = ChebyRequest {site:"PKS".into(),start_mjd:58000.0.into(),end_mjd:58000.03.into(),
                    segment_days:0.02.into(),overlap_days:0.002.into(),freq_start_mhz:1000.0.into(),
                    freq_end_mhz:2000.0.into(),time_coeffs:7,freq_coeffs:3};
                let values = t2accel::with_acceleration(acceleration, || {
                    assert!(cheby2d_int::construct_set(&request,&mut input.context,&mut runtime,&args[5].join("absent/fail.tim")).is_err());
                    let mut values = Vec::new();
                    for pass in 0..2 {
                        let set = cheby2d_int::construct_set(&request,&mut input.context,&mut runtime,
                            &args[5].join(format!("predictor-{caller}-{pass}.tim"))).unwrap();
                        let value = set.segments[0].phase(&58000.01.into(), &1400.0.into());
                        values.push(format!("{value:?}"));
                        tempo2_rust::tempo2pred::T2Predictor::Cheby(set).write(&args[5].join(format!("predictor-{caller}-{pass}.dat"))).unwrap();
                    }
                    values
                });
                diagnostics.extend(take_clock_diagnostics());
                std::fs::write(args[5].join(format!("diagnostics-{caller}.txt")),format!("{diagnostics:?}")).unwrap();
                serde_json::json!({"fits":fits,"predictor_phase":values})
            }));
            tx.send(result).unwrap();
        }));
    }
    drop(tx);
    let results = (0..4).map(|_| rx.recv_timeout(Duration::from_secs(90)).expect("bounded complete workflow completion")).collect::<Vec<_>>();
    for h in handles { h.join().unwrap(); }
    for r in &results { assert_eq!(r, &results[0]); }
    std::fs::write(args[5].join("scientific-products.json"), serde_json::to_vec(&results)?)?;
    println!("four callers: two repeated correlated two-pulsar fits and two predictors each; exact scientific equality, diagnostic ownership, failures/recovery, joined");
    Ok(())
}
