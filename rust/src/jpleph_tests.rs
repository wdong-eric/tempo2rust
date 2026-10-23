//! I-only observation driver corresponding to validation/jpl/probe.c commands.
//! Uses production Rust functions; control/fault commands are explicit test inputs.
use super::*;
use crate::stdio::OBSERVER;
use serde_json::{Value, json};
use std::{
    fs,
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::MetadataExt,
};

fn number(x: f64) -> Value {
    if x.is_finite() {
        json!({"class":"finite","decimal":x.to_string(),"bits":format!("{:016x}",x.to_bits())})
    } else {
        let c = if x.is_nan() {
            "NaN"
        } else if x.is_sign_negative() {
            "-Infinity"
        } else {
            "+Infinity"
        };
        json!({"class":c,"decimal":c})
    }
}
fn numbers(p: &[f64]) -> Value {
    Value::Array(p.iter().map(|x| number(*x)).collect())
}
fn bytes(p: &[u8]) -> String {
    p.iter().map(|x| format!("{x:02x}")).collect()
}
fn fill(p: &mut [f64]) {
    for (i, x) in p.iter_mut().enumerate() {
        *x = -9000.0 - i as f64;
    }
}
fn snapshot(e: &Ephemeris, controlled: bool, v: &mut Value) {
    v["cache_loc"] = json!(e.curr_cache_loc);
    v["cache_bytes"] = json!(bytes(&e.cache));
    if controlled {
        v["sun_epoch"] = number(e.pvsun_t.unwrap());
        v["sun"] = numbers(e.pvsun().unwrap());
        v["n_posn"] = json!(e.iinfo.n_posn_avail);
        v["n_vel"] = json!(e.iinfo.n_vel_avail);
        v["twot"] = number(e.iinfo.twot);
        v["posn_coeff"] = numbers(&e.iinfo.posn_coeff);
        v["vel_coeff"] = numbers(&e.iinfo.vel_coeff);
    }
}
fn header(e: &Ephemeris, v: &mut Value) {
    v["header_doubles"] = numbers(&[e.ephem_start, e.ephem_end, e.ephem_step, e.au, e.emrat]);
    for (key, n) in [
        ("ncon", e.ncon),
        ("version", e.ephemeris_version),
        ("kernel_size", e.kernel_size),
        ("recsize", e.recsize),
        ("ncoeff", e.ncoeff),
        ("swap", u32::from(e.swap_bytes)),
    ] {
        v[key] = json!(n);
    }
    v["ipt"] = json!(e.ipt);
}

