use crate::{
    PRECISION_BITS, bats::InputPair, pulsar_lifecycle::PulsarContext, source_par::*,
    source_text::SourceText, tempo2::Label::*,
};
use rug::Float;
use std::path::Path;

pub(crate) fn in_isolated_process(name: &str) -> bool {
    if std::env::var("TEMPO2_AC_TEST").as_deref() == Ok(name) {
        return true;
    }
    // These readers intentionally share source diagnostics and globals. A fresh
    // process prevents their warnings from entering another test's fit result.
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([name, "--exact", "--nocapture", "--test-threads=1"])
        .env("TEMPO2_AC_TEST", name)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{name}: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    false
}

fn context() -> PulsarContext {
    let mut c = PulsarContext::unwritten(1);
    c.initialise(1, 0, 1).unwrap();
    c
}
#[test]
fn general_par_source_order_links_and_scales() {
    if !in_isolated_process("source_par_tests::general_par_source_order_links_and_scales") {
        return;
    }
    let mut c = context();
    setup_parameter_file_defaults(&mut c.pulsars[0], Path::new("runtime")).unwrap();
    let mut source=SourceText::new(Path::new("source.par"),b"PSRJ named\nF0 10 1 0.1\nF0 20\nPEPOCH 55000\nRAJ 01:02:03 1 0.01\nDECJ -04:05:06 0 0.02\nDM 5\nPX 2\nDSHK PX\nDMMODEL DM 2\nPB 1\nFB1 2\nBINARY ELL1\nPBDOT 3\nDMX_0001 0.2\nDMXR1_0001 54999\nDMXR2_0001 55001\nTNRedAmp -14\nFDDC 1\nFDDI 2\nFD2 3\nSATJUMP -backend ac 0.0001 1\n".to_vec());
    read_simple_parfile(&mut c, 0, &mut source, Path::new("runtime")).unwrap();
    let p = &c.pulsars[0];
    assert_eq!(p.param.parameter(param_f).cell(0).unwrap().value(), &20);
    assert_eq!(
        p.param
            .parameter(param_f)
            .cell(0)
            .unwrap()
            .prefit_error()
            .unwrap(),
        &Float::with_val(PRECISION_BITS, Float::parse("0.1").unwrap())
    );
    assert_eq!(
        p.param.parameter(param_posepoch).cell(0).unwrap().value(),
        &55000
    );
    assert_eq!(
        p.param
            .parameter(param_dshk)
            .cell(0)
            .unwrap()
            .prefit()
            .unwrap(),
        &0.5
    );
    assert_eq!(p.param.parameter(param_dmmodel).n_link_to(), Some(1));
    assert_eq!(
        p.param.parameter(param_dmmodel).cell(0).unwrap().fit_flag(),
        2
    );
    assert_eq!(
        p.param.parameter(param_pbdot).cell(0).unwrap().param_set(),
        0
    );
    assert_eq!(p.param.parameter(param_fd).cell(1).unwrap().value(), &3);
    assert_eq!(p.ndmx, Some(1));
    assert_eq!(p.jumpSAT[1], Some(1));
    assert_eq!(p.ne_sw_ifuncN, Some(0));
}
#[test]
fn ordinary_files_need_no_seeded_uncertainty_or_solar_count() {
    if !in_isolated_process(
        "source_par_tests::ordinary_files_need_no_seeded_uncertainty_or_solar_count",
    ) {
        return;
    }
    let mut c = context();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/input_to_bat/fixtures");
    let pairs = [InputPair {
        par: root.join("ordinary.par"),
        tim: root.join("ordinary.tim"),
    }];
    let mut inputs = vec![];
    read_parfiles(&mut c, &pairs, Path::new("runtime"), &mut inputs).unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(c.pulsars[0].ne_sw_ifuncN, Some(0));
    for label in [
        param_raj,
        param_decj,
        param_pepoch,
        param_posepoch,
        param_dmepoch,
    ] {
        assert_eq!(
            c.pulsars[0]
                .param
                .parameter(label)
                .cell(0)
                .unwrap()
                .prefit_error()
                .unwrap(),
            &0
        );
    }
    // Unwritten, inactive parameter metadata stays unwritten.
    assert!(
        c.pulsars[0]
            .param
            .parameter(param_gamma)
            .cell(0)
            .unwrap()
            .prefit_error()
            .is_none()
    );
}
#[test]
fn ecliptic_pair_and_mixed_coordinate_error() {
    if !in_isolated_process("source_par_tests::ecliptic_pair_and_mixed_coordinate_error") {
        return;
    }
    let mut c = context();
    setup_parameter_file_defaults(&mut c.pulsars[0], Path::new("runtime")).unwrap();
    let mut source = SourceText::new(
        Path::new("ecl.par"),
        b"PEPOCH 55000\nELONG 65\nELAT -30\nPMELONG 5\nPMELAT -2\n".to_vec(),
    );
    read_simple_parfile(&mut c, 0, &mut source, Path::new("runtime")).unwrap();
    assert_eq!(c.pulsars[0].eclCoord, Some(1));
    let mut source = SourceText::new(Path::new("mix.par"), b"PMRA 2\n".to_vec());
    assert!(
        read_simple_parfile(&mut c, 0, &mut source, Path::new("runtime"))
            .unwrap_err()
            .message
            .contains("cannot mix")
    );
}

