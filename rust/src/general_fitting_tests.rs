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
fn sinc_fit_callback_uses_the_requested_knot() {
    let mut context = PulsarContext::unwritten(1);
    context.initialise(1, 1, 0).unwrap();
    context.pulsars[0].nobs = Some(1);
    let (p, obs) = context.pulsar_observations_mut(0).unwrap();
    p.ifuncN = Some(3);
    p.param.slots[param_ifunc as usize].cells[0].val = ld(1.0);
    for (i, value) in [53996.0, 54000.0, 54004.0].iter().enumerate() {
        p.ifuncT[i] = Some(*value);
    }
    obs[0].bat = Some(ld(54004.0));
    assert_eq!(
        crate::fit_callbacks::derivative(&mut context, 0, 0.0, 0, param_ifunc, 2).unwrap(),
        1.0
    );
}
#[test]
fn wave_scale_one_updates_the_loaded_coefficients() {
    let mut context = PulsarContext::unwritten(1);
    context.initialise(1, 1, 0).unwrap();
    let p = &mut context.pulsars[0];
    p.waveScale = Some(1.0);
    p.nWhite = Some(1);
    p.wave_cos[0] = Some(2.0);
    crate::fit_callbacks::update(&mut context, 0, param_wave_om, 0, 0.5, 1.25).unwrap();
    let p = &context.pulsars[0];
    assert_eq!(p.wave_cos[0], Some(1.5));
    assert_eq!(p.wave_cos_err[0], Some(1.25));
    assert_eq!(p.wave_cos_dm[0], None);
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
fn legacy_global_constraints_follow_global_then_local_coordinates() {
    use crate::{legacy_fitting, pulsar::Constraint::*};
    fn basis(
        _: &mut PulsarContext,
        _: usize,
        x: f64,
        _: usize,
        row: &mut [f64],
    ) -> Result<(), crate::TimingError> {
        crate::tkfit::polynomial_basis(x, row);
        Ok(())
    }
    for mode in 0..3 {
        let mut context = PulsarContext::unwritten(2);
        context.initialise(2, 16, 0).unwrap();
        for p in 0..2 {
            context.pulsars[p].nobs = Some(6);
            let (q, obs) = context.pulsar_observations_mut(p).unwrap();
            q.robust = Some(0);
            q.nconstraints = Some(if mode == 2 { 0 } else { 1 });
            q.constraint_efactor = Some(1.0);
            q.param.slots[param_pepoch as usize].cells[0].val = ld(54000.0);
            let cell = &mut q.param.slots[param_ifunc as usize].cells[0];
            cell.param_set = 1;
            cell.fit_flag = if mode == 1 && p == 0 { 2 } else { 1 };
            q.ifuncN = Some(2);
            q.ifuncT[0] = Some(53999.0);
            q.ifuncT[1] = Some(54005.0);
            q.constraints[0] = Some(if p == 0 {
                constraint_ifunc_0
            } else {
                constraint_ifunc_1
            });
            for (i, o) in obs[..6].iter_mut().enumerate() {
                o.deleted = Some(0);
                o.sat = Some(ld(54000.0 + i as f64));
                o.toaErr = Some(1.0);
            }
        }
        let x = vec![(0..6).map(|i| i as f64 / 4.0).collect::<Vec<_>>(); 2];
        let y: Vec<_> = (0..2)
            .map(|p| (0..6).map(|i| (i + 1) as f64 + 0.25 * p as f64).collect())
            .collect();
        let fit = legacy_fitting::global_pulsar(
            &mut context,
            &x,
            &y,
            &vec![(0..6).collect(); 2],
            &[if mode == 1 { 1 } else { 3 }, 3],
            if mode == 1 { 2 } else { 0 },
            &vec![vec![vec![1.0; 6]]; 2],
            basis,
            1e-27,
            false,
            None,
        )
        .unwrap();
        if mode == 2 {
            assert!(fit.constraints.is_empty());
        } else {
            assert_eq!(fit.constraints[1], [0.0, 0.0, 0.0, 0.0, -0.5, 2.5]);
            assert_eq!(
                fit.constraints[0],
                if mode == 1 {
                    [0.5, 0.5, 0.0, 0.0, 0.0, 0.0]
                } else {
                    [0.0, 0.5, 0.5, 0.0, 0.0, 0.0]
                }
            );
        }
        assert_eq!(fit.design.matrix.len(), 12);
        assert_eq!(fit.solver.dof, 12 + fit.constraints.len() - 6);
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
    let result = crate::cholesky::get_cholesky_matrix(
        &mut context,
        0,
        "unused-one-point-model",
        &[0],
        &[0.0],
        &[2e-6],
        0,
        &mut inputs,
        None,
    )
    .unwrap();
    assert!(inputs.is_empty());
    assert_eq!(result.covariance, vec![vec![4e-12]]);
    assert_eq!(result.lower, vec![vec![2e-6]]);
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
