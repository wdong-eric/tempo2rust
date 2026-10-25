//! Accepted P caller, complete fixed/parameter/observation state after every action.
#![allow(non_snake_case)]
use crate::{
    TimingError,
    corpus_wire::*,
    observation::{SourceObservation, write_string},
    preprocess as pp,
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    source_text::extended,
    tempo2::{Label, PARAMETER_LABELS},
};
use rug::Float;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
type Map = BTreeMap<String, Value>;
include!("preprocessing_corpus_fields.rs");
fn state(c: &mut PulsarContext, s: &mut Map, load: bool, caller: &Map) {
    for p in 0..c.pulsars.len() {
        let root = format!("p{p}.");
        fixed(&mut c.pulsars[p], &root, s, load);
        parameter_arrays(&mut c.pulsars[p], &root, s, load);
        let view = c.pulsars[p].obsn.unwrap().unwrap();
        let rows = c.observations_mut(view).unwrap();
        if !load {
            s.insert(root.clone() + "caller_capacity", json!(rows.len()));
        }
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
            macro_rules! fields{($($f:ident),*)=>{$(scalar(&mut o.$f,&format!("{root}{}",stringify!($f)),s,load);)*}}
            fields!(
                sat,
                origsat,
                bat,
                freq,
                toaErr,
                origErr,
                phaseOffset,
                nFlags,
                obsNjump,
                obsNfdjump,
                deleted,
                tobs
            );
        }
    }
    crate::clkcorr::with_process(|p| {
        let d = &mut p.diagnostics;
        scalar(
            &mut p.source.ecliptic_obliquity,
            "ECLIPTIC_OBLIQUITY",
            s,
            load,
        );
        scalar(&mut p.source.force_global_fit, "forceGlobalFit", s, load);
        scalar(&mut p.source.max_observations, "MAX_OBSN", s, load);
        scalar(&mut d.tcheck, "tcheck", s, load);
        if load {
            d.debug = s["debugFlag"] != 0;
            d.quiet = s["quietFlag"] != 0;
            d.display_version = s["displayCVSversion"] != 0;
        } else {
            for (k, v) in [
                ("MAX_PSR", json!(2)),
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
            // P caller plugin search metadata is initialized but unconsumed in the
            // non-plugin cases. AC does not expose or emulate the C plugin ABI.
            for k in [
                "tempo2_plug_path_len",
                "tempo2MachineType",
                "tempo2_plug_path",
            ] {
                s.insert(k.into(), caller[k].clone());
            }
        }
        Ok(())
    })
    .unwrap();
}
fn label(s: &str) -> Label {
    *PARAMETER_LABELS
        .iter()
        .find(|n| format!("{n:?}") == s)
        .unwrap()
}
fn apply(s: &mut Map, e: &Value) {
    for (k, v) in e["changes"].as_object().unwrap() {
        if v["after"].is_null() {
            s.remove(k);
        } else {
            s.insert(k.clone(), v["after"].clone());
        }
    }
}
fn normalize_nan(a: &mut [u8], b: &mut [u8], width: usize, tag: &str, out: &mut Vec<Value>) {
    for (i, (a, b)) in a
        .chunks_exact_mut(width)
        .zip(b.chunks_exact_mut(width))
        .enumerate()
    {
        let bits = |v: &[u8]| -> u128 { v.iter().rev().fold(0, |n, &x| (n << 8) | x as u128) };
        let (mantissa, exponent) = match width {
            4 => (23, 8),
            8 => (52, 11),
            16 => (112, 15),
            _ => panic!("floating width"),
        };
        let mask = (1u128 << mantissa) - 1;
        let e = ((1u128 << exponent) - 1) << mantissa;
        let sign = 1u128 << (width * 8 - 1);
        let (x, y) = (bits(a), bits(b));
        if x != y
            && x & e == e
            && y & e == e
            && x & mask != 0
            && y & mask != 0
            && x & sign == y & sign
        {
            out.push(json!({"location":tag,"element":i,"width":width,"native_bits":hex(b),"rust_bits":hex(a),"disposition":"0068 NaN class and sign agree; payload transport differs"}));
            a.copy_from_slice(b);
        }
    }
}
fn equal(
    a: &Value,
    b: &Value,
    key: &str,
    layout: &Value,
    tag: &str,
    transport: &mut Vec<Value>,
) -> bool {
    if a == b {
        return true;
    }
    if a.get("type").is_some() {
        if a["type"] != b["type"] || a.get("class") != b.get("class") {
            return false;
        }
        let width = match a["type"].as_str() {
            Some("binary128") => 16,
            Some("binary64") => 8,
            Some("binary32") => 4,
            _ => return false,
        };
        let (mut a, mut b) = (
            unhex(a["bits_le"].as_str().unwrap()),
            unhex(b["bits_le"].as_str().unwrap()),
        );
        normalize_nan(&mut a, &mut b, width, tag, transport);
        return a == b;
    }
    if a.get("size").is_none() || a["size"] != b["size"] {
        return false;
    }
    let (mut a, mut b) = (decode(a), decode(b));
    if let Some((_, field)) = key.split_once(".param.") {
        if ["val", "err", "prefit", "prefitErr"].contains(&field) {
            normalize_nan(&mut a, &mut b, 16, tag, transport);
        }
    } else if key.ends_with(".whole_record") {
        for (name, v) in layout.as_object().unwrap() {
            if name.starts_with("observation.") {
                let off = v[0].as_u64().unwrap() as usize;
                let n = v[1].as_u64().unwrap() as usize;
                normalize_nan(
                    &mut a[off..off + n],
                    &mut b[off..off + n],
                    v[2].as_u64().unwrap() as usize,
                    &format!("{tag} {name}"),
                    transport,
                );
            }
        }
        // The source clock_correction is a double followed by 32 char bytes.
        // Its complete 30-cell native extent is 1200 bytes at the P ABI offset.
        for i in 0..30 {
            let off = 984 + 40 * i;
            normalize_nan(
                &mut a[off..off + 8],
                &mut b[off..off + 8],
                8,
                &format!("{tag} correctionsTT[{i}]"),
                transport,
            );
        }
    } else if let Some((_, field)) = key.split_once('.')
        && let Some(v) = layout.get(format!("pulsar.{field}"))
    {
        normalize_nan(
            &mut a,
            &mut b,
            v[2].as_u64().unwrap() as usize,
            tag,
            transport,
        );
    }
    a == b
}
fn compare(
    c: &PulsarContext,
    a: &Map,
    b: &Map,
    tag: &str,
    layout: &Value,
    transport: &mut Vec<Value>,
    fail: &mut Vec<Value>,
) {
    if a.keys().collect::<Vec<_>>() != b.keys().collect::<Vec<_>>() {
        fail.push(json!({"tag":tag,"error":"membership","missing":b.keys().filter(|k|!a.contains_key(*k)).collect::<Vec<_>>(),"extra":a.keys().filter(|k|!b.contains_key(*k)).collect::<Vec<_>>()}));
    }
    for (k, b) in b {
        if let Some(a) = a.get(k)
            && !equal(a, b, k, layout, &format!("{tag} {k}"), transport)
        {
            if a.get("size").is_some() {
                let aa = decode(a);
                let bb = decode(b);
                let deltas: Vec<_> = aa
                    .iter()
                    .zip(&bb)
                    .enumerate()
                    .filter_map(|(i, (a, b))| (a != b).then_some(json!([i, a, b])))
                    .collect();
                let mut numbers = vec![];
                if let Some((p, field)) = k.split_once(".param.")
                    && ["val", "err", "prefit", "prefitErr"].contains(&field)
                {
                    let p = p.strip_prefix('p').unwrap().parse::<usize>().unwrap();
                    let mut offset = 0;
                    for (index, par) in c.pulsars[p].param.slots.iter().enumerate() {
                        for (j, cell) in par.cells.iter().enumerate() {
                            if aa[offset..offset + 16] != bb[offset..offset + 16] {
                                let actual = match field {
                                    "val" => &cell.val,
                                    "err" => &cell.err,
                                    "prefit" => cell.prefit.as_ref().unwrap(),
                                    _ => cell.prefit_err.as_ref().unwrap(),
                                };
                                let native = Float::take(&mut &bb[offset..offset + 16]);
                                numbers.push(json!({"parameter":format!("{:?}",PARAMETER_LABELS.get(index)),"index":j,"rust_precision":actual.prec(),"rust":actual.to_string_radix(10,None),"native_precision":113,"native":Float::with_val(113,native).to_string_radix(10,None)}));
                            }
                            offset += 16;
                        }
                    }
                }
                fail.push(json!({"tag":tag,"field":k,"byte_changes":deltas,"numbers":numbers}));
            } else {
                fail.push(json!({"tag":tag,"field":k,"rust":a,"native":b}));
            }
        }
    }
}
#[test]
#[ignore = "validation/input_to_bat/check_preprocessing.py supplies accepted P fixtures"]
fn p_corpus() {
    let work = PathBuf::from(std::env::var_os("AC_PREPROCESS_CORPUS").unwrap());
    let cases: Value = serde_json::from_slice(&fs::read(work.join("cases.json")).unwrap()).unwrap();
    let initial: Value =
        serde_json::from_slice(&fs::read(work.join("initial.json")).unwrap()).unwrap();
    let caller: Map = serde_json::from_value(initial["state"].clone()).unwrap();
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../validation/preprocessing/build/runtime")
        .canonicalize()
        .unwrap();
    let layout: Value =
        serde_json::from_slice(&fs::read(work.join("floating-layout.json")).unwrap()).unwrap();
    let mut transport = vec![];
    let original_cwd = std::env::current_dir().unwrap();
    let mut failures = vec![];
    let mut diagnostic_differences = vec![];
    for case in cases.as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        std::env::set_current_dir(case["workdir"].as_str().unwrap()).unwrap();
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            s.diagnostics.source_directory =
                format!("{}/probe_source/", runtime.parent().unwrap().display()).into_bytes();
            Ok(())
        })
        .unwrap();
        let mut c = PulsarContext::unwritten(2);
        c.initialise(2, 12, 0).unwrap();
        let mut expected = caller.clone();
        state(&mut c, &mut expected, true, &caller);
        for (i, a) in case["actions"].as_array().unwrap().iter().enumerate() {
            let a: Vec<&str> = a
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let num = |k: usize| a[k].parse::<usize>().unwrap();
            let tag = format!("{id} {i} {}", a[0]);
            let event = case["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["phase"] == "end" && e["action"] == i);
            let result = (|| -> Result<(), TimingError> {
                match a[0] {
                    "simple" => pp::pre_process_simple(&mut c, num(1), &runtime)?,
                    "stage1" => pp::pre_process_simple1(
                        &mut c,
                        num(1),
                        a[2].parse().unwrap(),
                        a[3].parse().unwrap(),
                        &runtime,
                    )?,
                    "stage2" => {
                        let dm: Vec<f32> = a[5..].iter().map(|x| x.parse().unwrap()).collect();
                        pp::pre_process_simple2(
                            &mut c,
                            num(1),
                            a[2].parse().unwrap(),
                            a[3].parse().unwrap(),
                            (!dm.is_empty()).then_some(dm.as_slice()),
                            a[4].parse().unwrap(),
                            &runtime,
                        )?;
                    }
                    "dmnull" => pp::pre_process_simple2(&mut c, 0, 58000., 3, None, 0, &runtime)?,
                    "stage3" => pp::pre_process_simple3(&mut c, num(1))?,
                    "units" => crate::units::transform_units(
                        &mut c.pulsars[num(1)],
                        a[2].parse().unwrap(),
                        a[3].parse().unwrap(),
                    )?,
                    "scale" | "mjd" => {
                        let p = &mut c.pulsars[num(1)].param.slots[label(a[2]) as usize];
                        let factor = extended(a[4].as_bytes())?;
                        if a[0] == "scale" {
                            crate::units::scale_param(p, num(3), &factor)?;
                        } else {
                            crate::units::xform_mjd(p, num(3), &factor)?;
                        }
                    }
                    "epoch" => crate::update_epoch::update_epoch(
                        &mut c.pulsars[num(1)],
                        &extended(a[2].as_bytes())?,
                    )?,
                    "epochstr" => {
                        crate::update_epoch::update_epoch_str(&mut c, num(1), a[2].as_bytes())?
                    }
                    "wrapper" | "select" | "logic" | "process" | "simul" => {
                        let n = if a[0] == "wrapper" { num(1) } else { num(2) };
                        let suffix = c.pulsars.split_off(n);
                        let result = match a[0] {
                            "wrapper" => pp::pre_process(
                                &mut c,
                                &a[2..]
                                    .iter()
                                    .map(|s| s.as_bytes().to_vec())
                                    .collect::<Vec<_>>(),
                                &runtime,
                                &mut vec![],
                            ),
                            "select" => crate::selection::use_select_file(
                                Path::new(a[1]),
                                &mut c,
                                &mut vec![],
                            ),
                            "logic" => crate::selection::logic_flag(a[1].as_bytes(), &mut c),
                            "process" => crate::selection::process_flag(a[1].as_bytes(), &mut c),
                            _ => crate::selection::process_simultaneous(a[1].as_bytes(), &mut c),
                        };
                        c.pulsars.extend(suffix);
                        result?;
                    }
                    "white" => pp::read_white_noise_model_file(&mut c, num(1), &mut vec![])?,
                    "param" => {
                        let cell =
                            &mut c.pulsars[num(1)].param.slots[label(a[2]) as usize].cells[num(3)];
                        cell.val = extended(a[4].as_bytes())?;
                        cell.prefit = Some(extended(a[5].as_bytes())?);
                        cell.err = extended(a[6].as_bytes())?;
                        cell.prefit_err = Some(extended(a[7].as_bytes())?);
                        cell.param_set = a[8].parse().unwrap();
                        cell.fit_flag = a[9].parse().unwrap();
                    }
                    "seed_units" => {
                        let p = &mut c.pulsars[num(1)];
                        for (i, par) in p.param.slots.iter_mut().enumerate() {
                            for (j, cell) in par.cells.iter_mut().enumerate() {
                                cell.val = Float::with_val(128, 50000 + i)
                                    + Float::with_val(128, j) * 0.125;
                                cell.prefit = Some(
                                    Float::with_val(128, 51000 + i)
                                        + Float::with_val(128, j) * 0.25,
                                );
                                cell.err = Float::with_val(128, j) + 0.5;
                                cell.prefit_err = Some(Float::with_val(128, j) + 0.75);
                                cell.param_set = (j % 2) as i32;
                                cell.fit_flag = (j % 3) as i32;
                            }
                        }
                        p.param.slots[Label::param_fddi as usize].cells[0].val =
                            Float::with_val(128, 1.5);
                        p.TNChromIdx = Some(4.25);
                    }
                    "link" => {
                        let p = &mut c.pulsars[num(1)].param.slots[label(a[2]) as usize];
                        p.n_link_to = Some(1);
                        p.link_to[0] = Some(label(a[3]) as i32);
                    }
                    "set" => {
                        let p = &mut c.pulsars[num(1)];
                        macro_rules! set{($($f:ident),*)=>{match a[2]{$(stringify!($f)=>p.$f=Some(a[3].parse().unwrap()),)*"dm_series_type"=>p.dm_series_type=Some(match a[3]{"0"=>crate::pulsar::SeriesType::series_undefined,"1"=>crate::pulsar::SeriesType::series_simple_pn,"2"=>crate::pulsar::SeriesType::series_taylor_pn,_=>panic!("P series enum")}),_=>{let dest=match a[2]{"filterStr"=>&mut p.filterStr[..],"passStr"=>&mut p.passStr[..],"fjumpID"=>&mut p.fjumpID[..],"tzrsite"=>&mut p.tzrsite[..],"whiteNoiseModelFile"=>&mut p.whiteNoiseModelFile[..],"clock"=>&mut p.clock[..],"name"=>&mut p.name[..],"binaryModel"=>&mut p.binaryModel[..],other=>{crate::clkcorr::with_process(|s|{match other{"debug"=>s.diagnostics.debug=a[3]!="0","quiet"=>s.diagnostics.quiet=a[3]!="0","cvs"=>s.diagnostics.display_version=a[3]!="0",_=>panic!("P set {other}")}Ok(())})?;return Ok(());}};write_string(dest,a[3].as_bytes(),"P set")?;}}}}
                        set!(
                            nobs,
                            nWhite,
                            nWhite_dm,
                            units,
                            fitMode,
                            nJumps,
                            nfdJumps,
                            nPhaseJump,
                            T2globalEfac,
                            TNGlobalEF,
                            TNGlobalEQ,
                            TNChromIdx,
                            nTNSECORR
                        );
                    }
                    "obs" => {
                        let view = c.pulsars[num(1)].obsn.unwrap().unwrap();
                        let o = &mut c.observations_mut(view).unwrap()[num(2)];
                        macro_rules! set{($($f:ident),*)=>{match a[3]{$(stringify!($f)=>o.$f=Some(a[4].parse().unwrap()),)*"sat"=>o.sat=Some(extended(a[4].as_bytes())?),"bat"=>o.bat=Some(extended(a[4].as_bytes())?),"fname"=>write_string(&mut o.fname,a[4].as_bytes(),"P fname")?,"telID"=>write_string(&mut o.telID,a[4].as_bytes(),"P telID")?,_=>panic!("P obs")}}}
                        set!(
                            freq,
                            phaseOffset,
                            origErr,
                            toaErr,
                            deleted,
                            nFlags,
                            obsNjump,
                            obsNfdjump
                        );
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
                    "jump" => {
                        let p = &mut c.pulsars[num(1)];
                        let (count, strings) = if a[2] == "fd" {
                            (&mut p.nfdJumps, &mut p.fdjumpStr)
                        } else {
                            (&mut p.nJumps, &mut p.jumpStr)
                        };
                        let n = count.unwrap() + 1;
                        *count = Some(n);
                        write_string(&mut strings[n as usize], a[3].as_bytes(), "jump")?;
                    }
                    "phase" => {
                        let p = &mut c.pulsars[num(1)];
                        let n = p.nPhaseJump.unwrap() as usize;
                        p.phaseJump[n] = Some(extended(a[2].as_bytes())?);
                        p.phaseJumpID[n] = Some(a[3].parse().unwrap());
                        p.nPhaseJump = Some(n as i32 + 1);
                    }
                    "noise" => {
                        let p = &mut c.pulsars[num(1)];
                        macro_rules! noise{($(($f:literal,$count:ident,$ids:ident,$values:ident,$numbers:ident)),*)=>{match a[2]{$($f=>{let n=p.$count.unwrap()as usize;write_string(&mut p.$ids[n],a[3].as_bytes(),"noise")?;write_string(&mut p.$values[n],a[4].as_bytes(),"noise")?;p.$numbers[n]=Some(a[5].parse().unwrap());p.$count=Some(n as i32+1);},)*_=>panic!("P noise")}}}
                        noise!(
                            ("T2efac", nT2efac, T2efacFlagID, T2efacFlagVal, T2efacVal),
                            (
                                "T2equad",
                                nT2equad,
                                T2equadFlagID,
                                T2equadFlagVal,
                                T2equadVal
                            ),
                            ("TNEF", nTNEF, TNEFFlagID, TNEFFlagVal, TNEFVal),
                            ("TNEQ", nTNEQ, TNEQFlagID, TNEQFlagVal, TNEQVal),
                            ("TNSQ", nTNSQ, TNSQFlagID, TNSQFlagVal, TNSQVal)
                        );
                    }
                    "reverse" => {
                        let view = c.pulsars[num(1)].obsn.unwrap().unwrap();
                        let n = c.pulsars[num(1)].nobs.unwrap() as usize;
                        c.observations_mut(view).unwrap()[..n].reverse();
                    }
                    "fjump_capacity" => {
                        let p = &mut c.pulsars[0];
                        p.nobs = Some(50);
                        write_string(&mut p.fjumpID[..], b"-f", "fjump")?;
                        let mut rows = vec![SourceObservation::default(); 50];
                        for (i, o) in rows.iter_mut().enumerate() {
                            observation_bytes(o, &mut vec![0; 7680], true);
                            o.nFlags = Some(40);
                            for j in 0..40 {
                                write_string(&mut o.flagID[j], b"-f", "flag")?;
                                write_string(
                                    &mut o.flagVal[j],
                                    format!("v{}", 40 * i + j).as_bytes(),
                                    "flag",
                                )?;
                            }
                        }
                        let old = p.obsn.unwrap().unwrap();
                        c.free_observations(old).unwrap();
                        let view = c.adopt_observations(rows);
                        c.pulsars[0].obsn = Some(Some(view));
                    }
                    _ => panic!("unmapped P action {}", a[0]),
                }
                Ok(())
            })();
            match (event, result) {
                (Some(e), Ok(())) => apply(&mut expected, e),
                (None, Err(_)) => {
                    let e = case["events"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|e| e["phase"] == "exit_state")
                        .unwrap();
                    apply(&mut expected, e);
                }
                (Some(_), Err(e)) => {
                    failures.push(json!({"tag":tag,"error":e.to_string()}));
                    break;
                }
                (None, Ok(())) => {
                    failures.push(json!({"tag":tag,"error":"missing native source exit"}));
                    break;
                }
            }
            let mut actual = Map::new();
            state(&mut c, &mut actual, false, &caller);
            compare(
                &c,
                &actual,
                &expected,
                &tag,
                &layout,
                &mut transport,
                &mut failures,
            );
            if event.is_none() {
                break;
            }
        }
        fs::write(
            work.join("differences.json"),
            serde_json::to_vec(&failures).unwrap(),
        )
        .unwrap();
        for (channel, actual) in ["stdout", "stderr"]
            .into_iter()
            .zip(crate::corpus_wire::diagnostic_streams())
        {
            if actual != case[channel] {
                diagnostic_differences.push(
                    json!({"case":id,"channel":channel,"rust":actual,"native":case[channel]}),
                );
            }
        }
        eprintln!("P {id}: {} cumulative differences", failures.len());
    }
    fs::write(
        work.join("differences.json"),
        serde_json::to_vec(&failures).unwrap(),
    )
    .unwrap();
    fs::write(
        work.join("transport-differences.json"),
        serde_json::to_vec_pretty(&transport).unwrap(),
    )
    .unwrap();
    std::env::set_current_dir(original_cwd).unwrap();
    fs::write(
        work.join("diagnostic-differences.json"),
        serde_json::to_vec_pretty(&diagnostic_differences).unwrap(),
    )
    .unwrap();
    assert!(
        diagnostic_differences.is_empty(),
        "{} P diagnostic differences",
        diagnostic_differences.len()
    );
    assert!(
        failures.is_empty(),
        "{} differences: {:?}",
        failures.len(),
        failures
            .iter()
            .take(5)
            .map(|v| (&v["tag"], &v["field"], &v["error"]))
            .collect::<Vec<_>>()
    );
}