#[test]
fn general_loader_and_default_preprocess_use_authored_files() {
    if !in_isolated_process(
        "source_par_tests::general_loader_and_default_preprocess_use_authored_files",
    ) {
        return;
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/input_to_bat/fixtures");
    let pairs = [InputPair {
        par: root.join("ordinary.par"),
        tim: root.join("ordinary.tim"),
    }];
    let mut input = crate::bats::load_pairs(&pairs, Path::new("runtime")).unwrap();
    assert_eq!(input.inputs.len(), 3);
    assert_eq!(input.context.pulsars[0].nobs, Some(4));
    crate::bats::preprocess(&mut input, &[], Path::new("runtime")).unwrap();
    let (p, rows) = input.context.pulsar_observations_mut(0).unwrap();
    assert_eq!(
        p.dm_series_type,
        Some(crate::pulsar::SeriesType::series_taylor_pn)
    );
    assert_eq!(rows[0].sat.as_ref().unwrap(), &55000.125);
    assert_eq!(rows[2].clockCorr, Some(0));
    assert_eq!(rows[2].delayCorr, Some(0));
    assert_eq!(rows[3].toaErr, Some(1.2));
    assert_eq!(
        crate::observation::read_string(&rows[3].fname, "fname").unwrap(),
        b"included"
    );
    assert!(rows[3].sat.as_ref().unwrap() > &55003.5);
}
#[test]
fn source_numeric_grammars_and_fixed_rounding() {
    use crate::source_text::{extended, fixed};
    assert_eq!(extended(b"1D-3").unwrap(), 1);
    assert_eq!(extended(b"0x1.8p+2tail").unwrap(), 6);
    assert_eq!(extended(b"1e+tail").unwrap(), 1);
    assert_eq!(extended(b"nonsense").unwrap(), 0);
    assert_eq!(fixed(&extended(b"-0").unwrap(), 3), b"-0.000");
    assert_eq!(fixed(&extended(b"1.125").unwrap(), 2), b"1.12");
    assert_eq!(fixed(&extended(b"1.375").unwrap(), 2), b"1.38");
    assert_eq!(
        hms_turn(&mut b"12:30".to_vec()),
        12.0 / 24.0 + 30.0 / 1440.0
    );
    assert_eq!(dms_turn(b"360").unwrap(), 0.0);
    assert_eq!(dms_turn(b"1:-30").unwrap(), -(1.0 / 360.0 + 30.0 / 21600.0));
}

#[test]
fn extended_input_preserves_nonfinite_and_zero_signs() {
    use crate::source_text::{extended, fixed};
    // Native parse_longdouble/quadmath_snprintf witness: check_numeric_classes.py.
    for (token, negative, output) in [
        ("nan", false, "nan"),
        ("-nan", true, "-nan"),
        ("nan(payload)", false, "nan"),
        ("-nan(payload)", true, "-nan"),
        ("0", false, "0.000"),
        ("-0", true, "-0.000"),
        ("inf", false, "inf"),
        ("-inf", true, "-inf"),
    ] {
        let value = extended(token.as_bytes()).unwrap();
        assert_eq!(value.is_sign_negative(), negative, "{token}");
        assert_eq!(value.is_nan(), token.contains("nan"), "{token}");
        assert_eq!(value.is_infinite(), token.contains("inf"), "{token}");
        assert_eq!(fixed(&value, 3), output.as_bytes(), "{token}");
    }
}
