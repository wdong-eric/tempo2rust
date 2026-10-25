use crate::{
    pulsar_lifecycle::PulsarContext, t2fit_nestlike::*, tempo2::Label::*,
    tkfit::constrained_least_squares, tksvd::ld,
};
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 5e-14 * b.abs().max(1.0),
        "{a:.17e} != {b:.17e}"
    );
}
#[test]
fn nonzero_constraint_target_and_zero_neighbor() {
    let a = vec![vec![1.0]; 3];
    let y = vec![0.0; 3];
    let c = vec![vec![2.0]];
    for (target, expected) in [(0.0, 0.0), (4.0, 8.0 / 7.0)] {
        let fit = constrained_least_squares(&y, &y, &a, &a, &c, &[target], 1e-27, false).unwrap();
        close(fit.increments[0].0.to_f64(), expected);
        close(fit.covariance[0][0].0.to_f64(), 1.0 / 7.0);
        close(fit.chi_square.0.to_f64(), 3.0 * expected * expected);
        assert_eq!(fit.dof, 3);
    }
}
#[test]
fn optional_solver_outputs_preserve_source_null_contract() {
    use crate::tkfit::{FitOutputs, least_squares_outputs};
    let a = vec![vec![1.0]; 3];
    for parameters in [false, true] {
        for errors in [false, true] {
            for covariance in [false, true] {
                let f = least_squares_outputs(
                    None,
                    None,
                    &a,
                    &a,
                    &[],
                    &[],
                    1e-27,
                    false,
                    FitOutputs {
                        parameters,
                        errors,
                        covariance,
                    },
                    None,
                )
                .unwrap();
                assert!(f.increments.is_empty());
                assert!(f.linearized_s.is_empty());
                assert_eq!(f.chi_square.0.to_f64(), 0.0);
                assert_eq!(f.errors.len(), usize::from(errors));
                assert_eq!(f.covariance.len(), usize::from(covariance));
                if errors {
                    close(f.errors[0].0.to_f64(), (1.0_f64 / 3.0).sqrt());
                }
                if covariance {
                    close(f.covariance[0][0].0.to_f64(), 1.0 / 3.0);
                }
            }
        }
    }
}

#[test]
fn diagnostic_numeric_transport_preserves_sign_and_class() {
    use crate::tkfit::FitNumber;
    for (value, expected) in [
        (f64::NAN, "NaN"),
        (-f64::NAN, "-NaN"),
        (f64::INFINITY, "+Infinity"),
        (f64::NEG_INFINITY, "-Infinity"),
    ] {
        assert_eq!(
            serde_json::to_value(FitNumber::from(value)).unwrap(),
            expected
        );
    }
    let text = serde_json::to_value(FitNumber::from(-0.0)).unwrap();
    assert!(text.as_str().unwrap().starts_with('-'));
}

#[test]
fn copying_unused_prefit_cells_preserves_unwritten_state() {
    let mut c = PulsarContext::unwritten(2);
    c.allocate_memory(0, 0).unwrap();
    c.allocate_memory(1, 0).unwrap();
    let (left, right) = c.pulsars.split_at_mut(1);
    let src = &mut left[0].param.slots[param_f as usize];
    let dst = &mut right[0].param.slots[param_f as usize];
    src.cells[0].prefit = Some(ld(10.0));
    src.cells[0].prefit_err = Some(ld(0.01));
    assert!(src.cells[1].prefit.is_none());
    crate::copy_param(src, dst).unwrap();
    assert_eq!(dst.cells[0].prefit, Some(ld(10.0)));
    assert!(dst.cells[1].prefit.is_none());
    assert!(dst.cells[1].prefit_err.is_none());
}
#[test]
#[allow(clippy::excessive_precision)] // Preserve the native probe's printed values.
fn native_robust_outlier_witness() {
    let a = vec![vec![1.0]; 3];
    let y = [1.0, 2.0, 100.0];
    for (kind, estimate, chi) in [
        (b'0', 34.333333333333336, 6468.666666666667),
        (b'H', 34.333333333333336, 6468.6666666666661),
        (b'B', 29.024299888193791, 9155.8432141201338),
        (b'W', 4.8784378310184371, 13221.431948216094),
        (b'X', 34.333333333333336, 6468.6666666666661),
    ] {
        let fit = crate::tkrobust::least_squares(&y, &y, &a, &a, 1e-27, false, kind).unwrap();
        close(fit.increments[0].0.to_f64(), estimate);
        close(fit.covariance[0][0].0.to_f64(), 1.0 / 3.0);
        close(fit.errors[0].0.to_f64(), (1.0f64 / 3.0).sqrt());
        close(fit.chi_square.0.to_f64(), chi);
    }
}

