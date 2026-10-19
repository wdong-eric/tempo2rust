//! Isolated precision study. No production timing API is defined here.
use num_traits::Num;
use qd::Quad;
use rug::{Float, float::Round, ops::Pow};
use std::{
    fs::{self, File},
    hint::black_box,
    io::{BufWriter, Write},
    time::Instant,
};

const ORACLE: u32 = 512;
const F0: &str = "716.12345678901234567890123456789";
const COEFFICIENTS: [&str; 3] = [
    "-1.234567890123456789e-15",
    "2.34567890123456789e-27",
    "-3.4567890123456789e-39",
];
const EPOCH: &str = "55000.1234567890123456789012345";

// This grammar is the experiment's finite decimal boundary, not a PAR/TIM parser.
fn decimal(s: &str, bits: u32) -> Result<Float, &'static str> {
    let s = s.replace(['D', 'd'], "e");
    let mut parts = s.split(['e', 'E']);
    let mantissa = parts.next().unwrap().trim_start_matches(['+', '-']);
    // Only one leading sign is allowed.
    let unsigned = s.strip_prefix(['+', '-']).unwrap_or(&s);
    if unsigned.starts_with(['+', '-']) {
        return Err("invalid decimal");
    }
    let mut digits = 0;
    let mut dots = 0;
    for c in mantissa.chars() {
        if c.is_ascii_digit() {
            digits += 1;
        } else if c == '.' {
            dots += 1;
        } else {
            return Err("invalid decimal");
        }
    }
    if digits == 0 || dots > 1 {
        return Err("invalid decimal");
    }
    if let Some(exponent) = parts.next() {
        let exponent = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        if exponent.is_empty() || !exponent.bytes().all(|c| c.is_ascii_digit()) {
            return Err("invalid exponent");
        }
    }
    if parts.next().is_some() {
        return Err("invalid exponent");
    }
    let x = Float::with_val(bits, Float::parse(&s).map_err(|_| "invalid decimal")?);
    if x.is_finite() {
        Ok(x)
    } else {
        Err("non-finite value")
    }
}
fn f(s: &str, bits: u32) -> Float {
    decimal(s, bits).unwrap()
}

// Diagnostic MPFR assistance only: qd 0.8.0 has no working decimal parser.
// Preserve the residual instead of promoting an already-rounded binary64 value.
fn split(x: &Float) -> Quad {
    let hi = x.to_f64_round(Round::Nearest);
    let lo = Float::with_val(ORACLE, x - hi).to_f64_round(Round::Nearest);
    Quad(hi, lo)
}
fn q(s: &str) -> Quad {
    split(&f(s, ORACLE))
}
fn exact(x: Quad) -> Float {
    let mut sum = Float::with_val(ORACLE, x.0);
    sum += x.1;
    sum
}
fn text(x: &Float) -> String {
    x.to_string_radix(10, None)
}

struct Results {
    file: BufWriter<File>,
    failures: usize,
    rug_failures: usize,
    reference_failures: usize,
    rows: usize,
}
impl Results {
    fn new() -> Self {
        let mut file = BufWriter::new(File::create("results/numerical.tsv").unwrap());
        writeln!(
            file,
            "group\tbackend\tcase\tactual\texpected\tabs_error\tunit\ttolerance\tstatus"
        )
        .unwrap();
        Self {
            file,
            failures: 0,
            rug_failures: 0,
            reference_failures: 0,
            rows: 0,
        }
    }
    fn check(
        &mut self,
        group: &str,
        backend: &str,
        case: &str,
        actual: &Float,
        expected: &Float,
        unit: &str,
        tolerance: Float,
    ) {
        let error = Float::with_val(ORACLE, actual - expected).abs();
        let passed = actual.is_finite() && expected.is_finite() && error <= tolerance;
        self.rows += 1;
        if !passed {
            self.failures += 1;
            if backend == "rug128" {
                self.rug_failures += 1;
            }
            if backend == "mpfr256" {
                self.reference_failures += 1;
            }
        }
        writeln!(
            self.file,
            "{group}\t{backend}\t{case}\t{}\t{}\t{}\t{unit}\t{}\t{}",
            text(actual),
            text(expected),
            text(&error),
            text(&tolerance),
            if passed { "PASS" } else { "FAIL" }
        )
        .unwrap();
    }
    fn abs(
        &mut self,
        group: &str,
        backend: &str,
        case: &str,
        actual: &Float,
        expected: &Float,
        unit: &str,
        tolerance: &str,
    ) {
        self.check(
            group,
            backend,
            case,
            actual,
            expected,
            unit,
            f(tolerance, ORACLE),
        );
    }
    fn scaled(
        &mut self,
        group: &str,
        backend: &str,
        case: &str,
        actual: &Float,
        expected: &Float,
        tolerance: &str,
        unit: &str,
    ) {
        let scale = expected.clone().abs().max(&Float::with_val(ORACLE, 1));
        self.check(
            group,
            backend,
            case,
            actual,
            expected,
            unit,
            scale * f(tolerance, ORACLE),
        );
    }
    fn oracle(&mut self, case: &str, a: &Float, b: &Float, unit: &str) {
        self.scaled(
            "reference_convergence",
            "mpfr256",
            case,
            a,
            b,
            "1e-60",
            unit,
        );
    }
}

