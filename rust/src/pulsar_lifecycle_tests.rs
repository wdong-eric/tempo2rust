#![allow(non_snake_case, unused_variables)]
use crate::{
    pulsar::*,
    pulsar_lifecycle::*,
    tempo2::{MAX_PARAMS, ParameterStore, SourceClockCorrection, SourceObservation},
};
use rug::Float;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
};

trait NativeBits: Clone {
    const WIDTH: usize;
    fn bits(&self) -> [u8; 16];
}
macro_rules! bits {($($t:ty),*)=>{$(impl NativeBits for $t {const WIDTH:usize=std::mem::size_of::<Self>();fn bits(&self)->[u8;16]{let mut b=[0;16];b[..Self::WIDTH].copy_from_slice(&self.to_le_bytes());b}})*};}
bits!(u8, i32, u32, i64, u64, f32, f64, usize);
impl NativeBits for Float {
    const WIDTH: usize = 16;
    fn bits(&self) -> [u8; 16] {
        let sign = (self.is_sign_negative() as u128) << 127;
        if self.is_zero() {
            return sign.to_le_bytes();
        }
        assert!(
            self.is_finite(),
            "S ordinary evidence is finite; nonfinite classes have separate tests"
        );
        let (integer, exponent) = self.clone().abs().to_integer_exp().unwrap();
        let n = integer.to_u128().unwrap();
        let len = 128 - n.leading_zeros();
        let top = exponent + len as i32 - 1;
        assert!((-16382..=16383).contains(&top));
        let mantissa = if len > 113 {
            assert_eq!(
                n & ((1 << (len - 113)) - 1),
                0,
                "not exactly representable as native binary128"
            );
            n >> (len - 113)
        } else {
            n << (113 - len)
        };
        (sign | (((top + 16383) as u128) << 112) | (mantissa - (1 << 112))).to_le_bytes()
    }
}
#[derive(Default)]
struct Dump(Vec<Value>);
impl Dump {
    fn unwritten(&mut self, p: &str) {
        self.0.push(json!({"path":p,"state":"unwritten"}));
    }
    fn symbol(&mut self, p: &str, s: &str) {
        self.0.push(json!({"path":p,"symbol":s}));
    }
    fn values<T: NativeBits>(&mut self, p: &str, values: impl IntoIterator<Item = Option<T>>) {
        let mut runs: Vec<(usize, Option<[u8; 16]>)> = Vec::new();
        let mut count = 0;
        let mut known = false;
        for v in values {
            count += 1;
            known |= v.is_some();
            let b = v.map(|v| v.bits());
            match runs.last_mut() {
                Some((n, last)) if *last == b => *n += 1,
                _ => runs.push((1, b)),
            }
        }
        if !known {
            self.unwritten(p);
            return;
        }
        self.0.push(json!({"path":p,"unit":T::WIDTH,"count":count,"runs":runs.into_iter().map(|(n,b)|(n,b.map(|b|b[..T::WIDTH].iter().map(|b|format!("{b:02x}")).collect::<String>()))).collect::<Vec<_>>()}));
    }
    fn pointers(&mut self, p: &str, values: impl IntoIterator<Item = Option<String>>) {
        let mut runs: Vec<(usize, Option<String>)> = Vec::new();
        let mut count = 0;
        for v in values {
            count += 1;
            match runs.last_mut() {
                Some((n, last)) if *last == v => *n += 1,
                _ => runs.push((1, v)),
            }
        }
        self.0
            .push(json!({"path":p,"pointer_count":count,"symbols":runs}));
    }
    fn prefix(&mut self, p: &str, bytes: &[Option<u8>]) {
        let n = bytes.iter().position(|v| *v == Some(0)).unwrap() + 1;
        self.values(p, bytes[..n].iter().copied());
        self.symbol(
            &format!("{p}.suffix"),
            &format!("unwritten:{}", bytes.len() - n),
        );
    }
}
#[derive(Default)]
struct Tags {
    obs: HashMap<ObservationOwner, String>,
    mat: HashMap<MatrixOwner, String>,
    payload: HashMap<PayloadOwner, String>,
    params: HashMap<usize, String>,
}
impl Tags {
    fn observation(&self, v: &ObservationView) -> String {
        format!("{}@{}", self.obs[&v.owner], v.offset)
    }
    fn matrix(&self, v: &MatrixOwner) -> String {
        self.mat[v].clone()
    }
    fn payload(&self, v: &PayloadView) -> String {
        assert_eq!(v.offset, 0);
        self.payload[&v.owner].clone()
    }
    fn params(&mut self, p: &ParameterStore, name: &str) {
        for (i, s) in p.slots.iter().enumerate() {
            self.params
                .insert(s.cells.as_ptr() as usize, format!("{name}.param[{i}]"));
        }
    }
    fn owner(&mut self, p: &SourcePulsar, name: &str) {
        self.obs.insert(
            p.obsn.unwrap().unwrap().owner,
            format!("{name}.observations"),
        );
        self.mat
            .insert(p.covar.unwrap().unwrap(), format!("{name}.covariance"));
        self.params(&p.param, name);
    }
}
fn params(out: &mut Dump, p: &str, store: &ParameterStore, tags: &Tags) {
    for (i, a) in store.slots.iter().enumerate() {
        let n = format!("{p}[{i}]");
        out.values(&format!("{n}.aSize"), [a.a_size]);
        out.values(&format!("{n}.nLinkTo"), [a.n_link_to]);
        out.values(&format!("{n}.nLinkFrom"), [a.n_link_from]);
        out.values(&format!("{n}.linkTo"), a.link_to);
        out.values(&format!("{n}.linkFrom"), a.link_from);
        let owner = &tags.params[&(a.cells.as_ptr() as usize)];
        for f in [
            "val",
            "err",
            "prefit",
            "prefitErr",
            "fitFlag",
            "paramSet",
            "label",
            "shortlabel",
        ] {
            out.symbol(&format!("{n}.{f}"), &format!("{owner}.{f}"));
        }
        out.values(
            &format!("{n}.val.data"),
            a.cells.iter().map(|c| Some(c.val.clone())),
        );
        out.values(
            &format!("{n}.err.data"),
            a.cells.iter().map(|c| Some(c.err.clone())),
        );
        out.values(
            &format!("{n}.fitFlag.data"),
            a.cells.iter().map(|c| Some(c.fit_flag)),
        );
        out.values(
            &format!("{n}.paramSet.data"),
            a.cells.iter().map(|c| Some(c.param_set)),
        );
        out.values(
            &format!("{n}.prefit.data"),
            a.cells.iter().map(|c| c.prefit.clone()),
        );
        out.values(
            &format!("{n}.prefitErr.data"),
            a.cells.iter().map(|c| c.prefit_err.clone()),
        );
        for (j, c) in a.cells.iter().enumerate() {
            for f in ["label", "shortlabel"] {
                out.symbol(&format!("{n}.{f}[{j}]"), &format!("{owner}.{f}[{j}]"));
            }
            out.values(
                &format!("{n}.label[{j}].data"),
                c.label.iter().copied().map(Some),
            );
            out.values(
                &format!("{n}.shortlabel[{j}].data"),
                c.shortlabel.iter().copied().map(Some),
            );
        }
    }
}
fn seed_string(s: &mut [Option<u8>], v: i32) {
    s.fill(Some(b'A' + (v % 20) as u8));
    s[..4].copy_from_slice(&[
        Some(b's'),
        Some(b'0' + (v / 10) as u8),
        Some(b'0' + (v % 10) as u8),
        Some(0),
    ]);
}
fn seed_params(p: &mut ParameterStore, v: i32) {
    for a in &mut p.slots {
        a.n_link_to = Some(v);
        a.n_link_from = Some(v);
        a.link_to.fill(Some(v));
        a.link_from.fill(Some(v));
        for c in &mut a.cells {
            c.val = Float::with_val(128, v as f64 + 0.125);
            c.err = c.val.clone();
            c.prefit = Some(c.val.clone());
            c.prefit_err = Some(c.val.clone());
            c.fit_flag = v;
            c.param_set = v;
            for (s, t) in [
                (&mut c.label, format!("label-{v}")),
                (&mut c.shortlabel, format!("s{v}")),
            ] {
                s.fill(b'A' + (v % 20) as u8);
                s[..t.len()].copy_from_slice(t.as_bytes());
                s[t.len()] = 0;
            }
        }
    }
}
fn deriv(_: &mut PulsarContext, _: i32, _: f64, _: i32, _: i32, _: i32) -> f64 {
    0.0
}
fn constraint(
    _: &mut PulsarContext,
    _: i32,
    _: i32,
    _: i32,
    _: i32,
    _: i32,
    _: Option<&[Option<u8>]>,
) -> f64 {
    0.0
}
fn update(_: &mut PulsarContext, _: i32, _: i32, _: i32, _: f64, _: f64) {}
include!("pulsar_probe_fields.rs");
fn seed_owner(
    c: &mut PulsarContext,
    i: usize,
    v: i32,
    name: &str,
    tags: &mut Tags,
    opaque: PayloadView,
) {
    seed_pulsar(&mut c.pulsars[i], v, opaque);
    c.allocate_memory(i, 0).unwrap();
    seed_params(&mut c.pulsars[i].param, v);
    let obs = c.allocate_observations(4).unwrap();
    for (j, o) in c.observations_mut(obs).unwrap().iter_mut().enumerate() {
        seed_observation(o, v + j as i32, opaque);
    }
    c.pulsars[i].obsn = Some(Some(obs));
    let m = c.malloc_blas(2, 3).unwrap();
    c.matrix_mut(m).unwrap().cells_mut().fill(v as f64 + 0.25);
    c.pulsars[i].covar = Some(Some(m));
    let m = c.malloc_blas(1, 2).unwrap();
    c.matrix_mut(m)
        .unwrap()
        .cells_mut()
        .copy_from_slice(&[v as f64, v as f64 + 1.0]);
    c.pulsars[i].ToAextraCovar = Some(Some(m));
    tags.mat.insert(m, format!("{name}.extraCovar"));
    for j in 0..MAX_PARAMS {
        let bytes = format!("constraint-{v}-{j}")
            .bytes()
            .chain([0])
            .map(Some)
            .collect();
        let view = c.allocate_payload(bytes).unwrap();
        c.pulsars[i].constraint_special[j] = Some(Some(view));
        tags.payload
            .insert(view.owner, format!("{name}.constraint[{j}]"));
    }
    tags.owner(&c.pulsars[i], name);
}
fn matrix(out: &mut Dump, p: &str, c: &PulsarContext, m: MatrixOwner) {
    let m = c.matrix(m).unwrap();
    let rows = m.get_blas_rows();
    let cols = m.get_blas_cols();
    out.values(&format!("{p}.rows"), [Some(rows)]);
    out.values(&format!("{p}.cols"), [Some(cols)]);
    out.values(
        &format!("{p}.row_offsets"),
        (0..rows).map(|r| {
            Some(
                (m.row(r).unwrap().as_ptr() as usize - m.cells().as_ptr() as usize)
                    / std::mem::size_of::<f64>(),
            )
        }),
    );
    out.values(&format!("{p}.cells"), m.cells().iter().copied().map(Some));
}
fn payloads(out: &mut Dump, p: &str, c: &PulsarContext, x: &SourcePulsar) {
    for (i, v) in x.constraint_special.iter().enumerate() {
        if let Some(Some(v)) = v {
            let b = c.payload(*v).unwrap();
            let n = b.iter().position(|b| *b == Some(0)).unwrap() + 1;
            out.values(
                &format!("{p}.constraint_special[{i}].data"),
                b[..n].iter().copied(),
            );
        }
    }
}
fn full(out: &mut Dump, p: &str, c: &PulsarContext, x: &SourcePulsar, tags: &Tags) {
    dump_pulsar(out, p, x, tags);
    payloads(out, p, c, x);
}
fn observation_data(out: &mut Dump, p: &str, c: &PulsarContext, x: &SourcePulsar, tags: &Tags) {
    for (i, o) in c
        .observations(x.obsn.unwrap().unwrap())
        .unwrap()
        .iter()
        .enumerate()
    {
        dump_observation(out, &format!("{p}.obsn.data[{i}]"), o, tags);
    }
}
fn state(out: &mut Dump, c: &PulsarContext, x: &SourcePulsar, tags: &Tags, seeded: bool) {
    if seeded {
        full(out, "p", c, x, tags);
    } else {
        natural(out, "p", x, tags);
    }
    matrix(out, "p.covar.data", c, x.covar.unwrap().unwrap());
    if seeded {
        if let Some(Some(m)) = x.ToAextraCovar {
            matrix(out, "p.ToAextraCovar.data", c, m);
        }
        observation_data(out, "p", c, x, tags);
    } else {
        out.unwritten("p.obsn.data");
    }
}
fn event(file: &mut impl Write, phase: &str, f: impl FnOnce(&mut Dump)) {
    let mut out = Dump::default();
    f(&mut out);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    for d in crate::take_clock_diagnostics() {
        if d.source_stderr {
            stderr.extend(d.bytes);
        } else {
            stdout.extend(d.bytes);
        }
    }
    serde_json::to_writer(&mut *file, &json!({"phase":phase,"state":out.0,"stdout":String::from_utf8(stdout).unwrap(),"stderr":String::from_utf8(stderr).unwrap()})).unwrap();
    writeln!(file).unwrap();
    file.flush().unwrap();
}
#[test]
#[ignore = "T challenged process probe"]
fn lifecycle_probe() {
    let input_path = std::env::var("TEMPO2_T_INPUT").unwrap();
    let bytes = fs::read(&input_path).unwrap();
    let input: Value = serde_json::from_slice(&bytes).unwrap();
    let mut file =
        BufWriter::new(fs::File::create(std::env::var("TEMPO2_T_OUTPUT").unwrap()).unwrap());
    use sha2::{Digest, Sha256};
    writeln!(file,"{}",json!({"phase":"launch","nonce":input["nonce"],"input_sha256":format!("{:x}",Sha256::digest(&bytes))})).unwrap();
    let args: Vec<String> = serde_json::from_value(input["args"].clone()).unwrap();
    let op = args[0].as_str();
    let arg = |i: usize| args[i].parse::<i32>().unwrap();
    let mut c = PulsarContext::unwritten(if op == "wrapper" {
        arg(1).max(0) as usize
    } else {
        2
    });
    let mut tags = Tags::default();
    let mut opaque = Vec::new();
    for (name, bytes) in [
        ("source", b"opaque-source".as_slice()),
        ("destination", b"opaque-destination".as_slice()),
    ] {
        let mut b = vec![Some(0); 32];
        for (dst, src) in b.iter_mut().zip(bytes) {
            *dst = Some(*src);
        }
        let v = c.allocate_payload(b).unwrap();
        tags.payload.insert(v.owner, format!("opaque:{name}"));
        opaque.push(v);
    }
    event(&mut file, "profile", |out| {
        out.values(
            "opaque_source",
            c.payload(opaque[0]).unwrap().iter().copied(),
        );
        out.values(
            "opaque_destination",
            c.payload(opaque[1]).unwrap().iter().copied(),
        );
        out.values("MAX_OBSN", [Some(4i32)]);
        out.values("MAX_FIT", [Some(MAX_FIT as i32)]);
    });
    match op {
        "matrix" | "uinv" => {
            event(&mut file, "before_allocate", |_| {});
            let result = if op == "uinv" {
                c.malloc_uinv(arg(1) as usize)
            } else {
                c.malloc_blas(arg(1) as usize, arg(2) as usize)
            };
            if let Ok(m) = result {
                event(&mut file, "allocated", |o| matrix(o, "matrix", &c, m));
                let a = c.matrix_mut(m).unwrap().cells_mut();
                a[0] = 3.25;
                *a.last_mut().unwrap() = -7.5;
                event(&mut file, "caller_writes", |o| matrix(o, "matrix", &c, m));
                if op == "uinv" {
                    c.free_uinv(m).unwrap()
                } else {
                    c.free_blas(m).unwrap()
                }
                event(&mut file, "destroy_returned", |_| {});
            } else {
                assert_eq!(result.unwrap_err().kind, "zero_dimension_assertion");
            }
        }
        "natural" | "wrapper" => {
            let count = if op == "wrapper" { arg(1) } else { 1 };
            event(&mut file, "before_initialise", |o| {
                o.symbol("all_pulsars", "unwritten");
                o.values("MAX_PSR", [Some(count)]);
            });
            if op == "wrapper" {
                let mut d = crate::clock_diagnostics::Diagnostics::default();
                if args.len() > 3 {
                    d.debug = arg(3) != 0;
                }
                if args.len() > 4 {
                    d.display_version = arg(4) == 1;
                }
                crate::clkcorr::with_diagnostics(|shared| {
                    shared.debug = d.debug;
                    shared.display_version = d.display_version;
                    Ok(())
                })
                .unwrap();
                c.initialise(count, 4, arg(2)).unwrap();
            } else {
                c.initialise_one(0, 4, arg(2), arg(1)).unwrap();
            }
            for i in 0..count as usize {
                tags.owner(&c.pulsars[i], &format!("owner{i}"));
                event(&mut file, &format!("initialised_{i}"), |o| {
                    state(o, &c, &c.pulsars[i], &tags, false)
                });
            }
            for i in 0..count as usize {
                c.destroy_one(i).unwrap();
            }
            event(&mut file, "destroy_returned", |_| {});
        }
        "allocate" => {
            event(&mut file, "before_allocate", |o| o.symbol("p", "unwritten"));
            c.allocate_memory(0, arg(1)).unwrap();
            tags.params(&c.pulsars[0].param, "first");
            event(&mut file, "allocated", |o| {
                params(o, "p.param", &c.pulsars[0].param, &tags)
            });
            seed_params(&mut c.pulsars[0].param, 11);
            event(&mut file, "caller_seeded", |o| {
                params(o, "p.param", &c.pulsars[0].param, &tags)
            });
            let old = c.allocate_memory(0, -arg(1)).unwrap();
            tags.params(&c.pulsars[0].param, "second");
            event(&mut file, "reallocated", |o| {
                params(o, "p.param", &c.pulsars[0].param, &tags);
                params(o, "old.param", &old, &tags);
            });
            c.pulsars[1].param = old;
            c.destroy_memory(0).unwrap();
            c.destroy_memory(1).unwrap();
            event(&mut file, "destroy_returned", |_| {});
        }
        "repeat" => {
            seed_owner(&mut c, 0, 11, "first", &mut tags, opaque[0]);
            event(&mut file, "caller_seeded", |o| {
                state(o, &c, &c.pulsars[0], &tags, true)
            });
            let mut old = c.pulsars[0].snapshot_fields();
            old.param = c.initialise_one(0, 4, arg(1), arg(2)).unwrap();
            tags.owner(&c.pulsars[0], "second");
            event(&mut file, "reinitialised", |o| {
                full(o, "p", &c, &c.pulsars[0], &tags);
                matrix(o, "p.covar.data", &c, c.pulsars[0].covar.unwrap().unwrap());
                o.unwritten("p.obsn.data");
                full(o, "old", &c, &old, &tags);
                matrix(o, "old.covar.data", &c, old.covar.unwrap().unwrap());
                matrix(
                    o,
                    "old.ToAextraCovar.data",
                    &c,
                    old.ToAextraCovar.unwrap().unwrap(),
                );
                observation_data(o, "old", &c, &old, &tags);
            });
            seed_params(&mut c.pulsars[0].param, 22);
            let view = c.pulsars[0].obsn.unwrap().unwrap();
            for (i, o) in c.observations_mut(view).unwrap().iter_mut().enumerate() {
                seed_observation(o, 22 + i as i32, opaque[1]);
            }
            event(&mut file, "second_caller_seeded", |o| {
                state(o, &c, &c.pulsars[0], &tags, true)
            });
            let mut old2 = c.pulsars[0].snapshot_fields();
            old2.param = c.initialise_one(0, 4, arg(2), arg(1)).unwrap();
            tags.owner(&c.pulsars[0], "third");
            event(&mut file, "initialised_again", |o| {
                full(o, "p", &c, &c.pulsars[0], &tags);
                matrix(o, "p.covar.data", &c, c.pulsars[0].covar.unwrap().unwrap());
                o.unwritten("p.obsn.data");
                full(o, "old2", &c, &old2, &tags);
                matrix(o, "old2.covar.data", &c, old2.covar.unwrap().unwrap());
                observation_data(o, "old2", &c, &old2, &tags);
            });
            c.pulsars.push(old2);
            c.pulsars.push(old);
            c.destroy_one(0).unwrap();
            c.destroy_one(2).unwrap();
            c.free_blas(c.pulsars[3].ToAextraCovar.unwrap().unwrap())
                .unwrap();
            c.destroy_one(3).unwrap();
            event(&mut file, "destroy_returned", |_| {});
        }
        "destroy" | "destroy_null" => {
            c.initialise_one(0, 4, 0, 1).unwrap();
            tags.owner(&c.pulsars[0], "owner");
            if op == "destroy_null" {
                c.free_observations(c.pulsars[0].obsn.unwrap().unwrap())
                    .unwrap();
                c.pulsars[0].obsn = Some(None);
            }
            for (i, b) in [
                (0, b"first".as_slice()),
                (MAX_PARAMS - 1, b"last".as_slice()),
            ] {
                let v = c
                    .allocate_payload(b.iter().copied().chain([0]).map(Some).collect())
                    .unwrap();
                c.pulsars[0].constraint_special[i] = Some(Some(v));
                tags.payload.insert(v.owner, format!("constraint:{i}"));
            }
            event(&mut file, "before_destroy", |o| {
                state(o, &c, &c.pulsars[0], &tags, false)
            });
            event(&mut file, "constraint_payloads", |o| {
                for i in [0, MAX_PARAMS - 1] {
                    let b = c
                        .payload(c.pulsars[0].constraint_special[i].unwrap().unwrap())
                        .unwrap();
                    o.prefix(&i.to_string(), b);
                }
            });
            c.destroy_one(0).unwrap();
            event(&mut file, "destroy_returned", |_| {});
        }
        "copy" | "copy_repeat" => {
            seed_owner(&mut c, 0, 11, "source", &mut tags, opaque[0]);
            seed_owner(&mut c, 1, 22, "destination", &mut tags, opaque[1]);
            c.pulsars[0].nobs = Some(arg(1));
            c.pulsars[0].nJumps = Some(arg(2));
            c.pulsars[1].nfdJumps = Some(arg(3));
            c.pulsars[0].nfdJumps = Some(arg(4));
            let alias = &args[5];
            let mut shared = None;
            if alias != "disjoint" {
                for i in 0..2 {
                    c.free_observations(c.pulsars[i].obsn.unwrap().unwrap())
                        .unwrap();
                }
                let v = c.allocate_observations(5).unwrap();
                for (i, o) in c.observations_mut(v).unwrap().iter_mut().enumerate() {
                    seed_observation(o, 31 + i as i32, opaque[1]);
                }
                tags.obs.insert(v.owner, "shared-observations".into());
                c.pulsars[0].obsn = Some(Some(ObservationView {
                    offset: usize::from(alias == "backward_overlap"),
                    ..v
                }));
                c.pulsars[1].obsn = Some(Some(ObservationView {
                    offset: usize::from(alias == "forward_overlap"),
                    ..v
                }));
                shared = Some(v);
            }
            for round in 0..if op == "copy_repeat" { 2 } else { 1 } {
                for after in [false, true] {
                    if after {
                        c.copy_psr(0, 1).unwrap();
                    }
                    event(
                        &mut file,
                        &format!("{}_copy_{round}", if after { "after" } else { "before" }),
                        |o| {
                            full(o, "source", &c, &c.pulsars[0], &tags);
                            full(o, "destination", &c, &c.pulsars[1], &tags);
                            for (i, name) in [(0, "source"), (1, "destination")] {
                                matrix(
                                    o,
                                    &format!("{name}.covar.data"),
                                    &c,
                                    c.pulsars[i].covar.unwrap().unwrap(),
                                );
                            }
                            for (i, name) in [(0, "source"), (1, "destination")] {
                                matrix(
                                    o,
                                    &format!("{name}.ToAextraCovar.data"),
                                    &c,
                                    c.pulsars[i].ToAextraCovar.unwrap().unwrap(),
                                );
                            }
                            if let Some(v) = shared {
                                for (i, r) in c.observations(v).unwrap().iter().enumerate() {
                                    dump_observation(o, &format!("shared.data[{i}]"), r, &tags);
                                }
                            } else {
                                for (i, name) in [(0, "source"), (1, "destination")] {
                                    observation_data(o, name, &c, &c.pulsars[i], &tags);
                                }
                            }
                        },
                    );
                }
                if round == 0 && op == "copy_repeat" {
                    c.pulsars[0].nobs = Some(1);
                    c.pulsars[0].nJumps = Some(0);
                    c.pulsars[1].nfdJumps = Some(1);
                }
            }
            if let Some(v) = shared {
                c.pulsars[0].obsn = Some(None);
                c.pulsars[1].obsn = Some(None);
                c.free_observations(v).unwrap();
            }
            for i in 0..2 {
                c.free_blas(c.pulsars[i].ToAextraCovar.unwrap().unwrap())
                    .unwrap();
                c.destroy_one(i).unwrap();
            }
            event(&mut file, "destroy_returned", |_| {});
        }
        _ => panic!("unknown T case"),
    }
    let diagnostics = crate::take_clock_diagnostics();
    fs::write(
        std::env::var("TEMPO2_T_DIAGNOSTICS").unwrap(),
        serde_json::to_vec(
            &diagnostics
                .iter()
                .map(|d| json!({"source_stderr":d.source_stderr,"bytes":d.bytes}))
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    )
    .unwrap();
}

#[test]
fn partial_links_and_retired_allocations_keep_original_owners() {
    let mut c = PulsarContext::unwritten(1);
    c.pulsars[0].param.slots[0].link_to[2] = Some(123);
    c.pulsars[0].param.slots[MAX_PARAMS - 1].link_from[4] = Some(-9);
    assert!(c.pulsars[0].param.slots[0].a_size.is_none());
    let unallocated = c.allocate_memory(0, -99).unwrap();
    assert!(unallocated.slots[0].a_size.is_none());
    assert_eq!(
        c.pulsars[0].param.slots[0].link_to,
        [None, None, Some(123), None, None]
    );
    let addr = c.pulsars[0].param.slots[0].cells.as_ptr();
    c.pulsars[0].param.slots[0].cells[0].prefit = Some(Float::with_val(128, 7));
    let old = c.allocate_memory(0, 12).unwrap();
    assert_eq!(old.slots[0].cells.as_ptr(), addr);
    assert_ne!(c.pulsars[0].param.slots[0].cells.as_ptr(), addr);
    assert_eq!(old.slots[0].cells[0].prefit.as_ref().unwrap(), &7);
    assert!(c.pulsars[0].param.slots[0].cells[0].prefit.is_none());
    assert_eq!(
        c.pulsars[0].param.slots[MAX_PARAMS - 1].link_from,
        [None, None, None, None, Some(-9)]
    );
    assert_eq!(
        c.pulsars[0].param.slots[MAX_PARAMS - 1].n_link_from,
        Some(0)
    );
}
#[test]
fn independent_clone_preserves_internal_aliases_and_detaches_matrix_writes() {
    let mut c = PulsarContext::unwritten(2);
    let v = c.allocate_observations(4).unwrap();
    c.observations_mut(v).unwrap()[1].freq = Some(11.0);
    c.pulsars[0].obsn = Some(Some(v));
    c.pulsars[1].obsn = Some(Some(ObservationView { offset: 1, ..v }));
    let m = c.malloc_blas(2, 3).unwrap();
    c.pulsars[0].covar = Some(Some(m));
    c.pulsars[1].covar = Some(Some(m));
    let p = c.allocate_payload(vec![Some(0), None, Some(255)]).unwrap();
    c.pulsars[0].constraint_special[0] = Some(Some(p));
    c.pulsars[1].fitinfo.constraintSpecial[0] = Some(Some(p));
    c.pulsars[0].fitinfo.paramDerivs[1] = Some(None);
    c.pulsars[0].fitinfo.paramDerivs[2] = Some(Some(Callback {
        identity: 77,
        callable: deriv,
    }));
    let mut clone = c.clone();
    let cm = clone.pulsars[0].covar.unwrap().unwrap();
    assert_eq!(clone.pulsars[1].covar, Some(Some(cm)));
    assert_eq!(
        c.matrix(m).unwrap().cells().as_ptr(),
        clone.matrix(cm).unwrap().cells().as_ptr()
    );
    clone.matrix_mut(cm).unwrap().cells_mut()[5] = 9.0;
    assert_eq!(c.matrix(m).unwrap().cells()[5], 0.0);
    assert_eq!(
        clone
            .matrix(clone.pulsars[1].covar.unwrap().unwrap())
            .unwrap()
            .cells()[5],
        9.0
    );
    let cv = clone.pulsars[0].obsn.unwrap().unwrap();
    clone.observations_mut(cv).unwrap()[1].freq = Some(22.0);
    assert_eq!(
        clone
            .observations(clone.pulsars[1].obsn.unwrap().unwrap())
            .unwrap()[0]
            .freq,
        Some(22.0)
    );
    assert_eq!(c.observations(v).unwrap()[1].freq, Some(11.0));
    assert_eq!(
        clone
            .payload(
                clone.pulsars[1].fitinfo.constraintSpecial[0]
                    .unwrap()
                    .unwrap()
            )
            .unwrap(),
        [Some(0), None, Some(255)]
    );
    assert!(clone.pulsars[0].fitinfo.paramDerivs[0].is_none());
    assert!(clone.pulsars[0].fitinfo.paramDerivs[1].unwrap().is_none());
    assert_eq!(
        clone.pulsars[0].fitinfo.paramDerivs[2]
            .unwrap()
            .unwrap()
            .identity,
        77
    );
    assert_eq!(clone.matrix(m).unwrap_err().kind, "foreign_owner");
    let moved = clone;
    assert_eq!(moved.matrix(cm).unwrap().get_blas_cols(), 3);
}
#[test]
fn copy_prefix_terminal_boundary_and_fresh_context_recovery() {
    let mut c = PulsarContext::unwritten(2);
    c.pulsars[0].name[0] = Some(b'A');
    c.pulsars[0].name[2] = Some(0);
    c.pulsars[1].name.fill(Some(b'z'));
    let error = c.copy_psr(0, 1).unwrap_err();
    assert_eq!(error.field, "name[1]");
    assert_eq!(c.pulsars[1].name[..3], [Some(b'A'), Some(b'z'), Some(b'z')]);
    assert_eq!(c.allocate_observations(1).unwrap_err(), error);
    let mut fresh = PulsarContext::unwritten(2);
    fresh.pulsars[0].name[..2].copy_from_slice(&[Some(b'A'), Some(0)]);
    fresh.pulsars[0].binaryModel[..3].copy_from_slice(&[Some(b'B'), Some(b'C'), Some(0)]);
    fresh.pulsars[1].binaryModel.fill(Some(b'z'));
    assert_eq!(fresh.copy_psr(0, 1).unwrap_err().field, "fitMode");
    assert_eq!(
        fresh.pulsars[1].binaryModel[..4],
        [Some(b'B'), Some(b'C'), Some(0), Some(b'z')]
    );
    let mut same = PulsarContext::unwritten(1);
    assert_eq!(same.copy_psr(0, 0).unwrap_err().kind, "overlapping_strcpy");
    assert!(same.pulsars[0].name[0].is_none());
    let mut missing = PulsarContext::unwritten(2);
    missing.pulsars[0].name.fill(Some(b'X'));
    assert_eq!(
        missing.copy_psr(0, 1).unwrap_err().kind,
        "missing_terminator"
    );
    assert!(missing.pulsars[1].name.iter().all(|v| *v == Some(b'X')));
}
#[test]
fn defined_forward_overlap_and_destination_fd_bound() {
    let mut c = PulsarContext::unwritten(2);
    let mut tags = Tags::default();
    let opaque = c.allocate_payload(vec![Some(0)]).unwrap();
    seed_owner(&mut c, 0, 11, "source", &mut tags, opaque);
    seed_owner(&mut c, 1, 22, "destination", &mut tags, opaque);
    let v = c.allocate_observations(4).unwrap();
    for (i, o) in c.observations_mut(v).unwrap().iter_mut().enumerate() {
        o.freq = Some(i as f64);
    }
    c.pulsars[0].obsn = Some(Some(v));
    c.pulsars[1].obsn = Some(Some(ObservationView { offset: 1, ..v }));
    c.pulsars[0].nobs = Some(3);
    c.pulsars[0].nJumps = Some(-1);
    c.pulsars[0].nfdJumps = Some(0);
    c.pulsars[1].nfdJumps = Some(2);
    let matrix = c.pulsars[1].covar;
    let param_ptr = c.pulsars[1].param.slots[0].cells.as_ptr();
    c.copy_psr(0, 1).unwrap();
    assert_eq!(
        c.observations(v)
            .unwrap()
            .iter()
            .map(|o| o.freq)
            .collect::<Vec<_>>(),
        vec![Some(0.0); 4]
    );
    assert_eq!(
        c.pulsars[1].fdjumpVal[..4],
        [Some(11.25), Some(11.25), Some(11.25), Some(22.25)]
    );
    assert_eq!(c.pulsars[1].nfdJumps, Some(2));
    assert_eq!(c.pulsars[1].fdjumpIdx[0], Some(22));
    assert_eq!(c.pulsars[1].covar, matrix);
    assert_eq!(c.pulsars[1].param.slots[0].cells.as_ptr(), param_ptr);
    assert_eq!(c.pulsars[1].jumpSAT[0], Some(b's'));
    assert_eq!(c.pulsars[1].jumpVal[0], Some(22.25));
}
#[test]
fn destruction_release_order_and_operation_local_boundaries() {
    let mut c = PulsarContext::unwritten(1);
    let v = c.allocate_observations(1).unwrap();
    let m = c.malloc_blas(2, 3).unwrap();
    let extra = c.malloc_blas(1, 1).unwrap();
    c.allocate_memory(0, 0).unwrap();
    c.pulsars[0].obsn = Some(Some(v));
    c.pulsars[0].covar = Some(Some(m));
    c.pulsars[0].ToAextraCovar = Some(Some(extra));
    c.pulsars[0].constraint_special.fill(Some(None));
    c.pulsars[0].nconstraints = Some(0);
    for i in [0, MAX_PARAMS - 1] {
        let p = c.allocate_payload(vec![Some(0)]).unwrap();
        c.pulsars[0].constraint_special[i] = Some(Some(p));
    }
    c.destroy_one(0).unwrap();
    assert_eq!(
        &c.release_trace()[..6],
        [
            "observations:0",
            "matrix:0",
            "payload:0",
            "payload:1",
            "param[0].label[0]",
            "param[0].shortlabel[0]"
        ]
    );
    assert_eq!(c.matrix(extra).unwrap().cells(), [0.0]);
    assert_eq!(c.observations(v).unwrap_err().kind, "released_owner");
    assert_eq!(c.pulsars[0].obsn, Some(Some(v)));
    assert_eq!(c.destroy_one(0).unwrap_err().kind, "released_owner");
    let mut invalid = PulsarContext::unwritten(1);
    let v = invalid.allocate_observations(2).unwrap();
    invalid.pulsars[0].obsn = Some(Some(v));
    invalid.pulsars[0].covar = Some(None);
    assert_eq!(invalid.destroy_one(0).unwrap_err().kind, "null_owner");
    assert_eq!(invalid.release_trace(), ["observations:0"]);
    let mut interior = PulsarContext::unwritten(1);
    let v = interior.allocate_observations(2).unwrap();
    assert_eq!(
        interior
            .free_observations(ObservationView { offset: 1, ..v })
            .unwrap_err()
            .kind,
        "interior_owner"
    );
    assert!(interior.release_trace().is_empty());
    let mut dims = PulsarContext::unwritten(0);
    assert_eq!(
        dims.malloc_blas(0, 1).unwrap_err().kind,
        "zero_dimension_assertion"
    );
    let mut overflow = PulsarContext::unwritten(0);
    assert_eq!(
        overflow.malloc_blas(usize::MAX, 2).unwrap_err().kind,
        "size_overflow"
    );
}

#[test]
#[ignore = "T helper comparison probe"]
fn helpers_probe() {
    use crate::t2fit_nestlike::*;
    use sha2::{Digest, Sha256};
    let bytes = fs::read(std::env::var("TEMPO2_T_INPUT").unwrap()).unwrap();
    let input: Value = serde_json::from_slice(&bytes).unwrap();
    let mut rows = Vec::new();
    for a in input["helpers"].as_array().unwrap() {
        let v = a[0].as_u64().unwrap();
        let r = a[1].as_i64().unwrap() as i32;
        let d = a[2].as_i64().unwrap() as i32;
        let k = a[3].as_i64().unwrap() as i32;
        rows.push(json!({"flags":t2DecodeTNsubtractPolyFlags(v),"red":t2DecodeTNRedSubPolyOrd(v),"dm":t2DecodeTNDMSubPolyOrd(v),"chrom":t2DecodeTNChromSubPolyOrd(v),"encode":t2EncodeTNsubtractPoly(v,r,d,k),"set_red":t2SetTNRedSubPolyOrd(v,r),"set_dm":t2SetTNDMSubPolyOrd(v,d),"set_chrom":t2SetTNChromSubPolyOrd(v,k)}));
    }
    let shifts: Vec<_> = input["shifts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| {
            t2DecodeTNSubPolyOrdStored(a[0].as_u64().unwrap(), a[1].as_i64().unwrap() as i32)
                .unwrap()
        })
        .collect();
    let clamps: Vec<_> = input["clamps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| t2ClampSubPolyOrder(a.as_i64().unwrap() as i32))
        .collect();
    let mut c = PulsarContext::unwritten(1);
    c.pulsars[0].param.slots[0].link_to[2] = Some(123);
    c.pulsars[0].param.slots[MAX_PARAMS - 1].link_from[4] = Some(-9);
    c.allocate_memory(0, 0).unwrap();
    let allocation = json!([
        c.pulsars[0].param.slots[0].link_to[2],
        c.pulsars[0].param.slots[MAX_PARAMS - 1].link_from[4]
    ]);
    let retained = c.initialise_one(0, 4, 1, 1).unwrap();
    let initialization = json!([
        c.pulsars[0].param.slots[0].link_to[2],
        c.pulsars[0].param.slots[MAX_PARAMS - 1].link_from[4]
    ]);
    let result = json!({"nonce":input["nonce"],"input_sha256":format!("{:x}",Sha256::digest(&bytes)),"helpers":rows,"shifts":shifts,"clamps":clamps,"initial":c.pulsars[0].TNsubtractPoly,"partial_links_allocate":allocation,"partial_links_initialise":initialization});
    fs::write(
        std::env::var("TEMPO2_T_OUTPUT").unwrap(),
        serde_json::to_vec(&result).unwrap(),
    )
    .unwrap();
    drop(retained);
}
#[test]
fn helper_shift_boundaries_are_explicit() {
    use crate::t2fit_nestlike::*;
    assert_eq!(
        t2DecodeTNSubPolyOrdStored(1, -1).unwrap_err().kind,
        "invalid_shift"
    );
    assert_eq!(
        t2DecodeTNSubPolyOrdStored(1, 64).unwrap_err().kind,
        "invalid_shift"
    );
    assert_eq!(t2DecodeTNSubPolyOrdStored(1 << 63, 63).unwrap(), 1);
}

#[test]
fn canonical_controls_strings_profiles_and_detached_snapshots() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut input = crate::load(
        &root.join("tests/reference/fixtures/original/test1.par"),
        &root.join("tests/reference/fixtures/original/test1.tim"),
    )
    .unwrap();
    let mut snapshot = input.fit_controls();
    snapshot.mode = 1;
    snapshot.nits = 99;
    assert_eq!(input.fit_controls().nits, 1);
    input.source.pulsars[0].nits = Some(2);
    input.source.pulsars[0].fitMode = Some(1);
    assert_eq!(input.fit_controls().nits, 2);
    assert_eq!(input.fit_controls().mode, 1);
    assert!(input.source_pulsar().ne_sw_ifuncN.is_none());
    assert_eq!(
        crate::observation::read_string(&input.source_pulsar().name[..], "name").unwrap(),
        input.text_parameters()["PSRJ"].tokens[0].as_bytes()
    );
    input.source.pulsars[0].units = Some(0);
    assert_eq!(
        crate::evaluate_barycentric(&input).unwrap_err().field,
        "UNITS"
    );
    let mut terrestrial = crate::load(
        &root.join("tests/terrestrial/fixtures/test1.par"),
        &root.join("tests/terrestrial/fixtures/test2.tim"),
    )
    .unwrap();
    crate::observation::write_string(
        &mut terrestrial.source.pulsars[0].clock[..],
        b"bad",
        "clock",
    )
    .unwrap();
    let mut runtime =
        crate::RuntimeContext::new(&root.parent().unwrap().join("tempo2/T2runtime")).unwrap();
    assert_eq!(
        crate::evaluate_terrestrial(&terrestrial, &mut runtime)
            .unwrap_err()
            .field,
        "CLK"
    );
}
#[test]
fn newly_mapped_string_capacities_are_fixed_utf8_byte_boundaries() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = root.join("verification/pulsar_lifecycle/build/string-boundaries");
    fs::create_dir_all(&dir).unwrap();
    let tim = dir.join("input.tim");
    fs::write(&tim, "FORMAT 1\na 1400 58000 1 @\n").unwrap();
    let par = dir.join("input.par");
    for (key, capacity) in [("PSRJ", 100), ("CLK", 16), ("EPHEM", 500), ("TZRSITE", 100)] {
        for multibyte in [false, true] {
            for fits in [true, false] {
                let bytes = capacity - usize::from(fits);
                let value = if multibyte {
                    format!("{}{}", "é".repeat(bytes / 2), "x".repeat(bytes % 2))
                } else {
                    "x".repeat(bytes)
                };
                let identity = if key == "PSRJ" {
                    String::new()
                } else {
                    "PSRJ bounded\n".into()
                };
                fs::write(
                    &par,
                    format!("{identity}F0 1\nPEPOCH 58000\nUNITS TCB\n{key} {value}\n"),
                )
                .unwrap();
                let result = crate::load(&par, &tim);
                assert_eq!(
                    result.is_ok(),
                    fits,
                    "{key} bytes={bytes} multibyte={multibyte}"
                );
                if let Err(e) = result {
                    assert_eq!(e.field, key);
                    assert!(e.line.is_some());
                }
            }
        }
    }
    fs::remove_file(par).unwrap();
    fs::remove_file(tim).unwrap();
}