#[test]
#[ignore = "executed in fresh processes by validation/jpl_port/run.py"]
fn probe() {
    OBSERVER.with(|o| *o.borrow_mut() = Default::default());
    init_error(-9);
    let commands = fs::read_to_string(std::env::var("JPL_COMMANDS").unwrap()).unwrap();
    let mut events = vec![json!({"op":"startup","init_error":jpl_init_error_code()})];
    let mut eph: Option<Ephemeris> = None;
    let mut controlled = false;
    for (event, line) in commands.lines().enumerate() {
        let args: Vec<_> = line.split_whitespace().collect();
        let op = args[0];
        let mut v = json!({"event":event,"op":op});
        let result: Result<()> = (|| {
            match op {
                "open" => {
                    let flags: u32 = args[2].parse().unwrap();
                    let mut names = vec![b'?'; 2048 * 6];
                    let mut values = [0.0; 2048];
                    fill(&mut values);
                    let mut values: Vec<u8> = values.iter().flat_map(|x| x.to_ne_bytes()).collect();
                    // Exercise the same-descriptor provenance pass on every H open.
                    let file = File::open(args[1]).and_then(|mut f| {
                        let mut observed = Vec::new();
                        f.read_to_end(&mut observed)?;
                        f.seek(SeekFrom::Start(0))?;
                        Ok(f)
                    });
                    eph = Ephemeris::init(
                        file,
                        if flags & 1 != 0 {
                            Some(&mut names)
                        } else {
                            None
                        },
                        if flags & 2 != 0 {
                            Some(&mut values)
                        } else {
                            None
                        },
                    )?;
                    controlled = false;
                    v["nonnull"] = json!(eph.is_some());
                    v["init_error"] = json!(jpl_init_error_code());
                    if let Some(e) = &eph {
                        header(e, &mut v);
                        if flags & 1 != 0 {
                            v["name_bytes"] = json!(bytes(&names[..e.ncon as usize * 6]));
                        }
                        if flags & 2 != 0 {
                            v["constants"] = numbers(
                                &(0..e.ncon as usize)
                                    .map(|i| double(&values, i * 8, false))
                                    .collect::<Vec<_>>(),
                            );
                        }
                    }
                }
                "control" => {
                    let e = eph.as_mut().unwrap();
                    e.pvsun_t = Some(f64::NAN);
                    let mut sun = [0.0; 9];
                    fill(&mut sun);
                    e.pvsun = Some(sun);
                    e.iinfo.twot = 0.0;
                    e.iinfo.n_posn_avail = 2;
                    e.iinfo.n_vel_avail = 2;
                    e.iinfo.posn_coeff[2..].fill(0.0);
                    e.iinfo.vel_coeff[2..].fill(0.0);
                    controlled = true;
                    snapshot(e, controlled, &mut v);
                }
                "state" | "state_nut4" | "state_wide" => {
                    let e = eph.as_mut().unwrap();
                    let mut pv = [0.0; 96];
                    fill(&mut pv);
                    let mut nut = [0.0; 12];
                    fill(&mut nut);
                    let mut list = [0; 14];
                    let (et, bary) = if op != "state_nut4" {
                        for i in 0..14 {
                            list[i] = args[3 + i].parse().unwrap();
                        }
                        (args[1].parse().unwrap(), args[2].parse().unwrap())
                    } else {
                        list[11] = 2;
                        (e.ephem_start + 1.0, 1)
                    };
                    let n = if op == "state_nut4" {
                        4
                    } else if op == "state_wide" {
                        12
                    } else {
                        6
                    };
                    let status = e.state(
                        et,
                        &list,
                        &mut pv.chunks_mut(6).collect::<Vec<_>>(),
                        &mut nut[..n],
                        bary,
                    )?;
                    v["status"] = json!(status);
                    if op != "state_nut4" {
                        v["pv"] = numbers(&pv);
                        v["nut"] = numbers(&nut[..n]);
                        snapshot(e, controlled, &mut v);
                    }
                }
                "pleph" | "pleph_status" => {
                    let e = eph.as_mut().unwrap();
                    let mut out = [0.0; 6];
                    fill(&mut out);
                    let s = e.pleph(
                        args[1].parse().unwrap(),
                        args[2].parse().unwrap(),
                        args[3].parse().unwrap(),
                        &mut out,
                        args[4].parse().unwrap(),
                    )?;
                    v["status"] = json!(s);
                    if op == "pleph" && s == 0 {
                        v["out"] = numbers(&out);
                    }
                    if controlled {
                        snapshot(e, controlled, &mut v);
                    }
                }
                "constant" => {
                    let mut name = [b'?'; 7];
                    let value = eph
                        .as_mut()
                        .unwrap()
                        .get_constant(args[1].parse().unwrap(), &mut name)?;
                    v["name_bytes"] = json!(bytes(&name));
                    v["value"] = number(value);
                }
                "getter" => {
                    let e = eph.as_ref().unwrap();
                    let offset = args[2].parse().unwrap();
                    v["value"] = if args[1] == "d" {
                        number(e.get_double(offset)?)
                    } else {
                        json!(e.get_long(offset)?)
                    };
                }
                "fault" => OBSERVER.with(|o| {
                    o.borrow_mut().fail[match args[1] {
                        "seek" => 0,
                        "read" => 1,
                        "alloc" => 2,
                        _ => panic!(),
                    }] = args[2].parse().unwrap()
                }),
                "truncate" => fs::OpenOptions::new()
                    .write(true)
                    .open(args[1])
                    .unwrap()
                    .set_len(args[2].parse().unwrap())
                    .unwrap(),
                "replace" => fs::rename(args[2], args[1]).unwrap(),
                "overwrite" => {
                    let data = fs::read(args[2]).unwrap();
                    let mut f = fs::OpenOptions::new().write(true).open(args[1]).unwrap();
                    f.write_all(&data).unwrap();
                    f.set_len(data.len() as u64).unwrap();
                }
                "close" => {
                    eph = None;
                    controlled = false;
                    v["init_error"] = json!(jpl_init_error_code());
                }
                "interp" => {
                    let mut iinfo = Interpolation::new();
                    let coef: Vec<u8> = (0..432)
                        .flat_map(|i| {
                            ((if i % 7 == 0 { -1.0 } else { 1.0 }) * (i + 1) as f64 / 16.0)
                                .to_ne_bytes()
                        })
                        .collect();
                    let mut out = [0.0; 9];
                    fill(&mut out);
                    iinfo.interp(
                        &coef,
                        [args[1].parse().unwrap(), args[2].parse().unwrap()],
                        args[3].parse().unwrap(),
                        args[4].parse().unwrap(),
                        args[5].parse().unwrap(),
                        args[6].parse().unwrap(),
                        &mut out,
                    )?;
                    v["out"] = numbers(&out);
                }
                "helpers" => {
                    let mut b: Vec<u8> = (0..16).collect();
                    swap_32_bit_val((&mut b[..4]).try_into().unwrap());
                    swap_64_bit_val(&mut b)?;
                    v["swap_bytes"] = json!(bytes(&b));
                    v["dimensions"] = json!((0..16).map(dimension).collect::<Vec<_>>());
                }
                _ => panic!("unknown {op}"),
            }
            if let Some(e) = &eph
                && ["open", "truncate", "replace", "overwrite"].contains(&op)
            {
                let handle = e.stream.metadata_file.metadata().unwrap();
                let path = fs::metadata(args[1]).unwrap();
                v["path_matches_open_inode"] =
                    json!(handle.dev() == path.dev() && handle.ino() == path.ino());
                v["open_file_size"] = json!(handle.len());
                v["path_file_size"] = json!(path.len());
            }
            Ok(())
        })();
        if let Err(e) = result {
            v["boundary"] = json!(e.0);
            if op == "open" {
                v["init_error"] = json!(jpl_init_error_code());
            }
        }
        OBSERVER.with(|o| {
            let o = o.borrow();
            v["io_calls"] = json!(o.calls);
            v["last_fread_size_requested_returned"] = json!(o.last_read);
        });
        let stopped = v.get("boundary").is_some();
        events.push(v);
        if stopped {
            break;
        }
    }
    fs::write(
        std::env::var("JPL_OUTPUT").unwrap(),
        serde_json::to_vec_pretty(&events).unwrap(),
    )
    .unwrap();
}

