//! O-only source storage probes; caller seeds are experimental inputs, not defaults.
use crate::initialise::{allocate_memory, initialise_parameters};
use crate::tempo2::{Label::*, PARAMETER_LABELS, PARAMETER_NAMES, ParameterStore, SourceParameter};
use crate::{PRECISION_BITS, copy_param};
use rug::Float;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{env, fs};

fn number(x: &Float) -> Value {
    if x.is_nan() {
        json!({"class":"NaN"})
    } else if x.is_infinite() {
        json!({"class":if x.is_sign_negative(){"-Infinity"}else{"+Infinity"}})
    } else {
        json!({"class":"finite","value":x.to_string_radix(10,None)})
    }
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn cstring(bytes: &[u8]) -> &str {
    std::str::from_utf8(&bytes[..bytes.iter().position(|b| *b == 0).unwrap()]).unwrap()
}
fn put(dst: &mut [u8], text: &str) {
    dst[..text.len()].copy_from_slice(text.as_bytes());
    dst[text.len()] = 0;
}
fn name(i: usize) -> String {
    PARAMETER_NAMES
        .get(i)
        .map_or_else(|| format!("unnamed_{i}"), |s| s.to_string())
}
fn flat(store: &ParameterStore, count: usize, natural: bool) -> Map<String, Value> {
    let mut out = Map::new();
    for (i, p) in store.slots.iter().take(count).enumerate() {
        let root = name(i);
        for (field, value) in [
            ("aSize", p.a_size),
            ("nLinkTo", p.n_link_to),
            ("nLinkFrom", p.n_link_from),
        ] {
            out.insert(format!("{root}.{field}"), json!(value));
        }
        for (j, c) in p.cells.iter().enumerate() {
            for (field, value) in [
                ("label", json!(cstring(&c.label))),
                ("shortlabel", json!(cstring(&c.shortlabel))),
                ("val", number(&c.val)),
                ("err", number(&c.err)),
                ("fitFlag", json!(c.fit_flag)),
                ("paramSet", json!(c.param_set)),
            ] {
                out.insert(format!("{root}.{j}.{field}"), value);
            }
            if !natural {
                for (field, x) in [("prefit", &c.prefit), ("prefitErr", &c.prefit_err)] {
                    out.insert(format!("{root}.{j}.{field}"), number(x.as_ref().unwrap()));
                }
            }
        }
    }
    out
}
fn snapshot(stores: &[ParameterStore]) -> Map<String, Value> {
    stores
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            flat(p, 166, false)
                .into_iter()
                .map(move |(k, v)| (format!("{i}.{k}"), v))
        })
        .collect()
}
fn kcase(actions: &[Value]) -> Vec<Value> {
    let mut stores = Vec::new();
    let mut events = Vec::new();
    for (i, action) in actions.iter().enumerate() {
        let a: Vec<&str> = action
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap())
            .collect();
        events.push(json!({"action":i,"op":a[0],"phase":"begin"}));
        let mut before = snapshot(&stores);
        let mut result = json!({});
        match a[0] {
            "init" => {
                stores = (0..a[1].parse::<usize>().unwrap())
                    .map(|_| initialise_parameters())
                    .collect();
                for p in &mut stores {
                    for c in p.slots.iter_mut().flat_map(|p| &mut p.cells) {
                        c.prefit = Some(Float::with_val(128, 17));
                        c.prefit_err = Some(Float::with_val(128, 19));
                    }
                }
                before = snapshot(&stores);
                result = json!({"MAX_PARAMS":2000});
            }
            "registry" => result = json!(flat(&stores[0], 2000, true)),
            "allocate" => result = json!(flat(&allocate_memory(a[1].parse().unwrap()), 2000, true)),
            "prior" => {
                let p = PARAMETER_NAMES.iter().position(|s| *s == a[1]).unwrap();
                let j = a[2].parse::<usize>().unwrap();
                let c = &mut stores[0].slots[p].cells[j];
                let val = |s: &str| Float::with_val(128, Float::parse(s).unwrap());
                c.val = val(a[3]);
                c.prefit = Some(val(a[4]));
                c.err = val(a[5]);
                c.prefit_err = Some(val(a[6]));
                c.fit_flag = a[7].parse().unwrap();
                c.param_set = a[8].parse().unwrap();
            }
            "copy" => {
                let src = PARAMETER_NAMES.iter().position(|s| *s == a[1]).unwrap();
                let dst = PARAMETER_NAMES.iter().position(|s| *s == a[2]).unwrap();
                assert!(src < dst);
                let (left, right) = stores[0].slots.split_at_mut(dst);
                copy_param(&left[src], &mut right[0]).unwrap();
            }
            "destroy" => {
                stores.clear();
                before.clear();
                result = json!({"matching_lifetime_complete":true});
            }
            _ => panic!("unprojected action"),
        }
        let after = snapshot(&stores);
        let changes: Map<String, Value> = after
            .iter()
            .filter(|(k, v)| before.get(*k) != Some(*v))
            .map(|(k, v)| (k.clone(), json!({"before":before.get(k),"after":v})))
            .collect();
        events.push(json!({"action":i,"op":a[0],"phase":"end","result":result,"changes":changes}));
    }
    events
}
fn seeded(seed: i32, count: i32) -> SourceParameter {
    let mut p = SourceParameter::allocated(4, 64);
    p.a_size = Some(count);
    p.n_link_to = Some(1);
    p.n_link_from = Some(2);
    for i in 0..5 {
        p.link_to[i] = Some(seed + i as i32);
        p.link_from[i] = Some(seed + 10 + i as i32);
    }
    for (i, c) in p.cells.iter_mut().enumerate() {
        let v = seed + i as i32;
        c.val = Float::with_val(128, f64::from(v) + 0.125);
        c.err = Float::with_val(128, f64::from(v) + 0.25);
        c.prefit = Some(Float::with_val(128, f64::from(v) + 0.5));
        c.prefit_err = Some(Float::with_val(128, f64::from(v) + 0.75));
        c.fit_flag = v;
        c.param_set = v + 4;
        put(&mut c.label, &format!("label-{seed}-{i}"));
        put(&mut c.shortlabel, &format!("short-{seed}-{i}"));
    }
    p
}
fn state(p: &SourceParameter) -> Value {
    json!({"aSize":p.a_size,"capacity":p.cells.len(),"nLinkTo":p.n_link_to,"nLinkFrom":p.n_link_from,"linkTo":p.link_to,"linkFrom":p.link_from,
        "cells":p.cells.iter().map(|c|json!({"val":number(&c.val),"err":number(&c.err),"prefit":c.prefit.as_ref().map(number),"prefitErr":c.prefit_err.as_ref().map(number),"fitFlag":c.fit_flag,"paramSet":c.param_set,"label_bytes":hex(&c.label),"shortlabel_bytes":hex(&c.shortlabel)})).collect::<Vec<_>>()})
}
fn mcopy(count: i32, short: bool) -> Value {
    let mut src = seeded(10, count);
    let mut dst = seeded(70, 4);
    if short {
        put(&mut src.cells[0].label, "x");
        put(&mut src.cells[0].shortlabel, "y");
    }
    let sb = state(&src);
    let db = state(&dst);
    let ptr = dst.cells.as_ptr();
    let lp: Vec<_> = dst
        .cells
        .iter()
        .map(|c| (c.label.as_ptr(), c.shortlabel.as_ptr()))
        .collect();
    copy_param(&src, &mut dst).unwrap();
    assert_eq!(ptr, dst.cells.as_ptr());
    assert_eq!(
        lp,
        dst.cells
            .iter()
            .map(|c| (c.label.as_ptr(), c.shortlabel.as_ptr()))
            .collect::<Vec<_>>()
    );
    json!({"source_before":sb,"source_after":state(&src),"destination_before":db,"destination_after":state(&dst),"pointers_retained":true})
}

