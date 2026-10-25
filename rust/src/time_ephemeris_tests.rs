//! Y's initialized V caller; all numerical work calls production counterparts.
#![allow(non_snake_case, dead_code)]
use crate::{
    TimingError,
    clkcorr::ClockSession,
    ifteph::*,
    observation::write_string,
    pulsar::*,
    pulsar_lifecycle::*,
    tempo2::{SourceClockCorrection, SourceObservation},
};
use rug::Float;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
include!("terrestrial_fixture_fields.rs");
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn num(v: f64) -> Value {
    json!({"type":"binary64","class":if v.is_nan(){"NaN"}else if v.is_infinite(){if v.is_sign_negative(){"-Infinity"}else{"+Infinity"}}else{"finite"},"bits":hex(&v.to_le_bytes())})
}
fn num32(v: f32) -> Value {
    json!({"type":"binary32","class":if v.is_nan(){"NaN"}else if v.is_infinite(){if v.is_sign_negative(){"-Infinity"}else{"+Infinity"}}else{"finite"},"bits":hex(&v.to_le_bytes())})
}
fn extended(x: &Float) -> Value {
    // Export projection only: the production value and every operation stay 128.
    let x = Float::with_val(113, x);
    let sign = (x.is_sign_negative() as u128) << 127;
    let bits = if x.is_zero() {
        sign
    } else {
        assert!(x.is_finite());
        let (n, e) = x.clone().abs().to_integer_exp().unwrap();
        let n = n.to_u128().unwrap();
        let len = 128 - n.leading_zeros();
        let mant = if len > 113 {
            n >> (len - 113)
        } else {
            n << (113 - len)
        };
        sign | (((e + len as i32 - 1 + 16383) as u128) << 112) | (mant - (1 << 112))
    };
    json!({"type":"binary128","bits":hex(&bits.to_le_bytes())})
}
fn bytes(b: &[u8]) -> Value {
    json!({"known_bytes":b.len(),"hex":hex(b)})
}
fn known(b: &[Option<u8>]) -> Value {
    bytes(&b.iter().map(|b| b.unwrap()).collect::<Vec<_>>())
}
fn numbers(v: &[f64]) -> Value {
    json!(v.iter().copied().map(num).collect::<Vec<_>>())
}
fn parse(s: &str) -> Float {
    Float::with_val(128, Float::parse(s).unwrap())
}
fn cache(s: &mut BTreeMap<String, Value>, prefix: &str, c: &InterpolationInfo) {
    s.insert(format!("{prefix}pc"), numbers(&c.pc));
    s.insert(format!("{prefix}vc"), numbers(&c.vc));
    s.insert(format!("{prefix}twot"), num(c.twot));
    s.insert(format!("{prefix}np"), json!(c.np));
    s.insert(format!("{prefix}nv"), json!(c.nv));
}
fn stream(s: &mut BTreeMap<String, Value>, prefix: &str, f: &mut crate::stdio::Stream) {
    let (p, e, r) = f.status();
    for (name, v) in [("position", p), ("eof", e.into()), ("error", r.into())] {
        s.insert(format!("{prefix}{name}"), json!(v));
    }
}
struct Probe {
    session: ClockSession,
    context: PulsarContext,
    views: Vec<ObservationView>,
    direct: InterpolationInfo,
    output: [f64; 16],
    chars: [u8; 64],
    filename: [u8; 1024],
    root: String,
    ifowner: String,
    fowner: String,
    igen: i32,
    fgen: i32,
    old: Vec<Option<crate::stdio::Stream>>,
}
impl Probe {
    fn new() -> Self {
        let mut context = PulsarContext::unwritten(2);
        let mut views = Vec::new();
        for p in 0..2 {
            let view = context.allocate_observations(4).unwrap();
            views.push(view);
            let psr = &mut context.pulsars[p];
            zero_pulsar(psr);
            psr.obsn = Some(Some(view));
            psr.nobs = Some(3);
            psr.timeEphemeris = Some(1);
            psr.units = Some(1);
            psr.dilateFreq = Some(1);
            psr.useCalceph = Some(0);
            for (i, o) in context
                .observations_mut(view)
                .unwrap()
                .iter_mut()
                .enumerate()
            {
                zero_observation(o);
                o.sat = Some(Float::with_val(128, 50001.0 + i as f64 * 0.125));
                o.clockCorr = Some(1);
                write_string(&mut o.telID, b"PKS", "telID").unwrap();
                o.nclock_correction = Some(3);
                for (j, c) in o.correctionsTT.iter_mut().enumerate() {
                    c.correction = Some((j + 1) as f64 * 0.125);
                    write_string(
                        &mut c.corrects_to,
                        format!("C{j}").as_bytes(),
                        "corrects_to",
                    )
                    .unwrap();
                }
                o.correctionTT_TB = Some(Float::with_val(128, 110 + p * 20 + i));
                o.correctionTT_Teph = Some(Float::with_val(128, 210 + p * 20 + i));
                o.correctionTT_calcEph = Some(Float::with_val(
                    128,
                    0.125 + p as f64 * 0.25 + i as f64 * 0.0625,
                ));
                o.einsteinRate = Some((310 + p * 20 + i) as f64);
                for j in 0..6 {
                    o.earth_ssb[j] = Some((j + 1) as f64 * 0.000125);
                }
                for j in 0..3 {
                    o.observatory_earth[j] = Some((j + 1) as f64 * 0.0125);
                    o.siteVel[j] = Some((j + 1) as f64 * 0.000003125);
                }
            }
        }
        let mut p = Self {
            session: ClockSession::default(),
            context,
            views,
            direct: InterpolationInfo::default(),
            output: std::array::from_fn(|i| 1000.0 + i as f64),
            chars: [0x5a; 64],
            filename: [0x57; 1024],
            root: String::new(),
            ifowner: "null".into(),
            fowner: "null".into(),
            igen: 0,
            fgen: 0,
            old: Vec::new(),
        };
        p.reset_direct(100.0);
        p
    }
    fn reset_direct(&mut self, seed: f64) {
        self.direct.pc = std::array::from_fn(|i| seed + i as f64);
        self.direct.vc = std::array::from_fn(|i| seed + 30.0 + i as f64);
        self.direct.pc[0] = 1.0;
        self.direct.pc[1] = 0.0;
        self.direct.vc[0] = 0.0;
        self.direct.vc[1] = 1.0;
        self.direct.vc[2] = 0.0;
        self.direct.np = 2;
        self.direct.nv = 3;
        self.direct.twot = 0.0;
    }
    fn snapshot(&mut self) -> BTreeMap<String, Value> {
        let mut s = BTreeMap::new();
        let f = &mut self.session.ifte;
        for (name, v) in [
            ("startJD", f.start_jd),
            ("endJD", f.end_jd),
            ("stepJD", f.step_jd),
            ("L_C", f.l_c),
        ] {
            s.insert(format!("ifte.{name}"), num(v));
        }
        for (name, v) in [
            ("ephver", f.ephver),
            ("swap_endian", f.swap_endian),
            ("reclen", f.reclen),
            ("irec", f.irec),
        ] {
            s.insert(format!("ifte.{name}"), json!(v));
        }
        s.insert("ifte.title".into(), bytes(&f.title));
        s.insert("ifte.buf".into(), numbers(&f.buf));
        s.insert("ifte.ipt0".into(), json!(f.ipt[0]));
        s.insert("ifte.ipt1".into(), json!(f.ipt[1]));
        cache(&mut s, "ifte.", &f.iinfo);
        cache(&mut s, "direct.", &self.direct);
        s.insert("ifte.owner".into(), json!(self.ifowner));
        s.insert("ifte.generation".into(), json!(self.igen));
        if let Some(f) = f.stream.as_mut() {
            stream(&mut s, "ifte.", f);
        }
        s.insert("fortran.owner".into(), json!(self.fowner));
        s.insert("fortran.generation".into(), json!(self.fgen));
        s.insert(
            "fortran.swapByte".into(),
            json!(self.session.fortran.swap_byte),
        );
        if let Some(f) = self.session.fortran.stream.as_mut() {
            stream(&mut s, "fortran.", f);
        }
        for (i, old) in self.old.iter_mut().enumerate() {
            let k = format!("old{i}.");
            s.insert(
                k.clone() + "owner",
                json!(if old.is_some() {
                    "live-saved"
                } else {
                    "released-by-caller"
                }),
            );
            if let Some(f) = old {
                stream(&mut s, &k, f);
            }
        }
        s.insert("output".into(), numbers(&self.output));
        s.insert("chars".into(), bytes(&self.chars));
        s.insert("filename".into(), bytes(&self.filename));
        s.insert("root".into(), json!(self.root));
        let d = &self.session.diagnostics;
        for (n, v) in [
            ("displayCVSversion", i32::from(d.display_version)),
            ("debugFlag", i32::from(d.debug)),
            ("quietFlag", i32::from(d.quiet)),
            ("TK_errorCount", d.errors as i32),
            ("TK_warnCount", d.warnings as i32),
        ] {
            s.insert(n.into(), json!(v));
        }
        s.insert("TK_errorlog".into(), bytes(&[0; 10800]));
        s.insert("TK_warnlog".into(), bytes(&[0; 10800]));
        for (p, ps) in self.context.pulsars.iter().enumerate() {
            let k = format!("p{p}.");
            for (n, v) in [
                ("nobs", ps.nobs),
                ("timeEphemeris", ps.timeEphemeris),
                ("units", ps.units),
                ("dilateFreq", ps.dilateFreq),
                ("useCalceph", ps.useCalceph),
            ] {
                s.insert(k.clone() + n, json!(v.unwrap()));
            }
            for (i, o) in self
                .context
                .observations(self.views[p])
                .unwrap()
                .iter()
                .enumerate()
            {
                let a = format!("{k}o{i}.");
                for (n, v) in [
                    ("sat", &o.sat),
                    ("correctionTT_TB", &o.correctionTT_TB),
                    ("correctionTT_Teph", &o.correctionTT_Teph),
                    ("correctionTT_calcEph", &o.correctionTT_calcEph),
                ] {
                    s.insert(a.clone() + n, extended(v.as_ref().unwrap()));
                }
                for (n, v) in [
                    ("clockCorr", o.clockCorr),
                    ("nclock_correction", o.nclock_correction),
                ] {
                    s.insert(a.clone() + n, json!(v.unwrap()));
                }
                s.insert(a.clone() + "einsteinRate", num(o.einsteinRate.unwrap()));
                s.insert(a.clone() + "telID", known(&o.telID));
                for (n, v) in [
                    ("earth_ssb", &o.earth_ssb[..]),
                    ("observatory_earth", &o.observatory_earth[..3]),
                    ("siteVel", &o.siteVel[..]),
                ] {
                    s.insert(
                        a.clone() + n,
                        numbers(&v.iter().map(|v| v.unwrap()).collect::<Vec<_>>()),
                    );
                }
                for (j, c) in o.correctionsTT.iter().enumerate() {
                    s.insert(
                        format!("{a}clock{j}.correction"),
                        num(c.correction.unwrap()),
                    );
                    s.insert(format!("{a}clock{j}.corrects_to"), known(&c.corrects_to));
                }
            }
        }
        s
    }
    fn event(&mut self, out: &mut fs::File, phase: &str, action: i32, op: &str, result: Value) {
        writeln!(
            out,
            "{}",
            json!({"phase":phase,"action":action,"op":op,"result":result,"state":self.snapshot()})
        )
        .unwrap();
    }
    fn act(&mut self, a: &[String]) -> Result<Value, TimingError> {
        let int = |i: usize| a[i].parse::<i32>().unwrap();
        let idx = |i: usize| a[i].parse::<usize>().unwrap();
        let fl = |i: usize| a[i].parse::<f64>().unwrap();
        let mut result = Value::Null;
        match a[0].as_str() {
            "root" => {
                self.root = a[1].clone();
                self.session.root = Some(a[1].clone().into());
            }
            "init" | "initroot" => {
                if a[0] == "init" {
                    self.session
                        .ifte
                        .init(Path::new(&a[1]), &mut self.session.diagnostics)?;
                } else {
                    crate::tt2tdb::init_ifte(&mut self.session)?;
                }
                self.ifowner = "live".into();
                self.igen += 1;
            }
            "save" => {
                self.old.push(self.session.ifte.stream.take());
                self.ifowner = "saved-for-replacement".into();
            }
            "cleanup" => {
                self.old[idx(1)] = None;
            }
            "close" => {
                self.session.ifte.close_file();
                if self.ifowner != "null" {
                    self.ifowner = "released".into();
                }
            }
            "get" => {
                let f = &mut self.session.ifte;
                let d = &mut self.session.diagnostics;
                match a[1].as_str() {
                    "values" => f.get_values(fl(2), fl(3), int(4), &mut self.output, d)?,
                    "dtboth" => f.get_delta_t_delta_t_dot(
                        fl(2),
                        fl(3),
                        &mut self.output,
                        idx(4),
                        idx(5),
                        d,
                    )?,
                    "dt" => self.output[0] = f.delta_t(fl(2), fl(3), d)?,
                    "dot" => self.output[0] = f.delta_t_dot(fl(2), fl(3), d)?,
                    "vboth" => {
                        f.get_ve_ve_dot(fl(2), fl(3), &mut self.output, idx(4), idx(5), d)?
                    }
                    "v" => self.output[..3].copy_from_slice(&f.get_ve(fl(2), fl(3), d)?),
                    "vdot" => self.output[..3].copy_from_slice(&f.get_ve_dot(fl(2), fl(3), d)?),
                    _ => panic!(),
                }
            }
            "ifdelta" => {
                self.output[0] = crate::tt2tdb::if_delta_t(&mut self.session, &parse(&a[1]))?
            }
            "resetdirect" => self.reset_direct(fl(1)),
            "interp" => {
                let (ncf, ncm, na, ifl) = (int(1), int(2), int(3), int(4));
                let coef: Vec<f64> = (0..ncf * ncm * na)
                    .map(|i| {
                        if a[7] == "simple" {
                            match i % ncf {
                                0 => 2.0,
                                1 => 3.0,
                                2 => 5.0,
                                _ => 0.0,
                            }
                        } else {
                            (i % 13 - 6) as f64 * 0.03125
                        }
                    })
                    .collect();
                self.direct.interpolate(
                    &coef,
                    [fl(5), fl(6)],
                    ncf,
                    ncm,
                    na,
                    ifl,
                    &mut self.output,
                )?;
            }
            "swap" => {
                let mut b: [u8; 32] = std::array::from_fn(|i| i as u8 + 1);
                for _ in 0..int(3) {
                    match a[1].as_str() {
                        "4" => IFTswap4((&mut b[..4]).try_into().unwrap()),
                        "8" => IFTswap8((&mut b[..8]).try_into().unwrap()),
                        "8N" => IFTswap8N(&mut b, int(2))?,
                        "int" | "ints" => {
                            let mut v: Vec<i32> = b
                                .chunks_exact(4)
                                .map(|c| i32::from_ne_bytes(c.try_into().unwrap()))
                                .collect();
                            if a[1] == "int" {
                                IFTswapInt(&mut v[0]);
                            } else {
                                IFTswapInts(&mut v, int(2))?;
                            }
                            for (c, v) in b.chunks_exact_mut(4).zip(v) {
                                c.copy_from_slice(&v.to_ne_bytes());
                            }
                        }
                        "double" | "doubles" => {
                            let mut v: Vec<f64> = b
                                .chunks_exact(8)
                                .map(|c| f64::from_ne_bytes(c.try_into().unwrap()))
                                .collect();
                            if a[1] == "double" {
                                IFTswapDouble(&mut v[0]);
                            } else {
                                IFTswapDoubles(&mut v, int(2))?;
                            }
                            for (c, v) in b.chunks_exact_mut(8).zip(v) {
                                c.copy_from_slice(&v.to_ne_bytes());
                            }
                        }
                        _ => panic!(),
                    }
                }
                result = bytes(&b);
            }
            "open" => {
                self.filename[..a[1].len()].copy_from_slice(a[1].as_bytes());
                self.filename[a[1].len()] = 0;
                let r = self
                    .session
                    .fortran
                    .open_file(&mut self.filename, &mut self.session.diagnostics)?;
                self.fowner = if r == 0 { "live" } else { "null-failed-open" }.into();
                self.fgen += 1;
                result = json!(r);
            }
            "fclose" => {
                self.session.fortran.close_file()?;
                self.fowner = "released".into();
            }
            "swapmode" => self.session.fortran.swap_byte = int(1),
            "read" => {
                let r = &mut self.session.fortran;
                result = match a[1].as_str() {
                    "char" => json!(r.read_char()? as i32),
                    "chars" => {
                        r.read_character(int(2), &mut self.chars)?;
                        Value::Null
                    }
                    "int" => json!(r.read_int()?),
                    "float" => num32(r.read_float()?),
                    "double" => num(r.read_double()?),
                    "record" => json!(r.read_record_int()?),
                    _ => panic!(),
                };
            }
            "fb" => {
                self.fowner = "live-inside-FB".into();
                self.fgen += 1;
                self.output[0] = crate::tt2tdb::fb_delta_t(&mut self.session, &parse(&a[1]))?;
                self.fowner = "released-by-FB".into();
            }
            "p" => {
                let p = &mut self.context.pulsars[idx(1)];
                let v = Some(int(3));
                match a[2].as_str() {
                    "nobs" => p.nobs = v,
                    "timeEphemeris" => p.timeEphemeris = v,
                    "units" => p.units = v,
                    "dilateFreq" => p.dilateFreq = v,
                    "useCalceph" => p.useCalceph = v,
                    _ => panic!(),
                }
            }
            "obs" => {
                let o = &mut self.context.observations_mut(self.views[idx(1)]).unwrap()[idx(2)];
                match a[3].as_str() {
                    "sat" => o.sat = Some(parse(&a[4])),
                    "clockCorr" => o.clockCorr = Some(int(4)),
                    "nclock_correction" => o.nclock_correction = Some(int(4)),
                    "correctionTT_calcEph" => o.correctionTT_calcEph = Some(parse(&a[4])),
                    "correctionTT_TB" => o.correctionTT_TB = Some(parse(&a[4])),
                    "correctionTT_Teph" => o.correctionTT_Teph = Some(parse(&a[4])),
                    "einsteinRate" => o.einsteinRate = Some(fl(4)),
                    "telID" => {
                        write_string(&mut o.telID, a[4].as_bytes(), "telID")?;
                    }
                    _ => panic!(),
                }
            }
            "clock" => {
                self.context.observations_mut(self.views[idx(1)]).unwrap()[idx(2)].correctionsTT
                    [idx(3)]
                .correction = Some(fl(4))
            }
            "vector" => {
                let o = &mut self.context.observations_mut(self.views[idx(1)]).unwrap()[idx(2)];
                let v = match a[3].as_str() {
                    "earth" => &mut o.earth_ssb[..],
                    "site" => &mut o.siteVel[..],
                    _ => &mut o.observatory_earth[..],
                };
                v[idx(4)] = Some(fl(5));
            }
            "tt" => {
                if a[2] == "present" {
                    self.ifowner = "live-inside-tt".into();
                    self.igen += 1;
                }
                crate::tt2tdb::tt2tb(&mut self.session, &mut self.context, int(1))?;
                self.ifowner = "released-by-tt".into();
                for p in 0..int(1) {
                    let ps = &self.context.pulsars[p as usize];
                    for i in 0..ps.nobs.unwrap() {
                        let o =
                            &self.context.observations(self.views[p as usize]).unwrap()[i as usize];
                        if ps.timeEphemeris == Some(2)
                            && (o.clockCorr != Some(0)
                                || crate::observation::read_string(&o.telID, "telID")? == b"STL")
                        {
                            self.fowner = "released-by-FB".into();
                            self.fgen += 1;
                        }
                    }
                }
            }
            "global" => {
                if a[1] == "cvs" {
                    self.session.diagnostics.display_version = int(2) == 1;
                } else {
                    self.session.diagnostics.debug = int(2) != 0;
                }
            }
            "replace" => {
                let temp = format!("{}.replacement", a[1]);
                fs::copy(&a[2], &temp).unwrap();
                fs::rename(temp, &a[1]).unwrap();
            }
            "truncate" => fs::OpenOptions::new()
                .write(true)
                .open(&a[1])
                .unwrap()
                .set_len(a[2].parse().unwrap())
                .unwrap(),
            _ => panic!("unknown action"),
        }
        Ok(result)
    }
}
#[test]
#[ignore = "Y fresh isolated probe caller"]
fn time_ephemeris_probe() {
    let input = fs::read(std::env::var("TEMPO2_Y_INPUT").unwrap()).unwrap();
    let data: Value = serde_json::from_slice(&input).unwrap();
    let dest = std::env::var("TEMPO2_Y_OUTPUT").unwrap();
    let mut out = fs::File::create(&dest).unwrap();
    writeln!(out,"{}",json!({"phase":"launch","nonce":data["nonce"],"input_sha256":format!("{:x}",Sha256::digest(&input)),"constants":{"K":extended(&(Float::with_val(128,1)+IFTE_KM1)),"KM1":num(IFTE_KM1),"LC":num(IFTE_LC),"TEPH0":num(IFTE_TEPH0),"MJD0":num(IFTE_MJD0),"JD0":num(2443144.5003725)}})).unwrap();
    let mut p = Probe::new();
    p.event(&mut out, "initial", -1, "initial", Value::Null);
    let actions: Vec<Vec<String>> = serde_json::from_value(data["actions"].clone()).unwrap();
    let mut outcome = "success";
    let mut error = Value::Null;
    let (mut step, mut op) = (-1, "initial".to_owned());
    for (i, a) in actions.iter().enumerate() {
        step = i as i32;
        op = a[0].clone();
        p.event(&mut out, "begin", step, &op, Value::Null);
        match p.act(a) {
            Ok(v) => p.event(&mut out, "end", step, &op, v),
            Err(e) => {
                if p.session.ifte.stream.is_none() && p.ifowner.starts_with("live") {
                    p.ifowner = "released-on-error".into();
                }
                if p.session.fortran.stream.is_none() && p.fowner.starts_with("live") {
                    p.fowner = "released-on-error".into();
                }
                outcome = if e.field == "source_exit" {
                    "source_exit"
                } else {
                    "boundary"
                };
                error = json!({"field":e.field,"message":e.message,"path":e.path});
                break;
            }
        }
    }
    p.event(&mut out, "exit", step, &op, Value::Null);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    for (e, b) in p.session.diagnostics.events {
        if e {
            stderr.extend(b);
        } else {
            stdout.extend(b);
        }
    }
    fs::write(format!("{dest}.stdout"), stdout).unwrap();
    fs::write(format!("{dest}.stderr"), stderr).unwrap();
    fs::write(format!("{dest}.outcome"), outcome).unwrap();
    fs::write(format!("{dest}.error.json"), error.to_string()).unwrap();
}

