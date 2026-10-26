//! Eccentric residual/energy and reusable histories after actual timing evaluation.
mod analysis_support;
use analysis_support::*;
use tempo2_rust::{gw_sim::*, source_par, source_text::SourceText};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    let (mut input, _, _) = load(&a)?;
    let path = std::path::Path::new(required(&a, "--control")?);
    source_par::read_simple_parfile(
        &mut input.context,
        0,
        &mut SourceText::new(path, std::fs::read(path)?),
        std::path::Path::new(required(&a, "--runtime")?),
    )?;
    let (p, _) = input.context.pulsar_observations_mut(0)?;
    let period = p.gwecc_orbital_period.unwrap() * 365.25 * 86400.;
    let mut state = EccentricState {
        coalesce_flag: 0,
        prev_p: period,
        prev_e: p.gwecc_e.unwrap(),
        prev_epoch: p.gwecc_epoch.unwrap(),
        prev_theta: p.gwecc_theta_0.unwrap(),
        prev_a: (6.67e-11
            * (p.gwecc_m1.unwrap() + p.gwecc_m2.unwrap())
            * 1.9891e30
            * (2.0 * std::f64::consts::PI / period).powf(-2.))
        .powf(1. / 3.),
    };
    for (call, i) in [0, 1, 0, 2].iter().enumerate() {
        let mut plain = state.clone();
        let (r, e) = eccentric_residual_with_energy(&mut input.context, 0, *i, &mut state)?;
        let p = eccentric_residual(&mut input.context, 0, *i, &mut plain)?;
        extended(&format!("ecc.{call}.residual"), "seconds", &(r / 1e9));
        extended(&format!("ecc.{call}.plain"), "seconds", &(p / 1e9));
        num(&format!("ecc.{call}.energy"), "f32", e as f64);
        emit(&format!("ecc.{call}.flag"), "exact", state.coalesce_flag);
        for (k, v) in [
            ("period", state.prev_p),
            ("ecc", state.prev_e),
            ("a", state.prev_a),
            ("epoch", state.prev_epoch),
            ("theta", state.prev_theta),
        ] {
            num(&format!("ecc.{call}.{k}"), "history", v);
        }
    }
    state.coalesce_flag = 1;
    let (r, e) = eccentric_residual_with_energy(&mut input.context, 0, 0, &mut state)?;
    extended("coalesced.residual", "seconds", &r);
    num("coalesced.energy", "f32", e as f64);
    for (k, v) in [
        ("Rs", rs(1e30)?),
        ("dadt", dadt(0.2, 1e12, 1e30, 2e30)?),
        ("dedt", dedt(0.2, 1e12, 1e30, 2e30)?),
        ("dtdt", dtdt(0.2, 0.3, 1e8)?),
        ("Fe", fe(0.2)?),
    ] {
        num(k, "history", v);
    }
    Ok(())
}
