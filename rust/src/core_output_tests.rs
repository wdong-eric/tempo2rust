use crate::{
    core_output::*,
    source_format::{self, Argument},
    tempo2::Label::*,
};
use std::io::Write;
fn ld(x: f64) -> rug::Float {
    rug::Float::with_val(crate::PRECISION_BITS, x)
}

fn formatted() -> Vec<u8> {
    let mut out = vec![];
    for f in [
        "%Lg",
        "%+.0Lf",
        "% 020.8Lf",
        "%-22.10Le",
        "%#.6Lg",
        "%.36Lg",
        "%+.3Le",
    ] {
        for value in [
            "0",
            "-0",
            "1.25",
            "-0.125",
            "65536",
            "0x1p-20",
            "1.9999847412109375",
            "inf",
            "-inf",
            "nan",
        ] {
            let mut b = vec![];
            let n = source_format::ld_sprintf(
                &mut b,
                f,
                &[Argument::from(
                    source_format::parse_longdouble(value.as_bytes()).unwrap(),
                )],
            )
            .unwrap();
            write!(out, "{f}\t{value}\t{n}\t").unwrap();
            out.extend(b);
            out.push(b'\n');
        }
    }
    let mut b = vec![];
    let n = source_format::ld_sprintf(
        &mut b,
        "%+07d %#08x %.0u %-6.3s %% %c %.17g",
        &[
            (-23i32).into(),
            42u32.into(),
            0u32.into(),
            "abcdef".into(),
            (b'X' as i32).into(),
            (-0f64).into(),
        ],
    )
    .unwrap();
    write!(out, "mixed\t{n}\t").unwrap();
    out.extend(b);
    out.push(b'\n');
    let mut b = vec![];
    let n = source_format::ld_sprintf(
        &mut b,
        "%hhd %hd %#.0x %#.0o %+u %08.4x",
        &[
            257i32.into(),
            65537i32.into(),
            0u32.into(),
            0u32.into(),
            42u32.into(),
            42u32.into(),
        ],
    )
    .unwrap();
    write!(out, "integers\t{n}\t").unwrap();
    out.extend(b);
    out.push(b'\n');
    out
}
fn helpers() -> Vec<u8> {
    let mut context = crate::pulsar_lifecycle::PulsarContext::unwritten(2);
    context.initialise(2, 3, 2).unwrap();
    let mut out = vec![];
    for k in 0..2 {
        context.pulsars[k].nobs = Some(3);
        let (p, rows) = context.pulsar_observations_mut(k).unwrap();
        p.nobs = Some(3);
        p.fitMode = Some(1);
        p.param.slots[param_f as usize].cells[0].val = ld(8.);
        for (l, v) in [
            (param_start, 50000. + 10. * k as f64),
            (param_finish, 50002. + 10. * k as f64),
        ] {
            let c = &mut p.param.slots[l as usize].cells[0];
            c.param_set = 1;
            c.fit_flag = 1;
            c.val = ld(v);
        }
        for (i, o) in rows.iter_mut().enumerate() {
            o.deleted = Some(i32::from(i == 2));
            o.sat = Some(ld(50000. + 10. * k as f64 + i as f64));
            o.bat = o.sat.clone();
            o.toaErr = Some(2.);
            o.prefitResidual = Some(ld((i as f64 - 1.) * 0.0009765625));
            o.residual = Some(ld(i as f64 * 0.000244140625));
            o.residualtn = Some(ld(i as f64 * 0.0001220703125));
        }
        let weight = calc_rms(&mut context, k).unwrap();
        let p = &context.pulsars[k];
        out.extend(
            source_format::format(
                "rms %d %.17g %.17g %.17g %.17g %d\n",
                &[
                    (k as i32).into(),
                    weight.into(),
                    p.rmsPre.unwrap().into(),
                    p.rmsPost.unwrap().into(),
                    p.rmstn.unwrap().into(),
                    p.param.slots[param_tres as usize].cells[0].param_set.into(),
                ],
            )
            .unwrap(),
        );
    }
    for si in [1., 0.5, 0.] {
        let m = m2(&ld(0.125), &ld(si), &ld(1.5), &mut out).unwrap();
        out.extend(source_format::format("m2 %.17g %.17g\n", &[si.into(), m.into()]).unwrap());
    }
    let p = &mut context.pulsars[0];
    for (l, v) in [
        (param_glf0, 0.0009765625),
        (param_glf1, 0.),
        (param_glf0d, 0.),
        (param_gltd, 0.),
    ] {
        p.param.slots[l as usize].cells[0].val = ld(v);
    }
    for phase in [0., 0.125, -0.25] {
        let result = dglep(p, 0, phase).unwrap();
        out.extend(
            source_format::format("glitch %.17g %.17g\n", &[phase.into(), result.into()]).unwrap(),
        );
    }
    assert!(dglep(p, -1, 0.125).is_err());
    assert!(dglep(p, i32::MAX, 0.125).is_err());
    out
}
fn logging(terminal: bool) -> (Vec<u8>, Vec<u8>) {
    let mut d = crate::Diagnostics::default();
    d.log_outfile_is_terminal = terminal;
    for i in 0..32 {
        d.log(b'w', "probe", 1, format!("warn {i}").as_bytes());
    }
    for i in 0..31 {
        d.log(b'e', "probe", 1, format!("err {i}").as_bytes());
    }
    d.events.clear();
    let mut out = vec![];
    let mut err = vec![];
    let count = d.check();
    d.stdout(format!("count={count}\n").as_bytes());
    d.log(b'w', "probe", 1, b"after reset");
    d.events.pop();
    let count = d.check();
    d.stdout(format!("count={count}\n").as_bytes());
    d.check_log(
        true,
        b"\x1b[31mconfigured stdout controls this stderr colour\x1b[0m\n",
    );
    let mut b = vec![b'z'; 499];
    b[350..355].copy_from_slice(b"\x1b[31m");
    d.check_log(false, &b);
    for (e, b) in d.events {
        if e { err.extend(b) } else { out.extend(b) }
    }
    (out, err)
}
fn snapshot(directory: &std::path::Path) {
    let mut c = crate::pulsar_lifecycle::PulsarContext::unwritten(1);
    c.initialise(1, 3, 2).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let runtime = root.join("validation/reference/build/source/T2runtime");
    crate::source_par::read_parfiles(
        &mut c,
        &[crate::bats::InputPair {
            par: root.join("validation/core_output/inputs/snapshot.par"),
            tim: std::path::PathBuf::new(),
        }],
        &runtime,
        &mut vec![],
    )
    .unwrap();
    crate::preprocess::pre_process(&mut c, &[b"ah-snapshot".to_vec()], &runtime, &mut vec![])
        .unwrap();
    c.pulsars[0].nobs = Some(3);
    let (p, obs) = c.pulsar_observations_mut(0).unwrap();
    p.nFit = Some(3);
    p.nParam = Some(2);
    p.fitNfree = Some(1);
    p.fitChisq = Some(2.);
    p.offset = Some(0.125);
    p.offset_e = Some(0.0625);
    p.globalNfit = Some(2);
    p.nGlobal = Some(1);
    p.rajStrPost.clone_from(&p.rajStrPre);
    p.decjStrPost.clone_from(&p.decjStrPre);
    p.fitinfo.nParams = Some(2);
    p.fitinfo.output.totalNfit = Some(2);
    for i in 0..2 {
        let label = if i == 0 { param_ZERO } else { param_f };
        p.fitinfo.paramIndex[i] = Some(label as i32);
        p.fitinfo.paramCounters[i] = Some(0);
        p.fitinfo.output.indexParam[i] = Some(if i == 0 { param_f } else { param_ZERO } as i32);
        p.fitinfo.output.indexCounter[i] = Some(0);
        p.fitinfo.output.indexPsr[i] = Some(if i == 0 { -1 } else { 0 });
    }
    for (i, o) in obs.iter_mut().enumerate() {
        o.deleted = Some(0);
        o.sat = Some(ld(50000. + i as f64));
        o.bat = o.sat.clone();
        o.toaErr = Some(2.);
        o.freq = Some(1408.);
        o.prefitResidual = Some(ld((i as f64 - 1.) * 0.001953125));
        o.residual = Some(ld((i as f64 - 1.) * 0.0009765625));
        o.residualtn = Some(ld((i as f64 - 1.) * 0.00048828125));
        crate::observation::write_string(&mut o.telID, b"@", "site").unwrap();
    }
    let owner = p.covar.flatten().unwrap();
    let matrix = c.matrix_mut(owner).unwrap();
    let stride = matrix.get_blas_cols();
    for i in 0..2 {
        for j in 0..2 {
            matrix.cells_mut()[i * stride + j] = 4.;
        }
    }
    let mut out = vec![];
    let mut err = vec![];
    let mut input = std::io::empty();
    let options = OutputOptions {
        new_par: Some("snapshot.par".into()),
        residuals: true,
        ..Default::default()
    };
    text_output(
        &mut c,
        &options,
        &mut OutputIo {
            stdout: &mut out,
            stderr: &mut err,
            stdin: &mut input,
            directory,
        },
    )
    .unwrap();
    std::fs::write(directory.join("stdout"), &out).unwrap();
    std::fs::write(directory.join("stderr"), &err).unwrap();
    let p = &c.pulsars[0];
    let args = [param_tzrmjd, param_tzrfrq, param_tres, param_trestn]
        .map(|l| Argument::from(p.param.slots[l as usize].cells[0].val.clone()));
    std::fs::write(
        directory.join("state.tsv"),
        source_format::format(
            "TZRMJD %.36Lg\nTZRFRQ %.36Lg\nTRES %.36Lg\nTRESTN %.36Lg\n",
            &args,
        )
        .unwrap(),
    )
    .unwrap();
    let mut again = vec![];
    text_output(
        &mut c,
        &options,
        &mut OutputIo {
            stdout: &mut again,
            stderr: &mut err,
            stdin: &mut input,
            directory,
        },
    )
    .unwrap();
    assert_eq!(out, again, "same-context repeated report");

    // A real failing destination must not poison the canonical owner or logger.
    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("AH injected stream failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    assert!(
        text_output(
            &mut c,
            &options,
            &mut OutputIo {
                stdout: &mut Broken,
                stderr: &mut err,
                stdin: &mut input,
                directory,
            }
        )
        .is_err()
    );
    again.clear();
    text_output(
        &mut c,
        &options,
        &mut OutputIo {
            stdout: &mut again,
            stderr: &mut err,
            stdin: &mut input,
            directory,
        },
    )
    .unwrap();
    assert_eq!(out, again, "same-context recovery after failed stream");

    // Reload the actual serialized values, independently of unrounded fit state.
    let mut reloaded = crate::pulsar_lifecycle::PulsarContext::unwritten(1);
    reloaded.initialise(1, 0, 2).unwrap();
    crate::source_par::read_parfiles(
        &mut reloaded,
        &[crate::bats::InputPair {
            par: directory.join("snapshot.par"),
            tim: std::path::PathBuf::new(),
        }],
        &runtime,
        &mut vec![],
    )
    .unwrap();
    let printed = std::fs::read_to_string(directory.join("snapshot.par")).unwrap();
    for (name, label, index) in [
        ("F0", param_f, 0),
        ("F1", param_f, 1),
        ("DM", param_dm, 0),
        ("PB", param_pb, 0),
        ("MTOT", param_mtot, 0),
        ("GLEP_1", param_glep, 0),
    ] {
        let line = printed
            .lines()
            .find(|s| s.split_whitespace().next() == Some(name))
            .unwrap();
        let value =
            source_format::parse_longdouble(line.split_whitespace().nth(1).unwrap().as_bytes())
                .unwrap();
        let cell = &reloaded.pulsars[0].param.slots[label as usize].cells[index];
        assert_eq!(cell.val, value, "printed {name}");
        assert_eq!(cell.param_set, 1);
    }
    assert_eq!(
        reloaded.pulsars[0].param.slots[param_f as usize].cells[0].fit_flag,
        1
    );

    // Defined empty-filename prompt reads the supplied stream at its source point.
    let mut answer = std::io::Cursor::new(b"prompt.par\n");
    text_output(
        &mut c,
        &OutputOptions {
            new_par: Some("".into()),
            ..Default::default()
        },
        &mut OutputIo {
            stdout: &mut vec![],
            stderr: &mut err,
            stdin: &mut answer,
            directory,
        },
    )
    .unwrap();
    assert_eq!(
        std::fs::read(directory.join("prompt.par")).unwrap(),
        std::fs::read(directory.join("snapshot.par")).unwrap()
    );
    std::fs::remove_file(directory.join("prompt.par")).unwrap();
}
#[test]
fn typed_formatter_rejects_mismatched_arguments_and_preserves_zeros() {
    assert!(source_format::format("%Lg", &[1f64.into()]).is_err());
    assert_eq!(
        source_format::format("%+.0Lf", &[ld(-0.).into()]).unwrap(),
        b"-0"
    );
    assert_eq!(
        source_format::format("%#.3Lg", &[ld(0.).into()]).unwrap(),
        b"0.00"
    );
}
#[test]
fn logger_counts_and_reset_are_independent_of_storage_capacity() {
    let (out, err) = logging(false);
    assert!(String::from_utf8_lossy(&out).contains("plus 1 errors not shown"));
    assert!(String::from_utf8_lossy(&out).contains("Warning #1: after reset"));
    assert!(!err.contains(&27));
    assert!(logging(true).1.contains(&27));
}
#[test]
fn rms_uses_current_pulsar_cutoffs_and_writes_tres() {
    let out = String::from_utf8(helpers()).unwrap();
    let mut rows = out.lines();
    let a = rows.next().unwrap();
    let b = rows.next().unwrap();
    assert_eq!(&a[6..], &b[6..]);
}
#[test]
#[ignore = "writes raw evidence to AH_PROBE_DIR for the native comparison runner"]
fn write_oracle_evidence() {
    let root = std::path::PathBuf::from(std::env::var_os("AH_PROBE_DIR").unwrap());
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("format.stdout"), formatted()).unwrap();
    std::fs::write(root.join("helpers.stdout"), helpers()).unwrap();
    for terminal in [false, true] {
        let (out, err) = logging(terminal);
        let name = if terminal { "terminal" } else { "log" };
        std::fs::write(root.join(format!("{name}.stdout")), out).unwrap();
        std::fs::write(root.join(format!("{name}.stderr")), err).unwrap();
    }
    let directory = root.join("snapshot");
    std::fs::create_dir_all(&directory).unwrap();
    snapshot(&directory);
}

