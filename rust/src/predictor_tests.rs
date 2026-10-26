//! AF analytic contracts, parser failures and owned lifecycle regressions.
use crate::{cheby2d::*, predictor_number::n, t1polyco::*, tempo2pred::*};
use std::io::Cursor;
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
    use rug::Float;
    let callback = |x: &[N], y: &[N]| -> Result<Vec<N>, String> {
        Ok(y.iter()
            .flat_map(|y| {
                x.iter().map(move |x| {
                    N::from_float(
                        &(Float::with_val(113, (x * 0.5).as_float())
                            + Float::with_val(113, x.as_float()).sin() * 0.1f64
                            + Float::with_val(113, (y * 0.7).as_float())
                            + Float::with_val(113, (y * 0.1 * y).as_float())),
                    )
                })
            })
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
    assert_eq!(parsed.coefficients(), m.coefficients());
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