fn epochs() -> Vec<(String, String)> {
    let mut values = Vec::new();
    for filename in ["test1.tim", "test2.tim"] {
        let data = fs::read_to_string(format!("../../tempo2/tests/test_data/{filename}")).unwrap();
        for (line, record) in data.lines().enumerate().skip(1) {
            let fields: Vec<_> = record.split_whitespace().collect();
            if fields.is_empty() {
                continue;
            }
            values.push((format!("{filename}:{}", line + 1), fields[2].to_owned()));
        }
    }
    values.extend(
        [
            ("boundary_before", "59999.99999999999999999999999"),
            ("boundary_after", "60000.00000000000000000000001"),
            ("exponent_e", "5.600078912398509389208687235e4"),
            ("exponent_E", "+5.600078912398509389208687235E+4"),
            ("par_exponent_D", "5.600078912398509389208687235D4"),
            ("par_exponent_d", "5.600078912398509389208687235d+4"),
        ]
        .map(|(a, b)| (a.to_owned(), b.to_owned())),
    );
    values
}

fn parsing_and_epochs(results: &mut Results, epochs: &[(String, String)]) {
    let mut io = BufWriter::new(File::create("results/parsing.tsv").unwrap());
    writeln!(
        io,
        "input\tqd_Num_native\trug_native\tfinite_decimal_boundary"
    )
    .unwrap();
    let invalid = [
        "", ".", "1.2.3", "1e", "1e+", "1e2junk", "1e2e3", "--1", "+-1", "1_000", " 1", "1 ",
        "NaN", "inf", "-inf", "infinity",
    ];
    for s in epochs.iter().map(|(_, s)| s.as_str()).chain(invalid) {
        let q_native = Quad::from_str_radix(s, 10);
        assert!(q_native.is_err(), "qd native parser behavior changed");
        let native = Float::parse(s).map(|x| Float::with_val(128, x));
        let native_status = match native {
            Ok(x) if x.is_finite() => "finite",
            Ok(_) => "nonfinite",
            Err(_) => "error",
        };
        writeln!(
            io,
            "{s:?}\terror\t{native_status}\t{}",
            if decimal(s, 128).is_ok() {
                "accepted"
            } else {
                "rejected"
            }
        )
        .unwrap();
    }
    for s in invalid {
        assert!(decimal(s, 128).is_err(), "accepted invalid input {s:?}");
    }
    assert_eq!(f("1D-3", 128), f("1e-3", 128));
    // A failed parse must not poison the next independent valid operation.
    assert!(decimal("bad", 128).is_err());
    assert_eq!(f("1.25", 128), 1.25);

    for (case, input) in epochs {
        let reference = f(input, ORACLE);
        let short = f(input, 256);
        results.oracle(&format!("parse/{case}"), &short, &reference, "day");
        let a = q(input);
        let b = f(input, 128);
        let binary64 = reference.to_f64();
        for (name, value) in [
            ("qd_assisted", exact(a)),
            ("rug128", Float::with_val(ORACLE, &b)),
            ("f64_control", Float::with_val(ORACLE, binary64)),
        ] {
            results.abs(
                "epoch_parse",
                name,
                case,
                &(value * 86400),
                &(reference.clone() * 86400),
                "s",
                "1e-12",
            );
        }
        let rug_serialized = text(&b);
        let rug_back = f(&rug_serialized, 128);
        assert_eq!(rug_back, b);
        results.abs(
            "decimal_roundtrip",
            "rug128",
            case,
            &rug_back,
            &b,
            "day",
            "0",
        );
        let q_serialized = text(&exact(a));
        let q_back = q(&q_serialized);
        assert_eq!(
            (q_back.0.to_bits(), q_back.1.to_bits()),
            (a.0.to_bits(), a.1.to_bits())
        );
        results.abs(
            "decimal_roundtrip",
            "qd_assisted",
            case,
            &exact(q_back),
            &exact(a),
            "day",
            "0",
        );
        // A separate lossless limb transport tests that both components survive I/O.
        let limbs = format!("{:016x}:{:016x}", a.0.to_bits(), a.1.to_bits());
        let (hi, lo) = limbs.split_once(':').unwrap();
        let restored = Quad(
            f64::from_bits(u64::from_str_radix(hi, 16).unwrap()),
            f64::from_bits(u64::from_str_radix(lo, 16).unwrap()),
        );
        assert_eq!(restored, a);

        for delay in ["1e-9", "-1e-9", "4e-9", "-4e-9"] {
            let case = format!("{case}/{delay}");
            let wanted = f(delay, ORACLE);
            let qa = ((a + q(delay) / Quad::from(86400.0)) - a) * Quad::from(86400.0);
            let rb = ((b.clone() + f(delay, 128) / 86400) - &b) * 86400;
            let r256 = ((short.clone() + f(delay, 256) / 86400) - &short) * 86400;
            let r512 = ((reference.clone() + &wanted / Float::with_val(ORACLE, 86400))
                - &reference)
                * 86400;
            results.oracle(&format!("delay/{case}"), &r256, &r512, "s");
            results.abs(
                "delay",
                "qd_assisted",
                &case,
                &exact(qa),
                &wanted,
                "s",
                "1e-12",
            );
            results.abs("delay", "rug128", &case, &rb, &wanted, "s", "1e-12");
            let control =
                ((binary64 + delay.parse::<f64>().unwrap() / 86400.0) - binary64) * 86400.0;
            results.abs(
                "delay",
                "f64_control",
                &case,
                &Float::with_val(ORACLE, control),
                &wanted,
                "s",
                "1e-12",
            );
            // Full transport -> parse -> subtract -> compare at extended precision.
            let changed_q = a + q(delay) / Quad::from(86400.0);
            let changed_r = b.clone() + f(delay, 128) / 86400;
            let recovered_q =
                (q(&text(&exact(changed_q))) - q(&q_serialized)) * Quad::from(86400.0);
            let recovered_r = (f(&text(&changed_r), 128) - f(&rug_serialized, 128)) * 86400;
            results.abs(
                "transport_delay",
                "qd_assisted",
                &case,
                &exact(recovered_q),
                &wanted,
                "s",
                "1e-12",
            );
            results.abs(
                "transport_delay",
                "rug128",
                &case,
                &recovered_r,
                &wanted,
                "s",
                "1e-12",
            );
        }
    }
    for (left, right) in [
        (
            "56000.78912398509389208687235",
            "56000.78912398509390366094642",
        ),
        ("59999.99999999999999", "60000.00000000000001"),
        ("50000.04000462962962962963", "50000.04"),
    ] {
        let case = format!("{right}-{left}");
        let expected = (f(right, ORACLE) - f(left, ORACLE)) * 86400;
        results.oracle(
            &format!("difference/{case}"),
            &((f(right, 256) - f(left, 256)) * 86400),
            &expected,
            "s",
        );
        results.abs(
            "epoch_difference",
            "qd_assisted",
            &case,
            &exact((q(right) - q(left)) * Quad::from(86400.0)),
            &expected,
            "s",
            "1e-12",
        );
        results.abs(
            "epoch_difference",
            "rug128",
            &case,
            &((f(right, 128) - f(left, 128)) * 86400),
            &expected,
            "s",
            "1e-12",
        );
    }
}