#[test]
#[ignore = "writes AH_UT1_DIR before/after diagnostic-state evidence"]
fn write_ut1_evidence() {
    let dest = std::path::PathBuf::from(std::env::var_os("AH_UT1_DIR").unwrap());
    std::fs::create_dir_all(&dest).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let runtime = root.join("validation/reference/build/source/T2runtime");
    for clock in [0, 1, 2] {
        let mut input = crate::bats::load_pairs(
            &[crate::bats::InputPair {
                par: root.join("validation/fitting/inputs/pks_all.par"),
                tim: root.join("validation/fitting/inputs/pks.tim"),
            }],
            &runtime,
        )
        .unwrap();
        crate::preprocess::pre_process(
            &mut input.context,
            &[b"ut1-witness".to_vec()],
            &runtime,
            &mut input.inputs,
        )
        .unwrap();
        let (_, rows) = input.context.pulsar_observations_mut(0).unwrap();
        for row in rows {
            row.clockCorr = Some(clock);
        }
        let mut r = crate::RuntimeContext::from_root(&runtime).unwrap();
        crate::global::form_bats_all(&mut input.context, &mut r).unwrap();
        crate::timing::form_residuals(&mut input.context, true).unwrap();
        let rows = output_rows(&input.context, 0).unwrap();
        let mut values = vec![];
        let mut state = String::new();
        for (i, row) in rows.iter().enumerate() {
            if let Some(ut1) = &row.correctionUT1 {
                values.extend(
                    source_format::format("%d %.17g\n", &[(i as i32).into(), ut1.to_f64().into()])
                        .unwrap(),
                );
            } else {
                writeln!(values, "{i} unwritten").unwrap();
            }
            let mut row = row.clone();
            row.correctionUT1 = None;
            // Complete canonical row, including BAT/residual, clocks and geometry.
            state.push_str(&format!("{row:#?}\n"));
        }
        std::fs::write(dest.join(format!("clock-{clock}.ut1")), values).unwrap();
        std::fs::write(dest.join(format!("clock-{clock}.state")), state).unwrap();
    }
}
