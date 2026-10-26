#![allow(dead_code)]
use rug::Float;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tempo2_rust::{PRECISION_BITS, RuntimeContext, bats, timing};
pub fn args() -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() % 2 != 0 {
        return Err("expected --name value pairs".into());
    }
    Ok(a.chunks(2).map(|p| (p[0].clone(), p[1].clone())).collect())
}
pub fn required<'a>(
    a: &'a BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    a.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing {key}").into())
}
pub fn value<T: std::str::FromStr>(
    a: &BTreeMap<String, String>,
    key: &str,
    default: T,
) -> Result<T, Box<dyn std::error::Error>>
where
    T::Err: std::error::Error + 'static,
{
    Ok(if let Some(v) = a.get(key) {
        v.parse()?
    } else {
        default
    })
}
pub fn ld(v: f64) -> Float {
    Float::with_val(PRECISION_BITS, v)
}
pub fn emit(key: &str, unit: &str, v: impl std::fmt::Display) {
    println!("{key}\t{unit}\t{v}");
}
pub fn num(key: &str, unit: &str, v: f64) {
    emit(key, unit, format!("{v:.17e}"));
}
pub fn extended(key: &str, unit: &str, v: &Float) {
    emit(key, unit, v.to_string_radix(10, Some(40)));
}
pub fn vector(key: &str, unit: &str, v: &[f64]) {
    emit(&format!("{key}.len"), "exact", v.len());
    for (i, &x) in v.iter().enumerate() {
        num(&format!("{key}.{i}"), unit, x);
    }
}
pub fn load(
    a: &BTreeMap<String, String>,
) -> Result<(bats::BatInput, RuntimeContext, timing::TimingResult), Box<dyn std::error::Error>> {
    let root = Path::new(required(a, "--runtime")?);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: PathBuf::from(required(a, "--par")?),
            tim: PathBuf::from(required(a, "--tim")?),
        }],
        root,
    )?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    let result = timing::evaluate(&mut input, &mut runtime, true)?;
    Ok((input, runtime, result))
}