fn fb_fixture(value: f64) -> Vec<u8> {
    let mut b = Vec::new();
    for x in [2450000.5f64, 2450016.5] {
        b.extend(x.to_be_bytes());
    }
    for x in [4i32, 1] {
        b.extend(x.to_be_bytes());
    }
    for x in [11f64, 12.0, 13.0, 14.0, 15.0, value, 2.0 * value] {
        b.extend(x.to_be_bytes());
    }
    b
}
#[test]
fn fb_incomplete_reads_reject_fabricated_eof_corrections() {
    // 0065's original-native witness: these prefixes return 2^-10, +0,
    // NaN and 0x1.fffffffffffffp+0, respectively. EOF assigns 0xff bytes;
    // incomplete FB reads are a bug correction, not uninitialized storage.
    let root = std::env::temp_dir().join(format!("fb-eof-{}", std::process::id()));
    fs::create_dir_all(root.join("ephemeris")).unwrap();
    let path = root.join("ephemeris/TDB.1950.2050");
    let bytes = fb_fixture(1.0 / 1024.0);
    for (length, mjd) in [(72, "50000"), (24, "49996"), (64, "50000"), (65, "50000")] {
        fs::write(&path, &bytes[..length]).unwrap();
        let mut s = ClockSession {
            root: Some(root.clone()),
            ..Default::default()
        };
        let result = crate::tt2tdb::fb_delta_t(&mut s, &parse(mjd));
        if length == 72 {
            assert_eq!(result.unwrap(), 1.0 / 1024.0);
        } else {
            assert_eq!(result.unwrap_err().field, "FB_incomplete_record");
            assert!(s.fb.content.is_none());
            assert_eq!(s.fb.tdbnrl, -1);
        }
        assert!(s.fortran.stream.is_none());
    }
    // A complete header is sufficient when the requested loops read no
    // coefficients. Preserve cold nr==1 and zero/negative count behavior.
    for (count, mjd) in [(1i32, "49996"), (0, "50000"), (-1, "50000")] {
        let mut header = bytes[..64].to_vec();
        header[20..24].copy_from_slice(&count.to_be_bytes());
        fs::write(&path, header).unwrap();
        let mut s = ClockSession {
            root: Some(root.clone()),
            ..Default::default()
        };
        let value = crate::tt2tdb::fb_delta_t(&mut s, &parse(mjd)).unwrap();
        assert_eq!(value.to_bits(), 0.0f64.to_bits());
        assert!(s.fortran.stream.is_none());
    }
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn fb_content_identity_retention_and_failure_publication() {
    let root = std::env::temp_dir().join(format!("fb-content-{}", std::process::id()));
    fs::create_dir_all(root.join("ephemeris")).unwrap();
    let path = root.join("ephemeris/TDB.1950.2050");
    let mut s = ClockSession {
        root: Some(root.clone()),
        ..Default::default()
    };
    fs::write(&path, fb_fixture(1.0)).unwrap();
    assert_eq!(
        crate::tt2tdb::fb_delta_t(&mut s, &parse("50000")).unwrap(),
        1.0
    );
    let key = s.fb.content.clone();
    // Same bytes, newly replaced inode: warm nr==1 must still retain buf.
    let replacement = root.join("replacement");
    fs::write(&replacement, fb_fixture(1.0)).unwrap();
    fs::rename(&replacement, &path).unwrap();
    assert_eq!(
        crate::tt2tdb::fb_delta_t(&mut s, &parse("49996")).unwrap(),
        1.0
    );
    assert_eq!(s.fb.content, key);
    // Same pathname, changed bytes and same record: read the new coefficients.
    fs::write(&path, fb_fixture(7.0)).unwrap();
    assert_eq!(
        crate::tt2tdb::fb_delta_t(&mut s, &parse("50000")).unwrap(),
        7.0
    );
    assert_ne!(s.fb.content, key);
    let key = s.fb.content.clone();
    let cache = s.fb.buf;
    let nr = s.fb.tdbnrl;
    fs::write(&path, &fb_fixture(3.0)[..65]).unwrap();
    assert!(crate::tt2tdb::fb_delta_t(&mut s, &parse("50000")).is_err());
    assert_eq!(s.fb.content, key);
    assert_eq!(s.fb.buf, cache);
    assert_eq!(s.fb.tdbnrl, nr);
    assert!(s.fortran.stream.is_none());
    fs::write(&path, fb_fixture(9.0)).unwrap();
    assert_eq!(
        crate::tt2tdb::fb_delta_t(&mut s, &parse("49996")).unwrap(),
        0.0
    );
    assert_eq!(
        crate::tt2tdb::fb_delta_t(&mut s, &parse("50000")).unwrap(),
        9.0
    );
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn calceph_single_and_multiple_use_current_pulsar() {
    // Supplied CALCEPH with unmatched selector and no dilation needs no IFTE
    // read in the shared body. Full driver lifecycle is covered by V histories.
    let mut p = Probe::new();
    for ps in &mut p.context.pulsars {
        ps.timeEphemeris = Some(0);
        ps.useCalceph = Some(1);
        ps.dilateFreq = Some(0);
    }
    let mut single = p.context.observations(p.views[1]).unwrap()[0].clone();
    crate::tt2tdb::compute_observation(
        &mut p.session,
        &p.context.pulsars[1],
        &mut single,
        &mut 0.0,
    )
    .unwrap();
    let mut retained = 0.0;
    for i in 0..2 {
        let (ps, obs) = p.context.pulsar_observations_mut(i).unwrap();
        crate::tt2tdb::compute_observation(&mut p.session, ps, &mut obs[0], &mut retained).unwrap();
    }
    let actual = &p.context.observations(p.views[1]).unwrap()[0];
    assert_eq!(actual.correctionTT_Teph, single.correctionTT_Teph);
    assert_eq!(actual.correctionTT_TB, single.correctionTT_TB);
    assert_ne!(
        actual.correctionTT_TB,
        p.context.observations(p.views[0]).unwrap()[0].correctionTT_TB
    );
}
#[test]
fn source_remainder_and_unresolved_precision_boundaries() {
    use crate::tempo2_util::{fortran_mod, fortran_nlong};
    let f = |x| Float::with_val(128, x);
    assert_eq!(fortran_mod(f(5.5), &f(2.0)).unwrap(), 1.5);
    for (a, p) in [(-4.0, 2.0), (-0.0, -2.0)] {
        let r = fortran_mod(f(a), &f(p)).unwrap();
        assert!(r.is_zero() && !r.is_sign_negative());
    }
    for a in [0.0, 1.0] {
        assert!(fortran_mod(f(a), &f(f64::INFINITY)).unwrap().is_nan());
    }
    for (a, p) in [
        (1.0, 0.0),
        (f64::INFINITY, 1.0),
        (f64::NAN, 1.0),
        (2147483648.0, 1.0),
    ] {
        assert_eq!(
            fortran_mod(f(a), &f(p)).unwrap_err().field,
            "fortran_mod_quotient_int"
        );
    }
    assert_eq!(fortran_mod(f(-2147483648.0), &f(1.0)).unwrap(), 0);
    // 0049 exact native binary128 input lifted to Rug128: the quotient rounds
    // differently at 113 bits. These assertions retain, not waive, the gap.
    let p = Float::with_val(128, Float::with_val(113, Float::parse("0.1").unwrap()));
    for sign in [-1, 1] {
        let r = fortran_mod(Float::with_val(128, sign), &p).unwrap();
        assert_ne!(r, 0);
        assert_eq!(
            r,
            Float::with_val(128, sign) - Float::with_val(128, &p * (sign * 9))
        );
    }
    let below = Float::with_val(128, 0.5) - (Float::with_val(128, 1) >> 114);
    assert_eq!(fortran_nlong(&below), 0);
    assert_eq!(fortran_nlong(&(-below)), 0);
}
