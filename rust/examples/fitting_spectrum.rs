//! Stateful T2fitSpectra selection and repeated-call examples.
use tempo2_rust::{
    cholesky_routines::{SpectralAnswers, SpectralFitSession, SpectralModel},
    tkfit::FitNumber,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scenario: usize = std::env::args()
        .nth(1)
        .ok_or("expected scenario")?
        .parse()?;
    let x = [0.125, 0.25, 0.5, 1., 2., 4., 8., 16.];
    let y = [4., 1., 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125];
    let mut session = SpectralFitSession::default();
    let mut model = SpectralModel {
        alpha: 3.,
        corner_per_year: 0.2,
        nfit: 4,
        scale: -10.,
        variance: -20.,
        beta: 0.5,
    };
    let mut output = vec![];
    for call in 0..if scenario == 0 { 4 } else { 1 } {
        let automatic = scenario == 0 && call == 0;
        let corner = if scenario == 0 {
            if call < 2 { 0.2 } else { -1. }
        } else {
            -2.
        };
        let exponent = if scenario == 0 {
            if call < 2 { 3. } else { 0. }
        } else {
            -2.
        };
        let points = if scenario == 0 {
            if call < 2 { 4 } else { -1 }
        } else if scenario == 1 {
            -2
        } else {
            1
        };
        let cutoff = if scenario == 2 {
            4.
        } else if scenario == 3 {
            1.
        } else {
            0.2
        };
        let amplitude = if scenario == 4 { 8. } else { -1. };
        let answers = SpectralAnswers {
            redo: Some(call != 2),
            corner_per_year: Some(0.5),
            beta: Some(0.25),
            alpha: Some(4.),
            nfit: Some(5),
        };
        let result = session.fit(
            &x,
            &y,
            &mut model,
            automatic,
            corner,
            exponent,
            points,
            amplitude,
            scenario == 0 && call == 3,
            cutoff,
            &answers,
        )?;
        let values = [
            model.alpha,
            model.corner_per_year,
            model.scale,
            model.variance,
            model.beta,
        ]
        .into_iter()
        .map(FitNumber::from)
        .collect::<Vec<_>>();
        output.push(
            serde_json::json!({"call":call, "result":result, "nfit":model.nfit, "values":values }),
        );
    }
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
