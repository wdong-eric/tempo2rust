//! Z complete public caller, including all allocated tails and exit prefixes.
use super::*;
use crate::{
    corpus_wire::{Wire, hex, observation_bytes},
    observation::{SourceObservation, read_string, write_string},
    pulsar_lifecycle::PulsarContext,
    source_text::extended,
    tempo2::{Label::*, SourceParameter},
};
use serde_json::{Value, json};

const PARAMETERS: [usize; 5] = [
    param_dphaseplanet as usize,
    param_dmassplanet as usize,
    param_tel_dx as usize,
    param_tel_dy as usize,
    param_tel_dz as usize,
];
fn number(x: f64) -> Value {
    if x.is_nan() {
        json!({"class":"NaN"})
    } else if x.is_infinite() {
        json!({"class":if x.is_sign_negative(){"-Infinity"}else{"+Infinity"}})
    } else {
        json!({"class":"finite","bits":format!("{:016x}",x.to_bits())})
    }
}
fn quad(x: &Float) -> Value {
    let mut b = vec![];
    x.put(&mut b);
    json!({"class":"finite","little_endian_bytes":hex(&b)})
}
fn chars(x: &[Option<u8>]) -> Value {
    json!({"initialized_bytes":x.len(),"hex":hex(&x.iter().map(|x|x.unwrap()).collect::<Vec<_>>())})
}
fn vector(x: &[Option<f64>]) -> Value {
    json!(x.iter().map(|x| number(x.unwrap())).collect::<Vec<_>>())
}
fn vectors(o: &SourceObservation) -> [&[Option<f64>; 6]; 11] {
    [
        &o.sun_ssb,
        &o.sun_earth,
        &o.jupiter_earth,
        &o.saturn_earth,
        &o.venus_earth,
        &o.uranus_earth,
        &o.neptune_earth,
        &o.earth_ssb,
        &o.earthMoonBary_ssb,
        &o.earthMoonBary_earth,
        &o.nutations,
    ]
}
fn vectors_mut(o: &mut SourceObservation) -> [&mut [Option<f64>; 6]; 11] {
    [
        &mut o.sun_ssb,
        &mut o.sun_earth,
        &mut o.jupiter_earth,
        &mut o.saturn_earth,
        &mut o.venus_earth,
        &mut o.uranus_earth,
        &mut o.neptune_earth,
        &mut o.earth_ssb,
        &mut o.earthMoonBary_ssb,
        &mut o.earthMoonBary_earth,
        &mut o.nutations,
    ]
}
const VECTOR_NAMES: [&str; 11] = [
    "sun_ssb",
    "sun_earth",
    "jupiter_earth",
    "saturn_earth",
    "venus_earth",
    "uranus_earth",
    "neptune_earth",
    "earth_ssb",
    "earthMoonBary_ssb",
    "earthMoonBary_earth",
    "nutations",
];
fn caller(c: &mut PulsarContext) -> Value {
    let mut pulsars = vec![];
    let mut observations = vec![];
    for j in 0..2 {
        let (p, rows) = c.pulsar_observation_backing_mut(j).unwrap();
        let path = read_string(&p.JPL_EPHEMERIS[..], "JPL").unwrap();
        let parameters=PARAMETERS.iter().enumerate().map(|(k,&slot)| {
            let p=&p.param.slots[slot];
            json!({"index":slot,"owner":j*5+k,"aSize":p.a_size.unwrap(),"values":p.cells.iter().map(|c|quad(&c.val)).collect::<Vec<_>>(),"set":p.cells.iter().map(|c|c.param_set).collect::<Vec<_>>()})
        }).collect::<Vec<_>>();
        let offsets=[(p.nTelDX,&p.telDX_t[..],&p.telDX_v[..]),(p.nTelDY,&p.telDY_t[..],&p.telDY_v[..]),(p.nTelDZ,&p.telDZ_t[..],&p.telDZ_v[..])].into_iter().map(|(n,t,v)|json!({"count":n.unwrap(),"backing":t.iter().zip(v).map(|(t,v)|json!([number(t.unwrap()),number(v.unwrap())])).collect::<Vec<_>>()})).collect::<Vec<_>>();
        pulsars.push(json!({"owner":j,"observation_owner":j*6,"nobs":p.nobs.unwrap(),"units":p.units.unwrap(),"eclCoord":p.eclCoord.unwrap(),"ephemeris":String::from_utf8(path.clone()).unwrap(),"ephemeris_tail":chars(&p.JPL_EPHEMERIS[path.len()+1..]),"parameters":parameters,"offsets":offsets}));
        for (i, o) in rows.iter().enumerate() {
            let v = VECTOR_NAMES
                .into_iter()
                .zip(vectors(o))
                .map(|(k, v)| (k.into(), vector(v)))
                .collect::<serde_json::Map<_, _>>();
            observations.push(json!({"owner":j*6+i,"sat":quad(o.sat.as_ref().unwrap()),"teph":quad(o.correctionTT_Teph.as_ref().unwrap()),"telID":chars(&o.telID),"nclock":o.nclock_correction.unwrap(),"clock_steps":o.correctionsTT.iter().map(|c|json!([number(c.correction.unwrap()),chars(&c.corrects_to)])).collect::<Vec<_>>(),"nFlags":o.nFlags.unwrap(),"flags":o.flagID.iter().zip(&o.flagVal).map(|(a,b)|json!([chars(a),chars(b)])).collect::<Vec<_>>(),"vectors":v,"planets":[o.planet_ssb.iter().map(|v|vector(v)).collect::<Vec<_>>(),o.planet_ssb_tmr.iter().map(|v|vector(v)).collect::<Vec<_>>(),o.planet_ssb_derv.iter().map(|v|vector(v)).collect::<Vec<_>>()]}));
        }
    }
    json!({"pulsars":pulsars,"observations":observations})
}
fn retained(c: &mut PulsarContext) -> (String, Vec<Vec<u8>>) {
    // Debug includes every canonical pulsar field, parameter cell and owner.
    // Compare the observation's full initialized projection with only the exact
    // Z writable vector set removed, as in Z's generated typed field checks.
    let pulsars = format!("{:?}", c.pulsars);
    let mut observations = vec![];
    for p in 0..2 {
        let (_, rows) = c.pulsar_observation_backing_mut(p).unwrap();
        for row in rows {
            let mut o = row.clone();
            for v in vectors_mut(&mut o) {
                v.fill(Some(0.0));
            }
            o.planet_ssb = [[Some(0.0); 6]; 9];
            o.planet_ssb_tmr = [[Some(0.0); 6]; 9];
            o.planet_ssb_derv = [[Some(0.0); 6]; 9];
            let mut b = vec![0; 7680];
            observation_bytes(&mut o, &mut b, false);
            observations.push(b);
        }
    }
    (pulsars, observations)
}
fn initial(reference: &Value) -> PulsarContext {
    let mut c = PulsarContext::unwritten(2);
    for j in 0..2 {
        let view = c.adopt_observations((0..6).map(|_| SourceObservation::zeroed()).collect());
        let p = &mut c.pulsars[j];
        p.obsn = Some(Some(view));
        p.nobs = Some(0);
        p.units = Some(2);
        p.eclCoord = Some(0);
        p.JPL_EPHEMERIS.fill(Some(b'~'));
        write_string(
            &mut p.JPL_EPHEMERIS[..],
            reference["pulsars"][j]["ephemeris"]
                .as_str()
                .unwrap()
                .as_bytes(),
            "JPL_EPHEMERIS",
        )
        .unwrap();
        for slot in PARAMETERS {
            p.param.slots[slot] = SourceParameter::allocated(9, 0);
        }
        p.nTelDX = Some(0);
        p.nTelDY = Some(0);
        p.nTelDZ = Some(0);
        for (axis, (t, v)) in [
            (&mut p.telDX_t, &mut p.telDX_v),
            (&mut p.telDY_t, &mut p.telDY_v),
            (&mut p.telDZ_t, &mut p.telDZ_v),
        ]
        .into_iter()
        .enumerate()
        {
            for k in 0..500 {
                t[k] = Some(49000.0 + k as f64);
                v[k] = Some((100 * axis) as f64 + k as f64 * 0.25);
            }
        }
        for (i, o) in c.observations_mut(view).unwrap().iter_mut().enumerate() {
            let j = j * 6 + i;
            o.sat = Some(extended(b"50010.25").unwrap());
            o.telID.fill(Some(b'~'));
            write_string(&mut o.telID, b"A", "telID").unwrap();
            for v in &mut o.flagID {
                v.fill(Some(b'~'));
            }
            for v in &mut o.flagVal {
                v.fill(Some(b'!'));
            }
            for (k, c) in o.correctionsTT.iter_mut().enumerate() {
                c.correction = Some(k as f64 * 0.125);
                c.corrects_to.fill(Some(b'~'));
                write_string(&mut c.corrects_to, b"UNUSED", "corrects_to").unwrap();
            }
            for (k, v) in vectors_mut(o).into_iter().enumerate() {
                for (a, x) in v.iter_mut().enumerate() {
                    *x = Some((1000 + 100 * j + 10 * k + a) as f64 + 0.25);
                }
            }
            for k in 0..9 {
                for a in 0..6 {
                    o.planet_ssb[k][a] = Some((2000 + 100 * j + 10 * k + a) as f64 + 0.25);
                    o.planet_ssb_tmr[k][a] = Some((3000 + 100 * j + 10 * k + a) as f64 + 0.75);
                    o.planet_ssb_derv[k][a] = Some((4000 + 100 * j + 10 * k + a) as f64);
                }
            }
        }
    }
    c
}
fn differences(path: &str, a: &Value, b: &Value, out: &mut Vec<Value>) {
    if a == b {
        return;
    }
    match (a, b) {
        (Value::Object(a), Value::Object(b)) if a.keys().eq(b.keys()) => {
            for (k, a) in a {
                differences(&format!("{path}.{k}"), a, &b[k], out);
            }
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (i, (a, b)) in a.iter().zip(b).enumerate() {
                differences(&format!("{path}[{i}]"), a, b, out);
            }
        }
        _ => out.push(json!({"path":path,"rust":a,"native":b})),
    }
}
#[test]
#[ignore = "check_planetary.py supplies accepted Z driver histories"]
fn z_driver_histories() {
    let path = PathBuf::from(std::env::var_os("AC_PLANETARY_CASES").unwrap());
    let cases: Vec<Value> = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let mut out = vec![];
    for case in cases {
        crate::clkcorr::with_process(|s| {
            *s = Default::default();
            Ok(())
        })
        .unwrap();
        crate::jpleph::reset_init_status_for_test();
        let mut c = initial(&case["events"][0]["caller"]);
        let root = PathBuf::from(
            case["events"][0]["caller"]["pulsars"][0]["ephemeris"]
                .as_str()
                .unwrap(),
        )
        .parent()
        .unwrap()
        .to_owned();
        let mut seeds = [0i64; 2];
        let mut stdout = vec![];
        let mut stderr = vec![];
        let mut debug = 0;
        let mut cvs = 0;
        let mut result = json!({});
        let mut error = None;
        for event in case["events"].as_array().unwrap() {
            let index = event["event"].as_u64().unwrap() as usize;
            let a = case["actions"][index]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect::<Vec<_>>();
            let k = |i: usize| a[i].parse::<i32>().unwrap();
            let n = |i: usize| a[i].parse::<f64>().unwrap();
            let q = |i: usize| extended(a[i].as_bytes()).unwrap();
            let phase = event["phase"].as_str().unwrap();
            let tag = format!("{}:{index}:{phase}", case["id"].as_str().unwrap());
            if phase == "begin" {
                let marker = format!("@BEGIN {index} {}\n", a[0]);
                stdout.extend(marker.as_bytes());
                stderr.extend(marker.as_bytes());
            }
            if !event["caller"].is_null() {
                differences(
                    &(tag.clone() + ".caller"),
                    &caller(&mut c),
                    &event["caller"],
                    &mut out,
                );
            }
            let globals=crate::clkcorr::with_process(|s|Ok(json!({"debug":debug,"cvs":cvs,"warn_count":s.diagnostics.warnings,"error_count":s.diagnostics.errors,"jpl_init_status":jpl_init_error_code()}))).unwrap();
            differences(
                &(tag.clone() + ".globals"),
                &globals,
                &event["globals"],
                &mut out,
            );
            differences(
                &(tag.clone() + ".seeds"),
                &json!(seeds),
                &event["seeds"],
                &mut out,
            );
            if phase != "begin" {
                differences(
                    &(tag.clone() + ".result"),
                    &result,
                    &event["result"],
                    &mut out,
                );
                assert_eq!(phase == "exit", error.is_some(), "{tag}: {error:?}");
                if phase == "end" {
                    let marker = format!("@END {index} {}\n", a[0]);
                    stdout.extend(marker.as_bytes());
                    stderr.extend(marker.as_bytes());
                }
                continue;
            }
            result = json!({});
            error = None;
            match a[0] {
                "profile" => {
                    // Native transport profile; production arithmetic remains Rug128.
                    result = json!({"double_digits":53,"longdouble_digits":113,"longdouble_size":16,"long_size":8,"max_flags":40,"flag_length":32});
                }
                "debug" => {
                    debug = k(1);
                    cvs = k(2);
                    crate::clkcorr::with_process(|s| {
                        s.diagnostics.debug = debug != 0;
                        s.diagnostics.display_version = cvs == 1;
                        Ok(())
                    })
                    .unwrap();
                }
                "pulsar" => {
                    let p = &mut c.pulsars[k(1) as usize];
                    p.nobs = Some(k(2));
                    p.units = Some(k(3));
                    p.eclCoord = Some(k(4));
                    write_string(
                        &mut p.JPL_EPHEMERIS[..],
                        root.join(format!("{}.bin", a[5]))
                            .as_os_str()
                            .as_encoded_bytes(),
                        "JPL_EPHEMERIS",
                    )
                    .unwrap();
                }
                "observation" | "tt" | "flag" | "clear_flags" => {
                    let row = k(1) as usize;
                    let (_, rows) = c.pulsar_observation_backing_mut(row / 6).unwrap();
                    let o = &mut rows[row % 6];
                    match a[0] {
                        "observation" => {
                            o.sat = Some(q(2));
                            write_string(&mut o.telID, a[3].as_bytes(), "telID").unwrap();
                            o.correctionTT_Teph = Some(q(4));
                        }
                        "tt" => {
                            o.nclock_correction = Some(k(2));
                            for j in 3..a.len() {
                                o.correctionsTT[j - 3].correction = Some(n(j));
                            }
                        }
                        "flag" => {
                            let i = o.nFlags.unwrap() as usize;
                            write_string(&mut o.flagID[i], a[2].as_bytes(), "flagID").unwrap();
                            write_string(&mut o.flagVal[i], a[3].as_bytes(), "flagVal").unwrap();
                            o.nFlags = Some(i as i32 + 1);
                        }
                        _ => o.nFlags = Some(0),
                    }
                }
                "parameter" => {
                    let v = &mut c.pulsars[k(1) as usize].param.slots
                        [PARAMETERS[usize::from(a[2] != "phase")]]
                    .cells[k(3) as usize];
                    v.param_set = k(4);
                    v.val = q(5);
                }
                "offset_mode" => {
                    c.pulsars[k(1) as usize].param.slots[PARAMETERS[2 + k(2) as usize]].cells[0]
                        .val = q(3)
                }
                "offset" => {
                    let p = &mut c.pulsars[k(1) as usize];
                    let (count, t, v) = match k(2) {
                        0 => (&mut p.nTelDX, &mut p.telDX_t, &mut p.telDX_v),
                        1 => (&mut p.nTelDY, &mut p.telDY_t, &mut p.telDY_v),
                        _ => (&mut p.nTelDZ, &mut p.telDZ_t, &mut p.telDZ_v),
                    };
                    *count = Some(k(3));
                    for j in 0..4 {
                        t[j] = Some(n(4 + 2 * j));
                        v[j] = Some(n(5 + 2 * j));
                    }
                }
                "seed" => seeds[k(1) as usize] = a[2].parse().unwrap(),
                "random" => {
                    result = json!({"value":number(random2(&mut seeds[k(1) as usize]).unwrap())})
                }
                "gaussian" => {
                    result = json!({"value":number(gasdev(&mut seeds[k(1) as usize]).unwrap())})
                }
                "driver" => {
                    let before = retained(&mut c);
                    let tail = c.pulsars.split_off(k(1).max(0) as usize);
                    error = read_ephemeris(&mut c, &root, &mut vec![], k(2)).err();
                    c.pulsars.extend(tail);
                    assert_eq!(before, retained(&mut c), "{tag} retained fields");
                    result = json!({"all_other_typed_fields_retained":true,"parameter_backing_retained":true});
                }
                _ => panic!("{}", a[0]),
            }

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
        }
        differences(
            &format!("{}.stdout", case["id"]),
            &json!(String::from_utf8(stdout).unwrap()),
            &case["stdout"],
            &mut out,
        );
        differences(
            &format!("{}.stderr", case["id"]),
            &json!(String::from_utf8(stderr).unwrap()),
            &case["stderr"],
            &mut out,
        );
        eprintln!("Z {}: {} cumulative differences", case["id"], out.len());
    }
    std::fs::write(
        path.with_file_name("differences.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    assert!(out.is_empty(), "{} Z differences", out.len());
}
