//! Localize polyco RMS differences by replaying native phases into the same fit.
use rug::Float;
use std::path::PathBuf;
use tempo2_rust::{
    PRECISION_BITS, RuntimeContext, Scalar,
    polyco::{self, PolycoRequest},
    predictor,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 5 {
        return Err("predictor_refit PAR RUNTIME SITE NATIVE_TSV FRESH_OUTPUT".into());
    }
    let root = PathBuf::from(&a[1]);
    let out = PathBuf::from(&a[4]);
    std::fs::create_dir(&out)?;
    let mut input = predictor::load_par(&PathBuf::from(&a[0]), &root, &[])?;
    let mut runtime = RuntimeContext::from_root(&root)?;
    predictor::cli_defaults(&mut input.context, &a[2])?;
    let q = |s: &str| s.parse::<Scalar>().map(|v| v.as_float().clone());
    let request = PolycoRequest {
        start_mjd: q("58000")?,
        end_mjd: q("58000")?,
        span_minutes: 60,
        coefficients: 12,
        max_hour_angle: q("1")?,
        site: a[2].clone(),
        frequency_mhz: q("1400")?,
        true_dm: true,
        file_prefix: out.join("refit-"),
    };
    let original = polyco::polyco(&mut input.context, &mut runtime, &request)?;
    let samples = polyco::atimfake(
        &mut input.context,
        &mut runtime,
        &request,
        &request.start_mjd,
    )?;
    let (_, rows) = input.context.pulsar_observations_mut(0)?;
    let mut count = 0;
    for line in std::fs::read_to_string(&a[3])?.lines() {
        let v: Vec<_> = line.split('\t').collect();
        if v[0] == "phase" {
            let i: usize = v[1].parse()?;
            rows[i].phase = Some(Float::with_val(PRECISION_BITS, Float::parse(v[2])?));
            count += 1;
        }
    }
    assert_eq!(count, 31 * samples.nsets + 1);
    let shared = polyco::tz_fit(&input.context, &samples, &request)?;
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"replaced_phases":count,"original_log10rms":original.models.segments.iter().map(|m|m.log10rms).collect::<Vec<_>>(),"shared_phase_log10rms":shared.iter().map(|m|m.model.log10rms).collect::<Vec<_>>()})
        )?
    );
    Ok(())
}
