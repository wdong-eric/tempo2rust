//! Legacy callbacks on real PAR/TIM state, with source-specific column order.
use tempo2_rust::{
    RuntimeContext, TimingError, bats, general_fitting, legacy_fitting,
    pulsar_lifecycle::PulsarContext, tempo2::Label::*, timing,
};
fn basis(
    context: &mut PulsarContext,
    p: usize,
    x: f64,
    observation: usize,
    row: &mut [f64],
) -> Result<(), TimingError> {
    row[0] = tempo2_rust::fit_callbacks::derivative(context, p, x, observation, param_f, 0)?;
    row[1] = 1.0;
    for (k, value) in row.iter_mut().enumerate().skip(2) {
        *value =
            tempo2_rust::fit_callbacks::derivative(context, p, x, observation, param_f, k - 1)?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 6 {
        return Err("expected MODE LOWER RUNTIME PAR0 PAR1 TIM".into());
    }
    let root = std::path::Path::new(&a[2]);
    let global = a[0] == "global";
    let pairs: Vec<_> = a[3..if global { 5 } else { 4 }]
        .iter()
        .map(|p| bats::InputPair {
            par: p.into(),
            tim: (&a[5]).into(),
        })
        .collect();
    let mut input = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    timing::evaluate(&mut input, &mut runtime, true)?;
    let (mut x, mut y, mut indices, mut sigma, mut lower) =
        (vec![], vec![], vec![], vec![], vec![]);
    for p in 0..pairs.len() {
        let rows = general_fitting::get_fit_data(&mut input.context, p)?;
        let n = rows.len();
        x.push(
            rows.iter()
                .map(|r| r.x_days.as_float().to_f64())
                .collect::<Vec<_>>(),
        );
        y.push(
            rows.iter()
                .map(|r| r.residual_s.as_float().to_f64())
                .collect::<Vec<_>>(),
        );
        indices.push(rows.iter().map(|r| r.observation).collect::<Vec<_>>());
        let errors: Vec<_> = rows.iter().map(|r| r.sigma_s.as_float().to_f64()).collect();
        let l = if a[0] == "sigma" {
            vec![errors.clone()]
        } else if a[1] == "full" {
            let mut l = vec![vec![0.; n]; n];
            for i in 0..n {
                l[i][i] = errors[i];
                if i > 0 {
                    l[i][i - 1] = errors[i] * 0.125;
                }
            }
            l
        } else {
            vec![errors.iter().map(|v| 1.0 / v).collect()]
        };
        sigma.push(errors);
        lower.push(l);
    }
    if global {
        let fit = legacy_fitting::global_pulsar(
            &mut input.context,
            &x,
            &y,
            &indices,
            &[3, 2],
            1,
            &lower,
            basis,
            1e-27,
            false,
            None,
        )?;
        println!("{}", serde_json::to_string_pretty(&fit)?);
    } else {
        let d = legacy_fitting::design_matrix(
            &mut input.context,
            0,
            &x[0],
            &y[0],
            &indices[0],
            4,
            &lower[0],
            basis,
        )?;
        let fit = match a[0].as_str() {
            "single" => legacy_fitting::single_pulsar(
                &mut input.context,
                &x[0],
                &y[0],
                &indices[0],
                4,
                &lower[0],
                basis,
                1e-27,
                false,
            )?,
            "dcm" => legacy_fitting::svd_psr_dcm(
                &mut input.context,
                &x[0],
                &y[0],
                &indices[0],
                4,
                &lower[0],
                basis,
                1e-27,
                1,
            )?,
            "sigma" => legacy_fitting::svd_psr(
                &mut input.context,
                &x[0],
                &y[0],
                &sigma[0],
                &indices[0],
                4,
                basis,
                1e-27,
                1,
            )?,
            _ => return Err("unknown legacy adapter".into()),
        };
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"design":d,"constraints":[],"solver":fit})
            )?
        );
    }
    Ok(())
}
