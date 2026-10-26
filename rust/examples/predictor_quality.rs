//! Independent off-node phase and centered-frequency diagnostic against canonical timing.
use std::path::PathBuf;
use tempo2_rust::{
    RuntimeContext,
    cheby2d::{Cheby2D, ChebyModel},
    cheby2d_int::cheby_model_func,
    predictor,
    predictor_number::PredictorNumber as N,
    tempo2pred::T2Predictor,
};
fn n(x: f64) -> N {
    x.into()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 4 {
        return Err("predictor_quality PREDICTOR PAR RUNTIME OUTPUT_DIR".into());
    }
    let file = T2Predictor::read(&PathBuf::from(&a[0]))?;
    let root = PathBuf::from(&a[2]);
    let mut runtime = RuntimeContext::from_root(&root)?;
    let mut input = predictor::load_par(&PathBuf::from(&a[1]), &root, &[])?;
    let site = file.predictor.metadata()?.sitename;
    predictor::cli_defaults(&mut input.context, &site)?;
    let out = PathBuf::from(&a[3]);
    std::fs::create_dir_all(&out)?;
    let ranges: Vec<_> = match &file.predictor {
        T2Predictor::Cheby(s) => s
            .segments
            .iter()
            .map(|m| {
                (
                    m.mjd_start.clone(),
                    m.mjd_end.clone(),
                    m.freq_start.clone(),
                    m.freq_end.clone(),
                )
            })
            .collect(),
        T2Predictor::T1(s) => s
            .segments
            .iter()
            .map(|m| {
                (
                    &m.mjd_mid - n(m.span as f64) / 2880.,
                    &m.mjd_mid + n(m.span as f64) / 2880.,
                    n(m.frequency_obs * 0.8),
                    n(m.frequency_obs * 1.2),
                )
            })
            .collect(),
        _ => return Err("empty predictor".into()),
    };
    let mut records = vec![];
    for (segment, (start, end, low, high)) in ranges.into_iter().enumerate() {
        let c = Cheby2D::new(2, 2)?;
        let mut m = ChebyModel {
            psrname: String::new(),
            sitename: site.clone(),
            mjd_start: start,
            mjd_end: end,
            freq_start: low,
            freq_end: high,
            dispersion_constant: n(0.),
            frequency_cheby: c.x_derivative()?,
            cheby: c,
        };
        let dx = n(0.5) / 86400. * 2. / (&m.mjd_end - &m.mjd_start);
        let mut x = vec![];
        for base in [-0.87, -0.31, 0.13, 0.71, 0.93] {
            x.extend([n(base) - &dx, n(base), n(base) + &dx]);
        }
        let y = vec![n(-0.7), n(0.1), n(0.8)];
        cheby_model_func(
            &mut m,
            &mut input.context,
            &mut runtime,
            &x,
            &y,
            false,
            &out.join("quality.tim"),
        )?;
        let rows = predictor::snapshot(&input.context)?;
        for iy in 0..3 {
            for ix in 0..5 {
                let i = 1 + iy * x.len() + ix * 3 + 1;
                let row = &rows[i];
                let time = N::from_float(row.sat_mjd.as_float());
                let freq = n(row.frequency_mhz);
                let prediction = file.predictor.evaluate(&time, &freq)?;
                let reference = if matches!(file.predictor, T2Predictor::Cheby(_)) {
                    row.phase_turn.as_float().clone() - rows[0].phase_turn.as_float()
                } else {
                    row.phase_turn.as_float().clone()
                };
                let full_hz = (rows[i + 1].phase_turn.as_float().clone()
                    - rows[i - 1].phase_turn.as_float())
                    / (rows[i + 1].sat_mjd.as_float().clone() - rows[i - 1].sat_mjd.as_float())
                    / 86400f64;
                records.push(serde_json::json!({"construction_segment":segment,"selected_segment":prediction.segment,"mjd":time,"frequency_mhz":freq,"phase_turn":prediction.phase_turn,"full_phase_turn":reference.to_string_radix(10,None),"frequency_hz":prediction.frequency_hz,"full_centered_frequency_hz":full_hz.to_string_radix(10,None)}));
            }
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"f0_hz":predictor::spin_frequency(&input.context)?.to_string_radix(10,None),"records":records,"frequency_difference_interval_s":"1"})
        )?
    );
    Ok(())
}
