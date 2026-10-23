//! Rust-only verification driver for the public fit APIs; TEMPO2 comparisons are external.
use std::{fs, path::PathBuf, process::Command};
use tempo2_rust::{FitOptions, RuntimeContext, fit_barycentric, fit_terrestrial, load};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting");
    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/m2");
    fs::create_dir_all(&out)?;
    let cases: serde_json::Value = serde_json::from_slice(&fs::read(root.join("cases.json"))?)?;
    let selected = std::env::args().nth(1);
    if selected.is_none() {
        // Each reference fixture starts a process. A new RuntimeContext now
        // correctly retains clock/observatory process state, so isolate cases
        // here while preserving within-case NITS and recovery call histories.
        for case in cases["timing"].as_array().unwrap() {
            let status = Command::new(std::env::current_exe()?)
                .arg(case["name"].as_str().unwrap())
                .status()?;
            assert!(
                status.success(),
                "fit fixture process failed: {}",
                case["name"]
            );
        }
        return Ok(());
    }
    let selected = selected.unwrap();
    assert!(
        cases["timing"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["name"] == selected)
    );
    for case in cases["timing"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["name"] == selected)
    {
        let input = load(
            &root.join("inputs").join(case["par"].as_str().unwrap()),
            &root.join("inputs").join(case["tim"].as_str().unwrap()),
        )?;
        let before = serde_json::to_vec(&input)?;
        let options = FitOptions {
            rescale_err_chisq: case["rescale"].as_bool().unwrap(),
        };
        let result = if input.observations()[0].original.site == "@" {
            fit_barycentric(&input, &options)?
        } else {
            fit_terrestrial(
                &input,
                &mut RuntimeContext::new(&root.join("../../tempo2/T2runtime"))?,
                &options,
            )?
        };
        assert_eq!(before, serde_json::to_vec(&input)?);
        fs::write(
            out.join(format!("{}.json", case["name"].as_str().unwrap())),
            serde_json::to_vec_pretty(&result)?,
        )?;
        println!("{}: completed, caller unchanged", case["name"]);
    }
    Ok(())
}
