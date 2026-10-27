use tempo2_rust::{
    t2accel as a,
    tk_matrix::BlasMatrix,
    tkfit::{self, FitOutputs},
};

#[test]
fn scoped_selection_restores_after_error() {
    assert_eq!(a::acceleration(), 0);
    let result: Result<(), &str> = a::with_acceleration(2, || {
        assert_eq!(a::acceleration(), 2);
        a::with_acceleration(0, || assert_eq!(a::acceleration(), 0));
        assert_eq!(a::acceleration(), 2);
        Err("expected")
    });
    assert!(result.is_err());
    assert_eq!(a::acceleration(), 0);
}

#[test]
fn rectangular_products_and_dispatch() {
    let x = BlasMatrix::from_rows(&[vec![1., 2., 3.], vec![-1., 0., 4.]]).unwrap();
    let y = BlasMatrix::from_rows(&[vec![2., -1.], vec![0., 3.], vec![5., 1.]]).unwrap();
    if cfg!(feature = "blas") {
        assert_eq!(
            a::accel_mult_matrix(&x, &y).unwrap().cells(),
            &[17., 8., 18., 5.]
        );
        assert_eq!(
            a::accel_mult_matrix_vec(&x, &[2., -1., 3.]).unwrap(),
            vec![9., 10.]
        );
        assert!(a::accel_mult_matrix_vec(&x, &[1.]).is_err());
    } else {
        assert!(a::accel_mult_matrix(&x, &y).is_err());
    }
    for mode in 0..=2 {
        a::with_acceleration(mode, || {
            assert_eq!(
                tempo2_rust::tk_matrix::mult_matrix(&x.to_rows(), &y.to_rows()).unwrap(),
                vec![vec![17., 8.], vec![18., 5.]]
            )
        });
    }
}

