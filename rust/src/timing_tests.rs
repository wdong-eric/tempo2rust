use super::*;
use crate::{PRECISION_BITS, observation::read, tempo2::Label};
use rug::Float;
use std::path::PathBuf;
fn prepared(name: &str) -> (BatInput, RuntimeContext) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let f = root.join("validation/timing_models/fixtures");
    let runtime = root.join("validation/reference/build/source/T2runtime");
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: f.join(format!("{name}.par")),
            tim: f.join(format!("{name}.tim")),
        }],
        &runtime,
    )
    .unwrap();
    bats::preprocess(&mut input, &[], &runtime).unwrap();
    let mut r = RuntimeContext::from_root(&runtime).unwrap();
    bats::form_bats_all(&mut input.context, &mut r).unwrap();
    (input, r)
}
fn values(context: &mut PulsarContext) -> Vec<(Float, Float, i64)> {
    let (p, rows) = context.pulsar_observations_mut(0).unwrap();
    rows.iter()
        .take(p.nobs.unwrap() as usize)
        .map(|o| {
            (
                o.phase.clone().unwrap(),
                o.residual.clone().unwrap(),
                o.pulseN.unwrap(),
            )
        })
        .collect()
}
#[test]
#[ignore = "requires local pinned runtime"]
fn repeated_residuals_and_missing_signal_recovery() {
    let (mut input, _) = prepared("high-spin");
    form_residuals(&mut input.context, true).unwrap();
    let first = values(&mut input.context);
    form_residuals(&mut input.context, true).unwrap();
    assert_eq!(first, values(&mut input.context));
    input.context.pulsars[0].TNsubtractRed = Some(1);
    assert!(
        form_residuals(&mut input.context, true)
            .unwrap_err()
            .to_string()
            .contains("TNRedSignal")
    );
    input.context.pulsars[0].TNsubtractRed = Some(0);
    form_residuals(&mut input.context, true).unwrap();
    assert_eq!(first, values(&mut input.context));
}
#[test]
#[ignore = "requires local pinned runtime"]
fn noise_mean_excludes_spare_and_removed_tzr() {
    for name in ["high-spin", "tzr"] {
        let (mut input, _) = prepared(name);
        let n = input.context.pulsars[0].nobs.unwrap() as usize;
        input.context.ensure_observation_capacity(0, n + 1).unwrap();
        {
            let (p, rows) = input.context.pulsar_observations_mut(0).unwrap();
            p.TNsubtractRed = Some(1);
            p.tzrobs.TNRedSignal = None;
            for (i, o) in rows.iter_mut().take(n).enumerate() {
                o.TNRedSignal = Some((i as f64 + 1.0) * 1e-6);
            }
            rows[n].residualtn = Some(Float::with_val(PRECISION_BITS, 991));
            rows[n].TNRedSignal = None;
            rows[n].deleted = None;
        }
        form_residuals(&mut input.context, true).unwrap();
        let (p, rows) = input.context.pulsar_observations_mut(0).unwrap();
        let mut mean = Float::with_val(PRECISION_BITS, 0);
        for o in rows.iter().take(n) {
            mean += o.residual.clone().unwrap()
                - Float::with_val(PRECISION_BITS, o.TNRedSignal.unwrap());
        }
        mean /= n as i32;
        for o in rows.iter().take(n) {
            assert_eq!(
                o.residualtn.as_ref().unwrap(),
                &(o.residual.clone().unwrap()
                    - Float::with_val(PRECISION_BITS, o.TNRedSignal.unwrap())
                    - mean.clone())
            );
        }
        if p.refphs == Some(1) {
            assert_eq!(rows[n].residualtn, p.tzrobs.residualtn);
            assert_eq!(p.nobs, Some(n as i32));
        } else {
            assert_eq!(
                rows[n].residualtn,
                Some(Float::with_val(PRECISION_BITS, 991))
            );
        }
    }
}
#[test]
#[ignore = "requires local pinned runtime"]
fn empty_active_counts_and_step_boundaries_are_explicit() {
    let (mut input, _) = prepared("ifunc-step");
    let before = input.context.clone();
    for count in [0, -1] {
        input.context.pulsars[0].nobs = Some(count);
        form_residuals(&mut input.context, true).unwrap();
        assert_eq!(input.context.pulsars[0].nobs, Some(count));
    }
    input.context = before;
    let (_, rows) = input.context.pulsar_observations_mut(0).unwrap();
    rows[0].sat = Some(Float::with_val(PRECISION_BITS, 53996));
    assert!(
        form_residuals(&mut input.context, false)
            .unwrap_err()
            .to_string()
            .contains("ival")
    );
}
#[test]
#[ignore = "requires local pinned runtime"]
fn eccentric_gw_requires_prior_residual_producer() {
    let (mut input, rt) = prepared("high-spin");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("validation/timing_models/fixtures/library/eccentric.par");
    crate::source_par::read_simple_parfile(
        &mut input.context,
        0,
        &mut crate::source_text::SourceText::new(&path, std::fs::read(&path).unwrap()),
        &rt.root,
    )
    .unwrap();
    assert!(
        form_residuals(&mut input.context, false)
            .unwrap_err()
            .to_string()
            .contains("residual")
    );
    assert_eq!(
        input.context.pulsars[0]
            .param
            .parameter(Label::param_gwecc)
            .cell(0)
            .unwrap()
            .param_set(),
        1
    );
    let (_, rows) = input.context.pulsar_observations_mut(0).unwrap();
    assert!(read(&rows[0].residual, "residual").is_err());
}
#[test]
#[ignore = "requires local pinned runtime"]
fn dm_averaging_missing_flag_retains_prior_fields() {
    let (mut input, _) = prepared("high-spin");
    form_residuals(&mut input.context, false).unwrap();
    let (p, rows) = input.context.pulsar_observations_mut(0).unwrap();
    p.AverageEpochWidth = Some(3.0);
    for (slot, value) in p.AverageFlag.iter_mut().zip(b"-f\0") {
        *slot = Some(*value);
    }
    rows[0].nFlags = Some(0);
    rows[0].averagedmbat = Some(54000.0);
    rows[0].averagedmres = Some(0.000123);
    rows[0].averagedmerr = Some(0.000004);
    average_dm_residuals(&mut input.context, 0).unwrap();
    let (_, rows) = input.context.pulsar_observations_mut(0).unwrap();
    assert_eq!(rows[0].averagedmbat, Some(54000.0));
    assert_eq!(rows[0].averagedmres, Some(0.000123));
    assert_eq!(rows[0].averagedmerr, Some(0.000004));
    assert_eq!(rows[1].averagedmres, None);
}
#[test]
fn extended_diagnostic_spelling() {
    for (token, expected) in [
        ("0.00001", "1e-05"),
        ("0.0001", "0.0001"),
        ("999999.9", "1e+06"),
        ("100.1234", "100.123"),
        ("-0", "-0"),
        ("1.234565000000000000000001", "1.23457"),
        ("1.234564999999999999999999", "1.23456"),
    ] {
        let f = Float::with_val(PRECISION_BITS, Float::parse(token).unwrap());
        assert_eq!(
            crate::clock_text::extended_general_six(&f),
            expected.as_bytes()
        );
    }
}
