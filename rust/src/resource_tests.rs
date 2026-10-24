//! Synthetic trust-boundary/recovery checks; numerical fixture checks are separate.
use crate::{
    eop::Eop,
    ifteph::IfteEphemeris,
    tabulated_function::TabulatedFunction,
    tropo::{Meteorology, MeteorologyFunction},
};
use std::path::Path;

fn table(path: &Path, text: &str) -> TabulatedFunction {
    TabulatedFunction::from_bytes(
        path,
        text.as_bytes(),
        &mut crate::clock_diagnostics::Diagnostics::default(),
    )
    .unwrap()
}

#[test]
fn table_steps_endpoints_and_source_rows() {
    let p = Path::new("synthetic.clk");
    let t = table(p, "# A B\n1 0\n2 0\n2 1\n3 1\n");
    let mut diagnostics = crate::clock_diagnostics::Diagnostics::default();
    for (x, y) in [(1.5, 0.0), (2.0, 1.0), (2.5, 1.0), (0.0, 0.0), (4.0, 1.0)] {
        assert_eq!(t.get_value(x, &mut diagnostics).unwrap(), y);
    }
    assert_eq!(diagnostics.warnings, 2);
    assert_eq!(table(p, "# A B\n1 0\nbad row\n2 0\n").samples.len(), 2);
    assert_eq!(table(p, "# A B\n2 0\n1 0\n").samples.len(), 2);
    assert!(
        table(p, "# A B\n1 0\n2 1\n2 2\n")
            .get_value(2.0, &mut diagnostics)
            .unwrap()
            .is_nan()
    );
    let singleton = table(p, "# A B\n1 0\n");
    assert_eq!(singleton.get_value(0.0, &mut diagnostics).unwrap(), 0.0);
    assert_eq!(
        singleton
            .get_value(1.0, &mut diagnostics)
            .unwrap_err()
            .field,
        "table_bracket_storage"
    );
}

#[test]
fn meteorology_valid_zero_missing_and_outside_coverage() {
    let table = |text| MeteorologyFunction {
        site: b"PKS\0".to_vec(),
        table: table(Path::new("pks.synthetic"), text),
    };
    let pressure = vec![table("# PKS\n53000 100\n53001 102\n")];
    let wet = vec![table("# PKS\n53000 1e-9\n53001 3e-9\n")];
    let mut diagnostics = crate::clock_diagnostics::Diagnostics::default();
    let mut met = Meteorology {
        initialized: true,
        pressure,
        wet,
    };
    let p = met
        .pressure(None, b"pks", 53000.5, -1, &mut diagnostics)
        .unwrap();
    let w = met
        .wet(None, b"pks", 53000.5, -1, &mut diagnostics)
        .unwrap();
    assert_eq!(p.value, 101.0);
    assert!((w.value - 2e-9).abs() < 1e-24);
    assert_eq!(p.reason, "table interpolation");
    assert!(diagnostics.events.is_empty());
    for (tables, mjd, reason) in [
        (
            vec![table("# PKS\n53000 0\n53001 0\n")],
            53000.5,
            "zero table value",
        ),
        (vec![], 53000.5, "no weather table for site"),
        (
            vec![table("# PKS\n53000 100\n53001 102\n")],
            50000.0,
            "epoch outside weather coverage",
        ),
    ] {
        met.pressure = tables.clone();
        met.wet = tables;
        let p = met
            .pressure(None, b"pks", mjd, -1, &mut diagnostics)
            .unwrap();
        let w = met.wet(None, b"pks", mjd, -1, &mut diagnostics).unwrap();
        assert_eq!(p.value, 101.325);
        assert_eq!(w.value, 0.0);
        assert_eq!(p.reason, reason);
        assert_eq!(w.reason, reason);
    }
    assert_eq!(diagnostics.events.len(), 6);
}

#[test]
fn eop_upper_boundary_recovers_and_leap_is_not_interpolated() {
    let mut diagnostics = crate::clock_diagnostics::Diagnostics::default();
    let eop = Eop {
        initialized: true,
        resource: None,
        samples: vec![
            [50000.0, 1.0, 2.0, -0.4],
            [50001.0, 3.0, 4.0, 0.59],
            [50002.0, 5.0, 6.0, 0.58],
        ],
    };
    let mut before = [0.0; 4];
    eop.get_eop(50000.5, &mut before, [0, 1, 2, 3], 0, &mut diagnostics)
        .unwrap();
    assert!((before[2] + 0.405).abs() < 1e-15);
    assert!((before[3] + 0.01 / 86400.0).abs() < 1e-18);
    for date in [50002.0, 51000.0] {
        let mut output = [0.0; 4];
        assert!(
            eop.get_eop(date, &mut output, [0, 1, 2, 3], 2, &mut diagnostics)
                .is_err()
        );
        assert_eq!(output, [5.0, 6.0, 0.58, 0.0]);
    }
    let mut after = [0.0; 4];
    eop.get_eop(50000.5, &mut after, [0, 1, 2, 3], 0, &mut diagnostics)
        .unwrap();
    assert_eq!(before, after);
    eop.get_eop(40000.0, &mut after, [0, 1, 2, 3], 0, &mut diagnostics)
        .unwrap();
    assert_eq!(after[2], -0.4);
    assert_eq!(diagnostics.warnings, 1);
}

