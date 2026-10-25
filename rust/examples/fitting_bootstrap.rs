//! Exact RNG and naturally loaded/fitted bootstrap example.
use tempo2_rust::{RuntimeContext, bats, bootstrap, general_fitting};
fn observations(
    context: &tempo2_rust::pulsar_lifecycle::PulsarContext,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut pulsars = vec![];
    for p in context.pulsars() {
        let rows = context.observations(p.obsn.flatten().ok_or("missing observation owner")?)?;
        let mut values = vec![];
        for o in &rows[..p.nobs.ok_or("missing nobs")? as usize] {
            let string = |bytes: &[Option<u8>]| {
                String::from_utf8(
                    bytes
                        .iter()
                        .map(|b| b.unwrap())
                        .take_while(|b| *b != 0)
                        .collect(),
                )
            };
            let mut flags = vec![];
            for i in 0..o.nFlags.ok_or("missing flag count")? as usize {
                flags.push(vec![string(&o.flagID[i])?, string(&o.flagVal[i])?]);
            }
            values.push(serde_json::json!({
                "sat":o.sat.as_ref().ok_or("missing SAT")?.to_string_radix(10,None),
                "bat":o.bat.as_ref().ok_or("missing BAT")?.to_string_radix(10,None),
                "bbat":o.bbat.as_ref().ok_or("missing BBAT")?.to_string_radix(10,None),
                "frequency":o.freq,"frequency_ssb":o.freqSSB,"uncertainty":o.toaErr,
                "deleted":o.deleted,"filename":string(&o.fname)?,"flags":flags,
                "earth":o.earth_ssb,"observatory":o.observatory_earth,
            }));
        }
        pulsars.push(values);
    }
    Ok(serde_json::to_value(pulsars)?)
}
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
    if args.len() != 4 && args.len() != 6 {
        return Err(
            "expected RUNTIME PAR TIM OUTPUT_DIRECTORY [SECOND_PAR SECOND_TIM], or rng SEED COUNT"
                .into(),
        );
    }
    let root = std::path::Path::new(&args[0]);
    let mut pairs = vec![bats::InputPair {
        par: args[1].clone().into(),
        tim: args[2].clone().into(),
    }];
    if args.len() == 6 {
        pairs.push(bats::InputPair {
            par: args[4].clone().into(),
            tim: args[5].clone().into(),
        });
    }
    let mut input = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    general_fitting::fit(&mut input, &mut runtime, &Default::default())?;
    let before = observations(&input.context)?;
    let result = bootstrap::bootstrap(
        &mut input.context,
        pairs.len() - 1,
        root,
        std::path::Path::new(&args[3]),
    )?;
    let after = observations(&input.context)?;
    let mut result = serde_json::to_value(result)?;
    result["before_observations"] = before;
    result["after_observations"] = after;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
