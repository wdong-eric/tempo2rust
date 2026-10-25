//! Direct scalar callbacks on canonical PAR/TIM state, without solving a fit.
use tempo2_rust::{FitNumber, RuntimeContext, bats, fit_callbacks, general_fitting as fit, timing};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("expected RUNTIME PAR TIM".into());
    }
    let root = std::path::Path::new(&args[0]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: args[1].clone().into(),
            tim: args[2].clone().into(),
        }],
        root,
    )?;
    bats::preprocess(&mut input, &[], root)?;
    timing::evaluate(&mut input, &mut RuntimeContext::from_root(root)?, true)?;
    let rows = fit::get_fit_data(&mut input.context, 0)?;
    let columns = fit::fill_fit_columns(&mut input.context, 0, &rows, &[], &Default::default())?;
    let mut output = serde_json::Map::new();
    for column in columns {
        let label = tempo2_rust::tempo2::PARAMETER_LABELS[column.parameter_index];
        let k = column.subindex;
        let key = format!("{}_{}", column.parameter_index, k);
        let values = rows
            .iter()
            .map(|r| {
                fit_callbacks::derivative(
                    &mut input.context,
                    0,
                    r.x_days.as_float().to_f64(),
                    r.observation,
                    label,
                    k,
                )
                .map(FitNumber::from)
            })
            .collect::<Result<Vec<_>, _>>()?;
        output.insert(format!("{key}_derivative"), serde_json::to_value(values)?);
        if label == tempo2_rust::tempo2::Label::param_ZERO {
            continue;
        }
        let before = input.context.clone();
        let coordinate =
            |p: &tempo2_rust::pulsar::SourcePulsar| -> Result<_, Box<dyn std::error::Error>> {
                use tempo2_rust::tempo2::Label::*;
                let array = match label {
                    param_tel_dx => Some((&p.telDX_v, &p.telDX_e)),
                    param_tel_dy => Some((&p.telDY_v, &p.telDY_e)),
                    param_tel_dz => Some((&p.telDZ_v, &p.telDZ_e)),
                    _ => None,
                };
                if let Some((values, errors)) = array {
                    Ok((
                        FitNumber::from(values[k].ok_or("unwritten telescope value")?)
                            .as_float()
                            .clone(),
                        FitNumber::from(errors[k].ok_or("unwritten telescope error")?)
                            .as_float()
                            .clone(),
                    ))
                } else {
                    let cell = p.param.parameter(label).cell(k)?;
                    Ok((cell.value().clone(), cell.error().clone()))
                }
            };
        let old = coordinate(&input.context.pulsars()[0])?.0;
        fit_callbacks::update(&mut input.context, 0, label, k, 1e-9, 2e-10)?;
        let (value, error) = coordinate(&input.context.pulsars()[0])?;
        output.insert(
            format!("{key}_change"),
            serde_json::to_value((value - old).to_string_radix(10, Some(40)))?,
        );
        output.insert(
            format!("{key}_error"),
            serde_json::to_value(error.to_string_radix(10, Some(40)))?,
        );
        input.context = before;
    }
    assert!(
        fit_callbacks::derivative(
            &mut input.context,
            0,
            0.,
            0,
            tempo2_rust::tempo2::Label::param_gwecc,
            0
        )
        .unwrap_err()
        .to_string()
        .contains("pointer is null")
    );
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
