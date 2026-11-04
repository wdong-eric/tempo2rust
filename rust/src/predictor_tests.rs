//! AF analytic contracts, parser failures and owned lifecycle regressions.
use crate::{cheby2d::*, predictor_number::n, t1polyco::*, tempo2pred::*};
use std::io::Cursor;

// Copies used to compare ownership forms must retain negative NaN inputs;
// Rug's ordinary Clone initializes a fresh positive-NaN destination.
fn signed_copy(x: &crate::predictor_number::N) -> crate::predictor_number::N {
    let copy = x.clone();
    if x.as_float().is_nan() && x.as_float().is_sign_negative() {
        -copy
    } else {
        copy
    }
}
#[track_caller]
fn same_number(actual: crate::predictor_number::N, expected: &crate::predictor_number::N) {
    assert_eq!(actual.as_float().prec(), expected.as_float().prec());
    assert_eq!(actual.as_float().is_nan(), expected.as_float().is_nan());
    assert_eq!(
        actual.as_float().is_sign_negative(),
        expected.as_float().is_sign_negative(),
        "actual={actual:?}, expected={expected:?}"
    );
    if !expected.as_float().is_nan() {
        assert_eq!(&actual, expected);
    }
}

#[test]
fn predictor_owned_operations_preserve_rounding_and_classes() {
    use crate::predictor_number::{N, NATIVE_BITS};
    let (tiny, normal, maximum) = if NATIVE_BITS == 53 {
        ("0x1p-1074", "0x1p-1022", "0x1.fffffffffffffp1023")
    } else {
        ("0x1p-16445", "0x1p-16382", "0x1.fffffffffffffffep16383")
    };
    let mut values = vec![
        n(0.),
        n(-0.),
        n(1.),
        n(-1.),
        n(2.),
        n(-2.),
        n(0.5),
        n(-0.5),
        n(f64::INFINITY),
        n(f64::NEG_INFINITY),
        n(f64::NAN),
        -n(f64::NAN),
    ];
    for text in [
        tiny,
        normal,
        maximum,
        "0x1.0000000000000002p0",
        "0x1.fffffffffffffp-1",
    ] {
        let x = N::parse_number(text).unwrap();
        values.extend([x.clone(), -x]);
    }
    let neighbors = if NATIVE_BITS == 53 {
        [
            "0x1p-1073",
            "0x0.fffffffffffffp-1022",
            "0x1.0000000000001p-1022",
            "0x1p1022",
        ]
    } else {
        [
            "0x1p-16444",
            "0x0.fffffffffffffffep-16382",
            "0x1.0000000000000002p-16382",
            "0x1p16382",
        ]
    };
    for text in neighbors {
        let x = N::parse_number(text).unwrap();
        values.extend([x.clone(), -x]);
    }
    macro_rules! compare_ownership {
        ($a:expr, $b:expr, $op:tt) => {{
            // The borrowed operation is the unchanged fresh-result path.
            let expected = $a $op $b;
            same_number(signed_copy($a) $op $b, &expected);
            same_number($a $op signed_copy($b), &expected);
            same_number(signed_copy($a) $op signed_copy($b), &expected);
        }};
    }
    for a in &values {
        for b in &values {
            compare_ownership!(a, b, +);
            compare_ownership!(a, b, -);
            compare_ownership!(a, b, *);
            compare_ownership!(a, b, /);
            for c in [n(0.), n(-0.), n(1.), n(f64::INFINITY), -n(f64::NAN)] {
                // The pre-reuse source boundary: hardware FMA on macOS,
                // separately rounded multiply and add on the 64-bit profile.
                let expected = if NATIVE_BITS == 53 {
                    n(a.to_f64().mul_add(b.to_f64(), c.to_f64()))
                } else {
                    let product = a * b;
                    &product + &c
                };
                same_number(a.source_mul_add(b, &c), &expected);
                same_number(signed_copy(a).source_mul_add_owned(b, &c), &expected);
            }
        }
        let live = a.as_float();
        let original = signed_copy(a);
        for b in [
            0.,
            -0.,
            1.,
            -1.,
            2.,
            -2.,
            0.5,
            -0.5,
            f64::from_bits(1),
            -f64::from_bits(1),
            f64::from_bits(2),
            -f64::from_bits(2),
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            f64::from_bits(f64::MIN_POSITIVE.to_bits() - 1),
            -f64::from_bits(f64::MIN_POSITIVE.to_bits() - 1),
            f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1),
            -f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1),
            f64::from_bits(1.0f64.to_bits() - 1),
            f64::from_bits(1.0f64.to_bits() + 1),
            f64::MAX,
            -f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            -f64::NAN,
        ] {
            macro_rules! mixed {
                ($op:tt) => {{
                    let expected = a $op &n(b);
                    same_number(a $op b, &expected);
                    same_number(signed_copy(a) $op b, &expected);
                }};
            }
            mixed!(+);
            mixed!(-);
            mixed!(*);
            mixed!(/);
        }
        assert!(std::ptr::eq(live, a.as_float()));
        same_number(signed_copy(a), &original);
    }
    // Detect the actual aliasing trap, rather than assume MPFR clears NaN sign.
    use rug::{Float, float::Round, ops::MulAssignRound};
    let mut negative_zero = Float::with_val(NATIVE_BITS, -0.0);
    negative_zero.mul_assign_round(&Float::with_val(NATIVE_BITS, f64::INFINITY), Round::Nearest);
    assert!(negative_zero.is_nan() && negative_zero.is_sign_negative());
    same_number(n(-0.) * n(f64::INFINITY), &n(f64::NAN));
}

