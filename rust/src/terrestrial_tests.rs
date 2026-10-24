//! W drives the production algorithms with U's explicitly value-initialized caller.
#![allow(non_snake_case, dead_code)]
use crate::{
    TimingError,
    clkcorr::{self, ClockSession},
    clock_text::{self},
    eop::load_eop,
    observation::{read, read_string, write_string},
    observatory::{Observatory, fang, grs80_to_itrf, itrf_to_grs80},
    pulsar::*,
    pulsar_lifecycle::*,
    tempo2::{SourceClockCorrection, SourceObservation},
    tropo::{self, MeteorologyFunction},
};
use rug::Float;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, io::Write, path::Path};
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
fn table(f: &MeteorologyFunction) -> String {
    obj(vec![
        ("filename", chars(&f.table.name_storage, 256)),
        ("header", chars(&f.table.header_storage, 256)),
        ("site", chars(&f.site, 256)),
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
        (
            "meteorology_initialized",
            q(i32::from(s.meteorology.initialized)),
        ),
        ("wet", array(s.meteorology.wet.iter().map(table))),
        ("pressure", array(s.meteorology.pressure.iter().map(table))),
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
fn observations(c: &PulsarContext, view: ObservationView) -> String {
    obj(vec![
        (
            "pulsars",
            array(c.pulsars.iter().map(|p| {
                obj(vec![
                    ("nobs", q(p.nobs.unwrap())),
                    ("correct", q(p.correctTroposphere.unwrap())),
                    ("warnings", q(p.noWarnings.unwrap())),
                    ("override", known(&p.clockFromOverride[..])),
                    (
                        "position",
                        array(p.posPulsarEquatorial.iter().map(|v| num(v.unwrap()))),
                    ),
                ])
            })),
        ),
        (
            "observations",
            array(c.observations(view).unwrap().iter().map(|o| {
                obj(vec![
                    ("sat_binary128", q(binary128(o.sat.as_ref().unwrap()))),
                    ("telID", known(&o.telID)),
                    ("delayCorr", q(o.delayCorr.unwrap())),
                    ("delay", num(o.troposphericDelay.unwrap())),
                    ("zenith", array(o.zenith.iter().map(|v| num(v.unwrap())))),
                ])
            })),
        ),
    ])
}
#[test]
#[ignore]
fn terrestrial_probe() {
    let input = fs::read(std::env::var("TEMPO2_W_INPUT").unwrap()).unwrap();
    let spec: serde_json::Value = serde_json::from_slice(&input).unwrap();
    let mut out = fs::File::create(std::env::var("TEMPO2_W_OUTPUT").unwrap()).unwrap();
    writeln!(
        out,
        "{}",
        obj(vec![
            ("phase", q("launch")),
            ("nonce", q(&spec["nonce"])),
            ("input_sha256", q(format!("{:x}", Sha256::digest(&input))))
        ])
    )
    .unwrap();
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut outcome = "success".to_string();
    let mut env = HashMap::<String, String>::new();
    let mut root_key = "TEMPO2".to_string();
    let mut context = PulsarContext::unwritten(2);
    for p in &mut context.pulsars {
        zero_pulsar(p);
    }
    let view = context.allocate_observations(12).unwrap();
    for o in context.observations_mut(view).unwrap() {
        zero_observation(o);
    }
    context.pulsars[0].obsn = Some(Some(view));
    context.pulsars[1].obsn = Some(Some(ObservationView { offset: 6, ..view }));
    let mut direct = MeteorologyFunction {
        site: vec![b'~'; 256],
        ..MeteorologyFunction::default()
    };
    direct.table.name_storage = vec![b'~'; 256];
    direct.table.header_storage = vec![b'~'; 256];
    let mut samples = Vec::new();
    clkcorr::with_resources(Path::new(""), |s| {
        s.diagnostics.source_directory = spec["source_directory"]
            .as_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        for (event, args) in spec["actions"].as_array().unwrap().iter().enumerate() {
            let a: Vec<_> = args
                .as_array()
                .unwrap()
                .iter()
                .map(|a| a.as_str().unwrap())
                .collect();
            let op = a[0];
            let k = |i: usize| a[i].parse::<i32>().unwrap();
            let n = |i: usize| clock_text::number_prefix(a[i].as_bytes()).unwrap().0;
            let marker = format!("@BEGIN {event} {op}\n");
            stdout.extend(marker.as_bytes());
            stderr.extend(marker.as_bytes());
            writeln!(
                out,
                "{}",
                obj(vec![
                    ("event", q(event)),
                    ("phase", q("begin")),
                    ("operation", q(op))
                ])
            )
            .unwrap();
            let mut r = Vec::new();
            let operation: Result<(), TimingError> = (|| {
                match op {
                    "profile" => r.extend([
                        ("double_digits", q(53)),
                        ("longdouble_size", q(16)),
                        ("longdouble_digits", q(128)),
                        (
                            "observation_size",
                            q(std::mem::size_of::<SourceObservation>()),
                        ),
                        ("pulsar_size", q(std::mem::size_of::<SourcePulsar>())),
                        ("glob_nomatch", q(-3)),
                    ]),
                    "env" => {
                        env.insert(a[1].to_owned(), a[2].to_owned());
                        s.root = env.get(&root_key).map(Into::into);
                        s.alias = env.get("TEMPO2_ALIAS").map(|v| v.as_bytes().to_vec());
                    }
                    "unset" => {
                        env.remove(a[1]);
                        s.root = env.get(&root_key).map(Into::into);
                        s.alias = env.get("TEMPO2_ALIAS").map(|v| v.as_bytes().to_vec());
                    }
                    "root_key" => {
                        root_key = a[1].into();
                        s.root = env.get(&root_key).map(Into::into);
                    }
                    "layout" => {}
                    "debug" => {
                        s.diagnostics.debug = k(1) != 0;
                        s.diagnostics.display_version = k(2) == 1;
                        s.diagnostics.quiet = k(3) != 0;
                    }
                    "read_obs" => s
                        .observatories
                        .read_file(Path::new(a[1]), &mut s.diagnostics)?,
                    "read_alias" => s
                        .observatories
                        .read_aliases(Path::new(a[1]), &mut s.diagnostics)?,
                    "init_obs" => s.observatories.initialize(
                        s.root.as_deref(),
                        s.alias.as_deref(),
                        &mut s.diagnostics,
                    )?,
                    "alias" => {
                        let mut src = [Some(b'~'); 1024];
                        let mut dst = [Some(b'!'); 1024];
                        write_string(&mut src, a[1].as_bytes(), "fixture")?;
                        if k(2) != 0 {
                            s.observatories.lookup_alias(
                                a[1].as_bytes(),
                                &mut src,
                                s.root.as_deref(),
                                s.alias.as_deref(),
                                &mut s.diagnostics,
                            )?;
                        } else {
                            s.observatories.lookup_alias(
                                a[1].as_bytes(),
                                &mut dst,
                                s.root.as_deref(),
                                s.alias.as_deref(),
                                &mut s.diagnostics,
                            )?;
                        }
                        r.push(("input", known(&src)));
                        r.push(("output", known(if k(2) != 0 { &src } else { &dst })));
                    }
                    "lookup" => {
                        let i = s.observatories.index(
                            a[1].as_bytes(),
                            s.root.as_deref(),
                            s.alias.as_deref(),
                            &mut s.diagnostics,
                        )?;
                        let o = &s.observatories.observatories[i];
                        r.extend([
                            ("index", q(i)),
                            ("coordinates", coords(o)),
                            ("name", text(&o.name()?)),
                        ]);
                    }
                    "fang" => r.push(("value", num(fang(k(1), n(2))?))),
                    "geometry" => {
                        let mut o = Observatory::default();
                        if a[1] == "forward" {
                            o.latitude = n(2);
                            o.longitude = n(3);
                            o.height = n(4);
                            grs80_to_itrf(&mut o);
                        } else {
                            o.xyz = [n(2), n(3), n(4)];
                            itrf_to_grs80(&mut o);
                        }
                        r.push(("first", coords(&o)));
                        if a[1] == "forward" {
                            itrf_to_grs80(&mut o)
                        } else {
                            grs80_to_itrf(&mut o)
                        }
                        r.push(("roundtrip", coords(&o)));
                    }
                    "eop_seed" => samples.push([n(1), n(2), n(3), n(4)]),
                    "load_eop" => {
                        load_eop(
                            &mut samples,
                            env.get("TEMPO2").map(Path::new),
                            a[1].as_bytes(),
                            &mut s.diagnostics,
                        )?;
                        r.push((
                            "samples",
                            array(samples.iter().map(|r| array(r.iter().copied().map(num)))),
                        ));
                    }
                    "eop" => {
                        s.eop.initialize(
                            env.get("TEMPO2").map(Path::new),
                            a[3].as_bytes(),
                            &mut s.diagnostics,
                        )?;
                        let mut v = [901.0, 902.0, 903.0, 904.0];
                        let mask = k(4);
                        s.eop.get_eop(
                            n(1),
                            &mut v,
                            [
                                0,
                                if mask == 1 { 0 } else { 1 },
                                if mask != 0 { 0 } else { 2 },
                                if mask == 2 { 0 } else { 3 },
                            ],
                            k(2),
                            &mut s.diagnostics,
                        )?;
                        r.push(("outputs", array(v.map(num))));
                    }
                    "met_load" => {
                        direct.load(Path::new(a[1]), &mut s.diagnostics)?;
                        r.push(("table", table(&direct)));
                    }
                    "met_value" => {
                        r.push(("value", num(direct.get_value(n(1), &mut s.diagnostics)?)))
                    }
                    "met_bounds" => {
                        r.extend([("start", num(direct.start()?)), ("end", num(direct.end()?))])
                    }
                    "met_init" => {
                        s.meteorology
                            .initialize(s.root.as_deref(), k(1), &mut s.diagnostics)?
                    }
                    "met_kind" => tropo::initialize_meteorology_table(
                        s.root.as_deref(),
                        k(1),
                        b"atmosphere",
                        a[2].as_bytes(),
                        if a[2] == "zwd" {
                            &mut s.meteorology.wet
                        } else {
                            &mut s.meteorology.pressure
                        },
                        a[2].as_bytes(),
                        &mut s.diagnostics,
                    )?,
                    "met" => r.push((
                        "value",
                        num(s
                            .meteorology
                            .get_value(
                                s.root.as_deref(),
                                a[1] != "zwd",
                                a[2].as_bytes(),
                                n(3),
                                k(4),
                                &mut s.diagnostics,
                            )?
                            .value),
                    )),
                    "wet" => r.push((
                        "value",
                        num(s
                            .meteorology
                            .wet(
                                s.root.as_deref(),
                                a[1].as_bytes(),
                                n(2),
                                k(3),
                                &mut s.diagnostics,
                            )?
                            .value),
                    )),
                    "pressure" => r.push((
                        "value",
                        num(s
                            .meteorology
                            .pressure(
                                s.root.as_deref(),
                                a[1].as_bytes(),
                                n(2),
                                k(3),
                                &mut s.diagnostics,
                            )?
                            .value),
                    )),
                    "nmf" => r.extend([
                        (
                            "hydrostatic",
                            num(tropo::nmf_hydrostatic(n(1), n(2), n(3), n(4))?),
                        ),
                        ("wet", num(tropo::nmf_wet(n(2), n(4))?)),
                    ]),
                    "pulsar" => {
                        let p = &mut context.pulsars[k(1) as usize];
                        p.nobs = Some(k(2));
                        p.correctTroposphere = Some(k(3));
                        p.noWarnings = Some(k(4));
                        write_string(&mut p.clockFromOverride[..], a[5].as_bytes(), "fixture")?;
                        for j in 0..3 {
                            p.posPulsarEquatorial[j] = Some(n(6 + j));
                        }
                        r.push(("state", observations(&context, view)));
                    }
                    "observation" => {
                        let o = &mut context.observations_mut(view).unwrap()[k(1) as usize];
                        o.sat = Some(Float::with_val(128, Float::parse(a[2]).unwrap()));
                        write_string(&mut o.telID, a[3].as_bytes(), "fixture")?;
                        o.delayCorr = Some(k(4));
                        o.troposphericDelay = Some(n(5));
                        for j in 0..3 {
                            o.zenith[j] = Some(n(6 + j));
                        }
                        r.push(("state", observations(&context, view)));
                    }
                    "compute" => {
                        r.push(("before", observations(&context, view)));
                        let old = context.observations(view).unwrap().to_vec();
                        let psrs: Vec<_> = context
                            .pulsars
                            .iter()
                            .map(SourcePulsar::snapshot_fields)
                            .collect();
                        tropo::compute_tropospheric_delays(s, &mut context, k(1))?;
                        for (a, b) in context.pulsars.iter().zip(&psrs) {
                            retained_pulsar(a, b);
                        }
                        for (a, b) in context.observations(view).unwrap().iter().zip(&old) {
                            retained_observation(a, b);
                        }
                        r.extend([
                            ("after", observations(&context, view)),
                            ("all_other_typed_fields_retained", q(true)),
                        ]);
                    }
                    "correction" => {
                        let o = &context.observations(view).unwrap()[k(1) as usize];
                        r.push((
                            "value",
                            num(s.run(|s| {
                                s.get_correction(o, Some(a[2].as_bytes()), Some(b"UTC"), k(3))
                            })?),
                        ));
                    }
                    "predict" => {
                        let p = &context.pulsars[k(1) as usize];
                        let o = &context.observations(p.obsn.flatten().unwrap()).unwrap()
                            [k(2) as usize];
                        let site = s.observatories.get(
                            &read_string(&o.telID, "telID")?,
                            s.root.as_deref(),
                            s.alias.as_deref(),
                            &mut s.diagnostics,
                        )?;
                        let el = (crate::tempo2_util::dot_product(
                            o.zenith.map(Option::unwrap),
                            std::array::from_fn(|i| p.posPulsarEquatorial[i].unwrap()),
                        ) / site.height)
                            .asin();
                        let code = site.code()?;
                        let sat = read(&o.sat, "sat")?.to_f64();
                        let warnings = p.noWarnings.unwrap();
                        let pressure = s
                            .meteorology
                            .pressure(s.root.as_deref(), &code, sat, warnings, &mut s.diagnostics)?
                            .value;
                        let correction = s.run(|s| {
                            s.get_correction(
                                o,
                                Some(&read_string(&p.clockFromOverride[..], "clock")?),
                                Some(b"UTC"),
                                warnings,
                            )
                        })?;
                        let utc = (read(&o.sat, "sat")?.clone() + correction / 86400.0).to_f64();
                        let zhd = 0.02268 * pressure
                            / (299792458.0
                                * ((-2.8e-7f64).mul_add(
                                    site.height,
                                    (-0.00266f64).mul_add(site.latitude.cos(), 1.0),
                                )));
                        let mh = tropo::nmf_hydrostatic(utc, site.latitude, site.height, el)?;
                        let wet = s
                            .meteorology
                            .wet(s.root.as_deref(), &code, sat, warnings, &mut s.diagnostics)?
                            .value;
                        let mw = tropo::nmf_wet(site.latitude, el)?;
                        r.extend([
                            ("elevation", num(el)),
                            ("utc_mjd", num(utc)),
                            ("correction", num(correction)),
                            ("pressure", num(pressure)),
                            ("wet", num(wet)),
                            ("hydrostatic_mapping", num(mh)),
                            ("wet_mapping", num(mw)),
                            ("delay", num(zhd.mul_add(mh, wet * mw))),
                        ]);
                    }
                    "copy" => {
                        fs::copy(a[1], a[2]).unwrap();
                    }
                    "remove" => {
                        fs::remove_file(a[1]).unwrap();
                    }
                    "glob_nomatch" => {
                        assert!(crate::clock_glob::files(a[1].as_bytes()).is_empty());
                        r.extend([
                            ("status", q(-3)),
                            ("path_count", q(0)),
                            ("pathv_null", q(true)),
                        ]);
                    }
                    _ => panic!("unknown operation {op}"),
                }
                Ok(())
            })();
            for (err, bytes) in s.diagnostics.events.drain(..) {
                if err {
                    stderr.extend(bytes)
                } else {
                    stdout.extend(bytes)
                }
            }
            if let Err(e) = operation {
                outcome = e.field.clone();
                writeln!(
                    out,
                    "{}",
                    obj(vec![
                        ("event", q(event)),
                        ("phase", q("exit")),
                        ("operation", q(op)),
                        ("state", state(s)),
                        (
                            "caller_eop_samples",
                            array(samples.iter().map(|r| array(r.iter().copied().map(num))))
                        ),
                        ("caller_meteorology", table(&direct))
                    ])
                )
                .unwrap();
                break;
            }
            writeln!(
                out,
                "{}",
                obj(vec![
                    ("event", q(event)),
                    ("phase", q("end")),
                    ("operation", q(op)),
                    ("result", obj(r)),
                    ("state", state(s))
                ])
            )
            .unwrap();
            let marker = format!("@END {event} {op}\n");
            stdout.extend(marker.as_bytes());
            stderr.extend(marker.as_bytes());
        }
        Ok(())
    })
    .unwrap();
    let dest = std::env::var("TEMPO2_W_STREAMS").unwrap();
    fs::write(format!("{dest}.stdout"), &stdout).unwrap();
    fs::write(format!("{dest}.stderr"), &stderr).unwrap();
    fs::write(format!("{dest}.outcome"), outcome).unwrap();
}

#[test]
fn wet_mapping_is_continuous_at_latitude_knots() {
    // Independent invariant: interpolation of one coefficient table approaches
    // the next tabulated coefficient from either side. The source's bs/cs mix
    // jumps at these knots; U's wet_mixed_coefficient mutant is a native witness.
    for degrees in [30.0f64, 45.0, 60.0, 75.0] {
        let latitude = degrees.to_radians();
        let before = tropo::nmf_wet(latitude - 1e-10, 0.1).unwrap();
        let after = tropo::nmf_wet(latitude + 1e-10, 0.1).unwrap();
        assert!((before - after).abs() < 1e-9, "{degrees}: {before} {after}");
    }
}

#[test]
fn resource_invalid_storage_has_contextual_boundaries() {
    use crate::{clock_diagnostics::Diagnostics, eop::Eop};
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e30] {
        assert!(fang(1, value).is_err());
        assert!(tropo::nmf_wet(value, 0.1).is_err());
    }
    let mut diagnostics = Diagnostics::default();
    for samples in [
        vec![],
        vec![[1.0, 0.0, 0.0, 0.0]],
        vec![[1.0, 0.0, 0.0, 0.0]; 2],
        vec![[2.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]],
    ] {
        let eop = Eop {
            initialized: true,
            samples,
            resource: None,
        };
        let mut out = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(
            eop.get_eop(1.0, &mut out, [0, 1, 2, 3], 2, &mut diagnostics)
                .unwrap_err()
                .field,
            "eop_bracket_storage"
        );
        assert_eq!(out, [1.0, 2.0, 3.0, 4.0]);
    }
    let eop = Eop {
        initialized: true,
        samples: vec![[1.0, 1.0, 2.0, 0.0], [2.0, 3.0, 4.0, 0.25]],
        resource: None,
    };
    let mut out = [901.0, 902.0, 903.0, 904.0];
    eop.get_eop(1.5, &mut out, [0, 0, 0, 0], 2, &mut diagnostics)
        .unwrap();
    assert_eq!(out, [0.125, 902.0, 903.0, 904.0]);
    assert!(
        eop.get_eop(1.5, &mut out, [0, 1, 2, 4], 2, &mut diagnostics)
            .is_err()
    );
    let root = std::env::temp_dir().join(format!("tempo2-W-boundaries-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let file = root.join("partial");
    fs::write(&file, b"1\n").unwrap();
    let mut session = ClockSession::default();
    assert_eq!(
        session
            .observatories
            .read_file(&file, &mut diagnostics)
            .unwrap_err()
            .field,
        "observatory_unwritten_coordinate"
    );
    fs::write(&file, b"2020 1\n").unwrap();
    let mut samples = Vec::new();
    assert_eq!(
        load_eop(&mut samples, Some(&root), b"partial", &mut diagnostics)
            .unwrap_err()
            .field,
        "eop_fixed_line_storage"
    );
    assert_eq!(
        session
            .meteorology
            .initialize(Some(&root), 1, &mut diagnostics)
            .unwrap_err()
            .field,
        "meteorology_glob_nomatch"
    );
    session
        .meteorology
        .initialize(Some(&root), 0, &mut diagnostics)
        .unwrap();
    assert!(session.meteorology.initialized);
    let mut context = PulsarContext::unwritten(1);
    let view = context.allocate_observations(1).unwrap();
    context.pulsars[0].obsn = Some(Some(view));
    context.pulsars[0].nobs = Some(1);
    context.free_observations(view).unwrap();
    assert_eq!(
        tropo::compute_tropospheric_delays(&mut session, &mut context, 1)
            .unwrap_err()
            .field,
        "pulsar_released_observation_owner"
    );
    fs::remove_dir_all(&root).unwrap();
}

#[test]
#[ignore]
fn resources_after_terminal_clock_fault() {
    let root = std::env::temp_dir().join(format!("tempo2-W-fault-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("observatories"), b"6378237 0 0 North A UTC\n").unwrap();
    fs::write(root.join("weather"), b"# A\n1 90\n2 100\n").unwrap();
    clkcorr::with_session(&root, |_| {
        Err::<(), _>(crate::clock_diagnostics::boundary(
            "intentional_clock_fault",
        ))
    })
    .unwrap_err();
    clkcorr::with_resources(&root, |s| {
        s.observatories
            .read_file(&root.join("observatories"), &mut s.diagnostics)?;
        let mut f = MeteorologyFunction::default();
        f.load(&root.join("weather"), &mut s.diagnostics)?;
        assert_eq!(f.get_value(1.5, &mut s.diagnostics)?, 95.0);
        let mut geometry = s.observatories.observatories[0].clone();
        grs80_to_itrf(&mut geometry);
        assert!(geometry.xyz[0].is_finite());
        Ok(())
    })
    .unwrap();
    assert_eq!(
        clkcorr::with_session(&root, |_| Ok(())).unwrap_err().field,
        "intentional_clock_fault"
    );
    crate::clkcorr::lifecycle_diagnostics(|d| d.stdout(b"independent lifecycle\n")).unwrap();
    fs::remove_dir_all(root).unwrap();
}
