//! Synthetic trust-boundary/recovery checks; numerical fixture checks are separate.
use crate::{
    clkcorr::ClockCorrections, eop::Eop, ifteph::IfteEphemeris, read_ephemeris::JplEphemeris,
    runtime::MeteorologyFunction, tabulated_function::TabulatedFunction,
    tropo::get_meteorological_value,
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "tempo2-resources-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn table_steps_endpoints_and_invalid_data() {
    let p = Path::new("synthetic.clk");
    let table = TabulatedFunction::parse(p, "# A B\n1 0\n2 0\n2 1\n3 1\n").unwrap();
    let mut warnings = Vec::new();
    assert_eq!(table.get_value(1.5, &mut warnings).unwrap(), 0.0);
    assert_eq!(table.get_value(2.0, &mut warnings).unwrap(), 1.0);
    assert_eq!(table.get_value(2.5, &mut warnings).unwrap(), 1.0);
    assert_eq!(table.get_value(0.0, &mut warnings).unwrap(), 0.0);
    assert_eq!(table.get_value(4.0, &mut warnings).unwrap(), 1.0);
    assert_eq!(warnings.len(), 2);
    for bad in [
        "",
        "A B\n1 1\n2 2\n",
        "# A B\n1 0\n",
        "# A B\n2 0\n1 0\n",
        "# A B\n1 NaN\n2 0\n",
        "# A B\n1 0\nbad row\n2 0\n",
        "# A B\n1 0\n1 1\n",
    ] {
        assert!(TabulatedFunction::parse(p, bad).is_err(), "{bad}");
    }
    let duplicate_end = TabulatedFunction::parse(p, "# A B\n1 0\n2 1\n2 2\n").unwrap();
    assert!(duplicate_end.get_value(2.0, &mut warnings).is_err());
    assert_eq!(table.get_value(2.5, &mut warnings).unwrap(), 1.0);
}

#[test]
fn meteorology_valid_zero_missing_and_outside_coverage() {
    let table = |text| MeteorologyFunction {
        site: "PKS".into(),
        table: TabulatedFunction::parse(Path::new("pks.synthetic"), text).unwrap(),
    };
    let pressure = vec![table("# PKS\n53000 100\n53001 102\n")];
    let wet = vec![table("# PKS\n53000 1e-9\n53001 3e-9\n")];
    let mut warnings = Vec::new();
    let p = get_meteorological_value(&pressure, "pks", 53000.5, true, &mut warnings).unwrap();
    let w = get_meteorological_value(&wet, "pks", 53000.5, false, &mut warnings).unwrap();
    assert_eq!(p.effective_value.0, 101.0);
    assert!((w.effective_value.0.to_f64() - 2e-9).abs() < 1e-24);
    assert_eq!(p.reason, "table interpolation");
    assert!(warnings.is_empty());
    let zero = [table("# PKS\n53000 0\n53001 0\n")];
    for (tables, mjd, reason) in [
        (&zero[..], 53000.5, "zero table value"),
        (&[][..], 53000.5, "no weather table for site"),
        (&pressure[..], 50000.0, "epoch outside weather coverage"),
    ] {
        let p = get_meteorological_value(tables, "pks", mjd, true, &mut warnings).unwrap();
        let w = get_meteorological_value(tables, "pks", mjd, false, &mut warnings).unwrap();
        assert_eq!(p.effective_value.0, 101.325);
        assert_eq!(w.effective_value.0, 0);
        assert_eq!(p.reason, reason);
        assert_eq!(w.reason, reason);
    }
    assert_eq!(
        warnings.iter().filter(|w| w.starts_with("TROP1")).count(),
        3
    );
    assert_eq!(
        warnings.iter().filter(|w| w.starts_with("TROP2")).count(),
        3
    );
}