fn put_f(bytes: &mut [u8], at: usize, x: f64, big: bool) {
    bytes[at..at + 8].copy_from_slice(&if big {
        x.to_be_bytes()
    } else {
        x.to_le_bytes()
    });
}
fn put_i(bytes: &mut [u8], at: usize, x: u32, big: bool) {
    bytes[at..at + 4].copy_from_slice(&if big {
        x.to_be_bytes()
    } else {
        x.to_le_bytes()
    });
}

fn ifte_image(big: bool) -> Vec<u8> {
    let n = 42;
    let mut bytes = vec![0; n * 8 * 3];
    for (at, x) in [(264, 2450000.5), (272, 2450032.5), (280, 32.0)] {
        put_f(&mut bytes, at, x, big);
    }
    put_i(&mut bytes, 288, 2, big);
    for (i, x) in [3, 10, 1, 13, 10, 1].into_iter().enumerate() {
        put_i(&mut bytes, 292 + 4 * i, x, big);
    }
    put_f(&mut bytes, n * 8, 405.0, big);
    put_f(&mut bytes, n * 8 + 8, crate::ifteph::IFTE_LC, big);
    // A linear Chebyshev series: delta=1+0.5*tc, derivative=1/32 per day.
    put_f(&mut bytes, n * 16 + 2 * 8, 1.0, big);
    put_f(&mut bytes, n * 16 + 3 * 8, 0.5, big);
    for i in 0..3 {
        put_f(&mut bytes, n * 16 + (12 + i * 10) * 8, (i + 1) as f64, big);
        put_f(&mut bytes, n * 16 + (13 + i * 10) * 8, 0.5, big);
    }
    bytes
}

#[test]
fn ifte_endianness_derivative_boundaries_and_recovery() {
    for big in [false, true] {
        let bytes = ifte_image(big);
        let path = Path::new("synthetic.ifte");
        let ifte = IfteEphemeris::init(path, &bytes).unwrap();
        assert_eq!(
            ifte.get_values(2450000.0, 8.5, false).unwrap(),
            vec![0.75, 0.03125]
        );
        assert_eq!(
            ifte.get_values(2450000.0, 8.5, true).unwrap(),
            vec![0.75, 1.75, 2.75, 0.03125, 0.03125, 0.03125]
        );
        assert_eq!(
            ifte.get_values(2450032.0, 0.5, false).unwrap(),
            vec![1.5, 0.03125]
        );
        for jd in [2449999.0, 2450033.0] {
            assert!(ifte.get_values(jd, 0.5, false).is_err());
        }
        assert_eq!(
            ifte.get_values(2450000.0, 8.5, false).unwrap(),
            vec![0.75, 0.03125]
        );
        assert!(IfteEphemeris::init(path, &bytes[..315]).is_err());
        assert!(IfteEphemeris::init(path, &bytes[..bytes.len() - 1]).is_err());
        let mut bad = bytes.clone();
        put_i(&mut bad, 296, 19, big);
        assert!(IfteEphemeris::init(path, &bad).is_err());
        let mut bad = bytes.clone();
        put_f(&mut bad, 42 * 16 + 16, f64::NAN, big);
        assert!(
            IfteEphemeris::init(path, &bad)
                .unwrap()
                .get_values(2450000.0, 8.5, false)
                .is_err()
        );
    }
}

#[test]
fn ifte_record_grid_rejects_impossible_steps_and_recovers() {
    let path = Path::new("synthetic.ifte");
    for big in [false, true] {
        let bytes = ifte_image(big);
        for step in [1e-300, 1e-320, 16.0, 31.0, 64.0] {
            let mut malformed = bytes.clone();
            put_f(&mut malformed, 280, step, big);
            let error = IfteEphemeris::init(path, &malformed)
                .err()
                .expect("impossible record grid accepted");
            assert_eq!(error.path, path);
            assert_eq!(error.field, "IF99");
            assert!(error.message.contains("record grid"));
            let restored = IfteEphemeris::init(path, &bytes).unwrap();
            assert_eq!(
                restored.get_values(2450032.0, 0.5, false).unwrap(),
                vec![1.5, 0.03125]
            );
        }
    }
}