fn fixture() -> Vec<u8> {
    // Reuses the former resource test's contiguous DE405 layout (382 doubles).
    let n = 382;
    let mut b = vec![0; n * 8 * 4];
    b[..84].fill(b' ');
    b[26..35].copy_from_slice(b"405/LE405");
    for (at, x) in [
        (2652, 2450000.5_f64),
        (2660, 2450064.5),
        (2668, 32.0),
        (2680, 149597870.7),
        (2688, 81.30056),
    ] {
        b[at..at + 8].copy_from_slice(&x.to_ne_bytes());
    }
    b[2676..2680].copy_from_slice(&2_u32.to_ne_bytes());
    let mut offset = 3_u32;
    for i in 0..13 {
        let at = if i == 12 { 2844 } else { 2696 + 12 * i };
        for (j, x) in [offset, 10, 1_u32].iter().enumerate() {
            b[at + 4 * j..at + 4 * j + 4].copy_from_slice(&x.to_ne_bytes());
        }
        offset += if i == 11 { 20 } else { 30 };
    }
    for record in 2..4 {
        for i in 2..n {
            let at = (record * n + i) * 8;
            b[at..at + 8].copy_from_slice(&(0.01 * record as f64).to_ne_bytes());
        }
    }
    b
}

#[test]
fn owned_boundaries_lifetimes_and_direct_acceleration() {
    // One test holds this synthetic fixture and checks errors without relying on
    // process-global init status while unrelated timing tests may initialize.
    let dir = std::env::temp_dir().join(format!("jpl-owned-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let path = dir.join("ephemeris");
    let image = fixture();
    fs::write(&path, &image).unwrap();
    let mut opened = File::open(&path).unwrap();
    let mut observed = Vec::new();
    opened.read_to_end(&mut observed).unwrap();
    use sha2::{Digest, Sha256};
    assert_eq!(Sha256::digest(&observed), Sha256::digest(&image));
    opened.seek(SeekFrom::Start(0)).unwrap();
    let replacement = dir.join("replacement");
    let mut other_image = image.clone();
    other_image[2680..2688].copy_from_slice(&123456789.0_f64.to_ne_bytes());
    fs::write(&replacement, &other_image).unwrap();
    fs::rename(&replacement, &path).unwrap();
    let mut e = Ephemeris::init(Ok(opened), None, None).unwrap().unwrap();
    assert_eq!(e.au, 149597870.7); // Parsing consumes the hashed descriptor, not path B.
    assert_eq!(
        Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .unwrap()
            .au,
        123456789.0
    );
    assert!(e.pvsun().is_none());
    let mut list = [0; 14];
    list[0] = 3;
    list[11] = 3;
    let mut pv = [[0.0; 9]; 10];
    let mut nut = [0.0; 9];
    assert_eq!(
        e.state(
            2450008.5,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut,
            1
        )
        .unwrap(),
        0
    );
    assert!(e.pvsun().is_some());
    assert!(pv[0][6..].iter().any(|x| *x != 0.0));
    assert!(nut[6..].iter().any(|x| *x != 0.0));
    let good = pv;
    assert_eq!(
        e.state(
            f64::NAN,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut,
            1
        ),
        Err(Boundary("record_conversion"))
    );
    assert_eq!(pv, good);
    list[0] = 0;
    list[11] = 2;
    assert_eq!(
        e.state(
            2450008.5,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut[..4],
            1
        ),
        Err(Boundary("output_storage"))
    );
    list[11] = 0;
    list[0] = -1;
    assert_eq!(
        e.state(
            2450008.5,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut,
            1
        ),
        Err(Boundary("output_storage"))
    );
    assert_eq!(e.get_double(1), Err(Boundary("double_selector")));
    assert_eq!(e.get_long(45), Err(Boundary("integer_selector")));
    let mut out = [1.0; 6];
    assert_eq!(e.pleph(2450008.5, 3, 12, &mut out, 1).unwrap(), 0);
    let baseline = out;
    // Independent state, drop/reopen, and calls after boundaries retain values.
    fs::write(&path, &image).unwrap();
    let mut independent = Ephemeris::init(File::open(&path), None, None)
        .unwrap()
        .unwrap();
    independent.pleph(2450038.5, 3, 12, &mut out, 1).unwrap();
    e.pleph(2450008.5, 3, 12, &mut out, 1).unwrap();
    assert_eq!(baseline, out);
    drop(independent);
    let mut reopened = Ephemeris::init(File::open(&path), None, None)
        .unwrap()
        .unwrap();
    reopened.pleph(2450008.5, 3, 12, &mut out, 1).unwrap();
    assert_eq!(baseline, out);
    for (at, bytes, boundary) in [
        (2668, 1e-300_f64.to_ne_bytes().to_vec(), "record_conversion"),
        (
            2696,
            10000000_u32.to_ne_bytes().to_vec(),
            "coefficient_storage",
        ),
        (2700, 18_u32.to_ne_bytes().to_vec(), "chebyshev_count"),
    ] {
        let mut damaged = image.clone();
        damaged[at..at + bytes.len()].copy_from_slice(&bytes);
        fs::write(&path, damaged).unwrap();
        let mut bad = Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .unwrap();
        let mut list = [0; 14];
        list[0] = 2;
        assert_eq!(
            bad.state(
                2450008.5,
                &list,
                &mut pv.each_mut().map(|p| p.as_mut_slice()),
                &mut nut,
                1
            ),
            Err(Boundary(boundary))
        );
    }
    let mut bad = image.clone();
    bad[26..84].fill(b'9');
    fs::write(&path, bad).unwrap();
    assert!(matches!(
        Ephemeris::init(File::open(&path), None, None),
        Err(Boundary("title_conversion"))
    ));
    // A short consumed header returns null; a malformed independent layout is
    // diagnosed when consumed, without broad finite/AU/record-grid restrictions.
    fs::write(&path, &image[..100]).unwrap();
    assert!(
        Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .is_none()
    );
    let mut info = Interpolation::new();
    assert_eq!(
        info.interp(&[], [0.5, 32.0], 0, 3, 1, 2, &mut nut),
        Err(Boundary("derivative_count"))
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn c_locale_title_whitespace_and_signed_versions() {
    let path = std::env::temp_dir().join(format!("jpl-title-{}", std::process::id()));
    for (prefix, version) in [
        (&b"430/"[..], 430),
        (&b"\x0b430/"[..], 430),
        (&b" \t\n\r\x0b\x0c430/"[..], 430),
        (&b"+430/"[..], 430),
        (&b"-430/"[..], (-430_i32) as u32),
        (&b"x"[..], 0),
        (&b"+ "[..], 0),
    ] {
        let mut image = fixture();
        image[26..84].fill(b' ');
        image[26..26 + prefix.len()].copy_from_slice(prefix);
        fs::write(&path, image).unwrap();
        let e = Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .unwrap();
        assert_eq!(e.ephemeris_version, version);
    }
    fs::remove_file(path).unwrap();
}

#[test]
#[ignore = "run alone by the JPL verifier so another test cannot reuse the inspected fd"]
fn title_scan_boundaries_close_stream_and_allow_reinitialization() {
    use std::os::fd::AsRawFd;
    let path = std::env::temp_dir().join(format!("jpl-title-boundary-{}", std::process::id()));
    for token in [
        vec![b' '; 58],
        [vec![b' '; 57], vec![b'+']].concat(),
        [vec![b' '; 57], vec![b'-']].concat(),
        [vec![b' '; 57], vec![b'4']].concat(),
        vec![b'9'; 58],
    ] {
        let mut image = fixture();
        image[26..84].copy_from_slice(&token);
        fs::write(&path, &image).unwrap();
        let file = File::open(&path).unwrap();
        let fd = file.as_raw_fd();
        assert!(matches!(
            Ephemeris::init(Ok(file), None, None),
            Err(Boundary("title_conversion"))
        ));
        assert!(fs::metadata(format!("/dev/fd/{fd}")).is_err());
        fs::write(&path, fixture()).unwrap();
        let mut good = Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .unwrap();
        let mut out = [0.0; 6];
        assert_eq!(good.pleph(2450008.5, 3, 12, &mut out, 1).unwrap(), 0);
        assert!(out.iter().all(|x| x.is_finite()));
    }
    fs::remove_file(path).unwrap();
}

#[test]
fn legacy_extended_header_rejects_unwritten_comparison() {
    let path = std::env::temp_dir().join(format!("jpl-legacy-header-{}", std::process::id()));
    for (version, ncon) in [(b"405", 2_u32), (b"430", 400_u32)] {
        let mut valid = fixture();
        valid[26..29].copy_from_slice(version);
        valid[2676..2680].copy_from_slice(&ncon.to_ne_bytes());
        let mut invalid = valid.clone();
        for (j, word) in [u32::MAX, 0, 0].iter().enumerate() {
            invalid[2844 + j * 4..2848 + j * 4].copy_from_slice(&word.to_ne_bytes());
        }
        fs::write(&path, invalid).unwrap();
        assert!(matches!(
            Ephemeris::init(File::open(&path), None, None),
            Err(Boundary("unwritten_extended_header"))
        ));
        fs::write(&path, valid).unwrap();
        let e = Ephemeris::init(File::open(&path), None, None)
            .unwrap()
            .unwrap();
        assert_eq!(e.ncoeff, 382);
        assert_eq!(e.ipt[13..], [[0; 3]; 2]);
    }
    fs::remove_file(path).unwrap();
}

#[test]
fn optional_demands_follow_interp_flags_and_actual_storage() {
    let path = std::env::temp_dir().join(format!("jpl-demand-{}", std::process::id()));
    fs::write(&path, fixture()).unwrap();
    let mut e = Ephemeris::init(File::open(&path), None, None)
        .unwrap()
        .unwrap();
    let mut pv = [[0.0; 6]; 10];
    let mut list = [0; 14];
    let mut results = Vec::new();
    for demand in [1, 2, 3, 4, -1, i32::MIN, i32::MAX] {
        list[11] = demand;
        let mut nut = [0.0; 9];
        fill(&mut nut);
        assert_eq!(
            e.state(
                2450008.5,
                &list,
                &mut pv.each_mut().map(|p| p.as_mut_slice()),
                &mut nut,
                1
            )
            .unwrap(),
            0
        );
        results.push(nut);
    }
    assert_eq!(results[0], results[4]);
    assert_eq!(results[0], results[5]);
    assert_eq!(results[1], results[3]);
    assert_eq!(results[1], results[6]);
    assert_ne!(results[1][6..], results[2][6..]);
    list[11] = 4;
    let mut nut = [0.0; 4];
    assert_eq!(
        e.state(
            2450008.5,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut,
            1
        ),
        Err(Boundary("output_storage"))
    );
    list[11] = 0;
    list[0] = 4;
    let mut nut = [0.0; 9];
    assert_eq!(
        e.state(
            2450008.5,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            &mut nut,
            1
        ),
        Err(Boundary("output_storage"))
    );
    fs::remove_file(path).unwrap();
}