#[test]
fn initialization_ownership_and_unwritten_fields() {
    let p = initialise_parameters();
    assert_eq!(p.slots.len(), 2000);
    assert_eq!(p.slots.iter().map(|p| p.cells.len()).sum::<usize>(), 9616);
    for c in p.slots.iter().flat_map(|p| &p.cells) {
        assert_eq!(c.val.prec(), 128);
        assert!(c.prefit.is_none() && c.prefit_err.is_none());
        assert_eq!(c.val, 0);
        assert_eq!(c.err, 0);
    }
    for s in &p.slots {
        assert_eq!(s.link_to, [None; 5]);
        assert_eq!(s.link_from, [None; 5]);
    }
    assert_eq!(
        cstring(&p.slots[param_daop as usize].cells[0].label),
        "IPERHARM"
    );
    assert_eq!(
        cstring(&p.slots[param_iperharm as usize].cells[0].label),
        "IPERHARM"
    );
    // Sequential strcpy retained the former D_AOP suffix after IPERHARM's NUL.
    assert_eq!(
        &p.slots[param_daop as usize].cells[0].label[9..16],
        b" (kpc)\0"
    );
    let mut cloned = p.clone();
    cloned.slots[2].cells[0].val = Float::with_val(128, 7);
    drop(p);
    assert_eq!(cloned.slots[2].cells[0].val, 7);
}