#[test]
fn one_point_covariance_takes_diagonal_branch() {
    let mut context = PulsarContext::unwritten(1);
    let mut inputs = vec![];
    let (covariance, lower) = crate::cholesky::get_cholesky_matrix(
        &mut context,
        0,
        "unused-one-point-model",
        &[0],
        &[0.0],
        &[2e-6],
        &mut inputs,
    )
    .unwrap();
    assert!(inputs.is_empty());
    assert_eq!(covariance, vec![vec![4e-12]]);
    assert_eq!(lower, vec![vec![2e-6]]);
    assert_eq!(context.pulsars[0].detL, Some(2e-6_f64.ln()));
}
#[test]
fn polynomial_projection_tracks_toas_and_frequency() {
    let mut c = PulsarContext::unwritten(1);
    c.initialise(1, 8, 1).unwrap();
    c.pulsars[0].nobs = Some(5);
    let (p, rows) = c.pulsar_observations_mut(0).unwrap();
    p.nobs = Some(5);
    p.TNRedC = Some(2);
    p.TNRedFLow = Some(0.0);
    p.TNsubtractPoly = Some(t2EncodeTNsubtractPoly(2, 2, 2, 2));
    for (l, v) in [
        (param_start, 58000.0),
        (param_finish, 58100.0),
        (param_pepoch, 58000.0),
    ] {
        p.param.slots[l as usize].cells[0].val = ld(v);
    }
    for (i, t) in [0.0, 20.0, 45.0, 70.0, 100.0].into_iter().enumerate() {
        rows[i].bbat = Some(ld(58000.0 + t));
    }
    close(
        derivative(p, rows, 30.0, 0, param_red_sin, 0).unwrap(),
        0.7732837925990843,
    );
    rows[2].bbat = Some(ld(58060.0));
    close(
        derivative(p, rows, 30.0, 0, param_red_sin, 0).unwrap(),
        1.0837740040674018,
    );
    p.TNRedFLow = Some(0.2);
    close(
        derivative(p, rows, 30.0, 0, param_red_sin, 0).unwrap(),
        -0.3588247533948237,
    );
}
#[test]
fn cholesky_whitening_and_inverse() {
    let c = vec![vec![4.0, 2.0], vec![2.0, 10.0]];
    let l = crate::cholesky::form_l(&c).unwrap();
    assert_eq!(l, vec![vec![2.0, 0.0], vec![1.0, 3.0]]);
    let x = crate::tk_matrix::forward_sub_vec(&l, &[4.0, 8.0]).unwrap();
    assert_eq!(x, vec![2.0, 2.0]);
    let inv = crate::cholesky::form_linv(&c).unwrap();
    let product = crate::tk_matrix::mult_matrix(&inv, &l).unwrap();
    for (i, r) in product.iter().enumerate() {
        for (j, v) in r.iter().enumerate() {
            close(*v, f64::from(i == j));
        }
    }
}

#[test]
#[ignore = "requires local pinned runtime"]
fn canonical_constraint_callbacks_survive_context_snapshot() {
    use crate::{RuntimeContext, bats, general_fitting};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let runtime = root.join("validation/reference/build/source/T2runtime");
    for name in ["parameter-prior", "red-noise"] {
        let mut input = bats::load_pairs(
            &[bats::InputPair {
                par: root.join(format!("validation/general_fitting/inputs/{name}.par")),
                tim: root.join("validation/fitting/inputs/bary.tim"),
            }],
            &runtime,
        )
        .unwrap();
        bats::preprocess(&mut input, &[], &runtime).unwrap();
        let mut runtime_context = RuntimeContext::from_root(&runtime).unwrap();
        let result =
            general_fitting::fit(&mut input, &mut runtime_context, &Default::default()).unwrap();
        let expected = &result.cycles.last().unwrap().systems[0].constraints;
        let mut snapshot = input.context.clone();
        drop(input);
        for (i, row) in expected.iter().enumerate() {
            let actual = general_fitting::constraint_matrix_row(&mut snapshot, 0, i).unwrap();
            for (a, b) in actual.iter().zip(row) {
                assert_eq!(*a, b.as_float().to_f64());
            }
        }
        let info = &snapshot.pulsars[0].fitinfo;
        let unused = info.nParams.unwrap() as usize;
        assert_eq!(info.paramIndex[unused], Some(0));
        assert!(info.paramDerivs[unused].unwrap().is_none());
    }
}