// Source order: formResiduals.C:515-535. Integer F0*integer-day contribution is
// omitted from reduced phase, then restored for absolute phase and pulse count.
fn spin_rug(
    epoch: &Float,
    pepoch: &Float,
    frequency: &Float,
    coefficients: &[Float],
) -> (Float, Float, Float, Float, Float) {
    let bits = epoch.prec();
    let nf0 = frequency.clone().trunc();
    let ntpd = epoch.clone().trunc() - pepoch.clone().trunc();
    let ff0 = frequency.clone() - &nf0;
    let ftpd = (epoch.clone() - epoch.clone().trunc()) - (pepoch.clone() - pepoch.clone().trunc());
    let delta = (epoch.clone() - pepoch) * 86400;
    let mut phase = (nf0.clone() * &ftpd + ntpd.clone() * &ff0 + &ftpd * &ff0) * 86400;
    let mut arg = Float::with_val(bits, &delta * &delta);
    let mut phase3 = (Float::with_val(bits, 0.5) * &coefficients[0]) * &arg;
    for (coefficient, divisor) in coefficients.iter().skip(1).zip([6, 24]) {
        arg *= &delta;
        phase3 += (coefficient.clone() / divisor) * &arg;
    }
    phase += phase3;
    let phaseint = nf0 * ntpd * 86400i32;
    let absolute = phaseint.clone() + &phase;
    let rounded = nlong_rug(&phase);
    let fractional = phase.clone() - &rounded;
    let residual = fractional.clone() / frequency;
    (phase, absolute, fractional, residual, phaseint + rounded)
}
fn nlong_rug(x: &Float) -> Float {
    (x.clone() + if x > &0 { 0.5f64 } else { -0.5f64 }).trunc()
}
fn nlong_qd(x: Quad) -> Quad {
    (x + Quad::from(if x > Quad::ZERO { 0.5 } else { -0.5 })).trunc()
}
fn spin_qd(epoch: Quad, pepoch: Quad, frequency: Quad, coefficients: &[Quad]) -> [Quad; 5] {
    let nf0 = frequency.trunc();
    let ntpd = epoch.trunc() - pepoch.trunc();
    let ff0 = frequency - nf0;
    let ftpd = (epoch - epoch.trunc()) - (pepoch - pepoch.trunc());
    let delta = (epoch - pepoch) * Quad::from(86400.0);
    let mut phase = (nf0 * ftpd + ntpd * ff0 + ftpd * ff0) * Quad::from(86400.0);
    let mut arg = delta * delta;
    let mut phase3 = (Quad::from(0.5) * coefficients[0]) * arg;
    for (coefficient, divisor) in coefficients.iter().skip(1).zip([6.0, 24.0]) {
        arg *= delta;
        phase3 += (*coefficient / Quad::from(divisor)) * arg;
    }
    phase += phase3;
    let phaseint = nf0 * ntpd * Quad::from(86400.0);
    let rounded = nlong_qd(phase);
    let fractional = phase - rounded;
    [
        phase,
        phaseint + phase,
        fractional,
        fractional / frequency,
        phaseint + rounded,
    ]
}
fn spin(results: &mut Results) {
    let zero_terms = COEFFICIENTS.map(|s| f(s, 128));
    let anchor = spin_rug(&f(EPOCH, 128), &f(EPOCH, 128), &f(F0, 128), &zero_terms);
    assert_eq!(anchor.0, 0);
    assert_eq!(anchor.1, 0);
    assert_eq!(anchor.4, 0);
    for days in [-20000, -10000, -1, 0, 1, 10000, 20000] {
        let input = text(&(f(EPOCH, ORACLE) + days + f("0.3141592653589793238462643383", ORACLE)));
        let case = format!("span_{days}_days_plus_fraction");
        let run = |p| {
            let c: Vec<_> = COEFFICIENTS.map(|s| f(s, p)).into();
            let (a, b, c, d, e) = spin_rug(&f(&input, p), &f(EPOCH, p), &f(F0, p), &c);
            [a, b, c, d, e]
        };
        let reference = run(ORACLE);
        // Independent unsplit polynomial checks the source-order transcription.
        let delta = (f(&input, ORACLE) - f(EPOCH, ORACLE)) * 86400i32;
        let mut polynomial = f(F0, ORACLE) * &delta;
        for ((coefficient, power), factorial) in COEFFICIENTS.iter().zip(2u32..=4).zip([2, 6, 24]) {
            polynomial += f(coefficient, ORACLE) * delta.clone().pow(power) / factorial;
        }
        results.scaled(
            "spin_formula_identity",
            "mpfr512",
            &case,
            &reference[1],
            &polynomial,
            "1e-140",
            "turn",
        );
        let convergence = run(256);
        let candidate = run(128);
        let qs = spin_qd(q(&input), q(EPOCH), q(F0), &COEFFICIENTS.map(q));
        for (i, group) in [
            "spin_reduced",
            "spin_absolute",
            "spin_fractional",
            "spin_residual",
            "pulse_number",
        ]
        .iter()
        .enumerate()
        {
            results.oracle(
                &format!("{group}/{case}"),
                &convergence[i],
                &reference[i],
                if i == 3 { "s" } else { "turn" },
            );
            let unit = if i == 3 { "s" } else { "turn" };
            let tolerance = if i == 4 {
                "0"
            } else if i == 3 {
                "1e-12"
            } else {
                "1e-9"
            };
            results.abs(
                group,
                "rug128",
                &case,
                &candidate[i],
                &reference[i],
                unit,
                tolerance,
            );
            results.abs(
                group,
                "qd_assisted",
                &case,
                &exact(qs[i]),
                &reference[i],
                unit,
                tolerance,
            );
        }
    }
}