#[test]
fn predictor_owned_subnormal_midpoint_keeps_first_rounding_direction() {
    use crate::predictor_number::{N, NATIVE_BITS};
    use rug::Float;
    let p = NATIVE_BITS;
    let normal_exp = if p == 53 { -1022 } else { -16382 };
    let wide = 2 * p + 4;
    let a = (Float::with_val(wide, 1) + (Float::with_val(wide, 1) >> (p - 1))) << normal_exp;
    let b = (Float::with_val(wide, 1) - (Float::with_val(wide, 1) >> p)) >> p;
    let a = N::from_float(&a);
    let b = N::from_float(&b);
    let tiny = N::parse_number(&format!("0x1p{}", normal_exp - (p as i32 - 1))).unwrap();
    // Exact product is just above half the minimum subnormal; the p-bit
    // intermediate is exactly that midpoint. Dropping direction returns zero.
    same_number(a.clone() * &b, &tiny);
    same_number(&a * b.clone(), &tiny);
    same_number((-a) * b, &(-tiny.clone()));

    // With e=2^-22, (1-e+e^2)(1+e)=1+e^3. Both inputs fit p=53/64,
    // while the exact mixed product lies just above the half-tiny midpoint.
    let a = (Float::with_val(wide, 1) - (Float::with_val(wide, 1) >> 22)
        + (Float::with_val(wide, 1) >> 44))
        << normal_exp;
    let a = N::from_float(&a);
    let b = (1. + 2.0f64.powi(-22)) * 2.0f64.powi(-(p as i32));
    let (midpoint, direction) =
        Float::with_val_round(p, a.as_float() * b, rug::float::Round::Nearest);
    assert_eq!(direction, std::cmp::Ordering::Less);
    assert_eq!(midpoint, Float::with_val(p, 1) << (normal_exp - p as i32));
    same_number(&a * b, &tiny);
    same_number(a.clone() * b, &tiny);
    same_number(&a * -b, &(-tiny.clone()));
    same_number(a * -b, &(-tiny));
}

