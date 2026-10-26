//! AA's initialized X caller. Only production routines perform calculations.
#![allow(non_snake_case, dead_code)]
use crate::{
    TimingError,
    clkcorr::ClockSession,
    observation::write_string,
    observatory::Observatory,
    pulsar::*,
    pulsar_lifecycle::*,
    tempo2::{Label::*, SourceClockCorrection, SourceObservation, SourceParameter},
};
use rug::Float;
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
include!("terrestrial_fixture_fields.rs");
fn q(v: impl serde::Serialize) -> String {
    serde_json::to_string(&v).unwrap()
}
fn obj(items: Vec<(&str, String)>) -> String {
    format!(
        "{{{}}}",
        items
            .into_iter()
            .map(|(k, v)| format!("{}:{v}", q(k)))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn array(items: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", items.into_iter().collect::<Vec<_>>().join(","))
}
fn num(v: f64) -> String {
    if v.is_nan() {
        obj(vec![("class", q("NaN"))])
    } else if v.is_infinite() {
        obj(vec![(
            "class",
            q(if v > 0.0 { "+Infinity" } else { "-Infinity" }),
        )])
    } else {
        obj(vec![
            ("class", q("finite")),
            ("bits", q(format!("{:016x}", v.to_bits()))),
        ])
    }
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}
fn chars(b: &[u8], capacity: usize) -> String {
    obj(vec![
        ("known", q(b.len())),
        ("capacity", q(capacity)),
        ("bytes", q(hex(b))),
    ])
}
fn known(b: &[Option<u8>]) -> String {
    let v: Vec<_> = b
        .iter()
        .take_while(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();
    assert!(b[v.len()..].iter().all(Option::is_none));
    chars(&v, b.len())
}
fn coords(o: &Observatory) -> String {
    array(
        o.xyz
            .into_iter()
            .chain([o.longitude, o.latitude, o.height])
            .map(num),
    )
}
fn text(b: &[u8]) -> String {
    q(String::from_utf8(b.to_vec()).unwrap())
}
fn state(s: &ClockSession) -> String {
    obj(vec![
        (
            "observatories_initialized",
            q(i32::from(s.observatories.initialized)),
        ),
        (
            "observatories",
            array(s.observatories.observatories.iter().map(|o| {
                obj(vec![
                    ("coordinates", coords(o)),
                    ("name", known(&o.name)),
                    ("code", known(&o.code)),
                    ("clock", known(&o.clock)),
                ])
            })),
        ),
        (
            "aliases",
            array(s.observatories.aliases.iter().map(|a| {
                obj(vec![
                    ("code", known(&a.code)),
                    ("warned", q(a.warned)),
                    ("aliases", array(a.aliases.iter().map(|a| text(a)))),
                ])
            })),
        ),
        ("clock_initialized", q(s.initialized)),
        (
            "clock_functions",
            array(s.functions.iter().map(|f| {
                obj(vec![
                    ("path", q(&f.path)),
                    ("name", text(&f.name)),
                    ("from", text(&f.from)),
                    ("to", text(&f.to)),
                    ("badness", num(f.badness as f64)),
                    ("loaded", q(f.loaded)),
                    ("start", num(f.start)),
                    ("end", num(f.end)),
                    (
                        "samples",
                        array(
                            f.table
                                .samples
                                .iter()
                                .map(|r| array(r.iter().copied().map(num))),
                        ),
                    ),
                ])
            })),
        ),
        (
            "clock_sequences",
            array(s.sequences.iter().map(|f| {
                obj(vec![
                    ("from", text(&f.from)),
                    ("to", text(&f.to)),
                    ("from_key", text(&f.from_key)),
                    ("to_key", text(&f.to_key)),
                    ("start", num(f.start)),
                    ("end", num(f.end)),
                    ("indices", q(&f.indices)),
                ])
            })),
        ),
        ("warnings", q(s.diagnostics.warnings)),
        ("errors", q(s.diagnostics.errors)),
        (
            "warning_log",
            array(s.diagnostics.warning_log.iter().map(|b| text(b))),
        ),
        (
            "error_log",
            array(s.diagnostics.error_log.iter().map(|b| text(b))),
        ),
    ])
}
fn binary128(x: &Float) -> String {
    let x = Float::with_val(113, x);
    let sign = (x.is_sign_negative() as u128) << 127;
    if x.is_zero() {
        return hex(&sign.to_le_bytes());
    }
    let (integer, exp) = x.clone().abs().to_integer_exp().unwrap();
    let n = integer.to_u128().unwrap();
    let len = 128 - n.leading_zeros();
    let mantissa = if len > 113 {
        n >> (len - 113)
    } else {
        n << (113 - len)
    };
    hex(
        &(sign | (((exp + len as i32 - 1 + 16383) as u128) << 112) | (mantissa - (1 << 112)))
            .to_le_bytes(),
    )
}
fn quad(v: &Float) -> String {
    if v.is_nan() {
        obj(vec![("class", q("NaN"))])
    } else if v.is_infinite() {
        obj(vec![(
            "class",
            q(if v.is_sign_negative() {
                "-Infinity"
            } else {
                "+Infinity"
            }),
        )])
    } else {
        obj(vec![
            ("class", q("finite")),
            ("little_endian_bytes", q(binary128(v))),
        ])
    }
}
fn vector(v: &[Option<f64>]) -> String {
    array(v.iter().map(|v| num(v.unwrap())))
}
fn parse(v: &str) -> Float {
    Float::with_val(128, Float::parse(v).unwrap())
}
const PARAMS: [crate::tempo2::Label; 13] = [
    param_raj,
    param_decj,
    param_pmra,
    param_pmdec,
    param_pmra2,
    param_pmdec2,
    param_telx,
    param_tely,
    param_telz,
    param_telEpoch,
    param_tel_dx,
    param_tel_dy,
    param_tel_dz,
];
struct Probe {
    session: ClockSession,
    context: PulsarContext,
    views: Vec<ObservationView>,
}
impl Probe {
    fn new() -> Self {
        let mut c = PulsarContext::unwritten(2);
        let mut views = vec![];
        for j in 0..2 {
            let view = c.allocate_observations(6).unwrap();
            views.push(view);
            let p = &mut c.pulsars[j];
            zero_pulsar(p);
            p.obsn = Some(Some(view));
            p.correctTroposphere = Some(1);
            write_string(&mut p.eopc04_file[..], b"eop", "file").unwrap();
            for label in PARAMS {
                p.param.slots[label as usize] = SourceParameter::allocated(4, 0);
            }
            for k in 0..3 {
                p.posPulsar[k] = Some((100 + j * 10 + k) as f64);
                p.posPulsarEquatorial[k] = Some((200 + j * 10 + k) as f64);
                p.velPulsar[k] = Some((300 + j * 10 + k) as f64);
                p.accPulsar[k] = Some((400 + j * 10 + k) as f64);
            }
            for (k, o) in c.observations_mut(view).unwrap().iter_mut().enumerate() {
                let n = j * 6 + k;
                zero_observation(o);
                o.telID.fill(Some(b'~'));
                write_string(&mut o.telID, b"A", "tel").unwrap();
                o.sat = Some(parse("58010.25"));
                o.delayCorr = Some(1);
                o.clockCorr = Some(1);
                o.correctionUT1 = Some(Float::with_val(128, 900 + n));
                for k in 0..6 {
                    o.observatory_earth[k] = Some((1000 + 10 * n + k) as f64);
                    o.nutations[k] = Some((k + 1) as f64 * 1e-6);
                }
                for k in 0..3 {
                    o.siteVel[k] = Some((2000 + 10 * n + k) as f64);
                    o.zenith[k] = Some((3000 + 10 * n + k) as f64);
                }
                for v in o.flagID.iter_mut() {
                    v.fill(Some(b'~'));
                }
                for v in o.flagVal.iter_mut() {
                    v.fill(Some(b'!'));
                }
            }
        }
        let mut session = ClockSession::default();
        session.diagnostics.source_directory = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../validation/observer_coordinates/build/ordinary/source/")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .as_bytes()
            .to_vec();
        session.diagnostics.source_directory.push(b'/');
        Self {
            session,
            context: c,
            views,
        }
    }
    fn observation(&mut self, i: usize) -> &mut SourceObservation {
        &mut self.context.observations_mut(self.views[i / 6]).unwrap()[i % 6]
    }
    fn caller(&self) -> String {
        obj(vec![
            (
                "pulsars",
                array(self.context.pulsars.iter().enumerate().map(|(j, p)| {
                    obj(vec![
                        ("owner", q(j)),
                        ("observation_owner", q(j * 6)),
                        ("nobs", q(p.nobs.unwrap())),
                        ("method", q(p.t2cMethod.unwrap())),
                        ("ecliptic", q(p.eclCoord.unwrap())),
                        ("warnings", q(p.noWarnings.unwrap())),
                        ("troposphere", q(p.correctTroposphere.unwrap())),
                        ("override", known(&p.clockFromOverride[..])),
                        ("eop_file", known(&p.eopc04_file[..])),
                        ("position", vector(&p.posPulsar[..])),
                        ("equatorial", vector(&p.posPulsarEquatorial[..])),
                        ("velocity", vector(&p.velPulsar[..])),
                        ("acceleration", vector(&p.accPulsar[..])),
                        (
                            "parameters",
                            array(PARAMS.iter().map(|&label| {
                                let par = &p.param.slots[label as usize];
                                obj(vec![
                                    ("index", q(label as i32)),
                                    ("values", array(par.cells.iter().map(|c| quad(&c.val)))),
                                    ("set", array(par.cells.iter().map(|c| q(c.param_set)))),
                                ])
                            })),
                        ),
                        (
                            "offsets",
                            array(
                                [
                                    (p.nTelDX, &p.telDX_t, &p.telDX_v),
                                    (p.nTelDY, &p.telDY_t, &p.telDY_v),
                                    (p.nTelDZ, &p.telDZ_t, &p.telDZ_v),
                                ]
                                .into_iter()
                                .map(|(n, t, v)| {
                                    obj(vec![
                                        ("count", q(n.unwrap())),
                                        (
                                            "backing",
                                            array((0..4).map(|i| {
                                                array([num(t[i].unwrap()), num(v[i].unwrap())])
                                            })),
                                        ),
                                    ])
                                }),
                            ),
                        ),
                    ])
                })),
            ),
            (
                "observations",
                array(self.views.iter().enumerate().flat_map(|(j, &view)| {
                    self.context
                        .observations(view)
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(move |(i, o)| {
                            obj(vec![
                                ("owner", q(j * 6 + i)),
                                ("sat", quad(o.sat.as_ref().unwrap())),
                                ("telID", known(&o.telID)),
                                ("delayCorr", q(o.delayCorr.unwrap())),
                                ("clockCorr", q(o.clockCorr.unwrap())),
                                (
                                    "correctionUT1",
                                    num(o.correctionUT1.as_ref().unwrap().to_f64()),
                                ),
                                ("position", vector(&o.observatory_earth)),
                                ("velocity", vector(&o.siteVel)),
                                ("zenith", vector(&o.zenith)),
                                ("nutations", vector(&o.nutations)),
                                ("nclock", q(o.nclock_correction.unwrap())),
                                (
                                    "clock_steps",
                                    array(o.correctionsTT[..2].iter().map(|c| {
                                        array([num(c.correction.unwrap()), known(&c.corrects_to)])
                                    })),
                                ),
                                ("nFlags", q(o.nFlags.unwrap())),
                                (
                                    "flags",
                                    array((0..o.nFlags.unwrap() as usize).map(|i| {
                                        array([known(&o.flagID[i]), known(&o.flagVal[i])])
                                    })),
                                ),
                            ])
                        })
                })),
            ),
        ])
    }
    fn private(&self) -> String {
        let u = &self.session.ut1;
        if !u.observed {
            return "null".into();
        }
        obj(vec![
            ("read", q(i32::from(u.read))),
            ("mjd1", num(u.mjd1)),
            ("mjd2", num(u.mjd2)),
            ("count", q(u.count)),
            ("count2", q(u.count2)),
            ("mjdVal", q(u.mjd_val.as_slice())),
            ("entry", q(u.entry.as_slice())),
        ])
    }
    fn action(&mut self, a: &[String]) -> Result<String, TimingError> {
        let k = |i: usize| a[i].parse::<i32>().unwrap();
        let n = |i: usize| crate::clock_text::number_prefix(a[i].as_bytes()).unwrap().0;
        let mut result = vec![];
        match a[0].as_str() {
            "profile" => {
                result = vec![
                    ("double_digits", q(53)),
                    ("longdouble_size", q(16)),
                    ("longdouble_digits", q(113)),
                ]
            }
            "env" => match a[1].as_str() {
                "TEMPO2" => self.session.root = Some(a[2].clone().into()),
                "TEMPO2_ALIAS" => self.session.alias = Some(a[2].as_bytes().to_vec()),
                x => panic!("unknown env {x}"),
            },
            "debug" => {
                self.session.diagnostics.debug = k(1) != 0;
                self.session.diagnostics.display_version = k(2) == 1;
                self.session.diagnostics.quiet = k(3) != 0;
            }
            "pulsar" => {
                let p = &mut self.context.pulsars[k(1) as usize];
                p.nobs = Some(k(2));
                p.t2cMethod = Some(k(3));
                p.eclCoord = Some(k(4));
                p.noWarnings = Some(k(5));
                write_string(&mut p.clockFromOverride[..], a[6].as_bytes(), "override")?;
                write_string(&mut p.eopc04_file[..], a[7].as_bytes(), "eop")?;
            }
            "observation" => {
                let o = self.observation(k(1) as usize);
                o.sat = Some(parse(&a[2]));
                write_string(&mut o.telID, a[3].as_bytes(), "tel")?;
                o.delayCorr = Some(k(4));
                o.clockCorr = Some(k(5));
                o.correctionUT1 = Some(Float::with_val(128, n(6)));
            }
            "parameter" => {
                let p = &mut self.context.pulsars[k(1) as usize].param.slots
                    [PARAMS[k(2) as usize] as usize]
                    .cells[k(3) as usize];
                p.val = parse(&a[4]);
                p.param_set = k(5);
            }
            "flag" => {
                let o = self.observation(k(1) as usize);
                let i = o.nFlags.unwrap() as usize;
                write_string(&mut o.flagID[i], a[2].as_bytes(), "flag")?;
                write_string(&mut o.flagVal[i], a[3].as_bytes(), "value")?;
                o.nFlags = Some(i as i32 + 1);
            }
            "clear_flags" => self.observation(k(1) as usize).nFlags = Some(0),
            "tt" => {
                let o = self.observation(k(1) as usize);
                o.nclock_correction = Some(k(2));
                for (i, to) in ["TAI", "TT(TAI)"].into_iter().enumerate() {
                    o.correctionsTT[i].correction = Some(n(i + 3));
                    write_string(&mut o.correctionsTT[i].corrects_to, to.as_bytes(), "to")?;
                }
            }
            "nutation" => {
                let o = self.observation(k(1) as usize);
                for i in 0..6 {
                    o.nutations[i] = Some(n(2 + i));
                }
            }
            "offset" => {
                let p = &mut self.context.pulsars[k(1) as usize];
                let axis = k(2) as usize;
                p.param.slots[PARAMS[10 + axis] as usize].cells[0].val = parse(&a[4]);
                let (count, t, v) = match axis {
                    0 => (&mut p.nTelDX, &mut p.telDX_t, &mut p.telDX_v),
                    1 => (&mut p.nTelDY, &mut p.telDY_t, &mut p.telDY_v),
                    _ => (&mut p.nTelDZ, &mut p.telDZ_t, &mut p.telDZ_v),
                };
                *count = Some(k(3));
                for i in 0..4 {
                    t[i] = Some(n(5 + 2 * i));
                    v[i] = Some(n(6 + 2 * i));
                }
            }
            "vector" | "driver" | "tai" => {
                let mut before = self
                    .context
                    .pulsars
                    .iter()
                    .map(SourcePulsar::snapshot_fields)
                    .collect::<Vec<_>>();
                let mut prev = self
                    .views
                    .iter()
                    .map(|&v| self.context.observations(v).unwrap().to_vec())
                    .collect::<Vec<_>>();
                match a[0].as_str() {
                    "vector" => crate::vector_pulsar::vector_pulsar(
                        &mut self.session,
                        &mut self.context,
                        k(1),
                    )?,
                    "driver" => {
                        crate::get_obs_coord::driver(&mut self.session, &mut self.context, k(1))?
                    }
                    _ => crate::tai2ut1::tai2ut1(&mut self.session, &mut self.context, k(1))?,
                };
                for (i, p) in self.context.pulsars.iter().enumerate() {
                    let b = &mut before[i];
                    b.posPulsar = p.posPulsar.clone();
                    b.posPulsarEquatorial = p.posPulsarEquatorial.clone();
                    b.velPulsar = p.velPulsar.clone();
                    b.accPulsar = p.accPulsar.clone();
                    b.correctTroposphere = p.correctTroposphere;
                    retained_pulsar(b, p);
                    for (o, b) in self
                        .context
                        .observations(self.views[i])
                        .unwrap()
                        .iter()
                        .zip(&mut prev[i])
                    {
                        b.observatory_earth = o.observatory_earth;
                        b.siteVel = o.siteVel;
                        b.zenith = o.zenith;
                        b.correctionUT1 = o.correctionUT1.clone();
                        retained_observation(b, o);
                    }
                }
                result.push(("other_typed_fields_retained", q(true)));
            }
            "ut1" => {
                let s = &mut self.session;
                result.push((
                    "value",
                    num(s
                        .ut1
                        .red(s.root.as_deref(), n(1), k(2), &mut s.diagnostics)?),
                ));
            }
            "lmst" => {
                let v = crate::get_obs_coord::lmst(n(1), n(2))?;
                result = vec![
                    ("return", num(0.0)),
                    ("outputs", array(v.into_iter().map(num))),
                ];
            }
            "precession" => {
                let v = crate::get_obs_coord::get_precession_matrix(n(1), n(2), n(3));
                result = vec![
                    ("return", num(0.0)),
                    (
                        "matrix",
                        array(v.into_iter().map(|r| array(r.into_iter().map(num)))),
                    ),
                ];
            }
            "ang" => result.push(("value", num(crate::get_obs_coord::ang(k(1), n(2))?))),
            "white" => {
                let mut b = [b'~'; 64];
                let v = a[1].as_bytes();
                b[..v.len()].copy_from_slice(v);
                b[v.len()] = 0;
                crate::get_obs_coord::remove_white(&mut b)?;
                result.push(("buffer", chars(&b, 64)));
            }
            "white_hex" => {
                let data = (0..a[1].len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&a[1][i..i + 2], 16).unwrap())
                    .collect::<Vec<_>>();
                let mut b = [0xa5; 66];
                b[1..1 + data.len()].copy_from_slice(&data);
                b[1 + data.len()] = 0;
                crate::get_obs_coord::remove_white(&mut b[1..65])?;
                result.push(("buffer", q(hex(&b))));
            }
            "iau" => {
                let trs = std::array::from_fn(|i| n(i + 4));
                let zen = std::array::from_fn(|i| n(i + 7));
                let (v, _) = crate::get_obs_coord::iau2000b(
                    &mut self.session,
                    trs,
                    zen,
                    &parse(&a[1]),
                    &parse(&a[2]),
                    a[3].as_bytes(),
                )?;
                result = vec![
                    ("input_position", array(trs.into_iter().map(num))),
                    ("input_zenith", array(zen.into_iter().map(num))),
                    ("position", array(v.observatory_earth.into_iter().map(num))),
                    ("zenith", array(v.zenith.into_iter().map(num))),
                    ("velocity", array(v.site_velocity.into_iter().map(num))),
                ];
            }
            "lookup" => {
                let s = &mut self.session;
                let i = s.observatories.index(
                    a[1].as_bytes(),
                    s.root.as_deref(),
                    s.alias.as_deref(),
                    &mut s.diagnostics,
                )?;
                result = vec![
                    ("owner", q(i)),
                    ("coordinates", coords(&s.observatories.observatories[i])),
                ];
            }
            "correction" => {
                let i = k(1) as usize;
                let o = &self.context.observations(self.views[i / 6]).unwrap()[i % 6];
                result.push((
                    "value",
                    num(self.session.get_correction(
                        o,
                        Some(a[2].as_bytes()),
                        Some(a[3].as_bytes()),
                        k(4),
                    )?),
                ));
            }
            "copy" => {
                fs::copy(&a[1], &a[2]).unwrap();
            }
            "remove" => {
                fs::remove_file(&a[1]).unwrap();
            }
            x => panic!("unknown action {x}"),
        }
        Ok(obj(result))
    }
}
#[test]
#[ignore = "AA canonical executable driver; controlled actions and output paths"]
fn observer_probe() {
    let input = std::env::var("TEMPO2_AA_INPUT").unwrap();
    let output = std::env::var("TEMPO2_AA_OUTPUT").unwrap();
    let raw = fs::read(&input).unwrap();
    let challenge: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    let actions: Vec<Vec<String>> = serde_json::from_value(challenge["actions"].clone()).unwrap();
    let mut f = fs::File::create(&output).unwrap();
    fs::write(format!("{output}.launch"),serde_json::to_vec(&serde_json::json!({"nonce":challenge["nonce"],"input_sha256":hex(&Sha256::digest(&raw))})).unwrap()).unwrap();
    let mut p = Probe::new();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let mut outcome = "success";
    for (i, a) in actions.iter().enumerate() {
        let op = &a[0];
        let marker = format!("@BEGIN {i} {op}\n");
        out.extend_from_slice(marker.as_bytes());
        err.extend_from_slice(marker.as_bytes());
        writeln!(
            f,
            "{}",
            obj(vec![
                ("event", q(i)),
                ("phase", q("begin")),
                ("operation", q(op)),
                ("caller", p.caller())
            ])
        )
        .unwrap();
        let result = p.action(a);
        for (stderr, bytes) in p.session.diagnostics.events.drain(..) {
            if stderr {
                err.extend(bytes);
            } else {
                out.extend(bytes);
            }
        }
        let mut event = vec![
            ("event", q(i)),
            ("phase", q(if result.is_ok() { "end" } else { "exit" })),
            ("operation", q(op)),
        ];
        if let Ok(r) = &result {
            event.push(("result", r.clone()));
        }
        event.extend([
            ("state", state(&p.session)),
            ("caller", p.caller()),
            ("private", p.private()),
        ]);
        writeln!(f, "{}", obj(event)).unwrap();
        if let Err(e) = result {
            outcome = if e.field == "source_exit" {
                "source_exit"
            } else {
                "boundary"
            };
            fs::write(format!("{output}.error"), format!("{e:?}")).unwrap();
            break;
        }
        let marker = format!("@END {i} {op}\n");
        out.extend_from_slice(marker.as_bytes());
        err.extend_from_slice(marker.as_bytes());
    }
    fs::write(format!("{output}.stdout"), out).unwrap();
    fs::write(format!("{output}.stderr"), err).unwrap();
    fs::write(format!("{output}.outcome"), outcome).unwrap();
}

#[test]
fn ownership_noops_and_revoked_rows() {
    let mut p = Probe::new();
    p.context.pulsars[0].nobs = Some(-1);
    p.context.pulsars[0].obsn = None;
    crate::get_obs_coord::driver(&mut p.session, &mut p.context, 1).unwrap();
    p.context.pulsars[0].t2cMethod = Some(2);
    crate::tai2ut1::tai2ut1(&mut p.session, &mut p.context, 1).unwrap();
    // Vector formation reads no observation owner, even with a positive nobs.
    p.context.pulsars[0].nobs = Some(1);
    crate::vector_pulsar::vector_pulsar(&mut p.session, &mut p.context, 1).unwrap();
    assert_eq!(
        crate::get_obs_coord::driver(&mut p.session, &mut p.context, 1)
            .unwrap_err()
            .field,
        "pulsar_observation_owner"
    );
    p.context.pulsars[0].obsn = Some(Some(p.views[0]));
    p.context.free_observations(p.views[0]).unwrap();
    assert_eq!(
        crate::get_obs_coord::driver(&mut p.session, &mut p.context, 1)
            .unwrap_err()
            .field,
        "pulsar_released_observation_owner"
    );
    let foreign = PulsarContext::unwritten(1)
        .allocate_observations(1)
        .unwrap();
    p.context.pulsars[0].obsn = Some(Some(foreign));
    assert_eq!(
        crate::get_obs_coord::driver(&mut p.session, &mut p.context, 1)
            .unwrap_err()
            .field,
        "pulsar_foreign_observation_owner"
    );
}
#[test]
fn whitespace_overlap_and_boundaries() {
    let mut b = *b"  x\0zzzz";
    crate::get_obs_coord::remove_white(&mut b).unwrap();
    assert_eq!(&b, b"x\0\0\0zzzz");
    let mut b = *b"a \n\tb\0!";
    crate::get_obs_coord::remove_white(&mut b).unwrap();
    assert_eq!(&b, b"a b\0\0\0!");
    let mut unterminated = *b"abc";
    assert!(crate::get_obs_coord::remove_white(&mut unterminated).is_err());
    assert!(crate::get_obs_coord::lmst(f64::NAN, 0.0).is_err());
    assert!(crate::get_obs_coord::lmst(i32::MIN as f64, 0.0).is_err());
    assert!(crate::get_obs_coord::ang(1, f64::INFINITY).is_err());
}
#[test]
fn ut1_failure_is_local_and_prefix_is_retained() {
    let mut p = Probe::new();
    let root = std::env::temp_dir().join(format!("tempo2-aa-ut1-{}", std::process::id()));
    fs::create_dir_all(root.join("clock")).unwrap();
    fs::write(root.join("clock/ut1.dat"), b"header\nheader\n59999\n").unwrap();
    let s = &mut p.session;
    assert_eq!(
        s.ut1
            .red(Some(&root), 58010.0, 0, &mut s.diagnostics)
            .unwrap_err()
            .field,
        "ut1_empty_table"
    );
    assert!(!s.ut1.read);
    assert_eq!(s.ut1.mjd_val[0], 59999);
    assert!(s.fault.is_none());
    // A failed load cannot be mistaken for a ready cache or retried over its prefix.
    fs::write(root.join("clock/ut1.dat"), b"a\nb\n58000 1 2\n58030 3 4\n").unwrap();
    assert!(
        s.ut1
            .red(Some(&root), 58010.0, 0, &mut s.diagnostics)
            .is_err()
    );
    assert_eq!(s.ut1.count, 0);
    crate::vector_pulsar::vector_pulsar(s, &mut p.context, 1).unwrap();
    p.context.pulsars[0].nobs = Some(1);
    write_string(
        &mut p.context.observations_mut(p.views[0]).unwrap()[0].telID,
        b"STL_BAT",
        "tel",
    )
    .unwrap();
    crate::get_obs_coord::driver(s, &mut p.context, 1).unwrap();
    assert_eq!(p.context.pulsars[0].correctTroposphere, Some(0));
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "isolated process: real accepted runtime and process-wide resource history"]
fn existing_frontend_observes_imag_alias_mutation() {
    *crate::clkcorr::test_session() = ClockSession::default();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = crate::load(
        &root.join("tests/terrestrial/fixtures/test1.par"),
        &root.join("tests/terrestrial/fixtures/test2.tim"),
    )
    .unwrap();
    let runtime_root = root.join("../tempo2/T2runtime");
    let mut existing = crate::RuntimeContext::new(&runtime_root).unwrap();
    let before = crate::evaluate_terrestrial(&input, &mut existing).unwrap();
    let mut probe = Probe::new();
    probe.context.pulsars[0].nobs = Some(1);
    probe.context.pulsars[0].t2cMethod = Some(1);
    write_string(
        &mut probe.context.pulsars[0].eopc04_file[..],
        b"earth/eopc04_IAU2000.62-now",
        "eop",
    )
    .unwrap();
    let observation = probe.observation(0);
    observation.sat = input.source_observations()[0].sat.clone();
    observation.clockCorr = Some(2);
    write_string(&mut observation.telID, b"IMAG", "tel").unwrap();
    observation.nFlags = Some(1);
    write_string(&mut observation.flagID[0], b"-telx", "flag").unwrap();
    {
        let mut session = crate::clkcorr::test_session();
        let index = session
            .observatories
            .observatories
            .iter()
            .position(|o| o.code().unwrap().eq_ignore_ascii_case(b"PKS"))
            .unwrap();
        let old = session.observatories.observatories[index].clone();
        write_string(
            &mut observation.flagVal[0],
            (old.xyz[0] + 10000.0).to_string().as_bytes(),
            "value",
        )
        .unwrap();
        // Explicit initialized alias fixture; the production index/driver resolves it.
        let mut code = [None; 100];
        write_string(&mut code, b"PKS", "alias code").unwrap();
        session.observatories.aliases.insert(
            0,
            crate::observatory::AliasList {
                code,
                aliases: vec![b"IMAG".to_vec()],
                warned: false,
            },
        );
        crate::get_obs_coord::driver(&mut session, &mut probe.context, 1).unwrap();
        let current = &session.observatories.observatories[index];
        assert_eq!(current.xyz[0], old.xyz[0] + 10000.0);
        assert_eq!(
            [current.latitude, current.longitude, current.height],
            [old.latitude, old.longitude, old.height]
        );
    }
    let after = crate::evaluate_terrestrial(&input, &mut existing).unwrap();
    let mut fresh = crate::RuntimeContext::new(&runtime_root).unwrap();
    let fresh = crate::evaluate_terrestrial(&input, &mut fresh).unwrap();
    assert_ne!(
        before.corrections[0].observatory_earth_light_s,
        after.corrections[0].observatory_earth_light_s
    );
    for (a, b) in after.corrections.iter().zip(&fresh.corrections) {
        assert_eq!(a.observatory_earth_light_s, b.observatory_earth_light_s);
    }
    *crate::clkcorr::test_session() = ClockSession::default();
}