fn operations(results: &mut Results) {
    for (a, b) in [
        ("56000.78912398509389208687235", "-56000.78912398509389"),
        ("716.12345678901234567890123456789", "86400"),
        ("1e-15", "1e9"),
        ("-2.5", "0.99999999999999999999"),
    ] {
        for op in ["add", "sub", "mul", "div"] {
            let evaluate = |bits| {
                let (x, y) = (f(a, bits), f(b, bits));
                match op {
                    "add" => x + y,
                    "sub" => x - y,
                    "mul" => x * y,
                    _ => x / y,
                }
            };
            let (x, y) = (q(a), q(b));
            let qa = match op {
                "add" => x + y,
                "sub" => x - y,
                "mul" => x * y,
                _ => x / y,
            };
            let expected = evaluate(ORACLE);
            let case = format!("{op}({a},{b})");
            results.oracle(&case, &evaluate(256), &expected, "dimensionless");
            results.scaled(
                "arithmetic",
                "rug128",
                &case,
                &evaluate(128),
                &expected,
                "1e-28",
                "dimensionless",
            );
            results.scaled(
                "arithmetic",
                "qd_assisted",
                &case,
                &exact(qa),
                &expected,
                "1e-28",
                "dimensionless",
            );
        }
    }
    for input in ["0", "1e-20", "0.1", "0.9", "1", "2", "100", "1e10"] {
        for op in ["sqrt", "ln", "log10", "exp"] {
            if (op == "ln" || op == "log10") && input == "0" {
                continue;
            }
            if op == "exp" && input == "1e10" {
                continue;
            }
            let evaluate = |bits| {
                let x = f(input, bits);
                match op {
                    "sqrt" => x.sqrt(),
                    "ln" => x.ln(),
                    "log10" => x.log10(),
                    _ => x.exp(),
                }
            };
            let x = q(input);
            let actual = match op {
                "sqrt" => x.sqrt(),
                "ln" => x.ln(),
                "log10" => x.log10(),
                _ => x.exp(),
            };
            let reference = evaluate(ORACLE);
            let case = format!("{op}({input})");
            results.oracle(&case, &evaluate(256), &reference, "dimensionless");
            results.scaled(
                "math",
                "rug128",
                &case,
                &evaluate(128),
                &reference,
                "1e-28",
                "dimensionless",
            );
            results.scaled(
                "math",
                "qd_assisted",
                &case,
                &exact(actual),
                &reference,
                "1e-28",
                "dimensionless",
            );
        }
    }
    for input in ["-100", "-1", "-1e-20"] {
        let reference = f(input, ORACLE).exp();
        results.oracle(
            &format!("exp({input})"),
            &f(input, 256).exp(),
            &reference,
            "dimensionless",
        );
        results.scaled(
            "math",
            "rug128",
            &format!("exp({input})"),
            &f(input, 128).exp(),
            &reference,
            "1e-28",
            "dimensionless",
        );
        results.scaled(
            "math",
            "qd_assisted",
            &format!("exp({input})"),
            &exact(q(input).exp()),
            &reference,
            "1e-28",
            "dimensionless",
        );
    }
    // qd has no trig API: only the established MPFR alternative is evaluated here.
    for input in [
        "-6.283185307179586476925286766559",
        "-3.141592653589793238462643383279",
        "-1e-20",
        "0",
        "1e-20",
        "0.7",
        "3.141592653589793238462643383279",
        "6.283185307179586476925286766559",
        "1e6",
    ] {
        for op in ["sin", "cos", "tan"] {
            let evaluate = |p| {
                let x = f(input, p);
                match op {
                    "sin" => x.sin(),
                    "cos" => x.cos(),
                    _ => x.tan(),
                }
            };
            let reference = evaluate(ORACLE);
            let case = format!("{op}({input})");
            results.oracle(&case, &evaluate(256), &reference, "dimensionless");
            results.scaled(
                "orbital_math",
                "rug128",
                &case,
                &evaluate(128),
                &reference,
                "1e-28",
                "dimensionless",
            );
        }
    }
    for input in [
        "-1",
        "-0.99999999999999999999",
        "0",
        "0.99999999999999999999",
        "1",
    ] {
        let reference = f(input, ORACLE).asin();
        results.oracle(
            &format!("asin({input})"),
            &f(input, 256).asin(),
            &reference,
            "rad",
        );
        results.scaled(
            "orbital_math",
            "rug128",
            &format!("asin({input})"),
            &f(input, 128).asin(),
            &reference,
            "1e-28",
            "rad",
        );
    }
    for (y, x) in [
        ("0", "-1"),
        ("1e-20", "-1"),
        ("-1e-20", "-1"),
        ("1", "1"),
        ("-1", "1"),
    ] {
        let evaluate = |p| f(y, p).atan2(&f(x, p));
        let reference = evaluate(ORACLE);
        let case = format!("atan2({y},{x})");
        results.oracle(&case, &evaluate(256), &reference, "rad");
        results.scaled(
            "orbital_math",
            "rug128",
            &case,
            &evaluate(128),
            &reference,
            "1e-28",
            "rad",
        );
    }
    for (a, b) in [
        ("0.1", "2"),
        ("0.9999999999", "0.5"),
        ("1e9", "3"),
        ("2", "-2.5"),
    ] {
        let evaluate = |p| f(a, p).pow(f(b, p));
        let reference = evaluate(ORACLE);
        let case = format!("pow({a},{b})");
        results.oracle(&case, &evaluate(256), &reference, "dimensionless");
        results.scaled(
            "orbital_math",
            "rug128",
            &case,
            &evaluate(128),
            &reference,
            "1e-28",
            "dimensionless",
        );
    }
    for input in [
        "-2.5",
        "-1.5",
        "-0.5",
        "0",
        "0.5",
        "1.5",
        "2.5",
        "0.49999999999999999999",
        "0.50000000000000000001",
        "-0.49999999999999999999",
        "-0.50000000000000000001",
        "0.99999999999999999999",
        "-0.99999999999999999999",
        "100000000000.49999999999999999999",
        "100000000000.50000000000000000001",
        "-100000000000.49999999999999999999",
        "-100000000000.50000000000000000001",
    ] {
        for op in ["trunc", "nlong", "remainder", "floor", "ceil"] {
            let evaluate = |p| {
                let x = f(input, p);
                match op {
                    "trunc" => x.trunc(),
                    "nlong" => nlong_rug(&x),
                    "remainder" => x % Float::with_val(p, 1),
                    "floor" => x.floor(),
                    _ => x.ceil(),
                }
            };
            let expected = evaluate(ORACLE);
            let case = format!("{op}({input})");
            let tolerance = if op == "remainder" { "1e-28" } else { "0" };
            results.oracle(&case, &evaluate(256), &expected, "turn");
            results.abs(
                "boundary",
                "rug128",
                &case,
                &evaluate(128),
                &expected,
                "turn",
                tolerance,
            );
            if op == "floor" || op == "ceil" {
                continue;
            }
            let x = q(input);
            let actual = match op {
                "trunc" => x.trunc(),
                "nlong" => nlong_qd(x),
                _ => x % Quad::ONE,
            };
            results.abs(
                "boundary",
                "qd_assisted",
                &case,
                &exact(actual),
                &expected,
                "turn",
                tolerance,
            );
        }
    }
    for input in [
        "0.49999999999999999999",
        "0.5",
        "0.50000000000000000001",
        "-0.49999999999999999999",
        "-0.5",
        "-0.50000000000000000001",
        "100000000000.49999999999999999999",
        "-100000000000.49999999999999999999",
    ] {
        let expected_fraction = f(input, ORACLE) - nlong_rug(&f(input, ORACLE));
        let expected_residual = expected_fraction.clone() / f(F0, ORACLE);
        let r = f(input, 128);
        let fractional_r = r.clone() - nlong_rug(&r);
        let fractional_q = q(input) - nlong_qd(q(input));
        results.abs(
            "boundary_fraction",
            "rug128",
            input,
            &fractional_r,
            &expected_fraction,
            "turn",
            "1e-9",
        );
        results.abs(
            "boundary_fraction",
            "qd_assisted",
            input,
            &exact(fractional_q),
            &expected_fraction,
            "turn",
            "1e-9",
        );
        results.abs(
            "boundary_residual",
            "rug128",
            input,
            &(fractional_r / f(F0, 128)),
            &expected_residual,
            "s",
            "1e-12",
        );
        results.abs(
            "boundary_residual",
            "qd_assisted",
            input,
            &exact(fractional_q / q(F0)),
            &expected_residual,
            "s",
            "1e-12",
        );
        // Conversion to a bounded machine pulse counter is explicit, never via f64.
        let actual = nlong_rug(&r).to_integer().unwrap().to_i64().unwrap();
        let expected = nlong_rug(&f(input, ORACLE))
            .to_integer()
            .unwrap()
            .to_i64()
            .unwrap();
        assert_eq!(actual, expected);
    }
    for input in ["-56000.78912398509389208687235", "-1e-20", "0", "1e-20"] {
        let expected = f(input, ORACLE).abs();
        results.scaled(
            "arithmetic",
            "rug128",
            &format!("abs({input})"),
            &f(input, 128).abs(),
            &expected,
            "1e-28",
            "dimensionless",
        );
        results.scaled(
            "arithmetic",
            "qd_assisted",
            &format!("abs({input})"),
            &exact(q(input).abs()),
            &expected,
            "1e-28",
            "dimensionless",
        );
    }
    // Exact discrete expected values independently anchor the MPFR oracle.
    for (input, rounded) in [("-2.5", -3), ("-0.5", -1), ("0.5", 1), ("2.5", 3)] {
        assert_eq!(nlong_rug(&f(input, 128)), rounded);
        assert_eq!(f(input, 128).round(), rounded);
    }
    let mut special = BufWriter::new(File::create("results/special.tsv").unwrap());
    writeln!(special, "case\tqd\trug128\tinterpretation").unwrap();
    for (case, a, b) in [
        ("sqrt(-1)", q("-1").sqrt(), f("-1", 128).sqrt()),
        ("ln(0)", q("0").ln(), f("0", 128).ln()),
        ("ln(-1)", q("-1").ln(), f("-1", 128).ln()),
        ("1/0", Quad::ONE / Quad::ZERO, Float::with_val(128, 1) / 0),
        ("exp(709)", q("709").exp(), f("709", 128).exp()),
        ("exp(-709)", q("-709").exp(), f("-709", 128).exp()),
    ] {
        writeln!(
            special,
            "{case}\t{a:?}\t{}\tdiagnostic; timing boundary must reject invalid results",
            text(&b)
        )
        .unwrap();
    }
    let a = Quad(1.0, 1e-30);
    let b = Quad(1.0, 0.0);
    writeln!(
        special,
        "inherent_le_same_hi\t{}\tfalse\tqd inherent le ignores low limb when highs equal",
        a.le(b)
    )
    .unwrap();
    writeln!(
        special,
        "operator_le_same_hi\t{}\tfalse\tderived PartialOrd differs from inherent le",
        a <= b
    )
    .unwrap();
    assert!(a.le(b));
    assert!(!(a <= b));
}

