//! K's initialized caller and complete snapshots, kept separate from production defaults.
#![allow(non_snake_case)]
use super::*;
use crate::{
    pulsar_lifecycle::PulsarContext,
    tempo2::{PARAMETER_LABELS, SourceParameter},
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
type Map = BTreeMap<String, Value>;
trait Wire {
    fn wire(&self) -> Value;
}
macro_rules! integers { ($($t:ty),*) => {$(impl Wire for $t { fn wire(&self)->Value {json!(self)} })*} }
integers!(u8, i32, u64);
impl<T: Wire> Wire for Option<T> {
    fn wire(&self) -> Value {
        self.as_ref().map(Wire::wire).unwrap_or(Value::Null)
    }
}
impl Wire for crate::pulsar::SeriesType {
    fn wire(&self) -> Value {
        json!(*self as i32)
    }
}
impl<const N: usize> Wire for [Option<u8>; N] {
    fn wire(&self) -> Value {
        json!(String::from_utf8(read_string(self, "K string").unwrap()).unwrap())
    }
}
impl<const N: usize> Wire for Box<[Option<u8>; N]> {
    fn wire(&self) -> Value {
        (**self).wire()
    }
}
fn class(x: f64) -> &'static str {
    if x.is_nan() {
        "NaN"
    } else if x.is_infinite() {
        if x.is_sign_negative() {
            "-Infinity"
        } else {
            "+Infinity"
        }
    } else {
        "finite"
    }
}
impl Wire for f64 {
    fn wire(&self) -> Value {
        json!({"type":"binary64","class":class(*self),"bits":format!("{:016x}",self.to_bits())})
    }
}
impl Wire for Float {
    fn wire(&self) -> Value {
        let class = if self.is_nan() {
            "NaN"
        } else if self.is_infinite() {
            if self.is_sign_negative() {
                "-Infinity"
            } else {
                "+Infinity"
            }
        } else {
            "finite"
        };
        if class == "finite" {
            json!({"type":"binary128","class":class,"value":self.to_string_radix(10,None)})
        } else {
            json!({"type":"binary128","class":class})
        }
    }
}
include!("source_parameter_controls.rs");
fn parameter(p: &SourceParameter, root: &str, s: &mut Map, natural: bool) {
    s.insert(format!("{root}aSize"), p.a_size.wire());
    s.insert(format!("{root}nLinkTo"), p.n_link_to.wire());
    s.insert(format!("{root}nLinkFrom"), p.n_link_from.wire());
    for (name, n, a) in [
        ("linkTo", p.n_link_to, &p.link_to),
        ("linkFrom", p.n_link_from, &p.link_from),
    ] {
        for (i, value) in a.iter().enumerate().take(n.unwrap() as usize) {
            s.insert(format!("{root}{name}[{i}]"), value.wire());
        }
    }
    for (i, c) in p.cells.iter().take(p.a_size.unwrap() as usize).enumerate() {
        let k = format!("{root}{i}.");
        for (name, v) in [
            ("val", c.val.wire()),
            ("err", c.err.wire()),
            ("fitFlag", c.fit_flag.wire()),
            ("paramSet", c.param_set.wire()),
            (
                "label",
                json!(
                    String::from_utf8(c.label.iter().copied().take_while(|&b| b != 0).collect())
                        .unwrap()
                ),
            ),
            (
                "shortlabel",
                json!(
                    String::from_utf8(
                        c.shortlabel
                            .iter()
                            .copied()
                            .take_while(|&b| b != 0)
                            .collect()
                    )
                    .unwrap()
                ),
            ),
        ] {
            s.insert(k.clone() + name, v);
        }
        if !natural {
            s.insert(k.clone() + "prefit", c.prefit.wire());
            s.insert(k + "prefitErr", c.prefit_err.wire());
        }
    }
}
fn snapshot(c: &PulsarContext) -> Map {
    let mut s = Map::new();
    crate::clkcorr::with_process(|p| {
        s.insert("global.TK_errorCount".into(), json!(p.diagnostics.errors));
        s.insert("global.TK_warnCount".into(), json!(p.diagnostics.warnings));
        Ok(())
    })
    .unwrap();
    for (z, p) in c.pulsars.iter().enumerate() {
        let root = format!("{z}.");
        controls(p, &mut s, &root);
        for (i, name) in PARAMETER_LABELS.iter().enumerate() {
            parameter(
                &p.param.slots[i],
                &format!("{root}{name:?}."),
                &mut s,
                false,
            );
        }
    }
    s
}
fn scratch(e: &EclipticParameters) -> Map {
    let mut s = Map::new();
    for (name, c) in [("param_raj", &e.longitude), ("param_decj", &e.latitude)] {
        let mut p = SourceParameter::allocated(1, 100);
        p.cells[0] = c.clone();
        parameter(&p, &format!("{name}."), &mut s, false);
    }
    s
}
fn seed(p: &mut SourcePulsar) {
    for par in &mut p.param.slots {
        for c in &mut par.cells {
            c.prefit = Some(Float::with_val(PRECISION_BITS, 17));
            c.prefit_err = Some(Float::with_val(PRECISION_BITS, 19));
        }
    }
    for (dest, value) in [
        (&mut p.clock[..], "CALLER_CLOCK"),
        (&mut p.ephemeris[..], "CALLER_EPHEM"),
        (&mut p.JPL_EPHEMERIS[..], "CALLER_PATH"),
        (&mut p.rajStrPre[..], "CALLER_RA_PRE"),
        (&mut p.rajStrPost[..], "CALLER_RA_POST"),
        (&mut p.decjStrPre[..], "CALLER_DEC_PRE"),
        (&mut p.decjStrPost[..], "CALLER_DEC_POST"),
    ] {
        write_string(dest, value.as_bytes(), "K seed").unwrap();
    }
    p.ne_sw_ifuncN = Some(117);
    p.nchromx = Some(0);
    p.waveScale = Some(23.0);
    p.setTelVelX = Some(7);
    p.setTelVelY = Some(7);
    p.setTelVelZ = Some(7);
}
fn apply(s: &mut Map, changes: &Value) {
    for (k, v) in changes.as_object().unwrap() {
        if v["after"].is_null() {
            s.remove(k);
        } else {
            s.insert(k.clone(), v["after"].clone());
        }
    }
}
fn matches(a: &Value, b: &Value) -> bool {
    if a.get("type") == Some(&json!("binary128")) && b.get("type") == a.get("type") {
        if a["class"] != b["class"] {
            return false;
        }
        if a["class"] != "finite" {
            return true;
        }
        let a = Float::with_val(113, Float::parse(a["value"].as_str().unwrap()).unwrap());
        let b = Float::with_val(113, Float::parse(b["value"].as_str().unwrap()).unwrap());
        return a == b && a.is_sign_negative() == b.is_sign_negative();
    }
    if a.get("class") == Some(&json!("NaN")) && b.get("class") == a.get("class") {
        return a["type"] == b["type"];
    }
    if let (Some(a), Some(b)) = (a.as_object(), b.as_object()) {
        return a.len() == b.len()
            && a.iter()
                .all(|(k, a)| b.get(k).is_some_and(|b| matches(a, b)));
    }
    a == b
}
// Decision 0073: compare decoded engine values, never printed decimal reals.
fn continuous(a: &Value, b: &Value, units: u32) -> bool {
    if a["type"] != "binary128"
        || b["type"] != "binary128"
        || a["class"] != "finite"
        || b["class"] != "finite"
    {
        return false;
    }
    let a = Float::with_val(128, Float::parse(a["value"].as_str().unwrap()).unwrap());
    let b = Float::with_val(113, Float::parse(b["value"].as_str().unwrap()).unwrap());
    if a.is_sign_negative() != b.is_sign_negative() {
        return false;
    }
    if a.is_zero() || b.is_zero() {
        return a == b;
    }
    let minimum = Float::with_val(256, 1) >> 16382;
    let a = Float::with_val(256, a);
    let b = Float::with_val(256, b);
    if a.clone().abs() < minimum || b.clone().abs() < minimum {
        return false;
    }
    let scale = a.clone().abs().max(&b.clone().abs());
    (a - b).abs() <= ((scale * units) >> 113)
}
fn checked_match(
    a: &Value,
    b: &Value,
    units: Option<u32>,
    tag: &str,
    qualified: &mut Vec<Value>,
) -> bool {
    if matches(a, b) {
        return true;
    }
    if let Some(units) = units
        && continuous(a, b, units)
    {
        qualified.push(json!({"location":tag,"native":b,"rust":a,"bound_u113":units,"disposition":"0073 continuous rounding; exact113 projection differs"}));
        return true;
    }
    false
}
fn scaling_policy(case: &str, key: &[u8], input: &str, rules: &mut BTreeMap<String, u32>) {
    if case != "threshold_scaling" {
        return;
    }
    let name = match key {
        b"PBDOT" => "pbdot",
        b"XPBDOT" => "xpbdot",
        b"A1DOT" | b"XDOT" => "a1dot",
        b"EPS1DOT" => "eps1dot",
        b"EPS2DOT" => "eps2dot",
        b"EDOT" | b"ECCDOT" => "edot",
        _ => return,
    };
    let scaled = Float::with_val(113, Float::parse(input).unwrap()).abs() > 1e-7f64;
    for field in ["val", "prefit"] {
        if name == "edot" && field == "prefit" {
            continue;
        }
        let field = format!("0.param_{name}.0.{field}");
        if scaled {
            rules.insert(field, 4);
        } else {
            rules.remove(&field);
        }
    }
}
fn compare(
    actual: &Map,
    expected: &Map,
    tag: &str,
    rules: &BTreeMap<String, u32>,
    qualified: &mut Vec<Value>,
    failures: &mut Vec<String>,
) {
    if actual.keys().collect::<Vec<_>>() != expected.keys().collect::<Vec<_>>() {
        failures.push(format!(
            "{tag} state membership: missing {:?}, extra {:?}",
            expected
                .keys()
                .filter(|k| !actual.contains_key(*k))
                .collect::<Vec<_>>(),
            actual
                .keys()
                .filter(|k| !expected.contains_key(*k))
                .collect::<Vec<_>>()
        ));
    }
    for (k, e) in expected {
        if let Some(a) = actual.get(k)
            && !checked_match(
                a,
                e,
                rules.get(k).copied(),
                &format!("{tag} {k}"),
                qualified,
            )
        {
            failures.push(format!("{tag} {k}: {a} != {e}"));
        }
    }
}