fn model(start: f64, end: f64) -> ChebyModel {
    let c =
        Cheby2D::from_coefficients(3, 2, vec![n(12.), n(4.), n(0.), n(2.), n(0.), n(0.)]).unwrap();
    ChebyModel {
        psrname: "J0000".into(),
        sitename: "@".into(),
        mjd_start: n(start),
        mjd_end: n(end),
        freq_start: n(1000.),
        freq_end: n(2000.),
        dispersion_constant: n(0.),
        frequency_cheby: c.x_derivative().unwrap(),
        cheby: c,
    }
}
#[test]
fn predictor_polynomial_derivative_and_off_grid() {
    let c = Cheby2D::construct(8, 4, |x, y| {
        Ok(y.iter()
            .flat_map(|y| {
                x.iter()
                    .map(move |x| n(3.) + x * 2. - y * 0.5 + x * x * 0.125)
            })
            .collect())
    })
    .unwrap();
    let d = c.x_derivative().unwrap();
    for x in [-0.77, -0.13, 0.38, 0.92] {
        let v = c.evaluate(&n(x), &n(0.31)).to_f64();
        assert!((v - (3. + 2. * x - 0.155 + 0.125 * x * x)).abs() < 2e-14);
        assert!((d.evaluate(&n(x), &n(0.31)).to_f64() - (2. + 0.25 * x)).abs() < 1e-13);
    }
}
#[test]
fn predictor_selection_ties_endpoints_gaps_keep() {
    let mut c = ChebyModelSet {
        segments: vec![model(0., 2.), model(1., 3.), model(5., 6.)],
    };
    assert_eq!(c.nearest_index(&n(1.5)), Some(0));
    assert_eq!(c.nearest_index(&n(3.)), Some(1));
    assert_eq!(c.nearest_index(&n(4.)), None);
    let copy = c.clone();
    c.insert(&copy);
    assert_eq!(c.nearest_index(&n(1.)), Some(0));
    c.keep(&[n(5.5), n(1.5)]);
    assert_eq!(c.segments.len(), 2);
    assert_eq!(c.segments[0].mjd_start, n(0.));
    assert_eq!(c.segments[1].mjd_start, n(5.));
    c.keep(&[]);
    assert!(T2Predictor::Cheby(c).metadata().is_err());
    // Source selection uses a strict 1e6 sentinel, even for an inclusive range.
    for (start, end, time, expected) in [
        (0., 2e6, 0., None),
        (0., 2e6, 2e6, None),
        (0., 2e6, 1., Some(0)),
        (f64::NAN, 2., 1., None),
        (0., f64::NAN, 1., None),
        (0., f64::INFINITY, 1., None),
        (f64::NEG_INFINITY, 2., 1., None),
        (0., 2., f64::NAN, None),
        (0., 2., f64::INFINITY, None),
        (0., 2., f64::NEG_INFINITY, None),
    ] {
        let set = ChebyModelSet {
            segments: vec![model(start, end)],
        };
        assert_eq!(set.nearest_index(&n(time)), expected);
    }
}
#[test]
fn predictor_plan_short_final_and_gap_status() {
    let p = T2Predictor::Cheby(ChebyModelSet {
        segments: vec![model(0., 2.)],
    });
    let plan = p.plan(&n(0.), &n(1.), &n(30000.), &n(1500.)).unwrap();
    assert_eq!(plan.segments.len(), 3);
    assert_eq!(plan.segments[2].end_mjd, n(1.));
    assert_eq!(p.evaluate(&n(3.), &n(1400.)).unwrap().phase_turn, n(-1.));
    assert!(p.evaluate(&n(3.), &n(1400.)).unwrap().out_of_range);
    assert!(p.plan(&n(1.), &n(0.), &n(1.), &n(1400.)).is_err());
}
#[test]
fn predictor_cheby_io_and_invalid_dimensions() {
    let p = T2Predictor::Cheby(ChebyModelSet {
        segments: vec![model(0., 2.)],
    });
    let mut bytes = vec![];
    p.write_stream(&mut bytes).unwrap();
    let q = T2Predictor::read_stream(Cursor::new(&bytes)).unwrap();
    assert_eq!(
        q.predictor.evaluate(&n(1.2), &n(1450.)).unwrap().phase_turn,
        p.evaluate(&n(1.2), &n(1450.)).unwrap().phase_turn
    );
    for invalid in [
        String::from_utf8(bytes.clone())
            .unwrap()
            .replace("NCOEFF_TIME 3", "NCOEFF_TIME -1"),
        String::from_utf8(bytes.clone())
            .unwrap()
            .replace("NCOEFF_TIME 3\n", ""),
        String::from_utf8(bytes.clone())
            .unwrap()
            .replace("NCOEFF_TIME 3", "NCOEFF_TIME 1"),
        String::from_utf8(bytes)
            .unwrap()
            .replace("ChebyModel END", "COEFFS 1 2\nChebyModel END"),
    ] {
        assert!(T2Predictor::read_stream(Cursor::new(invalid)).is_err());
    }
}
#[test]
fn predictor_zero_and_constant_extent() {
    let c = Cheby2D::construct(3, 4, |x, y| Ok(vec![n(0.); x.len() * y.len()])).unwrap();
    assert!(c.coefficients().iter().all(|v| *v == n(0.)));
    assert!(Cheby2D::new(1, 2).unwrap().x_derivative().is_err());
    assert!(Cheby2D::new(0, 2).is_err());
}
#[test]
fn predictor_t1_prefix_and_frequency_contract() {
    let mut m = T1Polyco::new(vec![n(0.1), n(0.2), n(0.03), n(0.004)]).unwrap();
    m.psrname = "J0000".into();
    m.sitename = "@".into();
    m.date_string = "26-Oct-26".into();
    m.utc_string = "120000.00".into();
    m.mjd_mid = n(60000.);
    m.span = 60;
    m.frequency_psr_0 = n(100.);
    m.frequency_obs = 1400.;
    m.dm = 10.;
    let set = T1PolycoSet {
        segments: vec![m.clone(), m.clone()],
        trailing_error: None,
    };
    let mut bytes = vec![];
    set.write(&mut bytes).unwrap();
    bytes.extend_from_slice(b"bad trailing input\n");
    let read = T1PolycoSet::read(&mut Cursor::new(bytes)).unwrap();
    assert_eq!(read.segments.len(), 2);
    assert!(read.trailing_error.is_some());
    assert_eq!(
        m.frequency(&n(60000.001), &n(400.)),
        m.frequency(&n(60000.001), &n(1400.))
    );
    assert_ne!(
        m.phase(&n(60000.001), &n(400.)),
        m.phase(&n(60000.001), &n(1400.))
    );
    assert_eq!(set.nearest_index(&n(65000.)).unwrap(), 0);
    let mut p = T2Predictor::T1(set);
    assert!(p.keep(&[]).is_err());
    assert!(T2Predictor::default().copy_from(&p).is_err());
}
#[test]
fn predictor_copy_survives_original_drop() {
    let p = T2Predictor::Cheby(ChebyModelSet {
        segments: vec![model(0., 2.)],
    });
    let mut q = T2Predictor::default();
    q.copy_from(&p).unwrap();
    drop(p);
    assert_eq!(q.metadata().unwrap().psrname, "J0000");
    let copy = q.clone();
    q.insert(&copy).unwrap();
    q.keep(&[n(1.)]).unwrap();
    assert_eq!(q.metadata().unwrap().start_mjd, n(0.));
}

