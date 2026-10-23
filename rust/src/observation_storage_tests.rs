//! Assignment R tests for tempo2.h storage, tempo2Util.C assignment and callers.
#[path = "observation_probe_fields.rs"]
mod fields;
use crate::{
    PRECISION_BITS, SourceFile,
    clkcorr::{self, ClockSession},
    observation::{read_string, write_string},
    tempo2::SourceObservation,
    toa2utc::{ClockPulsar, toa2utc},
};
use fields::{seeded, state};
use rug::Float;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};

fn edges(o: &mut SourceObservation, which: i32) {
    o.sat = Some(Float::with_val(128, -0.0));
    o.origsat = Some(Float::with_val(128, f64::INFINITY));
    o.sat_day = Some(Float::with_val(128, f64::NEG_INFINITY));
    o.sat_sec = Some(Float::with_val(128, f64::NAN));
    o.addedNoise = Some(-0.0);
    o.TNRedSignal = Some(f64::INFINITY);
    o.TNRedErr = Some(f64::NEG_INFINITY);
    o.TNDMSignal = Some(f64::NAN);
    o.clockCorr = Some(i32::MIN);
    o.delayCorr = Some(i32::MAX);
    o.deleted = Some(-1);
    o.nclock_correction = Some(-7);
    o.nFlags = Some(i32::MAX);
    o.obsNjump = Some(i32::MIN);
    o.obsNfdjump = Some(i32::MAX);
    o.jump[39] = Some(i32::MIN);
    o.fdjump[39] = Some(i32::MAX);
    o.pulseN = Some(if which == 1 { i64::MIN } else { i64::MAX });
    o.correctionsTT[29].correction = Some(-0.0);
}
fn assignments() -> Vec<Value> {
    (0..3).map(|case| {
        let mut src = seeded(10);
        let mut dst = seeded(70);
        if case != 0 {edges(&mut src, case);}
        let sb = state(&src);
        let db = state(&dst);
        dst.clone_from(&src);
        let da = state(&dst);
        assert_eq!(sb, da);
        let sa = state(&src);
        dst.fname[500] = Some(b'!');
        dst.sat = Some(Float::with_val(128, 2));
        assert_eq!(sb, state(&src));
        json!({"case":case,"source_before":sb,"destination_before":db,"source_after":sa,"destination_after":da,"independent":true})
    }).collect()
}
fn m_seeded(base: i32, i: usize) -> SourceObservation {
    let mut o = seeded(0); // M declares memset-zeroed complete caller storage.
    o.sat = Some(Float::with_val(128, base + i as i32) + 0.125);
    o.residual = Some(Float::with_val(128, base + i as i32) + 0.25);
    o.freq = Some((base + i as i32) as f64 + 0.5);
    o.pulseN = Some(base as i64 + i as i64);
    write_string(&mut o.fname, format!("obs-{base}-{i}").as_bytes(), "fname").unwrap();
    write_string(&mut o.telID, format!("tel-{base}").as_bytes(), "telID").unwrap();
    o.nFlags = Some(1);
    write_string(&mut o.flagID[0], b"-x", "flagID[0]").unwrap();
    write_string(&mut o.flagVal[0], base.to_string().as_bytes(), "flagVal[0]").unwrap();
    o
}
fn m_assignments() -> Vec<Value> {
    [2,2,0,3].into_iter().enumerate().map(|(case,count)| {
        let src:Vec<_>=(0..3).map(|i|m_seeded(10,i)).collect();
        let mut dst:Vec<_>=(0..3).map(|i|m_seeded(70,i)).collect();
        let sb:Vec<_>=src.iter().map(state).collect();
        let db:Vec<_>=dst.iter().map(state).collect();
        // Observation assignment projection only; not an implementation of copyPSR.
        dst[..count].clone_from_slice(&src[..count]);
        json!({"id":format!("copypsr_{case}"),"source_before":sb,"source_after":src.iter().map(state).collect::<Vec<_>>(),"destination_before":db,"destination_after":dst.iter().map(state).collect::<Vec<_>>()})
    }).collect()
}

