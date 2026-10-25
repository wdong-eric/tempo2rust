//! Standalone TKsvd APIs, including an extended RHS below binary64 range.
use tempo2_rust::{FitNumber, Scalar, tksvd};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matrix: Vec<Vec<FitNumber>> = [
        [1., 2., 0.5],
        [2., -1., 3.],
        [0.5, 4., -2.],
        [-1., 0.25, 1.],
        [3., 1., 2.],
    ]
    .into_iter()
    .map(|r| r.into_iter().map(Into::into).collect())
    .collect();
    let decomposition = tksvd::decompose(matrix.clone())?;
    let bidiagonal = tksvd::bidiagonal(matrix)?;
    let data = ["1e-400", "-2e-400", "3e-400", "4e-400", "-1e-400"]
        .into_iter()
        .map(|v| v.parse::<Scalar>().map(Into::into))
        .collect::<Result<Vec<FitNumber>, _>>()?;
    let solution = decomposition.back_substitute(&data)?;
    let mut dropped = decomposition.clone();
    dropped.singular_values[1] = 0.0.into();
    let dropped_solution = dropped.back_substitute(&data)?;
    let mut pythag = vec![];
    for (a, b) in [
        ("0", "0"),
        ("-3", "4"),
        ("1e300", "1e-300"),
        ("1e-400", "2e-400"),
    ] {
        pythag.push(tksvd::pythag(
            &a.parse::<Scalar>()?.into(),
            &b.parse::<Scalar>()?.into(),
        ));
    }
    let maximum = tempo2_rust::tkfit::find_max(&[
        "-1e400".parse::<Scalar>()?.into(),
        "2e400".parse::<Scalar>()?.into(),
        "1e400".parse::<Scalar>()?.into(),
    ])?;
    assert!(
        tempo2_rust::tkfit::find_max(&[(-0.0).into(), 0.0.into()])?
            .as_float()
            .is_sign_negative()
    );
    assert!(
        tempo2_rust::tkfit::find_max(&[f64::NAN.into(), 1.0.into()])?
            .as_float()
            .is_nan()
    );
    assert_eq!(
        tempo2_rust::tkfit::find_max(&[1.0.into(), f64::NAN.into()])?
            .as_float()
            .to_f64(),
        1.0
    );
    assert!(tempo2_rust::tkfit::find_max(&[]).is_err());
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "decomposition": decomposition, "bidiagonal": bidiagonal, "solution": solution,
            "dropped_solution": dropped_solution, "pythag": pythag, "maximum": maximum,
        }))?
    );
    Ok(())
}