#[test]
fn predictor_native_subnormal_single_rounding() {
    use crate::predictor_number::{NATIVE_BITS, PredictorNumber as N};
    use rug::Float;
    let exponent: i32 = if NATIVE_BITS == 53 { 1075 } else { 16446 };
    let halfway = Float::with_val(128, 1) >> exponent;
    let above = halfway.clone() + (Float::with_val(128, 1) >> (exponent + 105));
    let minimum = Float::with_val(128, 1) >> (exponent - 1);
    assert_eq!(N::from_float(&above).as_float(), &minimum);
    assert_eq!(
        above
            .to_string_radix(10, None)
            .parse::<N>()
            .unwrap()
            .as_float(),
        &minimum
    );
    let hex = format!("0x1.000000000000000000000000004p-{exponent}");
    assert_eq!(
        N::parse_prefix(hex.as_bytes()).unwrap().as_float(),
        &minimum
    );
    assert!(N::from_float(&halfway).as_float().is_zero());
    assert_eq!(N::from_float(&(-above)).as_float(), &(-minimum));
    for x in [1.5, f64::from_bits(1), f64::MIN_POSITIVE, -0.] {
        assert_eq!(N::from(x).to_f64().to_bits(), x.to_bits());
    }
    if NATIVE_BITS == 53 {
        let a = n(f64::from_bits(0x0010000000000001));
        let b = n(f64::from_bits(0x3c9fffffffffffff));
        assert_eq!((a * b).to_f64().to_bits(), 1);
        let overflow = Float::with_val(128, f64::MAX) * 2;
        assert!(N::from_float(&overflow).to_f64().is_infinite());
    }
}

