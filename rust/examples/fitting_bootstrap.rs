//! Exact RNG and naturally loaded/fitted bootstrap example.
use tempo2_rust::{RuntimeContext, bats, bootstrap, general_fitting};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("rng") && args.len() == 3 {
        let mut rng = bootstrap::BootstrapRng::default();
        let mut seed = args[1].parse()?;
        let mut values = Vec::new();
        for _ in 0..args[2].parse::<usize>()? {
            values.push(format!("{:016x}", rng.random(&mut seed)?.to_bits()));
        }
        println!("{}", serde_json::json!({"seed":seed,"values":values}));
        return Ok(());
    }
    if args.len() != 4 {
        return Err("expected RUNTIME PAR TIM OUTPUT_DIRECTORY, or rng SEED COUNT".into());
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
    general_fitting::fit(&mut input, &mut runtime, &Default::default())?;
    let result = bootstrap::bootstrap(&mut input.context, 0, root, std::path::Path::new(&args[3]))?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