#[test]
fn eop_upper_boundary_recovers_and_leap_is_not_interpolated() {
    let p = Path::new("synthetic.eop");
    let header = "FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2F12.6)\n";
    let eop = Eop::load_eop(
        p,
        &format!(
            "{header}2000 1 1 50000 1 2 -0.4\n2000 1 2 50001 3 4 0.59\n2000 1 3 50002 5 6 0.58\n"
        ),
    )
    .unwrap();
    let mut warnings = Vec::new();
    let before = eop.get_eop(50000.5, &mut warnings).unwrap();
    assert!((before[2] + 0.405).abs() < 1e-15);
    assert!((before[3] + 0.01 / 86400.0).abs() < 1e-18);
    for date in [50002.0, 51000.0] {
        assert!(eop.get_eop(date, &mut warnings).is_err());
    }
    assert_eq!(before, eop.get_eop(50000.5, &mut warnings).unwrap());
    assert_eq!(eop.get_eop(40000.0, &mut warnings).unwrap()[2], -0.4);
    assert_eq!(warnings.len(), 1);
    assert!(Eop::load_eop(p, "bad").is_err());
    assert!(Eop::load_eop(p, &format!("{header}2000 1 1 50000 NaN 0 0")).is_err());
}

#[test]
fn clock_routes_coverage_ties_fallback_and_recovery() {
    let dir = Directory::new();
    let root = &dir.0;
    fs::create_dir(root.join("clock")).unwrap();
    for (name, table) in [
        ("a.clk", "# UTC(PKS) A 1\n0 1e-6\n10 1e-6\n"),
        ("b.clk", "# A TT(TAI) 1\n0 32\n100 32\n"),
        ("c.clk", "# UTC(PKS) B 1\n10 2e-6\n100 2e-6\n"),
        ("d.clk", "# B TT(TAI) 1\n0 32\n100 32\n"),
        ("e.clk", "# UTC TT(TAI) 1\n0 32\n1000 32\n"),
    ] {
        fs::write(root.join("clock").join(name), table).unwrap();
    }
    let mut resources = Vec::new();
    let mut warnings = Vec::new();
    let mut clocks = ClockCorrections::load(root, &mut resources).unwrap();
    let a = clocks
        .get_clock_corrections(
            root,
            &mut resources,
            "UTC(PKS)",
            "TT(TAI)",
            5.0,
            &mut warnings,
        )
        .unwrap();
    assert_eq!(a[0].corrects_to, "A");
    let b = clocks
        .get_clock_corrections(
            root,
            &mut resources,
            "UTC(PKS)",
            "TT(TAI)",
            15.0,
            &mut warnings,
        )
        .unwrap();
    assert_eq!(b[0].corrects_to, "B");
    // Cached source sequence wins at its inclusive boundary, matching clkcorr.C.
    assert_eq!(
        clocks
            .get_clock_corrections(
                root,
                &mut resources,
                "UTC(PKS)",
                "TT(TAI)",
                10.0,
                &mut warnings
            )
            .unwrap()[0]
            .corrects_to,
        "A"
    );
    let fallback = clocks
        .get_clock_corrections(
            root,
            &mut resources,
            "UTC(PKS)",
            "TT(TAI)",
            150.0,
            &mut warnings,
        )
        .unwrap();
    assert_eq!(fallback.len(), 1);
    assert!(warnings.iter().any(|w| w.contains("assuming")));
    assert!(
        clocks
            .get_clock_corrections(
                root,
                &mut resources,
                "UTC(PKS)",
                "TT(TAI)",
                2000.0,
                &mut warnings
            )
            .is_err()
    );
    assert_eq!(
        clocks
            .get_clock_corrections(
                root,
                &mut resources,
                "UTC(PKS)",
                "TT(TAI)",
                5.0,
                &mut warnings
            )
            .unwrap()[0]
            .corrects_to,
        "A"
    );
    assert!(
        clocks
            .get_clock_corrections(root, &mut resources, "TT(TAI)", "A", 5.0, &mut warnings)
            .is_err()
    );
    fs::write(root.join("clock/bad.clk"), "# A C -1\n0 0\n10 0\n").unwrap();
    assert!(ClockCorrections::load(root, &mut Vec::new()).is_err());
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

fn jpl_image() -> (Vec<u8>, usize) {
    let n = 382;
    let mut bytes = vec![0; n * 8 * 4];
    bytes[..84].fill(b' ');
    bytes[26..35].copy_from_slice(b"405/LE405");
    for (at, x) in [
        (2652, 2450000.5),
        (2660, 2450064.5),
        (2668, 32.0),
        (2680, 149597870.7),
        (2688, 81.30056),
    ] {
        put_f(&mut bytes, at, x, false);
    }
    put_i(&mut bytes, 2676, 2, false);
    put_i(&mut bytes, 2840, 405, false);
    let mut offset = 3;
    for i in 0..13 {
        let at = if i == 12 { 2844 } else { 2696 + 12 * i };
        put_i(&mut bytes, at, offset, false);
        put_i(&mut bytes, at + 4, 10, false);
        put_i(&mut bytes, at + 8, 1, false);
        offset += if i == 11 { 20 } else { 30 };
    }
    for record in 2..4 {
        for i in 2..n {
            put_f(
                &mut bytes,
                (record * n + i) * 8,
                0.01 * (record as f64),
                false,
            );
        }
    }
    (bytes, n * 8)
}

#[test]
fn jpl_record_grid_bounds_and_endpoints_recover() {
    let dir = Directory::new();
    let path = dir.0.join("DE405");
    let (bytes, _) = jpl_image();
    fs::write(&path, &bytes).unwrap();
    let mut original = JplEphemeris::open(&path, &bytes).unwrap();
    let before = original.interpolate_state(2450000.5, 3, 12).unwrap();
    drop(original);
    // The last case has an integral, representable record count but its native
    // u32 byte offset overflows; it must fail before jpl_init_ephemeris.
    for step in [1e-300, 1e-320, 31.0, 64.0 / (1 << 21) as f64] {
        let mut malformed = bytes.clone();
        put_f(&mut malformed, 2668, step, false);
        fs::write(&path, &malformed).unwrap();
        let error = JplEphemeris::open(&path, &malformed)
            .err()
            .expect("impossible native record grid accepted");
        assert_eq!(error.path, path);
        assert_eq!(error.field, "JPL");
        assert!(error.message.contains("record grid"));
        fs::write(&path, &bytes).unwrap();
        let mut restored = JplEphemeris::open(&path, &bytes).unwrap();
        assert_eq!(
            before,
            restored.interpolate_state(2450000.5, 3, 12).unwrap()
        );
        assert!(restored.interpolate_state(2450032.5, 3, 12).is_ok());
        assert!(restored.interpolate_state(2450064.5, 3, 12).is_ok());
        for epoch in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(
                restored
                    .interpolate_state(epoch, 3, 12)
                    .unwrap_err()
                    .message
                    .contains("nonfinite")
            );
        }
        assert_eq!(
            before,
            restored.interpolate_state(2450000.5, 3, 12).unwrap()
        );
    }
}

