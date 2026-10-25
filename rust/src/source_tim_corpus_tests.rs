//! Complete N caller projection: every initialized observation cell and retained tail.
#![allow(non_snake_case)]
use crate::{
    TimingError,
    corpus_wire::{Typed, decode, encode, hex, observation_bytes, scalar, unhex},
    observation::{SourceObservation, write_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    source_text::extended,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};
type Map = BTreeMap<String, Value>;
fn fixed(p: &mut SourcePulsar, root: &str, s: &mut Map, load: bool) {
    macro_rules! fields {($($f:ident),*)=>{$(scalar(&mut p.$f,&format!("{root}{}",stringify!($f)),s,load);)*}}
    fields!(
        nobs,
        sorted,
        fixedFormat,
        jboFormat,
        noWarnings,
        fitMode,
        nPhaseJump,
        nT2efac,
        nT2equad,
        T2globalEfac,
        nToffset
    );
    macro_rules! fields {($($f:ident),*)=>{$(crate::corpus_wire::field(&mut p.$f,&format!("{root}{}",stringify!($f)),s,load);)*}}
    fields!(
        name,
        deleteFileName,
        phaseJump,
        phaseJumpID,
        phaseJumpDir,
        T2efacFlagID,
        T2efacFlagVal,
        T2efacVal,
        T2equadFlagID,
        T2equadFlagVal,
        T2equadVal,
        tOffset,
        tOffset_f1,
        tOffset_f2,
        tOffset_t1,
        tOffset_t2,
        tOffsetSite,
        tOffsetFlags
    );
    scalar(
        &mut p.param.slots[crate::tempo2::Label::param_f as usize].cells[0].val,
        &format!("{root}F0"),
        s,
        load,
    );
}
fn state(c: &mut PulsarContext, jump: &mut i32, fdjump: &mut i32, s: &mut Map, load: bool) {
    scalar(jump, "jump", s, load);
    scalar(fdjump, "fdjump", s, load);
    for p in 0..c.pulsars.len() {
        let root = format!("p{p}.");
        fixed(&mut c.pulsars[p], &root, s, load);
        let view = c.pulsars[p].obsn.unwrap().unwrap();
        let rows = c.observations_mut(view).unwrap();
        assert_eq!(rows.len(), 12, "N physical backing membership");
        for (i, o) in rows.iter_mut().enumerate() {
            let root = format!("{root}obs{i}.");
            let key = root.clone() + "whole_record";
            let mut b = if load {
                decode(&s[&key])
            } else {
                vec![0; 7680]
            };
            observation_bytes(o, &mut b, load);
            if !load {
                s.insert(key, encode(&b));
            }
            macro_rules! fields {($($f:ident),*)=>{$(scalar(&mut o.$f,&format!("{root}{}",stringify!($f)),s,load);)*}}
            fields!(
                sat,
                sat_day,
                sat_sec,
                bat,
                freq,
                toaErr,
                origErr,
                phaseOffset,
                toaDMErr,
                efac,
                equad,
                nFlags,
                obsNjump,
                obsNfdjump,
                deleted,
                clockCorr,
                delayCorr,
                pulseN
            );
        }
    }
    crate::clkcorr::with_process(|p| {
        let d = &mut p.diagnostics;
        scalar(&mut d.tcheck, "tcheck", s, load);
        if load {
            p.source.max_observations = s["MAX_OBSN"].as_i64().unwrap() as i32;
            p.fortran2.swap_byte = s["swapByte2"].as_i64().unwrap() as i32;
            d.debug = s["debugFlag"] != 0;
            d.quiet = s["quietFlag"] != 0;
            d.display_version = s["displayCVSversion"] != 0;
        } else {
            for (k, v) in [
                ("MAX_PSR", json!(c.pulsars.len())),
                ("MAX_OBSN", json!(p.source.max_observations)),
                ("swapByte2", json!(p.fortran2.swap_byte)),
                ("stream_open", json!(p.fortran2.stream.is_some())),
                ("TK_warnCount", json!(d.warnings)),
                ("TK_errorCount", json!(d.errors)),
                ("debugFlag", json!(i32::from(d.debug))),
                ("quietFlag", json!(i32::from(d.quiet))),
                ("displayCVSversion", json!(i32::from(d.display_version))),
            ] {
                s.insert(k.into(), v);
            }
            for (name, log) in [
                ("TK_warnlog", &d.warning_log),
                ("TK_errorlog", &d.error_log),
            ] {
                let mut b = vec![0; 10800];
                for (i, line) in log.iter().enumerate().take(30) {
                    b[i * 360..i * 360 + line.len()].copy_from_slice(line);
                }
                s.insert(name.into(), encode(&b));
            }
            if let Some(stream) = &mut p.fortran2.stream {
                let (pos, eof, error) = stream.status();
                s.insert("position".into(), json!(pos));
                s.insert("eof".into(), json!(i32::from(eof != 0)));
                s.insert("ferror".into(), json!(error));
            }
        }
        Ok(())
    })
    .unwrap();
}
fn apply(s: &mut Map, e: &Value) {
    for (k, d) in e["changes"].as_object().unwrap() {
        if d["after"].is_null() {
            s.remove(k);
        } else {
            s.insert(k.clone(), d["after"].clone());
        }
    }
}
fn equal(a: &Value, b: &Value) -> bool {
    if a.get("type") == Some(&json!("binary128")) {
        return a["bits_le"] == b["bits_le"];
    }
    a == b
}
fn compare(a: &Map, b: &Map, tag: &str, fail: &mut Vec<String>) {
    if a.keys().collect::<Vec<_>>() != b.keys().collect::<Vec<_>>() {
        fail.push(format!("{tag}: state membership"));
    }
    for (k, b) in b {
        if let Some(a) = a.get(k)
            && !equal(a, b)
        {
            let first = if a.get("size").is_some() {
                decode(a).iter().zip(decode(b)).position(|(a, b)| *a != b)
            } else {
                None
            };
            fail.push(format!(
                "{tag}: {k}, first byte {first:?}, {} != {}",
                if first.is_some() {
                    String::new()
                } else {
                    a.to_string()
                },
                if first.is_some() {
                    String::new()
                } else {
                    b.to_string()
                }
            ));
        }
    }
}
fn read(
    c: &mut PulsarContext,
    p: usize,
    path: &Path,
    root: &Path,
    jump: &mut i32,
    fdjump: &mut i32,
    jbo: bool,
) -> Result<(), TimingError> {
    let view = c.pulsars[p].obsn.unwrap().unwrap();
    let mut rows = c.observations(view).unwrap().to_vec();
    let r = if jbo {
        crate::read_jbo_bat::read_jbo_bat(path, &mut c.pulsars[p], &mut rows, &mut vec![])
    } else {
        crate::source_tim::read_tim(
            path,
            &mut c.pulsars[p],
            &mut rows,
            jump,
            fdjump,
            root,
            &mut vec![],
        )
    };
    c.free_observations(view).unwrap();
    let view = c.adopt_observations(rows);
    c.pulsars[p].obsn = Some(Some(view));
    r
}
#[test]
#[ignore = "validation/input_to_bat/check_tim_corpus.py supplies N execution identities"]
fn n_corpus() {
    let work = std::path::PathBuf::from(std::env::var_os("AC_TIM_CORPUS").unwrap());
    let cases: Value = serde_json::from_slice(&fs::read(work.join("cases.json")).unwrap()).unwrap();
    let mut failures = vec![];
    for case in cases.as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../validation/tim_io/build/runtime")
            .canonicalize()
            .unwrap();
        let root = std::path::PathBuf::from(case["workdir"].as_str().unwrap());
        for (name, data) in case["files"].as_object().unwrap() {
            let path = root.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, unhex(data.as_str().unwrap())).unwrap();
        }
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            s.diagnostics.source_directory = case["source_directory"]
                .as_str()
                .unwrap()
                .as_bytes()
                .to_vec();
            Ok(())
        })
        .unwrap();
        let mut c = PulsarContext::unwritten(2);
        c.initialise(2, 12, 0).unwrap();
        let mut expected: Map = serde_json::from_value(case["events"][0]["state"].clone()).unwrap();
        let (mut jump, mut fdjump) = (7, 13);
        state(&mut c, &mut jump, &mut fdjump, &mut expected, true);
        for (i, a) in case["actions"].as_array().unwrap().iter().enumerate() {
            let a: Vec<&str> = a
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let tag = format!("{id} {i} {}", a[0]);
            let event = case["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["phase"] == "end" && e["action"] == i);
            let num = |k: usize| a[k].parse::<usize>().unwrap();
            let result = (|| -> Result<Value, TimingError> {
                match a[0] {
                    "read" | "jbo" => read(
                        &mut c,
                        num(1),
                        Path::new(a[2]),
                        &runtime,
                        &mut jump,
                        &mut fdjump,
                        a[0] == "jbo",
                    )?,
                    "full" => crate::source_tim::read_timfiles(
                        &mut c,
                        &a[1..]
                            .iter()
                            .map(std::path::PathBuf::from)
                            .collect::<Vec<_>>(),
                        &runtime,
                        &mut vec![],
                    )?,
                    "sort" => crate::sort_toas::sort_toas(&mut c, num(1))?,
                    "compare" => {
                        let x = SourceObservation {
                            sat: Some(extended(a[1].as_bytes())?),
                            ..Default::default()
                        };
                        let y = SourceObservation {
                            sat: Some(extended(a[2].as_bytes())?),
                            ..Default::default()
                        };
                        return Ok(json!(crate::sort_toas::compare_obs(&x, &y)?));
                    }
                    "write" => crate::source_tim::write_tim(
                        Path::new(a[2]),
                        &mut c,
                        num(1),
                        a[3].as_bytes(),
                        &runtime,
                    )?,
                    "set" => {
                        let p = &mut c.pulsars[num(1)];
                        macro_rules! set{($($f:ident),*)=>{match a[2]{$(stringify!($f)=>p.$f=Some(a[3].parse().unwrap()),)*"deleteFileName"=>write_string(&mut p.deleteFileName[..],a[3].as_bytes(),"N seed")?,"F0"=>p.param.slots[crate::tempo2::Label::param_f as usize].cells[0].val=extended(a[3].as_bytes())?,"jump"=>jump=a[3].parse().unwrap(),"fdjump"=>fdjump=a[3].parse().unwrap(),other=>crate::clkcorr::with_process(|p|{let v=a[3].parse::<i32>().unwrap();match other{"tcheck"=>p.diagnostics.tcheck=v,"debug"=>p.diagnostics.debug=v!=0,"quiet"=>p.diagnostics.quiet=v!=0,"cvs"=>p.diagnostics.display_version=v!=0,"maxobs"=>p.source.max_observations=v,_=>panic!("N set {other}")}Ok(())})?}}}
                        set!(nobs, sorted, fixedFormat, jboFormat, noWarnings, fitMode);
                    }
                    "obs" => {
                        let view = c.pulsars[num(1)].obsn.unwrap().unwrap();
                        let o = &mut c.observations_mut(view).unwrap()[num(2)];
                        let v = extended(a[4].as_bytes())?;
                        match a[3] {
                            "sat" => o.sat = Some(v),
                            "fname" => write_string(&mut o.fname, a[4].as_bytes(), "fname")?,
                            "telID" => write_string(&mut o.telID, a[4].as_bytes(), "telID")?,
                            "deleted" => o.deleted = Some(v.to_f64() as i32),
                            "nFlags" => o.nFlags = Some(v.to_f64() as i32),
                            other => {
                                let dest = match other {
                                    "freq" => &mut o.freq,
                                    "phaseOffset" => &mut o.phaseOffset,
                                    "origErr" => &mut o.origErr,
                                    "toaErr" => &mut o.toaErr,
                                    "efac" => &mut o.efac,
                                    "equad" => &mut o.equad,
                                    _ => panic!("N obs {other}"),
                                };
                                *dest = Some(v.to_f64());
                            }
                        }
                    }
                    "flag" => {
                        let view = c.pulsars[num(1)].obsn.unwrap().unwrap();
                        let o = &mut c.observations_mut(view).unwrap()[num(2)];
                        let n = o.nFlags.unwrap() as usize;
                        write_string(&mut o.flagID[n], a[3].as_bytes(), "flag")?;
                        write_string(&mut o.flagVal[n], a[4].as_bytes(), "flag")?;
                        o.nFlags = Some(n as i32 + 1);
                    }
                    "offset" => {
                        let p = &mut c.pulsars[num(1)];
                        let n = p.nToffset.unwrap() as usize;
                        for (dest, k) in [
                            (&mut p.tOffset[n], 2),
                            (&mut p.tOffset_f1[n], 3),
                            (&mut p.tOffset_f2[n], 4),
                            (&mut p.tOffset_t1[n], 5),
                            (&mut p.tOffset_t2[n], 6),
                        ] {
                            *dest = Some(a[k].parse().unwrap());
                        }
                        write_string(&mut p.tOffsetSite[n], a[7].as_bytes(), "offset")?;
                        write_string(&mut p.tOffsetFlags[n], a[8].as_bytes(), "offset")?;
                        p.nToffset = Some(n as i32 + 1);
                    }
                    "date" => {
                        return Ok(crate::read_jbo_bat::date2mjd(a[1].parse().unwrap())?.typed());
                    }
                    "remove" => {
                        let mut b = a[1].as_bytes().to_vec();
                        crate::source_tim::remove_cr2(&mut b)?;
                        let mut raw = a[1].as_bytes().to_vec();
                        raw.push(0);
                        raw[b.len()] = 0;
                        return Ok(encode(&raw));
                    }
                    "swap" => {
                        let mut b = [0; 16];
                        b[..a[2].len()].copy_from_slice(a[2].as_bytes());
                        if a[1] == "4" {
                            crate::read_jbo_bat::swap4((&mut b[..4]).try_into().unwrap());
                        } else if a[1] == "8" {
                            crate::read_jbo_bat::swap8((&mut b[..8]).try_into().unwrap());
                        } else {
                            let mut v = f64::from_ne_bytes(b[..8].try_into().unwrap());
                            crate::read_jbo_bat::swap8_double(&mut v);
                            b[..8].copy_from_slice(&v.to_ne_bytes());
                        }
                        return Ok(encode(&b));
                    }
                    "chars" => {
                        let a: [u8; 200] = a[1].as_bytes().try_into().unwrap();
                        let chars = crate::read_jbo_bat::makechars(&a);
                        return Ok(encode(&chars.concat()));
                    }
                    "open" => {
                        let mut filename = a[1].as_bytes().to_vec();
                        filename.push(0);
                        let mut swap = -1;
                        crate::read_fortran2::open_file2(&mut filename, &mut swap)?;
                        return Ok(
                            json!({"filename":String::from_utf8(filename.into_iter().take_while(|&b|b!=0).collect()).unwrap(),"returned_swap":swap}),
                        );
                    }
                    "close" => crate::read_fortran2::close_file2()?,
                    "swapmode" => crate::clkcorr::with_process(|p| {
                        p.fortran2.swap_byte = a[1].parse().unwrap();
                        Ok(())
                    })?,
                    "primitive" => {
                        return Ok(match a[1] {
                            "int" => crate::read_fortran2::read_int2()?.typed(),
                            "float" => crate::read_fortran2::read_float2()?.typed(),
                            "double" => crate::read_fortran2::read_double2()?.typed(),
                            "record" => crate::read_fortran2::read_record_int2()?.typed(),
                            _ => {
                                let mut b = vec![0; num(2)];
                                crate::read_fortran2::read_character2(num(2) as i32, &mut b)?;
                                encode(&b)
                            }
                        });
                    }
                    _ => panic!("unmapped N action {}", a[0]),
                }
                Ok(Value::Null)
            })();
            match (event, result) {
                (Some(e), Ok(v)) => {
                    if !equal(&v, &e["result"]) {
                        failures.push(format!("{tag} result: {v} != {}", e["result"]));
                    }
                    apply(&mut expected, e);
                }
                (None, Err(_)) => {
                    let e = case["events"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|e| e["phase"] == "exit_state")
                        .unwrap();
                    apply(&mut expected, e);
                }
                (Some(e), Err(error)) if id == "writer_fail" => {
                    assert_eq!(error.field, "writeTim");
                    apply(&mut expected, e);
                }
                (Some(_), Err(e)) => {
                    failures.push(format!("{tag} unexpected error: {e}"));
                    break;
                }
                (None, Ok(_)) => {
                    failures.push(format!("{tag} missing native exit"));
                    break;
                }
            }
            let mut actual = Map::new();
            state(&mut c, &mut jump, &mut fdjump, &mut actual, false);
            compare(&actual, &expected, &tag, &mut failures);
            if event.is_none() {
                break;
            }
        }
        for (name, bytes) in case["outputs"].as_object().unwrap() {
            let actual = fs::read(root.join(name)).ok().map(|b| hex(&b));
            let wanted = bytes.as_str().map(str::to_string);
            if actual != wanted {
                failures.push(format!("{id} output {name} bytes"));
            }
        }
        crate::clkcorr::with_process(|s| {
            let mut out = vec![];
            let mut err = vec![];
            for (stderr, b) in &s.diagnostics.events {
                if *stderr {
                    err.extend(b);
                } else {
                    out.extend(b);
                }
            }
            fs::write(
                root.join("rust-streams.json"),
                serde_json::to_vec(&json!({"stdout_hex":hex(&out),"stderr_hex":hex(&err)}))
                    .unwrap(),
            )
            .unwrap();
            Ok(())
        })
        .unwrap();
        eprintln!("N {id}: {} cumulative differences", failures.len());
    }
    fs::write(
        work.join("differences.json"),
        serde_json::to_vec_pretty(&failures).unwrap(),
    )
    .unwrap();
    assert!(
        failures.is_empty(),
        "{} differences, first {:?}",
        failures.len(),
        &failures[..failures.len().min(8)]
    );
}
