//! External canonical two-pulsar timing/fitting/output and recovery client.
use tempo2_rust::{
    RuntimeContext,
    bats::{self, InputPair},
    core_cli, core_output, general_fitting, timing,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(a.len(), 5);
    let root = std::path::Path::new(&a[0]);
    let pairs = vec![
        InputPair {
            par: a[1].clone().into(),
            tim: a[2].clone().into(),
        },
        InputPair {
            par: a[3].clone().into(),
            tim: a[4].clone().into(),
        },
    ];
    let mut input = bats::load_pairs(&pairs, root)?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    let first = timing::evaluate(&mut input, &mut runtime, true)?;
    assert_eq!(first.pulsars.len(), 2);
    let before = serde_json::to_value(&first.pulsars)?;
    let second = timing::evaluate(&mut input, &mut runtime, true)?;
    assert_eq!(before, serde_json::to_value(&second.pulsars)?);
    let fitted = general_fitting::t2_fit(&mut input.context, &Default::default())?;
    assert!(!fitted.is_empty());
    let post = timing::evaluate(&mut input, &mut runtime, true)?;
    assert_eq!(post.pulsars.len(), 2);
    assert!(
        bats::load_pairs(
            &[InputPair {
                par: "missing.par".into(),
                tim: "missing.tim".into()
            }],
            root
        )
        .is_err()
    );
    let again = timing::evaluate(&mut input, &mut runtime, true)?;
    assert_eq!(
        serde_json::to_value(&post.pulsars)?,
        serde_json::to_value(&again.pulsars)?
    );
    let _ = core_output::calc_rms(&mut input.context, 0)?;
    let mut out = Vec::new();
    let mut err = Vec::new();
    let mut stdin = std::io::Cursor::new(Vec::<u8>::new());
    let mut io = core_output::OutputIo {
        stdout: &mut out,
        stderr: &mut err,
        stdin: &mut stdin,
        directory: std::path::Path::new("."),
    };
    core_output::display_parameters(
        2,
        &pairs.iter().map(|p| p.tim.clone()).collect::<Vec<_>>(),
        &pairs.iter().map(|p| p.par.clone()).collect::<Vec<_>>(),
        &input.context,
        &mut io,
    )?;
    let status = core_cli::run(
        &["external".into(), "-version".into()],
        &core_cli::Environment::from_process(),
        &mut io,
    )?;
    assert_eq!(status, 1);
    assert!(!out.is_empty());
    println!("two pulsars, timing, fit, output and recovery passed");
    Ok(())
}