// IEEE binary128 input is decoded exactly at Rug128 precision; no decimal text
// or binary64 intermediate. This test also checks source record assignment.
fn decode_binary128(bits: u128) -> Float {
    let sign = bits >> 127 != 0;
    let exponent = ((bits >> 112) & 0x7fff) as i32;
    let mantissa = bits & ((1u128 << 112) - 1);
    let mut value = if exponent == 0x7fff {
        Float::with_val(
            128,
            if mantissa == 0 {
                f64::INFINITY
            } else {
                f64::NAN
            },
        )
    } else {
        let significand = if exponent == 0 {
            mantissa
        } else {
            mantissa + (1 << 112)
        };
        let mut v = Float::with_val(128, significand);
        v <<= exponent.max(1) - 16383 - 112;
        v
    };
    if sign {
        value = -value;
    }
    value
}
#[test]
fn binary128_decode_and_record_assignment_preserve_exact_classes() {
    let mut c = PulsarContext::unwritten(0);
    let v = c.allocate_observations(2).unwrap();
    for bits in [
        0,
        1u128 << 127,
        1,
        (1u128 << 127) | 1,
        (0x3fffu128 << 112) | 1,
        (0x7ffeu128 << 112) | ((1u128 << 112) - 1),
        0x7fffu128 << 112,
        (1u128 << 127) | (0x7fffu128 << 112),
        (0x7fffu128 << 112) | 1,
    ] {
        let value = decode_binary128(bits);
        c.observations_mut(v).unwrap()[0].sat = Some(value.clone());
        let record = c.observations(v).unwrap()[0].clone();
        c.observations_mut(v).unwrap()[1] = record;
        let copied = c.observations(v).unwrap()[1].sat.as_ref().unwrap();
        assert_eq!(copied.prec(), 128);
        if value.is_nan() {
            assert!(copied.is_nan());
        } else {
            assert_eq!(&value, copied);
            assert_eq!(value.is_sign_negative(), copied.is_sign_negative());
        }
    }
    let one_plus = decode_binary128((0x3fffu128 << 112) | 1);
    assert_eq!(
        one_plus - Float::with_val(128, 1),
        Float::with_val(128, 1) >> 112
    );
    assert_eq!(decode_binary128(1), Float::with_val(128, 1) >> 16494);
    let mut empty = PulsarContext::unwritten(0);
    assert!(empty.initialise(-7, 4, -2).unwrap().is_empty());
}

#[test]
#[ignore = "run alone: checks shared clock terminal state without cross-test interference"]
fn lifecycle_logging_after_clock_fault() {
    let previous = {
        let mut session = crate::clkcorr::test_session();
        session
            .fault
            .replace(crate::clock_diagnostics::boundary("terminal-clock-fixture"))
    };
    let mut context = PulsarContext::unwritten(0);
    let result = context.malloc_blas(2, 3);
    {
        let mut session = crate::clkcorr::test_session();
        assert_eq!(
            session.fault.as_ref().unwrap().field,
            "terminal-clock-fixture"
        );
        session.fault = previous;
    }
    assert!(result.is_ok());
    assert!(context.boundary().is_none());
}