fn next(s: &mut SourceText) -> String {
    let pos = s.position;
    let n = s.token();
    s.position = pos;
    n.map(|v| String::from_utf8(v).unwrap())
        .unwrap_or("<EOF>".into())
}
fn label(s: &str) -> Label {
    *PARAMETER_LABELS
        .iter()
        .find(|n| format!("{n:?}") == s)
        .unwrap()
}
fn initial(registry: &Map, n: usize, controls: &Value) -> Map {
    let mut s = Map::new();
    for z in 0..n {
        for (k, v) in registry {
            if !k.starts_with("unnamed_") {
                s.insert(format!("{z}.{k}"), v.clone());
                if k.ends_with(".val") {
                    let root = k.strip_suffix("val").unwrap();
                    s.insert(
                        format!("{z}.{root}prefit"),
                        json!({"type":"binary128","class":"finite","value":"17"}),
                    );
                    s.insert(
                        format!("{z}.{root}prefitErr"),
                        json!({"type":"binary128","class":"finite","value":"19"}),
                    );
                }
            }
        }
        for (k, v) in controls.as_object().unwrap() {
            if !k.starts_with("caller_") {
                s.insert(format!("{z}.{k}"), v.clone());
            }
        }
        for (k, v) in [
            ("clock", json!("CALLER_CLOCK")),
            ("ephemeris", json!("CALLER_EPHEM")),
            ("JPL_EPHEMERIS", json!("CALLER_PATH")),
            ("rajStrPre", json!("CALLER_RA_PRE")),
            ("rajStrPost", json!("CALLER_RA_POST")),
            ("decjStrPre", json!("CALLER_DEC_PRE")),
            ("decjStrPost", json!("CALLER_DEC_POST")),
            ("ne_sw_ifuncN", json!(117)),
            ("nchromx", json!(0)),
            ("clockFromOverride", json!("")),
            ("waveScale", 23.0f64.wire()),
            ("setTelVelX", json!(7)),
            ("setTelVelY", json!(7)),
            ("setTelVelZ", json!(7)),
        ] {
            s.insert(format!("{z}.{k}"), v);
        }
    }
    s.insert("global.TK_errorCount".into(), json!(0));
    s.insert("global.TK_warnCount".into(), json!(0));
    s
}
#[test]
#[ignore = "read-only K corpus adapter"]
fn k_corpus() {
    let work = std::path::PathBuf::from(std::env::var_os("AC_PARAMETER_CASES").unwrap());
    let cases: Value =
        serde_json::from_slice(&std::fs::read(work.join("cases.json")).unwrap()).unwrap();
    let registry: Map =
        serde_json::from_slice(&std::fs::read(work.join("registry.json")).unwrap()).unwrap();
    let root = work.join("runtime");
    let mut failures = vec![];
    let mut qualified = vec![];
    let mut diagnostic_differences = vec![];
    let native_build = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("validation/parameters/build");
    for case in cases.as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            s.diagnostics.source_directory =
                format!("{}/source/", native_build.display()).into_bytes();
            Ok(())
        })
        .unwrap();
        let mut context = None;
        let mut expected = Map::new();
        let mut rules = BTreeMap::new();
        let mut e = EclipticParameters::default();
        for c in [&mut e.longitude, &mut e.latitude] {
            c.prefit = Some(Float::with_val(PRECISION_BITS, 17));
            c.prefit_err = Some(Float::with_val(PRECISION_BITS, 19));
        }
        let mut expected_e = scratch(&e);
        for (i, a) in case["actions"].as_array().unwrap().iter().enumerate() {
            let a: Vec<&str> = a
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap())
                .collect();
            let tag = format!("{id} {i} {}", a[0]);
            let event = case["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["phase"] == "end" && e["action"] == i);
            let result = (|| -> Result<Value, TimingError> {
                if a[0] == "init" {
                    let n: usize = a[1].parse().unwrap();
                    let mut c = PulsarContext::unwritten(n);
                    for p in 0..n {
                        c.initialise_one(p, 2, a[3].parse().unwrap(), a[2].parse().unwrap())
                            .unwrap();
                        seed(&mut c.pulsars[p]);
                    }
                    expected = initial(&registry, n, &event.unwrap()["result"]["natural_controls"]);
                    context = Some(c);
                    return Ok(event.unwrap()["result"].clone());
                }
                if a[0] == "parse" {
                    return Ok(extended(a[1].as_bytes())?.wire());
                }
                if a[0] == "coord" {
                    let mut b = a[2].as_bytes().to_vec();
                    let turn = if a[1] == "hms" {
                        hms_turn(&mut b)
                    } else {
                        dms_turn(&b)?
                    };
                    return Ok(
                        json!({"turn":turn.wire(),"degree":(turn*360.).wire(),"input_after":String::from_utf8(b).unwrap()}),
                    );
                }
                if a[0] == "remove" {
                    let mut b = a[1].as_bytes().to_vec();
                    remove_cr(&mut b)?;
                    return Ok(json!(String::from_utf8(b).unwrap()));
                }
                if a[0] == "allocate" {
                    let mut c = PulsarContext::unwritten(1);
                    c.allocate_memory(0, a[1].parse().unwrap()).unwrap();
                    let mut s = Map::new();
                    for (i, p) in c.pulsars[0].param.slots.iter().enumerate() {
                        let name = PARAMETER_LABELS
                            .get(i)
                            .map(|n| format!("{n:?}"))
                            .unwrap_or(format!("unnamed_{i}"));
                        parameter(p, &format!("{name}."), &mut s, true);
                    }
                    return Ok(json!(s));
                }
                let c = context.as_mut().unwrap();
                match a[0] {
                    "registry" => {
                        let mut s = Map::new();
                        for (i, p) in c.pulsars[0].param.slots.iter().enumerate() {
                            let name = PARAMETER_LABELS
                                .get(i)
                                .map(|n| format!("{n:?}"))
                                .unwrap_or(format!("unnamed_{i}"));
                            parameter(p, &format!("{name}."), &mut s, true);
                        }
                        return Ok(json!(s));
                    }
                    "destroy" => {
                        for p in 0..c.pulsars.len() {
                            c.destroy_one(p).unwrap();
                        }
                        context = None;
                        expected.clear();
                        return Ok(json!({"matching_lifetime_complete":true}));
                    }
                    "defaults" => {
                        setup_parameter_file_defaults(&mut c.pulsars[0], &root)?;
                        return Ok(json!(0));
                    }
                    "prior" => {
                        let p = cell_mut(&mut c.pulsars[0], label(a[1]), a[2].parse().unwrap());
                        p.val = extended(a[3].as_bytes())?;
                        p.prefit = Some(extended(a[4].as_bytes())?);
                        p.err = extended(a[5].as_bytes())?;
                        p.prefit_err = Some(extended(a[6].as_bytes())?);
                        p.fit_flag = a[7].parse().unwrap();
                        p.param_set = a[8].parse().unwrap();
                    }
                    "read" => {
                        let mut s =
                            SourceText::new(Path::new("caller.par"), a[4].as_bytes().to_vec());
                        let rc = read_value(
                            &mut c.pulsars[0],
                            a[1].as_bytes(),
                            &mut s,
                            label(a[2]),
                            a[3].parse().unwrap(),
                        )?;
                        return Ok(json!({"status":rc,"position":s.position,"next":next(&mut s)}));
                    }
                    "stream" => {
                        let mut s =
                            SourceText::new(Path::new("caller.par"), a[1].as_bytes().to_vec());
                        let mut token = 0;
                        while let Some(key) = s.token() {
                            let pos = s.position;
                            let input = next(&mut s);
                            scaling_policy(id, &key, &input, &mut rules);
                            check_line(c, 0, &key, &mut s, &mut e, &root)?;
                            if let Some(event) = event {
                                let t = &event["result"][token];
                                let actual = json!({"keyword":String::from_utf8(key).unwrap(),"start":pos,"end":s.position,"next":next(&mut s)});
                                let expected_pos = json!({"keyword":t["keyword"],"start":t["start"],"end":t["end"],"next":t["next"]});
                                if actual != expected_pos {
                                    failures.push(format!(
                                        "{tag} token {token} cursor: {actual} != {expected_pos}"
                                    ));
                                }
                                apply(&mut expected, &t["changes"]);
                                apply(&mut expected_e, &t["ecliptic_changes"]);
                                compare(
                                    &snapshot(c),
                                    &expected,
                                    &format!("{tag} token {token}"),
                                    &rules,
                                    &mut qualified,
                                    &mut failures,
                                );
                                compare(
                                    &scratch(&e),
                                    &expected_e,
                                    &format!("{tag} ecliptic {token}"),
                                    &BTreeMap::new(),
                                    &mut qualified,
                                    &mut failures,
                                );
                            }
                            token += 1;
                        }
                        if let Some(event) = event {
                            assert_eq!(
                                token,
                                event["result"].as_array().unwrap().len(),
                                "{tag} token membership"
                            );
                            return Ok(event["result"].clone());
                        }
                    }
                    "copy" => {
                        let source = c.pulsars[0].param.slots[label(a[1]) as usize].clone();
                        crate::tempo2_util::copy_param(
                            &source,
                            &mut c.pulsars[0].param.slots[label(a[2]) as usize],
                        )
                        .unwrap();
                    }
                    "get" => {
                        return Ok(crate::tempo2_util::get_parameter_value(
                            &c.pulsars[0],
                            label(a[1]) as usize,
                            a[2].parse().unwrap(),
                        )?
                        .wire());
                    }
                    "final" => check_all_set(
                        &mut c.pulsars[0],
                        &e,
                        &SourceText::new(Path::new("caller.par"), vec![]),
                    )?,
                    "counts" => {
                        let p = &mut c.pulsars[0];
                        p.nJumps = Some(3);
                        p.nPhaseJump = Some(2);
                        let sini = &mut p.param.slots[param_sini as usize];
                        sini.n_link_to = Some(1);
                        sini.link_to[0] = Some(param_kin as i32);
                        let kin = &mut p.param.slots[param_kin as usize];
                        kin.n_link_from = Some(1);
                        kin.link_from[0] = Some(param_sini as i32);
                    }
                    "simple" => {
                        let mut s = SourceText::new(Path::new(a[1]), std::fs::read(a[1]).unwrap());
                        read_simple_parfile(c, 0, &mut s, &root)?;
                        return Ok(json!({"status":0,"position":s.position}));
                    }
                    "full" | "global" => {
                        let pairs: Vec<_> = a[2..]
                            .chunks_exact(2)
                            .map(|p| crate::bats::InputPair {
                                par: p[0].into(),
                                tim: p[1].into(),
                            })
                            .collect();
                        let mut inputs = vec![];
                        if a[0] == "full" {
                            read_parfiles(c, &pairs, &root, &mut inputs)?;
                        } else {
                            read_parfile_global(c, &pairs[0].par, &root, &mut inputs)?;
                        }
                    }
                    "fixed" => get_value(
                        a[5].as_bytes(),
                        a[3].parse().unwrap(),
                        a[4].parse().unwrap(),
                        &mut c.pulsars[0],
                        label(a[1]),
                        a[2].parse().unwrap(),
                    )?,
                    _ => panic!("unmapped K action {}", a[0]),
                }
                Ok(json!({}))
            })();
            match (event, result) {
                (Some(event), Ok(result)) => {
                    let kin = matches!(
                        id,
                        "link_KIN_before" | "link_KIN_after" | "link_nonrecursive"
                    ) && a[0] == "get"
                        && a[1] == "param_kin"
                        && a[2] == "0"
                        && context.as_ref().unwrap().pulsars[0].param.slots[param_stig as usize]
                            .cells[0]
                            .val
                            == 0.5;
                    if !checked_match(
                        &result,
                        &event["result"],
                        kin.then_some(8),
                        &format!("{tag} result"),
                        &mut qualified,
                    ) {
                        failures.push(format!("{tag} result: {result} != {}", event["result"]));
                    }
                    apply(&mut expected, &event["changes"]);
                    if let Some(c) = &context {
                        compare(
                            &snapshot(c),
                            &expected,
                            &tag,
                            &rules,
                            &mut qualified,
                            &mut failures,
                        );
                    }
                }
                (None, Err(_)) => break,
                (Some(_), Err(e)) => {
                    failures.push(format!("{tag}: unexpected error {e}"));
                    break;
                }
                (None, Ok(_)) => {
                    failures.push(format!("{tag}: missing source exit"));
                    break;
                }
            }
        }
        for (channel, actual) in ["stdout", "stderr"]
            .into_iter()
            .zip(crate::corpus_wire::diagnostic_streams())
        {
            let actual = actual.replace(native_build.to_str().unwrap(), work.to_str().unwrap());
            if actual != case[channel] {
                diagnostic_differences.push(
                    json!({"case":id,"channel":channel,"rust":actual,"native":case[channel]}),
                );
            }
        }
        eprintln!("K {id}: {} cumulative differences", failures.len());
    }
    std::fs::write(
        work.join("qualified-rounding.json"),
        serde_json::to_vec_pretty(&qualified).unwrap(),
    )
    .unwrap();
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
        "{} K diagnostic differences",
        diagnostic_differences.len()
    );
    assert!(
        failures.is_empty(),
        "{} differences; first {:?}",
        failures.len(),
        &failures[..failures.len().min(5)]
    );
}