#[test]
fn native_read_failure_recovers_without_stale_cached_record() {
    let dir = Directory::new();
    let path = dir.0.join("DE405");
    let (mut bytes, record) = jpl_image();
    // Second data record is partial: native fread returns -2 after overwriting
    // part of its buffer. Repeated failed calls must never use that buffer.
    bytes.truncate(record * 3 + 128);
    fs::write(&path, &bytes).unwrap();
    let mut eph = JplEphemeris::open(&path, &bytes).unwrap();
    let before = eph.interpolate_state(2450008.5, 3, 12).unwrap();
    for _ in 0..2 {
        let error = eph.interpolate_state(2450040.5, 3, 12).unwrap_err();
        assert!(error.to_string().contains("native error -2"));
    }
    assert_eq!(before, eph.interpolate_state(2450008.5, 3, 12).unwrap());
    assert!(
        eph.interpolate_state(2450100.5, 3, 12)
            .unwrap_err()
            .to_string()
            .contains("native error -1")
    );
    assert_eq!(before, eph.interpolate_state(2450008.5, 3, 12).unwrap());
    let mut other = JplEphemeris::open(&path, &bytes).unwrap();
    assert_eq!(before, other.interpolate_state(2450008.5, 3, 12).unwrap());
    let mut bad = bytes.clone();
    put_i(&mut bad, 2700, 99, false);
    assert!(JplEphemeris::open(&path, &bad).is_err());
    let mut bad = bytes.clone();
    bad[26..84].fill(b'9');
    assert!(JplEphemeris::open(&path, &bad).is_err());
    assert!(JplEphemeris::open(&dir.0.join("missing"), &bytes).is_err());
    assert!(JplEphemeris::open(&path, &bytes[..100]).is_err());
}
