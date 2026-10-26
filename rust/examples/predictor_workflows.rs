//! Direct canonical construction, repeated contexts, prefix semantics and recovery.
use std::path::PathBuf;
use tempo2_rust::{
    RuntimeContext, Scalar,
    cheby2d_int::{self, ChebyRequest},
    polyco::{self, PolycoRequest},
    predictor,
    predictor_number::PredictorNumber as N,
    tempo2pred::T2Predictor,
};
fn n(x: f64) -> N {
    x.into()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("predictor_workflows PAR RUNTIME FRESH_OUTPUT_DIR".into());
    }
    let par = PathBuf::from(&a[0]);
    let root = PathBuf::from(&a[1]);
    let out = PathBuf::from(&a[2]);
    std::fs::create_dir(&out)?;
    let mut input = predictor::load_par(&par, &root, &[])?;
    let mut runtime = RuntimeContext::from_root(&root)?;
    predictor::cli_defaults(&mut input.context, "PKS")?;
    // Odd dimensions are retained by the direct API. CLI rounding is separate.
    let mut request = ChebyRequest {
        site: "PKS".into(),
        start_mjd: n(58000.),
        end_mjd: n(58000.03),
        segment_days: n(0.02),
        overlap_days: n(0.002),
        freq_start_mhz: n(1000.),
        freq_end_mhz: n(2000.),
        time_coeffs: 7,
        freq_coeffs: 3,
    };
    assert!(
        cheby2d_int::construct_set(
            &request,
            &mut input.context,
            &mut runtime,
            &out.join("missing/failed.tim")
        )
        .is_err()
    );
    let a = cheby2d_int::construct_set(
        &request,
        &mut input.context,
        &mut runtime,
        &out.join("direct.tim"),
    )?;
    assert_eq!(a.segments[0].cheby.nx(), 7);
    let b = cheby2d_int::construct_set(
        &request,
        &mut input.context,
        &mut runtime,
        &out.join("repeat.tim"),
    )?;
    let x = a.segments[0].phase(&n(58000.01), &n(1400.));
    let y = b.segments[0].phase(&n(58000.01), &n(1400.));
    assert!((x.to_f64() - y.to_f64()).abs() < 1e-8);
    request.site = "@".into();
    let bary = cheby2d_int::construct_set(
        &request,
        &mut input.context,
        &mut runtime,
        &out.join("changed-site.tim"),
    )?;
    let file = out.join("direct.dat");
    let mut owned = T2Predictor::Cheby(bary);
    owned.write(&file)?;
    assert!(owned.read_into(&out.join("missing.dat")).is_err());
    assert_eq!(owned.metadata()?.sitename, "@");
    owned.read_into(&file)?;
    let q = |s: &str| -> Result<_, Box<dyn std::error::Error>> {
        Ok(s.parse::<Scalar>()?.as_float().clone())
    };
    let mut r = PolycoRequest {
        start_mjd: q("58000")?,
        end_mjd: q("58000")?,
        span_minutes: 60,
        coefficients: 12,
        max_hour_angle: q("1")?,
        site: "PKS".into(),
        frequency_mhz: q("1400")?,
        true_dm: false,
        file_prefix: out.join("direct-"),
    };
    let terrestrial = polyco::polyco(&mut input.context, &mut runtime, &r)?;
    assert_eq!(terrestrial.models.segments[0].dm, 100.);
    r.site = "@".into();
    r.true_dm = true;
    r.file_prefix = out.join("bary-");
    let bary = polyco::polyco(&mut input.context, &mut runtime, &r)?;
    assert_eq!(bary.models.segments[0].dm, 0.);
    r.coefficients = 32;
    assert!(polyco::polyco(&mut input.context, &mut runtime, &r).is_err());
    r.coefficients = 12;
    r.max_hour_angle = q("13")?;
    assert!(polyco::atimfake(&mut input.context, &mut runtime, &r, &r.start_mjd).is_err());
    r.max_hour_angle = q("1")?;
    let recovered = polyco::polyco(&mut input.context, &mut runtime, &r)?;
    let defaults_par = out.join("direct-defaults.par");
    let text = std::fs::read_to_string(&par)?;
    std::fs::write(
        &defaults_par,
        text.lines()
            .filter(|line| !line.starts_with("TZR"))
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    input = predictor::load_par(&defaults_par, &root, &[])?;
    input
        .context
        .pulsar_observations_mut(0)?
        .0
        .tzrsite
        .fill(Some(0));
    request.start_mjd = n(58000.01);
    cheby2d_int::construct_set(
        &request,
        &mut input.context,
        &mut runtime,
        &out.join("direct-defaults.tim"),
    )?;
    {
        use tempo2_rust::tempo2::Label::*;
        let p = &input.context.pulsars()[0];
        assert_eq!(
            N::from_float(p.param.parameter(param_tzrmjd).cell(0)?.value()),
            request.start_mjd
        );
        assert_eq!(
            N::from_float(p.param.parameter(param_tzrfrq).cell(0)?.value()),
            request.freq_start_mhz
        );
        assert_eq!(p.tzrsite[0], Some(b'@'));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"direct_odd_dimensions":[7,3],"failed_output_then_repeated_context":true,"failed_read_then_valid":true,"changed_site_true_dm":true,"bounds_then_recovery":true,"retained_models":recovered.models.segments.len(),"resources":runtime.resources()})
        )?
    );
    Ok(())
}
