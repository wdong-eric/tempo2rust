//! Test-only initialized native field projection; no C layout in production.
use crate::PRECISION_BITS;
use rug::{Float, Integer};
use serde_json::{Value, json};
pub(crate) fn diagnostic_streams() -> [String; 2] {
    crate::clkcorr::with_process(|s| {
        let mut streams = [String::new(), String::new()];
        for (stderr, bytes) in s.diagnostics.events.drain(..) {
            streams[usize::from(stderr)].extend(bytes.into_iter().map(char::from));
        }
        Ok(streams)
    })
    .unwrap()
}
pub(crate) trait Wire: Sized {
    fn take(b: &mut &[u8]) -> Self;
    fn put(&self, b: &mut Vec<u8>);
}
macro_rules! wire {($($t:ty),*)=>{$(impl Wire for $t {fn take(b:&mut &[u8])->Self{let(a,rest)=b.split_at(std::mem::size_of::<Self>());*b=rest;Self::from_le_bytes(a.try_into().unwrap())}fn put(&self,b:&mut Vec<u8>){b.extend(self.to_le_bytes());}})*}}
wire!(u8, u32, i32, i64, u64, f32, f64);
impl<T: Wire> Wire for Option<T> {
    fn take(b: &mut &[u8]) -> Self {
        Some(T::take(b))
    }
    fn put(&self, b: &mut Vec<u8>) {
        self.as_ref().expect("unwritten Q fixture field").put(b);
    }
}
impl<T: Wire, const N: usize> Wire for [T; N] {
    fn take(b: &mut &[u8]) -> Self {
        std::array::from_fn(|_| T::take(b))
    }
    fn put(&self, b: &mut Vec<u8>) {
        for v in self {
            v.put(b);
        }
    }
}
impl<T: Wire, const N: usize> Wire for Box<[T; N]> {
    fn take(b: &mut &[u8]) -> Self {
        (0..N)
            .map(|_| T::take(b))
            .collect::<Vec<_>>()
            .into_boxed_slice()
            .try_into()
            .ok()
            .unwrap()
    }
    fn put(&self, b: &mut Vec<u8>) {
        (**self).put(b);
    }
}
impl Wire for Float {
    fn take(b: &mut &[u8]) -> Self {
        let (a, rest) = b.split_at(16);
        *b = rest;
        let bits = u128::from_le_bytes(a.try_into().unwrap());
        let e = ((bits >> 112) & 0x7fff) as i32;
        let m = bits & ((1u128 << 112) - 1);
        let mut x = if e == 0x7fff {
            Float::with_val(
                PRECISION_BITS,
                if m == 0 {
                    rug::float::Special::Infinity
                } else {
                    rug::float::Special::Nan
                },
            )
        } else if e == 0 {
            Float::with_val(PRECISION_BITS, Integer::from(m)) >> 16494
        } else {
            Float::with_val(PRECISION_BITS, Integer::from(m | (1 << 112))) << (e - 16383 - 112)
        };
        if bits >> 127 != 0 {
            x = -x;
        }
        x
    }
    fn put(&self, b: &mut Vec<u8>) {
        let x = Float::with_val(113, self);
        let sign = (x.is_sign_negative() as u128) << 127;
        let bits = if x.is_zero() {
            sign
        } else if x.is_nan() {
            sign | (0x7fffu128 << 112) | (1 << 111)
        } else if x.is_infinite() {
            sign | (0x7fffu128 << 112)
        } else {
            let (n, e) = x.clone().abs().to_integer_exp().unwrap();
            let n = n.to_u128().unwrap();
            let len = 128 - n.leading_zeros();
            let m = if len > 113 {
                n >> (len - 113)
            } else {
                n << (113 - len)
            };
            sign | (((e + len as i32 - 1 + 16383) as u128) << 112) | (m - (1 << 112))
        };
        b.extend(bits.to_le_bytes());
    }
}
pub(crate) fn decode(v: &Value) -> Vec<u8> {
    let mut b = vec![0; v["size"].as_u64().unwrap() as usize];
    for run in v["nonzero"].as_array().unwrap() {
        let offset = run[0].as_u64().unwrap() as usize;
        for (i, pair) in run[1]
            .as_str()
            .unwrap()
            .as_bytes()
            .chunks_exact(2)
            .enumerate()
        {
            b[offset + i] = u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
        }
    }
    b
}
pub(crate) fn encode(b: &[u8]) -> Value {
    let mut runs = vec![];
    let mut i = 0;
    while i < b.len() {
        if b[i] == 0 {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && b[i] != 0 {
            i += 1;
        }
        runs.push(json!([
            start,
            b[start..i]
                .iter()
                .map(|x| format!("{x:02x}"))
                .collect::<String>()
        ]));
    }
    json!({"size":b.len(),"nonzero":runs})
}

impl Wire for crate::observation::SourceClockCorrection {
    fn take(b: &mut &[u8]) -> Self {
        Self {
            correction: Wire::take(b),
            corrects_to: Wire::take(b),
        }
    }
    fn put(&self, b: &mut Vec<u8>) {
        self.correction.put(b);
        self.corrects_to.put(b);
    }
}
pub(crate) fn span<T: Wire>(value: &mut T, bytes: &mut [u8], load: bool) {
    if load {
        let mut raw = &*bytes;
        *value = T::take(&mut raw);
        assert!(raw.is_empty());
    } else {
        let mut raw = vec![];
        value.put(&mut raw);
        bytes.copy_from_slice(&raw);
    }
}
include!("observation_corpus_fields.rs");

pub(crate) fn field<T: Wire>(
    v: &mut T,
    key: &str,
    state: &mut std::collections::BTreeMap<String, Value>,
    load: bool,
) {
    if load {
        let bytes = decode(&state[key]);
        let mut b = bytes.as_slice();
        *v = T::take(&mut b);
        assert!(b.is_empty(), "{key}");
    } else {
        let mut b = vec![];
        v.put(&mut b);
        state.insert(key.into(), encode(&b));
    }
}

/// P's complete parameter allocation concatenation, including unnamed slots.
pub(crate) fn parameter_arrays(
    p: &mut crate::pulsar::SourcePulsar,
    root: &str,
    s: &mut std::collections::BTreeMap<String, Value>,
    load: bool,
) {
    for name in [
        "aSize",
        "nLinkTo",
        "nLinkFrom",
        "linkTo",
        "linkFrom",
        "val",
        "err",
        "prefit",
        "prefitErr",
        "fitFlag",
        "paramSet",
        "label",
        "shortlabel",
    ] {
        let key = format!("{root}param.{name}");
        let raw = if load { decode(&s[&key]) } else { vec![] };
        let mut rest = raw.as_slice();
        let mut bytes = vec![];
        for par in &mut p.param.slots {
            macro_rules! scalar {
                ($v:expr) => {
                    if load {
                        *$v = Wire::take(&mut rest);
                    } else {
                        $v.put(&mut bytes);
                    }
                };
            }
            match name {
                "aSize" => scalar!(&mut par.a_size),
                "nLinkTo" => scalar!(&mut par.n_link_to),
                "nLinkFrom" => scalar!(&mut par.n_link_from),
                "linkTo" => scalar!(&mut par.link_to),
                "linkFrom" => scalar!(&mut par.link_from),
                _ => {
                    for cell in &mut par.cells {
                        match name {
                            "val" => scalar!(&mut cell.val),
                            "err" => scalar!(&mut cell.err),
                            "prefit" => scalar!(&mut cell.prefit),
                            "prefitErr" => scalar!(&mut cell.prefit_err),
                            "fitFlag" => scalar!(&mut cell.fit_flag),
                            "paramSet" => scalar!(&mut cell.param_set),
                            "label" | "shortlabel" => {
                                let x = if name == "label" {
                                    &mut cell.label
                                } else {
                                    &mut cell.shortlabel
                                };
                                if load {
                                    let (a, b) = rest.split_at(x.len());
                                    x.copy_from_slice(a);
                                    rest = b;
                                } else {
                                    bytes.extend(x.iter());
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        if load {
            assert!(rest.is_empty(), "{key}");
        } else {
            s.insert(key, encode(&bytes));
        }
    }
}

pub(crate) fn hex(b: &[u8]) -> String {
    b.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect()
}
pub(crate) trait Typed: Wire {
    fn typed(&self) -> Value;
    fn parse(v: &Value) -> Self;
}
impl Typed for u8 {
    fn typed(&self) -> Value {
        json!(self)
    }
    fn parse(v: &Value) -> Self {
        v.as_u64().unwrap().try_into().unwrap()
    }
}
impl Typed for i32 {
    fn typed(&self) -> Value {
        json!(self)
    }
    fn parse(v: &Value) -> Self {
        v.as_i64().unwrap().try_into().unwrap()
    }
}
impl Typed for i64 {
    fn typed(&self) -> Value {
        json!(self)
    }
    fn parse(v: &Value) -> Self {
        v.as_i64().unwrap()
    }
}
fn class(v: f64) -> &'static str {
    if v.is_nan() {
        "NaN"
    } else if v.is_infinite() {
        if v.is_sign_negative() {
            "-Infinity"
        } else {
            "+Infinity"
        }
    } else {
        "finite"
    }
}
impl Typed for f64 {
    fn typed(&self) -> Value {
        json!({"type":"binary64","class":class(*self),"bits_le":hex(&self.to_le_bytes())})
    }
    fn parse(v: &Value) -> Self {
        Wire::take(&mut unhex(v["bits_le"].as_str().unwrap()).as_slice())
    }
}
impl Typed for f32 {
    fn typed(&self) -> Value {
        json!({"type":"binary32","class":class(*self as f64),"bits_le":hex(&self.to_le_bytes())})
    }
    fn parse(v: &Value) -> Self {
        Wire::take(&mut unhex(v["bits_le"].as_str().unwrap()).as_slice())
    }
}
impl Typed for Float {
    fn typed(&self) -> Value {
        let mut b = vec![];
        self.put(&mut b);
        json!({"type":"binary128","bits_le":hex(&b),"value":self.to_string_radix(10,None)})
    }
    fn parse(v: &Value) -> Self {
        Wire::take(&mut unhex(v["bits_le"].as_str().unwrap()).as_slice())
    }
}
impl<T: Typed> Typed for Option<T> {
    fn typed(&self) -> Value {
        self.as_ref().unwrap().typed()
    }
    fn parse(v: &Value) -> Self {
        Some(T::parse(v))
    }
}
pub(crate) fn scalar<T: Typed>(
    v: &mut T,
    key: &str,
    s: &mut std::collections::BTreeMap<String, Value>,
    load: bool,
) {
    if load {
        *v = T::parse(&s[key]);
    } else {
        s.insert(key.into(), v.typed());
    }
}