#[test]
#[ignore = "localize the P SIMUL discrete boundary without changing production precision"]
fn p_precision_diagnosis() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let work = root.join("validation/input_to_bat/build/preprocessing");
    let native: Value =
        serde_json::from_slice(&fs::read(work.join("simul-native.json")).unwrap()).unwrap();
    let input = "58000.0006944444444444444444444444444444";
    let mut results = vec![];
    for bits in [113, 128, 256, 512] {
        let sat = Float::with_val(bits, Float::parse(input).unwrap());
        let seconds = (sat.clone() - Float::with_val(bits, 58000)).abs() * 86400.0f64;
        let mut bytes = vec![];
        sat.put(&mut bytes);
        if bits == 113 {
            assert_eq!(hex(&bytes), native["sat"]["bits_le"].as_str().unwrap());
        }
        results.push(json!({"bits":bits,"SAT":sat.to_string_radix(10,None),"separation_s":seconds.to_string_radix(10,None),"strict_less_60":seconds<60}));
    }
    let output = json!({"case":"simul_123","source":"preProcess.C:930 strict fabs(SAT_i-SAT_j)*SECDAY < tdiff","native":native,"calculations":results,"production_precision_unchanged":true,"disposition":"unresolved discrete source branch difference; no allowance or production rounding added"});
    assert_eq!(output["calculations"][0]["strict_less_60"], false);
    assert_eq!(output["calculations"][1]["strict_less_60"], true);
    fs::write(
        work.join("simul-precision-diagnosis.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
}

#[test]
fn nan_projection_preserves_neighbor_bytes_and_signs() {
    let native = (0x7fffu128 << 112) | ((1u128 << 112) - 1);
    let rust = (0x7fffu128 << 112) | (1u128 << 111);
    let mut a = rust.to_le_bytes();
    let mut b = native.to_le_bytes();
    let mut notes = vec![];
    normalize_nan(&mut a, &mut b, 16, "NaN", &mut notes);
    assert_eq!(a, b);
    assert_eq!(notes.len(), 1);
    let mut negative = (rust | (1u128 << 127)).to_le_bytes();
    normalize_nan(&mut negative, &mut b, 16, "sign", &mut notes);
    assert_ne!(negative, b);
    for (x, y) in [
        (0u128, 1u128 << 127),
        (0x7fffu128 << 112, (0x7fffu128 << 112) | (1u128 << 127)),
        (1, 2),
    ] {
        let mut a = x.to_le_bytes();
        let mut b = y.to_le_bytes();
        normalize_nan(&mut a, &mut b, 16, "exact", &mut notes);
        assert_ne!(a, b);
    }
    let mut a = vec![0; 7680];
    let mut b = a.clone();
    a[..16].copy_from_slice(&rust.to_le_bytes());
    b[..16].copy_from_slice(&native.to_le_bytes());
    a[7680 - 1] = 1;
    let layout = json!({"observation.sat":[0,16,16]});
    assert!(!equal(
        &encode(&a),
        &encode(&b),
        "p0.obs0.whole_record",
        &layout,
        "neighbor",
        &mut notes
    ));
    a[7680 - 1] = 0;
    assert!(equal(
        &encode(&a),
        &encode(&b),
        "p0.obs0.whole_record",
        &layout,
        "payload",
        &mut notes
    ));
}