fn benchmark(epochs: &[(String, String)]) {
    let mut out = BufWriter::new(File::create("results/timing.tsv").unwrap());
    writeln!(out,"backend\tworkload\titems_per_trial\ttrials\tmin_ns_per_item\tmedian_ns_per_item\tmax_ns_per_item").unwrap();
    let mut measure = |backend: &str, workload: &str, n: usize, action: &mut dyn FnMut(usize)| {
        for i in 0..100 {
            action(i);
        }
        let mut times = Vec::new();
        for _ in 0..7 {
            let start = Instant::now();
            for i in 0..n {
                action(i);
            }
            times.push(start.elapsed().as_nanos() as f64 / n as f64);
        }
        times.sort_by(f64::total_cmp);
        writeln!(
            out,
            "{backend}\t{workload}\t{n}\t7\t{:.3}\t{:.3}\t{:.3}",
            times[0], times[3], times[6]
        )
        .unwrap();
    };
    let qvalues: Vec<_> = epochs.iter().map(|(_, s)| q(s)).collect();
    let rvalues: Vec<_> = epochs.iter().map(|(_, s)| f(s, 128)).collect();
    let qp = q(EPOCH);
    let qf = q(F0);
    let qc = COEFFICIENTS.map(q);
    let rp = f(EPOCH, 128);
    let rf = f(F0, 128);
    let rc = COEFFICIENTS.map(|s| f(s, 128));
    let n = 10000;
    let qdelay = q("1e-9") / Quad::from(86400.0);
    measure("qd_assisted", "source_F0_to_F3_phase", n, &mut |i| {
        black_box(spin_qd(black_box(qvalues[i % qvalues.len()]), qp, qf, &qc));
    });
    measure("rug128", "source_F0_to_F3_phase", n, &mut |i| {
        black_box(spin_rug(
            black_box(&rvalues[i % rvalues.len()]),
            &rp,
            &rf,
            &rc,
        ));
    });
    measure("qd_assisted", "add_then_subtract_1ns", n, &mut |i| {
        let a = black_box(qvalues[i % qvalues.len()]);
        black_box(((a + qdelay) - a) * Quad::from(86400.0));
    });
    let delay = f("1e-9", 128) / 86400;
    measure("rug128", "add_then_subtract_1ns", n, &mut |i| {
        let a = black_box(&rvalues[i % rvalues.len()]);
        black_box((a.clone() + &delay - a) * 86400);
    });
    measure("rug128", "parse_finite_decimal", n, &mut |i| {
        black_box(f(black_box(&epochs[i % epochs.len()].1), 128));
    });
    measure("qd_assisted", "parse_via_MPFR512_split", n, &mut |i| {
        black_box(q(black_box(&epochs[i % epochs.len()].1)));
    });
    measure("rug128", "decimal_serialize", n, &mut |i| {
        black_box(text(black_box(&rvalues[i % rvalues.len()])));
    });
    measure("qd_assisted", "decimal_via_MPFR512_sum", n, &mut |i| {
        black_box(text(&exact(black_box(qvalues[i % qvalues.len()]))));
    });
}
// units.C affine transform: these two constants are binary64 upstream.
fn affine_epochs(results: &mut Results, epochs: &[(String, String)]) {
    for (case, input) in epochs {
        for inverse in [false, true] {
            let evaluate = |bits| {
                let origin = Float::with_val(bits, 43144.0003725f64);
                let k = Float::with_val(bits, 1) + Float::with_val(bits, 1.55051979176e-8f64);
                let factor = if inverse {
                    Float::with_val(bits, 1) / k
                } else {
                    k
                };
                (f(input, bits) - &origin) * factor + origin
            };
            let expected = evaluate(ORACLE);
            let case = format!("{case}/inverse={inverse}");
            results.oracle(&format!("affine/{case}"), &evaluate(256), &expected, "day");
            let origin = Quad::from(43144.0003725f64);
            let k = Quad::ONE + Quad::from(1.55051979176e-8f64);
            let factor = if inverse { k.recip() } else { k };
            let actual = (q(input) - origin) * factor + origin;
            results.abs(
                "affine_epoch",
                "rug128",
                &case,
                &(evaluate(128) * 86400),
                &(expected.clone() * 86400),
                "s",
                "1e-12",
            );
            results.abs(
                "affine_epoch",
                "qd_assisted",
                &case,
                &(exact(actual) * 86400),
                &(expected * 86400),
                "s",
                "1e-12",
            );
        }
    }
}

