//! AB recursive typed state, complete membership, tails and ordered histories.
#![allow(non_snake_case)]
use crate::{
    TimingError,
    corpus_wire::{Wire, hex, unhex},
    observation::{SourceObservation, write_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::{ObservationView, PulsarContext},
    source_text::extended,
    tempo2::{Label, PARAMETER_LABELS, SourceParameter},
};
use rug::Float;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
type Map = BTreeMap<String, Value>;
include!("propagation_corpus_fields.rs");

fn bytes(v: &Value) -> Vec<u8> {
    let w = v["width"].as_u64().unwrap() as usize;
    let mut result = vec![];
    for r in v["runs"].as_array().unwrap() {
        let s = r[1].as_str().unwrap();
        let b = match s {
            "NaN" | "+Inf" | "-Inf" => {
                let f = match s {
                    "NaN" => f64::NAN,
                    "+Inf" => f64::INFINITY,
                    _ => f64::NEG_INFINITY,
                };
                match w {
                    4 => (f as f32).to_le_bytes().to_vec(),
                    8 => f.to_le_bytes().to_vec(),
                    16 => {
                        let mut v = vec![];
                        Float::with_val(crate::PRECISION_BITS, f).put(&mut v);
                        v
                    }
                    _ => panic!("class width"),
                }
            }
            _ => unhex(s),
        };
        assert_eq!(b.len(), w);
        for _ in 0..r[0].as_u64().unwrap() {
            result.extend(&b);
        }
    }
    assert_eq!(result.len(), w * v["count"].as_u64().unwrap() as usize);
    result
}
fn record(b: &[u8], width: usize, floating: bool) -> Value {
    assert_eq!(b.len() % width, 0);
    let mut runs: Vec<Value> = vec![];
    let mut chunks = b.chunks_exact(width).peekable();
    while let Some(v) = chunks.next() {
        let mut count = 1;
        while chunks.peek().is_some_and(|next| *next == v) {
            chunks.next();
            count += 1;
        }
        let text = if floating {
            let bits = v.iter().rev().fold(0u128, |n, b| (n << 8) | *b as u128);
            let (m, e) = match width {
                4 => (23, 8),
                8 => (52, 11),
                16 => (112, 15),
                _ => panic!("float width"),
            };
            let mask = ((1u128 << e) - 1) << m;
            if bits & mask == mask {
                if bits & ((1u128 << m) - 1) != 0 {
                    "NaN".into()
                } else if bits >> (width * 8 - 1) != 0 {
                    "-Inf".into()
                } else {
                    "+Inf".into()
                }
            } else {
                hex(v)
            }
        } else {
            hex(v)
        };
        if let Some(last) = runs.last_mut()
            && last[1] == text
        {
            last[0] = json!(last[0].as_u64().unwrap() + count);
        } else {
            runs.push(json!([count, text]));
        }
    }
    json!({"width":width,"count":b.len()/width,"runs":runs})
}
fn ab_field<T: Wire>(
    value: &mut T,
    key: &str,
    width: usize,
    floating: bool,
    s: &mut Map,
    load: bool,
) {
    if load {
        let b = bytes(&s[key]);
        let mut tail = b.as_slice();
        *value = T::take(&mut tail);
        assert!(tail.is_empty(), "{key}");
    } else {
        let mut b = vec![];
        value.put(&mut b);
        s.insert(key.into(), record(&b, width, floating));
    }
}
fn null_pointer<T>(value: &mut Option<Option<T>>, key: &str, s: &mut Map, load: bool) {
    if load {
        assert_eq!(s[key], json!({"owner":"null"}));
        *value = Some(None);
    } else {
        assert!(matches!(value, Some(None)), "{key} pointer changed");
        s.insert(key.into(), json!({"owner":"null"}));
    }
}
fn null_array<T, const N: usize>(
    values: &mut [Option<Option<T>>; N],
    key: &str,
    s: &mut Map,
    load: bool,
) {
    if load {
        assert_eq!(s[key], json!({"pointer_count":N,"runs":[[N,"null"]]}));
        for v in values {
            *v = Some(None);
        }
    } else {
        assert!(
            values.iter().all(|v| matches!(v, Some(None))),
            "{key} pointer changed"
        );
        s.insert(key.into(), json!({"pointer_count":N,"runs":[[N,"null"]]}));
    }
}
fn set_cell<T: Wire>(
    value: &mut T,
    width: usize,
    kind: &str,
    array: bool,
    index: usize,
    input: &str,
) {
    let mut b = vec![];
    value.put(&mut b);
    if kind == "char" && array {
        let input = input.as_bytes();
        assert!(input.len() < b.len());
        b[..input.len()].copy_from_slice(input);
        b[input.len()] = 0;
    } else {
        let x = match kind {
            "longdouble" => {
                let mut b = vec![];
                extended(input.as_bytes()).unwrap().put(&mut b);
                b
            }
            "double" => input.parse::<f64>().unwrap().to_le_bytes().to_vec(),
            "float" => (input.parse::<f64>().unwrap() as f32)
                .to_le_bytes()
                .to_vec(),
            _ => input.parse::<i64>().unwrap().to_le_bytes()[..width].to_vec(),
        };
        let offset = if array { index * width } else { 0 };
        b[offset..offset + width].copy_from_slice(&x);
    }
    *value = T::take(&mut b.as_slice());
}
fn label(name: &str) -> Label {
    *PARAMETER_LABELS
        .iter()
        .find(|l| format!("{l:?}") == format!("param_{name}"))
        .unwrap()
}
fn parameters(p: &mut SourcePulsar, root: &str, s: &mut Map, load: bool) {
    let key = format!("{root}.param");
    if load {
        let mut i = 0;
        for run in s[&key]["runs"].as_array().unwrap() {
            for _ in 0..run[0].as_u64().unwrap() {
                let r = &run[1];
                let size = i32::take(&mut bytes(&r["aSize"]).as_slice());
                let mut slot = SourceParameter::allocated(size as usize, 0);
                slot.link_from = Wire::take(&mut bytes(&r["linkFrom"]).as_slice());
                slot.link_to = Wire::take(&mut bytes(&r["linkTo"]).as_slice());
                slot.n_link_from = Wire::take(&mut bytes(&r["nLinkFrom"]).as_slice());
                slot.n_link_to = Wire::take(&mut bytes(&r["nLinkTo"]).as_slice());
                // Native unallocated members map to canonical unwritten prefit and
                // empty labels; cells own their val/flag storage together in Rust.
                for name in [
                    "err",
                    "fitFlag",
                    "label",
                    "shortlabel",
                    "prefit",
                    "prefitErr",
                ] {
                    assert_eq!(r[name], json!({"owner":"null"}));
                }
                p.param.slots[i] = slot;
                i += 1;
            }
        }
        assert_eq!(i, 2000);
    }
    let mut records: Vec<Value> = vec![];
    for (i, p) in p.param.slots.iter_mut().enumerate() {
        let mut r = serde_json::Map::new();
        for (name, b) in [
            ("aSize", p.a_size.unwrap().to_le_bytes().to_vec()),
            ("nLinkFrom", p.n_link_from.unwrap().to_le_bytes().to_vec()),
            ("nLinkTo", p.n_link_to.unwrap().to_le_bytes().to_vec()),
        ] {
            r.insert(name.into(), record(&b, 4, false));
        }
        for (name, v) in [("linkFrom", &p.link_from), ("linkTo", &p.link_to)] {
            let mut b = vec![];
            v.put(&mut b);
            r.insert(name.into(), record(&b, 4, false));
        }
        for name in [
            "err",
            "fitFlag",
            "label",
            "shortlabel",
            "prefit",
            "prefitErr",
        ] {
            r.insert(name.into(), json!({"owner":"null"}));
        }
        for (name, suffix) in [("val", "val"), ("paramSet", "flags")] {
            let owner = if p.cells.is_empty() {
                "null".into()
            } else {
                format!(
                    "{root}.{}.{suffix}",
                    format!("{:?}", PARAMETER_LABELS[i]).trim_start_matches("param_")
                )
            };
            r.insert(name.into(), json!({"owner":owner}));
        }
        if !p.cells.is_empty() {
            let vk = format!("{root}.param[{i}].val.data");
            let fk = format!("{root}.param[{i}].paramSet.data");
            if load {
                let vb = bytes(&s[&vk]);
                let fb = bytes(&s[&fk]);
                let (mut vb, mut fb) = (vb.as_slice(), fb.as_slice());
                for c in &mut p.cells {
                    c.val = Wire::take(&mut vb);
                    c.param_set = Wire::take(&mut fb);
                }
                assert!(vb.is_empty() && fb.is_empty());
            } else {
                let (mut vb, mut fb) = (vec![], vec![]);
                for c in &p.cells {
                    c.val.put(&mut vb);
                    c.param_set.put(&mut fb);
                    assert!(
                        c.label.is_empty()
                            && c.shortlabel.is_empty()
                            && c.err.is_zero()
                            && c.fit_flag == 0
                            && c.prefit.is_none()
                            && c.prefit_err.is_none()
                    );
                }
                s.insert(vk, record(&vb, 16, true));
                s.insert(fk, record(&fb, 4, false));
            }
        }
        if let Some(last) = records.last_mut()
            && last[1] == Value::Object(r.clone())
        {
            last[0] = json!(last[0].as_u64().unwrap() + 1);
        } else {
            records.push(json!([1, r]));
        }
    }
    if !load {
        s.insert(key, json!({"parameter_count":2000,"runs":records}));
    }
}
fn state(c: &mut PulsarContext, s: &mut Map, load: bool, views: &[ObservationView]) {
    for i in 0..2 {
        let p = &mut c.pulsars[i];
        let root = format!("p{i}");
        ab_pulsar(p, &root, s, load);
        parameters(p, &root, s, load);
        let key = format!("{root}.obsn");
        if load {
            assert_eq!(s[&key], json!({"owner":format!("obs{}",i*6)}));
            p.obsn = Some(Some(views[i]));
        } else {
            let owner = views.iter().position(|v| Some(Some(*v)) == p.obsn).unwrap();
            s.insert(key, json!({"owner":format!("obs{}",owner*6)}));
        }
        for (j, o) in c.observations_mut(views[i]).unwrap().iter_mut().enumerate() {
            ab_observation(o, &format!("obs{}", i * 6 + j), s, load);
        }
    }
    crate::clkcorr::with_process(|session| {
        for (key, v) in [
            ("TK_warnCount", &mut session.diagnostics.warnings),
            ("TK_errorCount", &mut session.diagnostics.errors),
        ] {
            ab_field(v, key, 4, false, s, load);
        }
        Ok(())
    })
    .unwrap();
}
fn action(
    c: &mut PulsarContext,
    a: &[&str],
    root: &mut PathBuf,
) -> Result<Vec<Value>, TimingError> {
    let k = |i: usize| a[i].parse::<i32>().unwrap();
    let n = |i: usize| a[i].parse::<f64>().unwrap();
    let q = |i: usize| extended(a[i].as_bytes()).unwrap();
    match a[0] {
        "profile" => {
            return Ok(vec![
                json!(8),
                json!(53),
                json!(8),
                json!(53),
                json!(16),
                json!(113),
            ]);
        }
        "env" => {
            assert_eq!(a[1], "TEMPO2");
            *root = PathBuf::from(a[2]);
        }
        "debug" => crate::clkcorr::with_process(|s| {
            s.diagnostics.debug = k(1) != 0;
            s.diagnostics.display_version = k(2) == 1;
            s.diagnostics.quiet = k(3) != 0;
            Ok(())
        })?,
        "fast" => crate::clkcorr::with_process(|s| {
            s.source.very_fast = k(1);
            Ok(())
        })?,
        "p" => set_pulsar(&mut c.pulsars[k(1) as usize], a[2], k(3) as usize, a[4]),
        "o" => {
            let i = k(1) as usize;
            let (_, rows) = c.pulsar_observation_backing_mut(i / 6)?;
            set_observation(&mut rows[i % 6], a[2], k(3) as usize, a[4]);
        }
        "parameter" => {
            let cell = &mut c.pulsars[k(1) as usize].param.slots[label(a[2]) as usize].cells
                [k(3) as usize];
            cell.val = q(4);
            cell.param_set = k(5);
        }
        "link" => {
            let p = &mut c.pulsars[k(1) as usize].param.slots[label(a[2]) as usize];
            p.n_link_to = Some(k(3));
            p.link_to[0] = Some(label(a[4]) as i32);
        }
        "flag" => {
            let i = k(1) as usize;
            let (_, rows) = c.pulsar_observation_backing_mut(i / 6)?;
            let o = &mut rows[i % 6];
            let j = o.nFlags.unwrap() as usize;
            write_string(&mut o.flagID[j], a[2].as_bytes(), "flagID")?;
            write_string(&mut o.flagVal[j], a[3].as_bytes(), "flagVal")?;
            o.nFlags = Some(j as i32 + 1);
        }
        "shape_event" => {
            let p = &mut c.pulsars[k(1) as usize];
            let i = k(2) as usize;
            p.nTNShapeletEvents = Some(i as i32 + 1);
            p.TNShapeletEvFScale[i] = Some(n(3));
            p.TNShapeletEvN[i] = Some(k(4));
            p.TNShapeletEvPos[i] = Some(n(5));
            p.TNShapeletEvWidth[i] = Some(n(6));
            for j in 0..8 {
                p.TNShapeletEvCoef[i][j] = Some(n(7 + j));
            }
        }
        "bclt" | "bats" | "secular" | "init" | "clean" => {
            let tail = c.pulsars.split_off(k(1).max(0) as usize);
            let result = match a[0] {
                "bclt" => crate::propagation::calculate_bclt(c, root, &mut vec![]),
                "bats" => crate::propagation::form_bats(c),
                "secular" => crate::propagation::secular_motion(c),
                "init" => crate::refphs::refphs_init(c),
                _ => crate::refphs::refphs_clean(c),
            };
            c.pulsars.extend(tail);
            result?;
        }
        "dm" => crate::propagation::dm_delays(
            c,
            k(1) as usize,
            k(2) as usize,
            n(3),
            n(4),
            root,
            &mut vec![],
        )?,
        "shapiro" => {
            let (p, rows) = c.pulsar_observation_backing_mut(k(1) as usize)?;
            crate::propagation::shapiro_delay(p, &mut rows[k(2) as usize], n(3), n(4))?;
        }
        "precision" => crate::propagation::record_precision(
            &mut c.pulsars[0],
            q(1).to_f64(),
            a[2].as_bytes(),
            a[3].as_bytes(),
        ),
        "solar" => {
            let (p, rows) = c.pulsar_observation_backing_mut(k(1) as usize)?;
            let value =
                crate::solar_wind::solar_wind_model(p, &rows[k(2) as usize], root, &mut vec![])?;
            return Ok(vec![
                record(&value.to_le_bytes(), 8, true)["runs"][0][1].clone(),
            ]);
        }
        _ => return crate::propagation_helper_tests::evaluate(a),
    }
    Ok(vec![])
}
#[test]
#[ignore = "check_propagation.py supplies the full AB histories"]
fn ab_histories() {
    let path = PathBuf::from(std::env::var_os("AC_PROPAGATION_CASES").unwrap());
    let cases: Vec<Value> = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let mut differences = vec![];
    for case in cases {
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            s.source.max_observations = 6;
            s.diagnostics.source_directory = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("validation/propagation/build/ordinary/source/")
                .as_os_str()
                .as_encoded_bytes()
                .to_vec();
            Ok(())
        })
        .unwrap();
        let mut expected = Map::new();
        for change in case["events"][0]["state"]["changes"].as_array().unwrap() {
            expected.insert(change[0].as_str().unwrap().into(), change[1].clone());
        }
        let mut c = PulsarContext::unwritten(2);
        let views = (0..2)
            .map(|_| c.adopt_observations(vec![SourceObservation::default(); 6]))
            .collect::<Vec<_>>();
        state(&mut c, &mut expected, true, &views);
        let mut root = PathBuf::from(case["work"].as_str().unwrap()).join("root");
        let mut answer = Ok(vec![]);
        let (mut stdout, mut stderr) = (vec![], vec![]);
        let mut count = 0;
        for e in case["events"].as_array().unwrap() {
            let i = e["event"].as_u64().unwrap() as usize;
            let phase = e["phase"].as_str().unwrap();
            let a = case["actions"][i]
                .as_array()
                .unwrap()
                .iter()
                .map(|a| a.as_str().unwrap())
                .collect::<Vec<_>>();
            for v in e["state"]["changes"].as_array().unwrap() {
                expected.insert(v[0].as_str().unwrap().into(), v[1].clone());
            }
            let mut actual = Map::new();
            state(&mut c, &mut actual, false, &views);
            assert_eq!(
                actual.keys().collect::<Vec<_>>(),
                expected.keys().collect::<Vec<_>>(),
                "{} membership",
                case["id"]
            );
            assert_eq!(
                actual.len(),
                e["state"]["field_count"].as_u64().unwrap() as usize
            );
            for (key, a) in &actual {
                if a != &expected[key] {
                    differences.push(json!({"case":case["id"],"action":i,"phase":phase,"field":key,"rust":a,"native":expected[key]}));
                    count += 1;
                }
            }
            if phase == "begin" {
                let marker = format!("@BEGIN {i} {}\n", a[0]);
                stdout.extend(marker.as_bytes());
                stderr.extend(marker.as_bytes());
                answer = action(&mut c, &a, &mut root);
                crate::clkcorr::with_process(|s| {
                    for (err, b) in s.diagnostics.events.drain(..) {
                        if err {
                            stderr.extend(b)
                        } else {
                            stdout.extend(b)
                        }
                    }
                    Ok(())
                })
                .unwrap();
            } else {
                match (&answer, phase) {
                    (Ok(a), "end") => {
                        if json!(a) != e["result"] {
                            differences.push(json!({"case":case["id"],"action":i,"field":"result","rust":a,"native":e["result"]}));
                        }
                    }
                    (Err(_), "exit") => {}
                    _ => {
                        differences.push(json!({"case":case["id"],"action":i,"field":"disposition","rust":format!("{answer:?}"),"native":phase}));
                        if answer.is_err() {
                            break;
                        }
                    }
                }
                if phase == "end" {
                    let marker = format!("@END {i} {}\n", a[0]);
                    stdout.extend(marker.as_bytes());
                    stderr.extend(marker.as_bytes());
                }
            }
        }
        if case["status"] != 0 && answer.is_ok() {
            differences.push(json!({"case":case["id"],"field":"disposition","rust":"success","native":case["status"]}));
        }
        for (channel, data) in [("stdout", stdout), ("stderr", stderr)] {
            let actual = String::from_utf8(data).unwrap().replace(
                case["work"].as_str().unwrap(),
                case["original_root"].as_str().unwrap(),
            );
            let native = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("validation/propagation");
            let actual = actual
                .replace(
                    native.join("build/ordinary/source").to_str().unwrap(),
                    "{source}",
                )
                .replace(native.join("build/cases").to_str().unwrap(), "{cases}")
                .replace(native.to_str().unwrap(), "{propagation}");
            if actual != case[channel] {
                differences.push(
                    json!({"case":case["id"],"field":channel,"rust":actual,"native":case[channel]}),
                );
            }
        }
        fs::write(
            path.with_file_name("differences.json"),
            serde_json::to_vec_pretty(&differences).unwrap(),
        )
        .unwrap();
        eprintln!(
            "AB {}: {count} state differences; {} total",
            case["id"],
            differences.len()
        );
    }
    fs::write(
        path.with_file_name("differences.json"),
        serde_json::to_vec_pretty(&differences).unwrap(),
    )
    .unwrap();
    assert!(
        differences.is_empty(),
        "{} AB differences",
        differences.len()
    );
}