#[test]
fn predictor_stream_position_and_failed_read_recovery() {
    let p = T2Predictor::Cheby(ChebyModelSet {
        segments: vec![model(0., 2.)],
    });
    let mut bytes = vec![];
    p.write_stream(&mut bytes).unwrap();
    let length = bytes.len();
    p.write_stream(&mut bytes).unwrap();
    let mut input = Cursor::new(bytes);
    T2Predictor::read_stream(&mut input).unwrap();
    assert_eq!(input.position(), length as u64);
    T2Predictor::read_stream(&mut input).unwrap();
    assert_eq!(input.position(), (2 * length) as u64);
    let mut bad = Cursor::new(b"not a predictor\n".to_vec());
    assert!(T2Predictor::read_stream(&mut bad).is_err());
    assert_eq!(bad.position(), 0);
    bad = input;
    bad.set_position(0);
    assert_eq!(
        T2Predictor::read_stream(&mut bad)
            .unwrap()
            .predictor
            .metadata()
            .unwrap()
            .psrname,
        "J0000"
    );
}

#[test]
fn predictor_compiled_source_testfunc() {
    // cheby2d.c::testFunc/testCheby2D: sinl maps to sinq, then the result
    // narrows to the ordinary-long-double callback grid.
    use crate::predictor_number::PredictorNumber as N;
    let callback = |x: &[N], y: &[N]| -> Result<Vec<N>, String> {
        Ok(y.iter()
            .flat_map(|y| x.iter().map(move |x| crate::cheby2d::test_function(x, y)))
            .collect())
    };
    let c = Cheby2D::construct(30, 30, callback).unwrap();
    assert!(c.test(90, 90, callback).unwrap().mav_turn.to_f64() < 5e-14);
}

#[test]
fn predictor_t1_legacy_d_exponents_and_reader_bounds() {
    let mut m = T1Polyco::new(vec![n(0.1), n(0.2), n(0.3)]).unwrap();
    m.psrname = "J0000".into();
    m.sitename = "@".into();
    m.date_string = "26-Oct-26".into();
    m.utc_string = "120000.00".into();
    m.mjd_mid = n(60000.);
    m.span = 60;
    m.frequency_obs = 1400.;
    let mut bytes = vec![];
    m.write(&mut bytes).unwrap();
    let text = String::from_utf8(bytes).unwrap().replace('e', "D");
    let parsed = T1Polyco::read(&mut Cursor::new(text.as_bytes())).unwrap();
    // t1polyco.c:106 writes 18 significant decimal digits (%25.17Le).
    // They round-trip binary64, but not every x87 long-double coefficient.
    // Check the actual source-format decimals at the reader's native precision.
    let expected: Vec<crate::predictor_number::PredictorNumber> = [
        "1.00000000000000006e-1",
        "2.00000000000000011e-1",
        "2.99999999999999989e-1",
    ]
    .into_iter()
    .map(|s| s.parse().unwrap())
    .collect();
    assert_eq!(parsed.coefficients(), expected);
    for count in [1, 33] {
        let mut lines = text.lines().map(str::to_owned).collect::<Vec<_>>();
        lines[1].replace_range(48..53, &format!("{count:5}"));
        assert!(T1Polyco::read(&mut Cursor::new(lines.join("\n"))).is_err());
    }
}

#[test]
fn predictor_legacy_overwidth_doppler_preserves_next_field() {
    let mut m = T1Polyco::new(vec![n(0.), n(0.), n(0.)]).unwrap();
    m.psrname = "J0000".into();
    m.sitename = "@".into();
    m.date_string = "26-Oct-26".into();
    m.utc_string = "120000.00".into();
    m.mjd_mid = n(60000.);
    m.frequency_obs = 1400.;
    m.span = 60;
    for d in [-9999.99, -12.345, 0., 12.345, 9999.99] {
        m.doppler = d;
        m.log10rms = -16.517;
        let mut bytes = vec![];
        m.write(&mut bytes).unwrap();
        let p = T1Polyco::read(&mut Cursor::new(bytes)).unwrap();
        assert_eq!(p.doppler, d);
        assert_eq!(p.log10rms, m.log10rms);
    }
}