// A's exported barycentric epochs are inputs here. This is a phase comparison,
// not a new implementation of clocks, ephemerides, or observatory corrections.
fn reference_replay(results: &mut Results) {
    use std::collections::BTreeMap;
    for filename in [
        "test1.tim-standard.tsv",
        "test1.tim-subtractBATs.tsv",
        "test2.tim-standard.tsv",
        "long_span.tsv",
    ] {
        let content = fs::read_to_string(format!("results/reference_inputs/{filename}")).unwrap();
        let mut stages: BTreeMap<String, BTreeMap<usize, BTreeMap<String, String>>> =
            BTreeMap::new();
        let mut config = BTreeMap::new();
        let mut parameters = BTreeMap::new();
        for line in content.lines() {
            let fields: Vec<_> = line.split('\t').collect();
            match fields[0] {
                "obs" => {
                    stages
                        .entry(fields[1].into())
                        .or_default()
                        .entry(fields[2].parse().unwrap())
                        .or_default()
                        .insert(fields[3].into(), fields[4].into());
                }
                "config" => {
                    config.insert(fields[1], fields[2]);
                }
                "param" => {
                    parameters.insert(fields[1], fields[4]);
                }
                _ => panic!("unexpected A export record"),
            }
        }
        assert_eq!(config["remove_mean"], "0");
        assert_eq!(config["fit_performed"], "false");
        assert_eq!(config["binary_model"], "NONE");
        assert_eq!(config["refphs"], "0");
        for name in ["F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"] {
            assert_eq!(parameters[name], "0", "only F0-F3 supported in replay");
        }
        for (stage, observations) in stages {
            if !observations[&0].contains_key("phase_turn") {
                continue;
            }
            let first = &observations[&0]["bbat_mjd"];
            for (id, fields) in &observations {
                assert_eq!(fields["binary_delay_s"], "0");
                let input = &fields["bbat_mjd"];
                for (backend, bits) in [("rug128", 128), ("mpfr113_reference_replay", 113)] {
                    let coefficients = ["F1", "F2", "F3"].map(|name| f(parameters[name], bits));
                    let epoch = f(parameters["PEPOCH"], bits);
                    let frequency = f(parameters["F0"], bits);
                    let initial = spin_rug(&f(first, bits), &epoch, &frequency, &coefficients).0;
                    let offset = initial % Float::with_val(bits, 1);
                    let (phase, _, _, _, _) =
                        spin_rug(&f(input, bits), &epoch, &frequency, &coefficients);
                    let phaseint = frequency.clone().trunc()
                        * (f(input, bits).trunc() - epoch.clone().trunc())
                        * 86400i32;
                    let reduced = phase - offset.clone();
                    let rounded = nlong_rug(&reduced);
                    let residual = (reduced.clone() - &rounded) / &frequency;
                    for (name, actual) in [
                        ("phase_turn", phaseint.clone() + &reduced),
                        ("nphase_turn", rounded.clone()),
                        ("pulse_number", phaseint + rounded),
                        ("residual_s", residual),
                    ] {
                        let tolerance = if name == "pulse_number" || name == "nphase_turn" {
                            "0"
                        } else if name == "residual_s" {
                            "1e-12"
                        } else {
                            "1e-9"
                        };
                        results.abs(
                            "A_phase_replay",
                            backend,
                            &format!("{filename}/{stage}/{id}/{name}"),
                            &actual,
                            &f(&fields[name], ORACLE),
                            if name == "residual_s" { "s" } else { "turn" },
                            tolerance,
                        );
                    }
                }
                let coefficients = ["F1", "F2", "F3"].map(|name| q(parameters[name]));
                let frequency = q(parameters["F0"]);
                let initial =
                    spin_qd(q(first), q(parameters["PEPOCH"]), frequency, &coefficients)[0];
                let offset = initial % Quad::ONE;
                let values = spin_qd(q(input), q(parameters["PEPOCH"]), frequency, &coefficients);
                let phaseint = frequency.trunc()
                    * (q(input).trunc() - q(parameters["PEPOCH"]).trunc())
                    * Quad::from(86400.0);
                let reduced = values[0] - offset;
                let rounded = nlong_qd(reduced);
                for (name, actual) in [
                    ("phase_turn", phaseint + reduced),
                    ("nphase_turn", rounded),
                    ("pulse_number", phaseint + rounded),
                    ("residual_s", (reduced - rounded) / frequency),
                ] {
                    let tolerance = if name == "pulse_number" || name == "nphase_turn" {
                        "0"
                    } else if name == "residual_s" {
                        "1e-12"
                    } else {
                        "1e-9"
                    };
                    results.abs(
                        "A_phase_replay",
                        "qd_assisted",
                        &format!("{filename}/{stage}/{id}/{name}"),
                        &exact(actual),
                        &f(&fields[name], ORACLE),
                        if name == "residual_s" { "s" } else { "turn" },
                        tolerance,
                    );
                }
            }
        }
    }
}

fn main() {
    fs::create_dir_all("results").unwrap();
    let epochs = epochs();
    let mut results = Results::new();
    parsing_and_epochs(&mut results, &epochs);
    operations(&mut results);
    affine_epochs(&mut results, &epochs);
    spin(&mut results);
    reference_replay(&mut results);
    results.file.flush().unwrap();
    assert_eq!(
        results.reference_failures, 0,
        "reference convergence failed"
    );
    assert_eq!(results.rug_failures, 0, "Rug candidate failed a study gate");
    benchmark(&epochs);
    println!(
        "{} numerical rows; {} failed gates (qd/control); Rug128 failures {}; reference convergence failures {}",
        results.rows, results.failures, results.rug_failures, results.reference_failures
    );
    println!(
        "Native qd parser unavailable, missing API compile probes are separate; assisted qd results are not a native I/O pass."
    );
}