#[test]
#[ignore = "diagnose exact K projection differences without changing production precision"]
fn k_precision_diagnosis() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let reference: Value = serde_json::from_slice(
        &std::fs::read(root.join("validation/parameters/observations.json")).unwrap(),
    )
    .unwrap();
    let kin = reference
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "link_KIN_after")
        .unwrap();
    let kin_expected = kin["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["phase"] == "end" && e["op"] == "get")
        .unwrap()["result"]
        .clone();
    let scaling = reference
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "threshold_scaling")
        .unwrap();
    let scaling_expected = scaling["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["phase"] == "end" && e["op"] == "stream")
        .unwrap()["result"][0]["changes"]["0.param_pbdot.0.val"]["after"]
        .clone();
    let mut values = vec![];
    for bits in [113, 128] {
        let threshold = Float::with_val(bits, Float::parse("1e-7").unwrap());
        let scaled = threshold * 1e-12f64;
        let stig = Float::with_val(bits, 0.5);
        let argument = stig.clone() * 2f64 / (Float::with_val(bits, 1) + stig.square());
        let kin = argument.clone().asin() * 180f64 / std::f64::consts::PI;
        values.push(json!({"bits":bits,"scaled":scaled.wire(),"KIN_argument":argument.wire(),"KIN":kin.wire(),"scaled_native_projection_equal":matches(&scaled.wire(),&scaling_expected),"KIN_native_projection_equal":matches(&kin.wire(),&kin_expected)}));
    }
    let result = json!({"native_scaled":scaling_expected,"native_KIN":kin_expected,"evaluations":values,"production_precision_unchanged":true,"gate":"exact projection diagnostic; decision 0073 separately supplies narrow continuous rounding bounds"});
    std::fs::write(
        root.join("validation/input_to_bat/build/parameters/precision-diagnosis.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    assert_eq!(
        result["evaluations"][0]["scaled_native_projection_equal"],
        true
    );
    assert_eq!(
        result["evaluations"][0]["KIN_native_projection_equal"],
        true
    );
    assert_eq!(
        result["evaluations"][1]["scaled_native_projection_equal"],
        false
    );
    assert_eq!(
        result["evaluations"][1]["KIN_native_projection_equal"],
        false
    );
}

#[test]
fn continuous_policy_negative_checks() {
    let mut rules = BTreeMap::new();
    scaling_policy("threshold_scaling", b"PBDOT", "1e-7", &mut rules);
    assert_eq!(rules.get("0.param_pbdot.0.val"), Some(&4));
    assert!(!rules.contains_key("0.param_pbdot.0.err"));
    let native = json!({"type":"binary128","class":"finite","value":"9.99999999999999979886647629255615307e-20"});
    let good = (Float::with_val(128, Float::parse("1e-7").unwrap()) * 1e-12f64).wire();
    let mut q = vec![];
    assert!(checked_match(&good, &native, Some(4), "good", &mut q));
    assert_eq!(q.len(), 1);
    for bad in [
        Float::with_val(128, Float::parse("1e-7").unwrap()),
        Float::with_val(128, Float::parse("1e-19").unwrap()),
        -Float::with_val(128, Float::parse("1e-19").unwrap()),
    ] {
        assert!(!checked_match(
            &bad.wire(),
            &native,
            Some(4),
            "bad factor/branch/sign",
            &mut q
        ));
    }
    let mut bad = good.clone();
    bad["class"] = json!("+Infinity");
    assert!(!checked_match(&bad, &native, Some(4), "class", &mut q));
    assert!(!checked_match(
        &good,
        &native,
        None,
        "unrelated exact field",
        &mut q
    ));
    scaling_policy("threshold_scaling", b"EDOT", "1e-7", &mut rules);
    assert!(!rules.contains_key("0.param_edot.0.prefit"));
    scaling_policy("threshold_scaling", b"PBDOT", "1e-15", &mut rules);
    assert!(!rules.contains_key("0.param_pbdot.0.val"));
    let mut other = BTreeMap::new();
    scaling_policy("other", b"PBDOT", "1e-7", &mut other);
    assert!(other.is_empty());
    for (text, units) in [("1e-19", 4), ("53.1301023541559807742470719993286259", 8)] {
        let n = Float::with_val(113, Float::parse(text).unwrap());
        let x = Float::with_val(128, &n)
            * (Float::with_val(128, 1) + Float::with_val(128, Float::parse("1e-30").unwrap()));
        assert!(!continuous(&x.wire(), &n.wire(), units));
    }
    let zero = Float::with_val(128, 0);
    assert!(!continuous(&(-zero.clone()).wire(), &zero.wire(), 4));
    let sub: Float = Float::with_val(128, 1) >> 16400;
    assert!(!continuous(&sub.wire(), &sub.wire(), 4));
}
