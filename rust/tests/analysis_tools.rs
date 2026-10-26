use tempo2_rust::{
    pulsar_lifecycle::PulsarContext,
    sigma_z::SigmaZ,
    spectrum::{Options, analyze},
};
#[test]
fn weighted_dispatch_warning_backing_and_recovery() {
    let x: Vec<_> = (0..8).map(|i| i as f64).collect();
    let y = vec![1., 3., 2., 7., 4., 9., 5., 8.];
    for estimator in [4, 5] {
        for (prewhite, name) in [
            (0, None),
            (1, Some("first differencing")),
            (2, Some("second differencing")),
        ] {
            let r = analyze(
                &mut x.clone(),
                &mut y.clone(),
                &mut [1.; 8],
                &Options {
                    estimator,
                    prewhite,
                    ..Default::default()
                },
            )
            .unwrap();
            assert_eq!(r.warnings, name.map(|s|format!("TKspectrum: {s} retains original positional errors without propagated uncertainty/covariance")).into_iter().collect::<Vec<_>>());
        }
        for diagnostic in 1..=5 {
            let r = analyze(
                &mut x.clone(),
                &mut y.clone(),
                &mut [],
                &Options {
                    estimator,
                    fit_spline: 1,
                    diagnostic,
                    ..Default::default()
                },
            )
            .unwrap();
            assert!(r.warnings.is_empty());
            assert_eq!(r.source_exit_status, Some(1));
            assert!(!r.reported_count_written);
        }
        let r = analyze(
            &mut x.clone(),
            &mut y.clone(),
            &mut [],
            &Options {
                estimator,
                calc_white: 1,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(r.warnings.is_empty());
        assert!(!r.reported_count_written);
        assert!(
            analyze(
                &mut x.clone(),
                &mut y.clone(),
                &mut [1.; 8],
                &Options {
                    estimator,
                    fit_spline: 1,
                    ..Default::default()
                }
            )
            .is_err()
        );
        let r = analyze(
            &mut x.clone(),
            &mut y.clone(),
            &mut [1.; 8],
            &Options {
                estimator,
                average_time: 2,
                smooth_type: 1,
                smooth_width: 3,
                prewhite: 2,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            r.warnings.last().unwrap(),
            "TKspectrum: averaging, Hann smoothing, second differencing retains original positional errors without propagated uncertainty/covariance"
        );
    }
    // Unweighted spline consumes no errors, and a reached weighted branch needs only
    // the transformed count when sorted input already has valid positional entries.
    assert!(
        analyze(
            &mut x.clone(),
            &mut y.clone(),
            &mut [],
            &Options {
                fit_spline: 1,
                estimator: 3,
                ..Default::default()
            }
        )
        .is_ok()
    );
    assert!(
        analyze(
            &mut x.clone(),
            &mut y.clone(),
            &mut [1.; 6],
            &Options {
                smooth_type: 1,
                smooth_width: 3,
                estimator: 4,
                ..Default::default()
            }
        )
        .is_ok()
    );
}
#[test]
fn sigmaz_failure_cannot_reuse_partial_state() {
    let mut context = PulsarContext::unwritten(1);
    context.initialise_one(0, 16, 1, 1).unwrap();
    context.pulsar_observations_mut(0).unwrap().0.nobs = Some(16);
    let (_, rows) = context.pulsar_observations_mut(0).unwrap();
    for (i, r) in rows.iter_mut().enumerate() {
        r.sat = Some(rug::Float::with_val(128, 54000 + i * 30));
        r.residual = Some(rug::Float::with_val(128, (i * i % 7) as f64 * 1e-6));
        r.toaErr = Some(1.0);
    }
    let mut sigma = SigmaZ::default();
    assert!(sigma.calculate(&mut context, 0, -1, 0.03).is_ok());
    assert!(sigma.getweights(1).is_ok());
    assert!(sigma.calculate(&mut context, 0, -4, 0.03).is_err());
    assert!(sigma.getweights(1).is_err());
    assert!(sigma.calculate(&mut context, 0, -1, 0.03).is_ok());
}
#[test]
fn fft_singleton_and_exact_aligned_gw_are_defined() {
    let (mut x, mut y) = ([2.0], [-3.0]);
    tempo2_rust::spectrum::fft(1, &mut x, &mut y).unwrap();
    assert_eq!(x, [2.]);
    assert_eq!(y, [-3.]);
    use tempo2_rust::gw_sim::*;
    let ld = |x| rug::Float::with_val(128, x);
    let mut g = TensorSource::from_prefix([0., 0., 1., 0., 0., 1., 2., 3., 4.].map(ld));
    g.setup();
    let kp = [ld(0.), ld(0.), ld(-1.)];
    assert!(g.residual(&kp, &ld(1.), &ld(1.)).unwrap().is_zero());
}
