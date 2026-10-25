//! Same-process failure/recovery on naturally loaded canonical fitting state.
use tempo2_rust::{RuntimeContext, bats, bootstrap, general_fitting as fit, timing};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 4 {
        return Err("expected RUNTIME PAR TIM OUTPUT_DIRECTORY".into());
    }
    let root = std::path::Path::new(&a[0]);
    let out = std::path::Path::new(&a[3]);
    let mut input = bats::load_pairs(
        &[bats::InputPair {
            par: a[1].clone().into(),
            tim: a[2].clone().into(),
        }],
        root,
    )?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    timing::evaluate(&mut input, &mut runtime, true)?;
    let timed = input.context.clone();
    let expected = fit::t2_fit(&mut input.context, &Default::default())?;
    let fitted = input.context.clone();
    let missing = out.join("missing-directory");
    assert!(!missing.exists());
    let mut records = vec![];
    for (name, options) in [
        (
            "missing-covariance",
            fit::FitOptions {
                covariance: Some(missing.join("covariance.model").display().to_string()),
                ..Default::default()
            },
        ),
        (
            "diagnostic-write",
            fit::FitOptions {
                diagnostics: Some(tempo2_rust::tkfit::FitDiagnostics {
                    directory: missing.clone(),
                    write_residuals: 2,
                    debug: false,
                }),
                ..Default::default()
            },
        ),
    ] {
        input.context = timed.clone();
        let error = fit::t2_fit(&mut input.context, &options).unwrap_err();
        let actual = fit::t2_fit(&mut input.context, &Default::default())?;
        assert_eq!(
            serde_json::to_value(&actual)?,
            serde_json::to_value(&expected)?
        );
        records
            .push(serde_json::json!({"case":name,"error":error.to_string(),"recovery_equal":true}));
    }
    input.context = fitted;
    let n = input.context.pulsars().len();
    let error = bootstrap::bootstrap(&mut input.context, 0, root, &missing).unwrap_err();
    assert_eq!(input.context.pulsars().len(), n);
    assert!(input.context.boundary().is_none());
    let result = bootstrap::bootstrap(&mut input.context, 0, root, out)?;
    assert_eq!(input.context.pulsars().len(), n);
    assert!(!result.iterations.is_empty());
    records.push(serde_json::json!({"case":"bootstrap-write","error":error.to_string(),"recovered_iterations":result.iterations.len(),"active_pulsars":n,"release_trace":input.context.release_trace()}));
    input.context.pulsar_observations_mut(0)?.0.bootStrap = Some(13);
    let error = bootstrap::bootstrap(&mut input.context, 0, root, out).unwrap_err();
    assert_eq!(input.context.pulsars().len(), n);
    input.context.pulsar_observations_mut(0)?.0.bootStrap = Some(0);
    assert_eq!(
        bootstrap::bootstrap(&mut input.context, 0, root, out)?
            .iterations
            .len(),
        1
    );
    records.push(serde_json::json!({"case":"bootstrap-capacity","error":error.to_string(),"recovered_iterations":1}));
    use tempo2_rust::cholesky_routines::{SpectralAnswers, SpectralFitSession, SpectralModel};
    let x = [0.125, 0.25, 0.5, 1., 2., 4., 8., 16.];
    let y = [4., 1., 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125];
    let mut model = SpectralModel {
        alpha: 3.,
        corner_per_year: 0.2,
        nfit: 4,
        scale: 0.,
        variance: 0.,
        beta: 0.5,
    };
    let mut session = SpectralFitSession::default();
    let error = session
        .fit(
            &x,
            &y,
            &mut model,
            false,
            0.2,
            3.,
            -2,
            -1.,
            false,
            -1.,
            &Default::default(),
        )
        .unwrap_err();
    let answers = SpectralAnswers {
        redo: Some(true),
        nfit: Some(4),
        ..Default::default()
    };
    let mut fresh_model = model.clone();
    assert!(session.fit(
        &x, &y, &mut model, false, 0.2, 3., -1, -1., false, 0.2, &answers
    )?);
    assert!(SpectralFitSession::default().fit(
        &x,
        &y,
        &mut fresh_model,
        false,
        0.2,
        3.,
        -1,
        -1.,
        false,
        0.2,
        &answers
    )?);
    assert_eq!(
        (model.scale, model.variance),
        (fresh_model.scale, fresh_model.variance)
    );
    records.push(serde_json::json!({"case":"spectral-cutoff","error":error.to_string(),"recovery_equal":true}));
    println!("{}", serde_json::to_string_pretty(&records)?);
    Ok(())
}
