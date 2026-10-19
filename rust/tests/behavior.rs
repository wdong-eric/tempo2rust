use rug::{Float, float::Round, ops::Pow};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
use tempo2_rust::{PRECISION_BITS, Scalar, TimingInput, evaluate_barycentric, load};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Inputs(PathBuf);
impl Inputs {
    fn new(par: &str, tim: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "tempo2-rust-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("model.par"), par).unwrap();
        fs::write(directory.join("arrival.tim"), tim).unwrap();
        Self(directory)
    }
    fn load(&self) -> Result<TimingInput, tempo2_rust::TimingError> {
        load(&self.0.join("model.par"), &self.0.join("arrival.tim"))
    }
}
impl Drop for Inputs {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const PAR: &str = "PSRJ TEST\nF0 1\nPEPOCH 50000\nUNITS TCB\n";
const TIM: &str = "FORMAT 1\na 1400 50000 10 @\nb 1400 50000.125 10 @\n";

fn high(token: &str) -> Float {
    Float::with_val(512, Float::parse(token).unwrap())
}

#[test]
fn strict_decimals_and_nanosecond_round_trips() {
    for token in [
        "0",
        "-0",
        "+1.25",
        ".125",
        "12.",
        "1e-20",
        "1D+3",
        "-1d-8",
        "57305.2468135792468598755431098752962962963",
    ] {
        let value = token.parse::<Scalar>().unwrap();
        assert_eq!(value.as_float().prec(), PRECISION_BITS);
        assert_eq!(value, value.to_string().parse::<Scalar>().unwrap());
    }
    for token in [
        "",
        ".",
        "nan",
        "NaN",
        "Inf",
        "-infinity",
        "0x1p2",
        "1_000",
        "1 2",
        "1.2x",
        "1e",
        "1e+",
        "1e2e3",
        "--1",
        " 1",
        "1 ",
        "1e999999999999999999999",
        "1e-999999999999999999999",
    ] {
        assert!(token.parse::<Scalar>().is_err(), "{token}");
    }
    let base = "50000.99999999999999999999999999"
        .parse::<Scalar>()
        .unwrap();
    for delay in ["1e-9", "-1e-9", "4e-9", "-4e-9"] {
        let seconds = delay.parse::<Scalar>().unwrap();
        let shifted = base.as_float().clone() + seconds.as_float().clone() / 86400i32;
        let text = shifted.to_string_radix(10, None);
        let recovered =
            (text.parse::<Scalar>().unwrap().as_float().clone() - base.as_float()) * 86400i32;
        assert!((Float::with_val(512, recovered) - high(delay)).abs() < high("2e-29"));
    }
}

#[test]
fn nonzero_f3_agrees_with_independent_unsplit_polynomial() {
    let coefficients = [
        "123.456789123456789123456789",
        "-1.23456789123456789e-15",
        "3.456789123456789e-27",
        "2.4e-18",
    ];
    let epochs = ["50000.28125", "49999.75", "50000.25", "50001.25"];
    let par = format!(
        "PSRJ F3TEST\nF0 {}\nF1 {}\nF2 {}\nF3 {}\nPEPOCH 50000.25\nUNITS TCB\n",
        coefficients[0], coefficients[1], coefficients[2], coefficients[3]
    );
    let tim = format!(
        "FORMAT 1\n{}",
        epochs
            .iter()
            .enumerate()
            .map(|(i, epoch)| format!("row{i} 1400 {epoch} 10 @\n"))
            .collect::<String>()
    );
    let files = Inputs::new(&par, &tim);
    let input = files.load().unwrap();
    let actual = evaluate_barycentric(&input).unwrap();
    let polynomial = |epoch: &str| {
        let t = (high(epoch) - high("50000.25")) * 86400i32;
        let mut value = high("0");
        for (order, (coefficient, denominator)) in
            coefficients.iter().zip([1i32, 2, 6, 24]).enumerate()
        {
            value += high(coefficient) * t.clone().pow((order + 1) as u32) / denominator;
        }
        value
    };
    let first = polynomial(epochs[0]);
    let reference = first.clone() - first.trunc();
    let mut measurements = Vec::new();
    for (index, row) in actual.observations.iter().enumerate() {
        let expected_phase = polynomial(epochs[index]) - &reference;
        let rounded = expected_phase.clone().round();
        let expected_residual = (expected_phase.clone() - &rounded) / high(coefficients[0]);
        let phase_error = (Float::with_val(512, row.phase_turn.as_float()) - &expected_phase).abs();
        let residual_error =
            (Float::with_val(512, row.residual_s.as_float()) - &expected_residual).abs();
        assert!(phase_error < high("1e-27"));
        assert!(residual_error < high("1e-29"));
        assert_eq!(
            row.pulse_number,
            rounded
                .to_integer_round(Round::Zero)
                .unwrap()
                .0
                .to_i64()
                .unwrap()
        );
        measurements.push(
            json!({"index": index, "phase_error_turn": phase_error.to_string_radix(10, None),
            "residual_error_s": residual_error.to_string_radix(10, None)}),
        );
    }
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/analytic-f3.json");
    fs::write(output, serde_json::to_vec_pretty(&measurements).unwrap()).unwrap();
}

#[test]
fn end_to_end_signed_half_pulses() {
    for (frequency, pulse) in [
        ("0.124999999999999999999", 337),
        ("0.125", 338),
        ("0.125000000000000000001", 338),
    ] {
        let par = format!("PSRJ HALVES\nF0 {frequency}\nPEPOCH 0\nUNITS TCB\n");
        let files = Inputs::new(
            &par,
            "FORMAT 1\nref 1400 0 0 @\npositive 1400 0.03125 0 @\nnegative 1400 -0.03125 0 @\n",
        );
        let result = evaluate_barycentric(&files.load().unwrap()).unwrap();
        assert_eq!(result.observations[1].pulse_number, pulse);
        assert_eq!(result.observations[2].pulse_number, -pulse);
        if frequency == "0.125" {
            assert_eq!(result.observations[1].residual_s.as_float(), &-4);
            assert_eq!(result.observations[2].residual_s.as_float(), &4);
        }
    }
    let files = Inputs::new(
        "PSRJ SIGNED\nF0 0.125\nPEPOCH 0\nUNITS TCB\n",
        "FORMAT 1\nref 1400 -0.03125 10 @\nnext 1400 0 10 @\n",
    );
    let result = evaluate_barycentric(&files.load().unwrap()).unwrap();
    assert_eq!(result.reference_phase_turn.as_float(), &-0.5);
    assert_eq!(result.observations[0].pulse_number, -337);
    assert_eq!(result.observations[1].pulse_number, 1);
    assert_eq!(result.observations[1].residual_s.as_float(), &-4);
    assert!(!result.input.parameters().contains_key("DM"));
}

#[test]
fn preserves_metadata_identity_and_original_order() {
    let files = Inputs::new(
        "# comment\nPSRJ TEST\nF0 1D0 1 2d-12\nF1 -1e-15 2\nPEPOCH 50000\nEPHVER 5\nRAJ 01:00:00\nDECJ -00:00:00\nDM 100 0.25\nTZRMJD 51000\nTZRSITE @\n",
        "C comment\nFORMAT 1\nsame 1400.000 50001.25 1D1 @\nsame 1400 49999.75 10 @\n",
    );
    let input = files.load().unwrap();
    assert_eq!(input.parameters()["F0"].fit_flag, 1);
    assert_eq!(input.parameters()["F1"].fit_flag, 2);
    assert!(!input.parameters()["F3"].set);
    assert_eq!(
        input.parameters()["DM"]
            .uncertainty
            .as_ref()
            .unwrap()
            .as_float(),
        &0.25
    );
    assert_eq!(
        input.parameters()["F0"].original.as_ref().unwrap().tokens[0],
        "1D0"
    );
    assert!(
        input.parameters()["DECJ"]
            .value
            .as_float()
            .is_sign_negative()
    );
    assert_eq!(input.observations()[0].original.line, 3);
    assert_eq!(
        input.observations()[0].original.frequency_mhz_text,
        "1400.000"
    );
    let result = evaluate_barycentric(&input).unwrap();
    // Negative F1 puts the first phase just below 108000; its signed fraction
    // is subtracted, leaving 107999 (not a nearest-pulse reference).
    assert_eq!(result.observations[0].pulse_number, 107999);
    assert_eq!(result.observations[0].residual_s.as_float(), &0);
    assert!(!result.input.conventions().fit_performed);
    assert!(!result.input.conventions().remove_mean);
    assert!(result.observations[1].sat_mjd.as_float() < result.observations[0].sat_mjd.as_float());
}

#[test]
fn rejects_malformed_and_unsupported_inputs_with_context() {
    for (extra, field) in [
        ("F4 0", "F4"),
        ("TRACK 0", "TRACK"),
        ("BINARY DDH", "BINARY"),
        ("DM1 0", "DM1"),
        ("DSHK 0", "DSHK"),
        ("JUMP -f x 0", "JUMP"),
        ("START 50000", "START"),
        ("REFPHS TZR", "REFPHS"),
        ("F0 2", "F0"),
        ("F1 NaN", "F1"),
        ("F2 1e-20 3 0.1", "F2"),
        ("RAJ 24:00:00", "RAJ"),
        ("DECJ -90:00:01", "DECJ"),
        ("EPHVER 2", "EPHVER"),
        ("MYSTERY 0", "MYSTERY"),
    ] {
        let files = Inputs::new(&format!("{PAR}{extra}\n"), TIM);
        let error = files.load().unwrap_err();
        assert_eq!(error.field, field);
        assert_eq!(error.line, Some(5));
        assert!(error.path.ends_with("model.par"));
    }
    for (par, field) in [
        ("PSRJ X\nPEPOCH 50000\nUNITS TCB\n", "F0"),
        ("PSRJ X\nF0 1\nUNITS TCB\n", "PEPOCH"),
        ("PSRJ X\nF0 1\nPEPOCH 50000\n", "UNITS/EPHVER"),
        ("PSRJ X\nF0 0\nPEPOCH 50000\nUNITS TCB\n", "F0"),
        ("PSRJ X\nF0 1\nPEPOCH 50000\nUNITS TDB\n", "UNITS"),
    ] {
        assert_eq!(Inputs::new(par, TIM).load().unwrap_err().field, field);
    }
    for bad in [
        "",
        "FORMAT 1\n",
        "FORMAT 2\n",
        "x 1400 50000 10 @\n",
        "FORMAT 1\nINCLUDE extra.tim\n",
        "FORMAT 1\nx 1400 50000x 10 @\n",
        "FORMAT 1\nx 1400 50000 10 pks\n",
        "FORMAT 1\nx 1400 50000 10 @ -pn 0\n",
        "FORMAT 1\nx 1e999 50000 10 @\n",
        "FORMAT 1\nx 1400 50000 -1 @\n",
        "FORMAT 1\nx 1400 50000 10 @\nFORMAT 1\n",
        "FORMAT 1\nTIME 1400 50000 10 @\n",
    ] {
        let error = Inputs::new(PAR, bad).load().unwrap_err();
        assert!(error.path.ends_with("arrival.tim"));
    }
}

#[test]
fn failed_library_processing_then_valid_processing_is_stateless() {
    let valid = Inputs::new(PAR, TIM);
    let before = evaluate_barycentric(&valid.load().unwrap()).unwrap();
    let missing = load(&valid.0.join("missing.par"), &valid.0.join("arrival.tim"));
    assert!(missing.unwrap_err().to_string().contains("missing.par"));
    assert!(
        Inputs::new(PAR, "FORMAT 1\nx 1400 malformed 10 @\n")
            .load()
            .is_err()
    );
    for (model, tim, expected) in [
        (
            "PSRJ OVERFLOW\nF0 1\nF1 3e9\nPEPOCH 0\nUNITS TCB\n",
            "FORMAT 1\nref 1400 0 10 @\nbad 1400 1 10 @\n",
            "pulse count",
        ),
        (
            "PSRJ OVERFLOW\nF0 2147483648\nPEPOCH 0\nUNITS TCB\n",
            "FORMAT 1\nref 1400 0 10 @\n",
            "F0",
        ),
        (
            "PSRJ OVERFLOW\nF0 100000\nPEPOCH 0\nUNITS TCB\n",
            "FORMAT 1\nref 1400 0 10 @\nbad 1400 30000 10 @\n",
            "integer F0 * day",
        ),
    ] {
        let files = Inputs::new(model, tim);
        let input = files.load().unwrap();
        let snapshot = serde_json::to_vec(&input).unwrap();
        let error = evaluate_barycentric(&input).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        assert_eq!(serde_json::to_vec(&input).unwrap(), snapshot);
    }
    // Reduced count fits i64 but restoring the integral F0*day term overflows.
    let f1 = (high("9223372036854735807") * 2i32) / high("7464960000");
    let model = format!(
        "PSRJ FULLCOUNT\nF0 1\nF1 {}\nPEPOCH 0\nUNITS TCB\n",
        f1.to_string_radix(10, None)
    );
    let files = Inputs::new(&model, "FORMAT 1\nref 1400 0 10 @\nbad 1400 1 10 @\n");
    assert!(
        evaluate_barycentric(&files.load().unwrap())
            .unwrap_err()
            .message
            .contains("full pulse count")
    );
    let after = evaluate_barycentric(&valid.load().unwrap()).unwrap();
    assert_eq!(
        serde_json::to_vec(&before).unwrap(),
        serde_json::to_vec(&after).unwrap()
    );
}

#[test]
fn cli_publishes_complete_json_or_no_result() {
    let files = Inputs::new(PAR, TIM);
    let command = |par: &Path| {
        Command::new(env!("CARGO_BIN_EXE_tempo2-rust"))
            .arg("residuals")
            .arg("--par")
            .arg(par)
            .arg("--tim")
            .arg(files.0.join("arrival.tim"))
            .output()
            .unwrap()
    };
    let failure = command(&files.0.join("missing.par"));
    assert!(!failure.status.success());
    assert!(failure.stdout.is_empty());
    assert!(String::from_utf8_lossy(&failure.stderr).contains("missing.par"));
    let output = command(&files.0.join("model.par"));
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["status"], "completed");
    assert!(document["observations"][1]["pulse_number"].is_string());
    assert!(document["observations"][1]["phase_turn"].is_string());
    assert_eq!(document["engine"]["significand_bits"], 128);
    fs::write(
        files.0.join("model.par"),
        "PSRJ OVERFLOW\nF0 1\nF1 3e9\nPEPOCH 0\nUNITS TCB\n",
    )
    .unwrap();
    let failure = command(&files.0.join("model.par"));
    assert!(!failure.status.success());
    assert!(failure.stdout.is_empty());
}