fn boundaries() -> Vec<Value> {
    let mut out = Vec::new();
    for kind in [
        "source_cell",
        "destination_cell",
        "label_nul",
        "shortlabel_nul",
        "label_capacity",
        "shortlabel_capacity",
        "prefit",
        "prefitErr",
    ] {
        let mut src = seeded(10, 3);
        let mut dst = seeded(70, 4);
        match kind {
            "source_cell" => src.cells.truncate(1),
            "destination_cell" => dst.cells.truncate(1),
            "label_nul" => src.cells[1].label = vec![b'z'; 3],
            "shortlabel_nul" => src.cells[1].shortlabel = vec![b'z'; 3],
            "label_capacity" => dst.cells[1].label = vec![b'q'; 2],
            "shortlabel_capacity" => dst.cells[1].shortlabel = vec![b'q'; 2],
            "prefit" => src.cells[1].prefit = None,
            "prefitErr" => src.cells[1].prefit_err = None,
            _ => unreachable!(),
        }
        let sb = state(&src);
        let before = state(&dst);
        let error = copy_param(&src, &mut dst).unwrap_err();
        let after = state(&dst);
        assert_eq!(error.cell, 1);
        assert_eq!(dst.a_size, Some(4));
        assert_eq!(state(&src), sb);
        assert_eq!(after["cells"][0], sb["cells"][0]);
        assert_eq!(dst.cell(0).unwrap_err(), error);
        assert_eq!(copy_param(&seeded(90, 1), &mut dst).unwrap_err(), error);
        assert_eq!(state(&dst), after);
        out.push(json!({"case":kind,"error":error,"before":before,"after":after,"source":sb,"reentry":"same boundary"}));
    }
    out
}
#[test]
fn operation_local_boundaries_and_terminal_destination() {
    boundaries();
}
#[test]
fn signed_counts_flags_nonfinite_and_zero_copy() {
    for n in [-7, 0, 1, 3, 4] {
        mcopy(n, false);
    }
    let mut src = seeded(10, 1);
    let mut dst = seeded(70, 4);
    src.cells[0].val = Float::with_val(PRECISION_BITS, -0.0);
    src.cells[0].err = Float::with_val(PRECISION_BITS, f64::INFINITY);
    src.cells[0].prefit = Some(Float::with_val(128, f64::NAN));
    src.cells[0].fit_flag = -123;
    src.cells[0].param_set = i32::MIN;
    copy_param(&src, &mut dst).unwrap();
    assert!(dst.cells[0].val.is_zero() && dst.cells[0].val.is_sign_negative());
    assert!(dst.cells[0].err.is_infinite());
    assert!(dst.cells[0].prefit.as_ref().unwrap().is_nan());
    assert_eq!(dst.cells[0].fit_flag, -123);
    assert_eq!(dst.cells[0].param_set, i32::MIN);
}
#[test]
fn canonical_store_drives_evaluation_and_snapshots_are_detached() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting/inputs");
    let mut input = crate::load(&root.join("all.par"), &root.join("bary.tim")).unwrap();
    let original = crate::evaluate_barycentric(&input).unwrap();
    let mut snapshot = input.parameters();
    snapshot.get_mut("F0").unwrap().value = "999".parse().unwrap();
    assert_ne!(snapshot["F0"].value, input.parameters()["F0"].value);
    assert!(input.parameter_cell("F0").prefit.is_none());
    assert_eq!(input.parameter_cell("F0").err, 0);
    let before = input.parameter_cell("F0").val.clone();
    crate::std_freq::update_std_freq(&mut input, 0, 1e-6, 0.75);
    assert_ne!(
        before,
        input.source.pulsars[0]
            .param
            .parameter(param_f)
            .cell(0)
            .unwrap()
            .value()
            .clone()
    );
    assert_eq!(
        input.parameters()["F0"].value.0,
        input.parameter_cell("F0").val
    );
    assert_eq!(input.parameter_cell("F0").err, 0.75);
    let updated = crate::evaluate_barycentric(&input).unwrap();
    assert_ne!(
        original.observations[1].residual_s,
        updated.observations[1].residual_s
    );
    assert_eq!(
        serde_json::to_value(&updated.input).unwrap()["parameters"],
        serde_json::to_value(input.parameters()).unwrap()
    );
}

#[test]
#[ignore = "O runner supplies explicit actions and fresh launch identity"]
fn probe() {
    let bytes = fs::read(env::var("PARAMETER_INPUT").unwrap()).unwrap();
    let cases: Vec<Value> = serde_json::from_slice(&bytes).unwrap();
    let mut results = Vec::new();
    for case in cases {
        let actions = case["actions"].as_array().unwrap();
        let events = if case["reference"] == "parameters" {
            kcase(actions)
        } else {
            let count = actions[0][1].as_str().unwrap().parse().unwrap();
            vec![
                json!({"action":0,"op":"copyparam","phase":"begin","stdout_position":0}),
                json!({"action":0,"op":"copyparam","phase":"end","stdout_position":0,"result":mcopy(count,false)}),
            ]
        };
        results.push(json!({"id":case["id"],"actions":actions,"events":events}));
    }
    let store = initialise_parameters();
    let registry:Vec<_>=store.slots.iter().enumerate().map(|(i,p)|json!({"enum_index":PARAMETER_LABELS.get(i).map_or(i as i32, |label| *label as i32),"name":name(i),"aSize":p.a_size,"unwritten":["prefit","prefitErr","linkTo entries beyond nLinkTo","linkFrom entries beyond nLinkFrom"]})).collect();
    let output = json!({"token":env::var("PARAMETER_TOKEN").unwrap(),"input_sha256":format!("{:x}",Sha256::digest(&bytes)),"cases":results,"registry":registry,"boundaries":boundaries(),"supplemental":[mcopy(-7,false),mcopy(1,true)],"initialized_bytes":store.slots.iter().map(state).collect::<Vec<_>>()});
    fs::write(
        env::var("PARAMETER_OUTPUT").unwrap(),
        serde_json::to_vec(&output).unwrap(),
    )
    .unwrap();
}