fn boundaries() -> Vec<Value> {
    let mut out = Vec::new();
    let mut o = SourceObservation::default();
    o.fname[3] = Some(b'z');
    let before = state(&o);
    let error = read_string(&o.fname, "fname").unwrap_err();
    assert_eq!(error.field, "observation_unwritten_fname[0]");
    out.push(json!({"case":"isolated_byte","before":before,"after":state(&o),"error":error.field}));
    o.fname[..3].copy_from_slice(&[Some(b'a'), Some(0), None]);
    assert_eq!(read_string(&o.fname, "fname").unwrap(), b"a");
    let mut cloned = o.clone();
    cloned.fname[3] = Some(b'x');
    assert_eq!(o.fname[3], Some(b'z'));
    assert_eq!(cloned.fname[2], None);
    out.push(json!({"case":"through_nul_and_holes","before":state(&o),"after":state(&cloned)}));
    let mut short = [None; 3];
    let error = write_string(&mut short, b"abcd", "fname").unwrap_err();
    assert_eq!(short, [Some(b'a'), Some(b'b'), Some(b'c')]);
    assert_eq!(error.field, "observation_capacity_fname[3]");
    out.push(json!({"case":"write_capacity","after":short,"error":error.field}));
    let error = read_string(&short, "fname").unwrap_err();
    assert_eq!(error.field, "observation_missing_terminator_fname");
    out.push(json!({"case":"missing_nul","after":short,"error":error.field}));

    // Equal explicit clock names return before SAT/site; count reset is retained.
    let mut o = SourceObservation::default();
    let mut session = ClockSession::default();
    session
        .run(|s| s.get_clock_corrections(&mut o, Some(b"A"), Some(b" a "), 0))
        .unwrap();
    assert_eq!(o.nclock_correction, Some(0));
    assert!(o.sat.is_none());
    assert_eq!(
        session
            .get_correction(&o, Some(b"A"), Some(b"A"), 0)
            .unwrap(),
        0.0
    );
    out.push(json!({"case":"identity_without_sat_site","after":state(&o)}));
    // Signed negative counts do not consume correction backing.
    o.nclock_correction = Some(-7);
    assert_eq!(clkcorr::get_correction_tt(&o).unwrap(), 0.0);
    out.push(json!({"case":"negative_count","after":state(&o)}));
    o.nclock_correction = Some(1);
    let e = clkcorr::get_correction_tt(&o).unwrap_err();
    assert_eq!(e.field, "observation_unwritten_correctionsTT[0].correction");
    out.push(json!({"case":"unwritten_active_correction","after":state(&o),"error":e.field}));
    o.nclock_correction = Some(31);
    for c in &mut o.correctionsTT {
        c.correction = Some(0.0);
    }
    let e = clkcorr::get_correction_tt(&o).unwrap_err();
    assert_eq!(e.field, "clock_correction_storage");
    out.push(json!({"case":"correction_capacity","after":state(&o),"error":e.field}));
    // With no pulsar jumps, obsNjump is irrelevant even when unwritten.
    let p = ClockPulsar {
        nobs: 1,
        ..Default::default()
    };
    let mut obs = [vec![SourceObservation {
        clockCorr: Some(2),
        ..Default::default()
    }]];
    toa2utc(&mut session, &[p], &mut obs, 1).unwrap();
    out.push(json!({"case":"disabled_clock_no_jumps","after":state(&obs[0][0])}));
    // strncpy writes scalar and byte prefix before an unwritten source byte.
    for mode in [
        "hole",
        "no_nul",
        "early_nul",
        "jump_capacity",
        "jump_unwritten",
        "negative_jumps",
    ] {
        let mut session = ClockSession::default();
        let mut p = ClockPulsar {
            nobs: 1,
            njumps: 1,
            ..Default::default()
        };
        p.jump_sat[0] = 1;
        p.jump_value[1] = 2.5;
        p.jump_string[1] = b"x".to_vec();
        let mut o = SourceObservation {
            clockCorr: Some(0),
            obsNjump: Some(1),
            nclock_correction: Some(0),
            ..Default::default()
        };
        o.jump[0] = Some(1);
        o.correctionsTT[0].correction = Some(9.5);
        o.correctionsTT[0].corrects_to[0] = Some(b'a');
        o.correctionsTT[1].corrects_to[31] = Some(222);
        match mode {
            "no_nul" => o.correctionsTT[0].corrects_to[..31].fill(Some(b'x')),
            "early_nul" => o.correctionsTT[0].corrects_to[1] = Some(0),
            "jump_capacity" => {
                o.obsNjump = Some(41);
                o.jump.fill(Some(0));
            }
            "jump_unwritten" => o.jump[0] = None,
            "negative_jumps" => o.obsNjump = Some(-7),
            _ => {}
        }
        let before = state(&o);
        let mut obs = [vec![o]];
        let result = session.run(|s| toa2utc(s, &[p], &mut obs, 1));
        let o = &obs[0][0];
        let error = result.err().map(|e| {
            let again = session.run(|_| Ok(())).unwrap_err();
            assert_eq!(again.field, e.field);
            e.field
        });
        match mode {
            "hole" => {
                assert_eq!(o.nclock_correction, Some(1));
                assert_eq!(o.correctionsTT[1].correction, Some(9.5));
                assert_eq!(o.correctionsTT[1].corrects_to[..2], [Some(b'a'), None]);
                assert_eq!(
                    error.as_deref(),
                    Some("observation_unwritten_correctionsTT[0].corrects_to[1]")
                );
            }
            "no_nul" => {
                assert!(error.is_none());
                assert_eq!(o.correctionsTT[1].corrects_to[..31], [Some(b'x'); 31]);
            }
            "early_nul" => {
                assert!(error.is_none());
                assert_eq!(o.correctionsTT[1].corrects_to[1..31], [Some(0); 30]);
            }
            "jump_capacity" => assert_eq!(error.as_deref(), Some("clock_jump_storage")),
            "jump_unwritten" => assert_eq!(error.as_deref(), Some("observation_unwritten_jump[0]")),
            "negative_jumps" => {
                assert!(error.is_none());
                assert_eq!(state(o), before);
            }
            _ => unreachable!(),
        }
        assert_eq!(o.correctionsTT[1].corrects_to[31], Some(222));
        out.push(json!({"case":mode,"before":before,"after":state(o),"error":error,"terminal":session.fault.is_some()}));
    }
    out
}
#[test]
fn partial_bytes_and_operation_local_clock_boundaries() {
    boundaries();
}
#[test]
fn whole_record_clone_preserves_classes_precision_and_independence() {
    assignments();
}
#[test]
fn frontend_capacity_is_bytes_and_only_supplied_fields_are_written() {
    for (name, ok) in [
        ("a".repeat(500), true),
        ("a".repeat(501), false),
        ("é".repeat(250), true),
        (format!("{}a", "é".repeat(250)), false),
    ] {
        let tim = SourceFile {
            path: "boundary.tim".into(),
            sha256: String::new(),
            text: format!("FORMAT 1\n{name} 1400 58000 1 @\n"),
        };
        let result = crate::read_timfile::read_timfile(&tim);
        if ok {
            let (obs, meta) = result.unwrap();
            assert_eq!(
                read_string(&obs[0].fname, "fname").unwrap(),
                name.as_bytes()
            );
            assert_eq!(meta[0].name, name);
            let s = state(&obs[0]);
            for f in s.as_array().unwrap() {
                if !["sat", "freq", "toaErr", "fname", "telID"]
                    .contains(&f["name"].as_str().unwrap())
                {
                    assert_eq!(
                        f,
                        state(&SourceObservation::default())
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|x| x["name"] == f["name"])
                            .unwrap()
                    );
                }
            }
            assert_eq!(obs[0].sat.as_ref().unwrap().prec(), PRECISION_BITS);
        } else {
            let e = result.unwrap_err();
            assert_eq!(e.line, Some(2));
            assert_eq!(e.field, "fname");
            assert!(e.message.contains("observation_capacity_fname[501]"));
        }
    }
}
#[test]
fn canonical_fields_drive_evaluation_fit_and_detached_json() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting/inputs");
    let mut input = crate::load(&root.join("all.par"), &root.join("bary.tim")).unwrap();
    let before = crate::evaluate_barycentric(&input).unwrap();
    let saved = serde_json::to_value(&input).unwrap();
    let mut snapshot = input.observations();
    snapshot[0].sat_mjd = "9".parse().unwrap();
    snapshot[0].frequency_mhz = 77.0;
    snapshot[0].original.site = "pks".into();
    assert_eq!(saved, serde_json::to_value(&input).unwrap());
    *input.observations[1].sat.as_mut().unwrap() += Float::with_val(128, 1) >> 20;
    input.observations[0].freq = Some(999.25);
    input.observations[0].toaErr = Some(12.5);
    let after = crate::evaluate_barycentric(&input).unwrap();
    assert_ne!(
        before.observations[1].residual_s,
        after.observations[1].residual_s
    );
    assert_eq!(
        after.observations[1].sat_mjd.0,
        input.observations[1].sat.as_ref().unwrap().clone()
    );
    let fit = crate::fit_barycentric(&input, &Default::default()).unwrap();
    assert_eq!(fit.cycles[0].sigma_s[0].0.to_f64(), 12.5 * 1e-6);
    let current = serde_json::to_value(&input).unwrap();
    assert_eq!(current["observations"][0]["frequency_mhz"], 999.25);
    assert_eq!(
        current["observations"][0]["original"],
        saved["observations"][0]["original"]
    );
    assert_eq!(
        current.as_object().unwrap().keys().collect::<Vec<_>>(),
        saved.as_object().unwrap().keys().collect::<Vec<_>>()
    );
    assert!(input.observations[0].bat.is_none());
    assert!(input.observations[0].residual.is_none());
    input.observation_metadata[0].site = "pks".into();
    assert!(crate::evaluate_barycentric(&input).is_ok());
    write_string(&mut input.observations[0].telID, b"pks", "telID").unwrap();
    assert!(crate::evaluate_barycentric(&input).is_err());
    assert_eq!(input.observation_site(0).unwrap(), b"pks");
}
#[test]
fn canonical_frequency_sat_and_site_drive_terrestrial_evaluation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixtures = root.join("tests/terrestrial/fixtures");
    let mut input = crate::load(&fixtures.join("test1.par"), &fixtures.join("test2.tim")).unwrap();
    let mut runtime = crate::RuntimeContext::new(&root.join("../tempo2/T2runtime")).unwrap();
    let before = crate::evaluate_terrestrial(&input, &mut runtime).unwrap();
    input.observations[0].freq = Some(input.observations[0].freq.unwrap() * 2.0);
    *input.observations[0].sat.as_mut().unwrap() += Float::with_val(128, 1) >> 20;
    input.observation_metadata[0].site = "@".into();
    let after = crate::evaluate_terrestrial(&input, &mut runtime).unwrap();
    assert!(
        after.corrections[0].frequency_ssb_hz.as_float()
            > before.corrections[0].frequency_ssb_hz.as_float()
    );
    assert!(
        after.corrections[0].dispersion_s.as_float()
            < before.corrections[0].dispersion_s.as_float()
    );
    assert_eq!(
        after.timing.observations[0].sat_mjd.0,
        input.observations[0].sat.as_ref().unwrap().clone()
    );
    assert!(input.observations[0].nclock_correction.is_none());
    assert!(input.observations[0].correctionsTT[0].correction.is_none());
    write_string(&mut input.observations[0].telID, b"@", "telID").unwrap();
    assert!(crate::evaluate_terrestrial(&input, &mut runtime).is_err());
}
#[test]
#[ignore = "R fresh-process exact source comparison probe"]
fn probe() {
    let input = fs::read(env::var("OBSERVATION_INPUT").unwrap()).unwrap();
    let plan: Value = serde_json::from_slice(&input).unwrap();
    assert_eq!(
        plan,
        json!({"schema":1,"cases":[0,1,2],"m_cases":["copypsr_0","copypsr_1","copypsr_2","copypsr_3"]})
    );
    let result = json!({"token":env::var("OBSERVATION_TOKEN").unwrap(),"input_sha256":format!("{:x}",Sha256::digest(&input)),"cases":assignments(),"m_cases":m_assignments(),"unwritten":state(&SourceObservation::default()),"boundaries":boundaries()});
    fs::write(
        env::var("OBSERVATION_OUTPUT").unwrap(),
        serde_json::to_vec(&result).unwrap(),
    )
    .unwrap();
}
