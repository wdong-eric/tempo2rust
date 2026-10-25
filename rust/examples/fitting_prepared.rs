//! Assemble and apply a single-pulsar fit through the public preparation APIs.
//! Compare installed callback rows and canonical postfit timing with t2_fit.
use tempo2_rust::{RuntimeContext, bats, general_fitting as fit, timing};
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
    let mut runtime = RuntimeContext::from_root(root)?;
    timing::evaluate(&mut input, &mut runtime, true)?;
    let prepared = input.context.clone();
    let options = fit::FitOptions::default();
    let systems = fit::t2_fit(&mut input.context, &options)?;
    let expected = timing::evaluate(&mut input, &mut runtime, true)?;
    input.context = prepared;
    fit::prefit(&mut input.context, 0)?;
    let globals = fit::global_fit_columns(&input.context.pulsars()[0], &options)?;
    let rows = fit::get_fit_data(&mut input.context, 0)?;
    let columns = fit::fill_fit_columns(&mut input.context, 0, &rows, &globals, &options)?;
    assert_eq!(
        serde_json::to_value(&columns)?,
        serde_json::to_value(&systems[0].columns)?
    );
    for (i, row) in rows.iter().enumerate() {
        let actual = fit::design_matrix_row(
            &mut input.context,
            0,
            row.x_days.as_float().to_f64(),
            row.observation,
        )?;
        assert_eq!(
            actual,
            systems[0].matrix[i]
                .iter()
                .map(|v| v.as_float().to_f64())
                .collect::<Vec<_>>()
        );
    }
    let constraints = fit::fill_constraint_info(&mut input.context, 0, &columns)?;
    for (i, expected) in constraints.matrix.iter().enumerate() {
        assert_eq!(
            *expected,
            fit::constraint_matrix_row(&mut input.context, 0, i)?
        );
        assert_eq!(
            *expected,
            systems[0].constraints[i]
                .iter()
                .map(|v| v.as_float().to_f64())
                .collect::<Vec<_>>()
        );
    }
    // The full driver's independently checked solver supplies the same local
    // coordinates here; this example checks preparation and mutation wiring.
    let solver = &systems[0].solver;
    fit::update_parameters(
        &mut input.context,
        0,
        &solver
            .increments
            .iter()
            .map(|v| v.as_float().to_f64())
            .collect::<Vec<_>>(),
        &solver
            .errors
            .iter()
            .map(|v| v.as_float().to_f64())
            .collect::<Vec<_>>(),
    )?;
    fit::postfit(&mut input.context, None)?;
    let actual = timing::evaluate(&mut input, &mut runtime, true)?;
    assert_eq!(
        serde_json::to_value(&actual.pulsars)?,
        serde_json::to_value(&expected.pulsars)?
    );
    println!(
        "{}",
        serde_json::json!({"rows":rows.len(),"columns":columns.len(),"constraints":constraints.matrix.len(),"canonical_postfit_equal":true})
    );
    Ok(())
}
