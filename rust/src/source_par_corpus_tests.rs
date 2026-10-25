//! Q initialized-byte projection; no fixture value becomes a production default.
#![allow(non_snake_case)]
use crate::{
    TimingError,
    pulsar::{Constraint, SeriesType, SourcePulsar},
    pulsar_lifecycle::PulsarContext,
    source_par as par,
    source_text::SourceText,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
type Map = BTreeMap<String, Value>;
use crate::corpus_wire::{Wire, decode, encode, field};
include!("source_par_corpus_fields.rs");
fn state(c: &mut PulsarContext, s: &mut Map, load: bool) {
    for (z, p) in c.pulsars.iter_mut().enumerate() {
        let root = format!("p{z}.");
        fixed_fields(p, &root, s, load);
        for (i, param) in p.param.slots.iter_mut().enumerate() {
            let k = format!("{root}param[{i}].");
            if !load {
                s.insert(k.clone() + "aSize", json!(param.a_size.unwrap()));
            }
            field(&mut param.n_link_to, &(k.clone() + "nLinkTo"), s, load);
            field(&mut param.n_link_from, &(k.clone() + "nLinkFrom"), s, load);
            field(&mut param.link_to, &(k.clone() + "linkTo"), s, load);
            field(&mut param.link_from, &(k.clone() + "linkFrom"), s, load);
            for name in [
                "val",
                "prefit",
                "err",
                "prefitErr",
                "fitFlag",
                "paramSet",
                "labels",
                "shortlabels",
            ] {
                let key = k.clone() + name;
                let raw = if load { decode(&s[&key]) } else { vec![] };
                let mut remaining = raw.as_slice();
                let mut bytes = vec![];
                for cell in &mut param.cells {
                    macro_rules! cell {
                        ($x:expr) => {
                            if load {
                                *$x = Wire::take(&mut remaining);
                            } else {
                                $x.put(&mut bytes);
                            }
                        };
                    }
                    match name {
                        "val" => cell!(&mut cell.val),
                        "prefit" => cell!(&mut cell.prefit),
                        "err" => cell!(&mut cell.err),
                        "prefitErr" => cell!(&mut cell.prefit_err),
                        "fitFlag" => cell!(&mut cell.fit_flag),
                        "paramSet" => cell!(&mut cell.param_set),
                        "labels" | "shortlabels" => {
                            let x = if name == "labels" {
                                &mut cell.label
                            } else {
                                &mut cell.shortlabel
                            };
                            if load {
                                let (a, b) = remaining.split_at(x.len());
                                x.copy_from_slice(a);
                                remaining = b;
                            } else {
                                bytes.extend(x.iter());
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                if load {
                    assert!(remaining.is_empty(), "{key}");
                } else {
                    s.insert(key, encode(&bytes));
                }
            }
        }
    }
    if !load {
        for (z, p) in c.pulsars.iter().enumerate() {
            for (i, owner) in p.constraint_special.iter().enumerate() {
                if let Some(Some(owner)) = owner {
                    let b = c.payload(*owner).unwrap();
                    assert_eq!(b.last(), Some(&None));
                    let known: Vec<_> = b[..b.len() - 1].iter().map(|x| x.unwrap()).collect();
                    s.insert(format!("p{z}.constraint_special[{i}]"),json!({"allocated":b.len(),"initialized_prefix":encode(&known),"unwritten_tail":1}));
                }
            }
        }
        crate::clkcorr::with_process(|session| {
            let d = &session.diagnostics;
            s.insert("TK_errorCount".into(), json!(d.errors));
            s.insert("TK_warnCount".into(), json!(d.warnings));
            for (name, log) in [
                ("TK_errorlog", &d.error_log),
                ("TK_warnlog", &d.warning_log),
            ] {
                let mut b = vec![0; 10800];
                for (i, line) in log.iter().enumerate() {
                    b[360 * i..360 * i + line.len()].copy_from_slice(line);
                }
                s.insert(name.into(), encode(&b));
            }
            Ok(())
        })
        .unwrap();
    }
}
fn compare(c: &mut PulsarContext, expected: &Map, label: &str, failures: &mut Vec<String>) {
    let mut actual = Map::new();
    state(c, &mut actual, false);
    let expected: Map = expected
        .iter()
        .filter(|(k, _)| !k.starts_with("stream."))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    if actual.keys().collect::<Vec<_>>() != expected.keys().collect::<Vec<_>>() {
        failures.push(format!("{label}: state membership"));
    }
    for (k, v) in actual {
        if expected.get(&k) != Some(&v) {
            let want = expected.get(&k).unwrap_or(&Value::Null);
            let first = if v.get("size").is_some() && want.get("size").is_some() {
                decode(&v)
                    .iter()
                    .zip(decode(want))
                    .position(|(a, b)| *a != b)
            } else {
                None
            };
            failures.push(format!("{label}: {k}, first byte {first:?}"));
        }
    }
}
fn action(
    c: &mut PulsarContext,
    a: &[String],
    root: &Path,
    ecl: &mut par::EclipticParameters,
) -> Result<Value, TimingError> {
    let k = |i: usize| a[i].parse::<usize>().unwrap();
    match a[0].as_str() {
        "dispatch" => {
            let mut s = SourceText::new(Path::new("Q-dispatch"), a[2].as_bytes().to_vec());
            while let Some(key) = s.token() {
                par::check_line(c, k(1), &key, &mut s, ecl, root)?;
            }
            Ok(json!({"position":s.position,"remaining":""}))
        }
        "simple" => {
            let path = Path::new(&a[2]);
            let mut s = SourceText::new(path, std::fs::read(path).unwrap());
            par::read_simple_parfile(c, k(1), &mut s, root)?;
            Ok(json!({"status":0,"position":s.position,"remaining":""}))
        }
        "full" => {
            let pairs: Vec<_> = a[2..]
                .iter()
                .map(|p| crate::bats::InputPair {
                    par: p.into(),
                    tim: "".into(),
                })
                .collect();
            par::read_parfiles(c, &pairs, root, &mut vec![])?;
            Ok(Value::Null)
        }
        "global" => {
            par::read_parfile_global(c, Path::new(&a[2]), root, &mut vec![])?;
            Ok(Value::Null)
        }
        "set" => {
            let p = &mut c.pulsars[k(1)];
            let value = a[3].parse::<i32>();
            macro_rules! set{($($f:ident),*)=>{match a[2].as_str(){$(stringify!($f)=>p.$f=Some(value.unwrap()),)*"jumpSAT1"=>p.jumpSAT[1]=Some(value.unwrap() as u8),"TNsubtractPoly"=>p.TNsubtractPoly=Some(u64value(&a[3])),"gwecc_paramSet"=>{let cell=&mut p.param.slots[crate::tempo2::Label::param_gwecc as usize].cells[0];cell.param_set=value.unwrap();cell.label[..13].copy_from_slice(b"CALLER_GWECC\0");cell.shortlabel[..13].copy_from_slice(b"CALLER_GWECC\0");},_=>panic!("Q set {}",a[2])}}}
            set!(
                nJumps,
                nfdJumps,
                nToffset,
                dmoffsDMnum,
                dmoffsCMnum,
                ne_sw_ifuncN,
                nWhite,
                nWhite_dm,
                nconstraints,
                nT2efac,
                nT2equad,
                nTNEF,
                nTNEQ,
                nTNSQ,
                nTNECORR,
                nTNSECORR,
                nTNBandNoise,
                nTNGroupNoise,
                nDMEvents,
                nTNShapeletEvents,
                nPhaseJump
            );
            Ok(Value::Null)
        }
        "helpers" => {
            use crate::t2fit_nestlike::*;
            let v = u64value(&a[1]);
            let (r, d, ch) = (
                a[2].parse().unwrap(),
                a[3].parse().unwrap(),
                a[4].parse().unwrap(),
            );
            Ok(
                json!({"flags":t2DecodeTNsubtractPolyFlags(v),"red":t2DecodeTNRedSubPolyOrd(v),"dm":t2DecodeTNDMSubPolyOrd(v),"chrom":t2DecodeTNChromSubPolyOrd(v),"encode":t2EncodeTNsubtractPoly(v,r,d,ch),"set_red":t2SetTNRedSubPolyOrd(v,r),"set_dm":t2SetTNDMSubPolyOrd(v,d),"set_chrom":t2SetTNChromSubPolyOrd(v,ch)}),
            )
        }
        "route" => crate::clkcorr::with_resources(root, |s| {
            let mut o = crate::observation::SourceObservation::zeroed();
            o.sat = Some(crate::source_text::extended(a[1].as_bytes())?);
            s.get_clock_corrections(&mut o, Some(a[2].as_bytes()), Some(a[3].as_bytes()), 0)?;
            let mut map = serde_json::Map::new();
            let count = o.nclock_correction.unwrap();
            map.insert("count".into(), json!(count));
            for i in 0..count as usize {
                let x = &o.correctionsTT[i];
                map.insert(format!("{i}.correction"),json!({"type":"binary64","class":"finite","bits_le":x.correction.unwrap().to_le_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>()}));
                map.insert(
                    format!("{i}.to"),
                    json!(
                        String::from_utf8(
                            crate::observation::read_string(&x.corrects_to, "clock").unwrap()
                        )
                        .unwrap()
                    ),
                );
            }
            Ok(Value::Object(map))
        }),
        _ => panic!("Q action {}", a[0]),
    }
}
fn u64value(s: &str) -> u64 {
    if let Some(h) = s.strip_prefix("0x") {
        u64::from_str_radix(h, 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}
#[test]
#[ignore = "check_par.py supplies accepted Q corpus"]
fn q_corpus() {
    let work = std::path::PathBuf::from(std::env::var_os("AC_PAR_CASES").unwrap());
    let initial: Map =
        serde_json::from_slice(&std::fs::read(work.join("initial.json")).unwrap()).unwrap();
    let cases: Value =
        serde_json::from_slice(&std::fs::read(work.join("cases.json")).unwrap()).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../validation/par_specialized/build/runtime")
        .canonicalize()
        .unwrap();
    let mut failures = vec![];
    let mut diagnostic_differences = vec![];
    let source = root.parent().unwrap().join("probe_source");
    for case in cases.as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            s.diagnostics.source_directory = format!("{}/", source.display()).into_bytes();
            Ok(())
        })
        .unwrap();
        let mut c = PulsarContext::unwritten(2);
        c.initialise(2, 0, 0).unwrap();
        let mut expected = initial.clone();
        state(&mut c, &mut expected, true);
        let mut ecl = par::EclipticParameters::default();
        let mut returned_error = false;
        for (i, a) in case["actions"].as_array().unwrap().iter().enumerate() {
            let args: Vec<String> = a
                .as_array()
                .unwrap()
                .iter()
                .map(|s| {
                    s.as_str().unwrap().replace(
                        "{case}",
                        root.parent()
                            .unwrap()
                            .join("cases")
                            .join(id)
                            .to_str()
                            .unwrap(),
                    )
                })
                .collect();
            let result = action(&mut c, &args, &root, &mut ecl);
            let event = case["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["phase"] == "end" && e["action"] == i);
            if let Some(event) = event {
                for (k, d) in event["changes"].as_object().unwrap() {
                    if d["after"].is_null() {
                        expected.remove(k);
                    } else {
                        expected.insert(k.clone(), d["after"].clone());
                    }
                }
                match result {
                    Ok(result) => {
                        if result != event["result"] {
                            failures
                                .push(format!("{id} {i} result: {result} != {}", event["result"]));
                        }
                        compare(&mut c, &expected, &format!("{id} {i}"), &mut failures);
                    }
                    Err(e) => {
                        failures.push(format!("{id} {i} unexpected error: {e}"));
                        returned_error = true;
                        break;
                    }
                }
            } else {
                assert_ne!(case["returncode"], 0);
                match result {
                    Ok(_) => failures.push(format!("{id} {i} missing expected source exit")),
                    Err(e) => {
                        if case["expected_boundary"] == true {
                            assert!(e.to_string().contains("capacity"), "{id}: {e}");
                            assert_eq!(e.path, Path::new("Q-dispatch"));
                            assert!(e.line.is_some());
                            assert!(
                                e.message
                                    .contains(args[2].split_whitespace().next().unwrap())
                            );
                            compare(
                                &mut c,
                                &expected,
                                &format!("{id} retained prefix"),
                                &mut failures,
                            );
                        } else {
                            let exit = case["events"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|e| e["phase"] == "exit_state")
                                .unwrap();
                            for (k, d) in exit["changes"].as_object().unwrap() {
                                if d["after"].is_null() {
                                    expected.remove(k);
                                } else {
                                    expected.insert(k.clone(), d["after"].clone());
                                }
                            }
                            compare(
                                &mut c,
                                &expected,
                                &format!("{id} source exit prefix"),
                                &mut failures,
                            );
                        }
                    }
                }
                returned_error = true;
                break;
            }
        }
        for (channel, actual) in ["stdout", "stderr"]
            .into_iter()
            .zip(crate::corpus_wire::diagnostic_streams())
        {
            let actual = actual
                .replace(source.to_str().unwrap(), "{source}")
                .replace(root.parent().unwrap().to_str().unwrap(), "{build}");
            if case["expected_boundary"] != true && actual != case[channel] {
                diagnostic_differences.push(
                    json!({"case":id,"channel":channel,"rust":actual,"native":case[channel]}),
                );
            }
        }
        eprintln!("Q {id}: {} cumulative discrepancies", failures.len());
        if case["returncode"] != 0 && !returned_error {
            failures.push(format!("{id} missing source exit"));
        }
    }
    std::fs::write(
        work.join("differences.json"),
        serde_json::to_vec_pretty(&failures).unwrap(),
    )
    .unwrap();
    std::fs::write(
        work.join("diagnostic-differences.json"),
        serde_json::to_vec_pretty(&diagnostic_differences).unwrap(),
    )
    .unwrap();
    assert!(
        diagnostic_differences.is_empty(),
        "{} Q diagnostic differences",
        diagnostic_differences.len()
    );
    assert!(
        failures.is_empty(),
        "{} differences; first {:?}",
        failures.len(),
        &failures[..failures.len().min(5)]
    );
}
