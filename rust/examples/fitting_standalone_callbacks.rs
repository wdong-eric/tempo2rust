//! Source callbacks that are not all selected by the default fit dispatcher.
use std::path::PathBuf;
use tempo2_rust::{
    RuntimeContext, bats, fit_callbacks, tempo2::Label::*, timing, tkfit::FitNumber,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("expected RUNTIME PAR TIM".into());
    }
    let root = PathBuf::from(&a[0]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: a[1].clone().into(),
            tim: a[2].clone().into(),
        }],
        &root,
    )?;
    bats::preprocess(&mut input, &[], &root)?;
    let mut runtime = RuntimeContext::from_root(&root)?;
    timing::evaluate(&mut input, &mut runtime, true)?;
    let n = input.context.pulsars()[0].nobs.ok_or("unwritten nobs")? as usize;
    let mut result = serde_json::Map::new();
    for (name, label) in [
        ("ifunc", param_ifunc),
        ("plus", param_quad_ifunc_p),
        ("cross", param_quad_ifunc_c),
    ] {
        for k in 0..3 {
            let mut values = vec![];
            for i in 0..n {
                values.push(FitNumber::from(fit_callbacks::sinc_derivative(
                    &mut input.context,
                    0,
                    0.,
                    i,
                    label,
                    k,
                )?));
            }
            result.insert(format!("{name}_{k}"), serde_json::to_value(values)?);
        }
    }
    for k in 0..2 {
        let mut values = vec![];
        for i in 0..n {
            values.push(FitNumber::from(fit_callbacks::gwb_amplitude(
                &mut input.context,
                0,
                0.,
                i,
                k,
            )?));
        }
        result.insert(format!("gwb_{k}"), serde_json::to_value(values)?);
    }
    for (name, label, value, error) in [
        ("glf0", param_glf0, 1e-9, 2e-9),
        ("glf2", param_glf2, 0.2, 0.03),
    ] {
        fit_callbacks::std_glitch_update(&mut input.context, 0, label, 0, value, error)?;
        let cell = input.context.pulsars()[0].param.parameter(label).cell(0)?;
        result.insert(
            format!("{name}_value"),
            serde_json::to_value(cell.value().to_string_radix(10, Some(40)))?,
        );
        result.insert(
            format!("{name}_error"),
            serde_json::to_value(cell.error().to_string_radix(10, Some(40)))?,
        );
    }
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
