//! Unchanged checkIdeal lifecycle: simple preprocessing and no residual mean.
//! This is a direct API consumer; the general timing --fit CLI retains mean=1.
use tempo2_rust::{
    RuntimeContext, bats,
    general_fitting::{self, FitCycle, FitResult},
    preprocess, timing,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("expected RUNTIME PAR TIM".into());
    }
    let root = std::path::Path::new(&a[0]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: (&a[1]).into(),
            tim: (&a[2]).into(),
        }],
        root,
    )?;
    preprocess::pre_process_simple(&mut input.context, 0, root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    let prefit = timing::evaluate(&mut input, &mut runtime, false)?;
    let options = Default::default();
    let systems = general_fitting::t2_fit(&mut input.context, &options)?;
    let postfit = timing::evaluate(&mut input, &mut runtime, false)?;
    let result = FitResult {
        schema_version: "tempo2-general-fit-v1",
        options,
        remove_mean: false,
        fit_performed: true,
        covariance_coordinates: "raw source fit coordinates; global columns then pulsar-local blocks",
        cycles: vec![FitCycle {
            iteration: 1,
            prefit,
            systems,
            postfit,
        }],
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