#[test]
fn factors_fail_then_recover_with_defined_triangles() {
    let original = vec![vec![4., 2.], vec![2., 5.]];
    let mut m = BlasMatrix::from_rows(&original).unwrap();
    if !cfg!(feature = "lapack") {
        assert!(a::accel_cholfac(&mut m).is_err());
        return;
    }
    assert_eq!(a::accel_cholfac(&mut m).unwrap(), 0);
    assert_eq!(m.cells(), &[2., 2., 1., 2.]);
    m = BlasMatrix::from_rows(&original).unwrap();
    assert_eq!(a::accel_uinv(&mut m).unwrap(), 0);
    assert_eq!(m.cells(), &[0.5, 0., -0.25, 0.5]);
    m = BlasMatrix::from_rows(&[vec![-1., 0.], vec![0., 1.]]).unwrap();
    assert_eq!(a::accel_cholfac(&mut m).unwrap(), 1);
    assert_eq!(a::accel_uinv(&mut m).unwrap(), 1);
    let mut rectangular = BlasMatrix::from_rows(&[vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
    assert!(a::accel_cholfac(&mut rectangular).is_err());
    a::with_acceleration(1, || {
        assert_eq!(
            tempo2_rust::cholesky::form_l(&original).unwrap(),
            vec![vec![2., 2.], vec![1., 2.]]
        )
    });
}

#[test]
fn qr_real_rhs_optional_outputs_constraints_and_recovery() {
    let matrix = vec![vec![1., -1.], vec![1., 0.], vec![1., 1.]];
    let data = [1., 2., 4.];
    a::with_acceleration(2, || {
        for constrained in [false, true] {
            let c = if constrained {
                vec![vec![0., 1.]]
            } else {
                vec![]
            };
            let targets = if constrained { vec![3.] } else { vec![] };
            for rescale in [false, true] {
                let full = tkfit::least_squares_outputs(
                    Some(&data),
                    Some(&data),
                    &matrix,
                    &matrix,
                    &c,
                    &targets,
                    1e-27,
                    rescale,
                    FitOutputs::ALL,
                    None,
                )
                .unwrap();
                assert_eq!(
                    full.solver,
                    if cfg!(feature = "lapack") {
                        "QR"
                    } else {
                        "SVD"
                    }
                );
                for mask in 0..8 {
                    let fit = tkfit::least_squares_outputs(
                        Some(&data),
                        Some(&data),
                        &matrix,
                        &matrix,
                        &c,
                        &targets,
                        1e-27,
                        rescale,
                        FitOutputs {
                            parameters: mask & 1 != 0,
                            errors: mask & 2 != 0,
                            covariance: mask & 4 != 0,
                        },
                        None,
                    )
                    .unwrap();
                    if cfg!(feature = "lapack") {
                        assert_eq!(fit.chi_square.as_float(), full.chi_square.as_float());
                        assert_eq!(fit.rank, None);
                        assert!(fit.singular_values.is_empty());
                        assert!(fit.retained.is_empty());
                        let json = serde_json::to_value(&fit).unwrap();
                        for key in ["rank", "singular_values", "retained"] {
                            assert!(json.get(key).is_none());
                        }
                    }
                    assert_eq!(fit.increments.len(), if mask & 1 != 0 { 2 } else { 0 });
                    assert_eq!(fit.errors.len(), if mask & 2 != 0 { 2 } else { 0 });
                    assert_eq!(fit.covariance.len(), if mask & 4 != 0 { 2 } else { 0 });
                    // Source SVD skips the residual statistic without parameters;
                    // rescaling therefore zeroes requested errors. SVD covariance
                    // remains unscaled for every optional-output combination.
                    let same_scale = cfg!(feature = "lapack") || !rescale || mask & 1 != 0;
                    if mask & 2 != 0 && same_scale {
                        assert_eq!(
                            serde_json::to_value(&fit.errors).unwrap(),
                            serde_json::to_value(&full.errors).unwrap()
                        );
                    }
                    if mask & 4 != 0 {
                        assert_eq!(
                            serde_json::to_value(&fit.covariance).unwrap(),
                            serde_json::to_value(&full.covariance).unwrap()
                        );
                    }
                    if !same_scale {
                        assert_eq!(fit.chi_square.as_float().to_f64(), 0.0);
                        assert!(fit.errors.iter().all(|v| v.as_float().is_zero()));
                    }
                }
            }
        }
        if cfg!(feature = "lapack") {
            let mut m = BlasMatrix::from_rows(&matrix).unwrap();
            let mut short_rhs = [1., 2.];
            assert!(a::accel_lsq_qr(&mut m, &mut short_rhs, None, None, false).is_err());
            let mut rhs = data;
            let mut short_parameters = [0.];
            assert!(
                a::accel_lsq_qr(&mut m, &mut rhs, Some(&mut short_parameters), None, false)
                    .is_err()
            );
            let mut wrong_covariance = BlasMatrix::malloc_uinv(3).unwrap();
            assert!(
                a::accel_lsq_qr(&mut m, &mut rhs, None, Some(&mut wrong_covariance), false)
                    .is_err()
            );
            assert!(
                tkfit::least_squares_outputs(
                    None,
                    None,
                    &matrix,
                    &matrix,
                    &[],
                    &[],
                    1e-27,
                    false,
                    FitOutputs::ALL,
                    None
                )
                .is_err()
            );
            let bad = vec![vec![1., 0.]; 3];
            assert!(tkfit::tk_least_squares(&data, &data, &bad, &bad, false).is_err());
            assert!(tkfit::tk_least_squares(&data, &data, &matrix, &matrix, false).is_ok());
            let io = tkfit::FitDiagnostics {
                directory: "/a-missing-native-backends-output-directory".into(),
                write_residuals: 2,
                debug: false,
            };
            assert!(
                tkfit::least_squares_outputs(
                    Some(&data),
                    Some(&data),
                    &matrix,
                    &matrix,
                    &[],
                    &[],
                    1e-27,
                    false,
                    FitOutputs::ALL,
                    Some(&io)
                )
                .is_err()
            );
            assert!(tkfit::tk_least_squares(&data, &data, &matrix, &matrix, false).is_ok());
            let directory =
                std::env::temp_dir().join(format!("tempo2-ai-qr-{}", std::process::id()));
            std::fs::create_dir(&directory).unwrap();
            std::fs::create_dir(directory.join("awhite.res")).unwrap();
            let diagnostics = tkfit::FitDiagnostics {
                directory: directory.clone(),
                write_residuals: 2,
                debug: false,
            };
            tempo2_rust::take_clock_diagnostics();
            assert!(
                tkfit::least_squares_outputs(
                    Some(&data),
                    Some(&data),
                    &matrix,
                    &matrix,
                    &[],
                    &[],
                    1e-27,
                    false,
                    FitOutputs::ALL,
                    Some(&diagnostics)
                )
                .is_err()
            );
            assert!(tempo2_rust::take_clock_diagnostics().iter().any(|event| {
                event
                    .bytes
                    .windows(b"QR nparams=".len())
                    .any(|s| s == b"QR nparams=")
            }));
            assert!(directory.join("adesign.matrix").is_file());
            std::fs::remove_dir(directory.join("awhite.res")).unwrap();
            assert!(
                tkfit::least_squares_outputs(
                    Some(&data),
                    Some(&data),
                    &matrix,
                    &matrix,
                    &[],
                    &[],
                    1e-27,
                    false,
                    FitOutputs::ALL,
                    Some(&diagnostics)
                )
                .is_ok()
            );
            assert_eq!(
                std::fs::read_to_string(directory.join("awhite.res")).unwrap(),
                "0 1\n1 2\n2 4\n"
            );
            std::fs::remove_dir_all(directory).unwrap();
        }
    });
}

#[cfg(feature = "calceph")]
#[test]
#[ignore = "requires AI_EPHEMERIS pointing to the recorded real INPOP17A file"]
fn calceph_real_provider_failures_release_and_recover() {
    use rug::Float;
    use tempo2_rust::{calceph, pulsar_lifecycle::PulsarContext};
    let path = std::env::var("AI_EPHEMERIS").expect("recorded real ephemeris required");
    let fd_before = std::fs::read_dir("/dev/fd").unwrap().count();
    let mut c = PulsarContext::unwritten(2);
    c.initialise(2, 4, 1).unwrap();
    for p in 0..2 {
        c.pulsar_observations_mut(p).unwrap().0.nobs = Some(p as i32 + 2);
        let (s, rows) = c.pulsar_observations_mut(p).unwrap();
        for (i, b) in path.bytes().chain([0]).enumerate() {
            s.JPL_EPHEMERIS[i] = Some(b);
        }
        for (i, o) in rows.iter_mut().take(p + 2).enumerate() {
            o.sat = Some(Float::with_val(
                128,
                58000.125 + p as f64 * 20. + i as f64 / 8.,
            ));
            o.nclock_correction = Some(0);
            o.correctionTT_Teph = Some(Float::with_val(128, 0));
        }
    }
    calceph::tt2tb_calceph(&mut c).unwrap();
    let first = c.pulsar_observations_mut(0).unwrap().1[0]
        .correctionTT_calcEph
        .clone();
    let second = c.pulsar_observations_mut(1).unwrap().1[2]
        .correctionTT_calcEph
        .clone();
    assert_ne!(first, second);
    calceph::read_ephemeris_calceph(&mut c).unwrap();
    assert_eq!(
        c.pulsar_observations_mut(0).unwrap().1[0].planet_ssb_derv[0][0],
        None
    );
    let position_only =
        std::env::var("AI_POSITION_ONLY").expect("recorded JPL file without target 16 required");
    for (i, b) in position_only.bytes().chain([0]).enumerate() {
        c.pulsar_observations_mut(0).unwrap().0.JPL_EPHEMERIS[i] = Some(b);
    }
    assert!(calceph::tt2tb_calceph(&mut c).is_err());
    for (i, b) in path.bytes().chain([0]).enumerate() {
        c.pulsar_observations_mut(0).unwrap().0.JPL_EPHEMERIS[i] = Some(b);
    }
    calceph::tt2tb_calceph(&mut c).unwrap();
    for _ in 0..8 {
        c.pulsar_observations_mut(0).unwrap().1[0].sat = Some(Float::with_val(128, 999999));
        assert!(calceph::tt2tb_calceph(&mut c).is_err());
        assert!(calceph::read_ephemeris_calceph(&mut c).is_err());
        assert_eq!(
            c.pulsar_observations_mut(0).unwrap().1[0].correctionTT_calcEph,
            first
        );
        c.pulsar_observations_mut(0).unwrap().1[0].sat = Some(Float::with_val(128, 58000.125));
        calceph::tt2tb_calceph(&mut c).unwrap();
        calceph::read_ephemeris_calceph(&mut c).unwrap();
        assert_eq!(
            c.pulsar_observations_mut(0).unwrap().1[0].correctionTT_calcEph,
            first
        );
    }
    let missing = "/missing-ai-ephemeris";
    for (i, b) in missing.bytes().chain([0]).enumerate() {
        c.pulsar_observations_mut(0).unwrap().0.JPL_EPHEMERIS[i] = Some(b);
    }
    assert!(calceph::tt2tb_calceph(&mut c).is_err());
    for (i, b) in path.bytes().chain([0]).enumerate() {
        c.pulsar_observations_mut(0).unwrap().0.JPL_EPHEMERIS[i] = Some(b);
    }
    calceph::tt2tb_calceph(&mut c).unwrap();
    assert_eq!(
        c.pulsar_observations_mut(1).unwrap().1[2].correctionTT_calcEph,
        second
    );
    assert_eq!(std::fs::read_dir("/dev/fd").unwrap().count(), fd_before);
}
