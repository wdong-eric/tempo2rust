//! constraints.C classical rows and t2fit.C parameter-prior construction.
use crate::{
    TimingError,
    clock_diagnostics::boundary,
    general_fitting::Column,
    observation::{read, read_string},
    pulsar::{Constraint, Constraint::*, SourcePulsar},
    pulsar_lifecycle::PulsarContext,
    tempo2::{Label, Label::*},
};
use std::f64::consts::PI;

/// computeConstraintWeights, baseline CONSTRAINT_WEIGHTS-disabled source profile.
pub fn compute_weights(p: &mut SourcePulsar) -> Result<(), TimingError> {
    for (n, weights) in [
        (p.ifuncN, &mut p.ifunc_weights[..]),
        (p.dmoffsDMnum, &mut p.dmoffsDM_weight[..]),
        (p.dmoffsCMnum, &mut p.dmoffsCM_weight[..]),
    ] {
        let n = *read(&n, "constraint knot count")?;
        if n < 0 || n as usize > weights.len() {
            return Err(boundary("constraint knot capacity"));
        }
        for v in &mut weights[..n as usize] {
            *v = Some(1.0 / n as f64);
        }
    }
    Ok(())
}

/// matrixDMConstraintWeights, including its additional DM factor and 1/e/e order.
/// This standalone routine is distinct from the baseline uniform weight policy.
pub fn matrix_dm_weights(context: &mut PulsarContext, index: usize) -> Result<(), TimingError> {
    use crate::tksvd::ld;
    let (p, obs) = context.pulsar_observations_mut(index)?;
    if p.param.slots[param_dmmodel as usize].cells[0].fit_flag != 1 {
        return Ok(());
    }
    let ndm = *read(&p.dmoffsDMnum, "dmoffsDMnum")? as usize;
    let ncm = *read(&p.dmoffsCMnum, "dmoffsCMnum")? as usize;
    let mut selected = Vec::new();
    for (i, o) in obs
        .iter()
        .take(*read(&p.nobs, "nobs")? as usize)
        .enumerate()
    {
        if crate::general_fitting::fit_observation_selected(p, o)? {
            let x = (read(&o.bbat, "bbat")?.clone()
                - &p.param.slots[param_pepoch as usize].cells[0].val)
                .to_f64();
            let sigma = *read(&o.toaErr, "toaErr")? * 1e-6;
            let freq = *read(&o.freqSSB, "freqSSB")? / 1e6;
            let dm = (ld(1.0) / (ld(2.41e-4) * ld(freq).square())).to_f64();
            selected.push((i, x, sigma, dm));
        }
    }
    let mut matrix = Vec::with_capacity(selected.len());
    for (i, x, sigma, dm) in selected {
        let mut row = Vec::with_capacity(ndm + ncm);
        for k in 0..ndm + ncm {
            let v = crate::fit_callbacks::derivative(context, index, x, i, param_dmmodel, k)?;
            row.push(if k < ndm { dm * v / sigma } else { v / sigma });
        }
        matrix.push(row);
    }
    let fit = crate::tkfit::least_squares_outputs(
        None,
        None,
        &matrix,
        &matrix,
        &[],
        &[],
        1e-20,
        false,
        crate::tkfit::FitOutputs {
            parameters: false,
            errors: true,
            covariance: false,
        },
        None,
    )
    .map_err(|e| boundary(&e))?;
    let p = &mut context.pulsars[index];
    let mut sums = [0.0; 2];
    for (k, error) in fit.errors.iter().enumerate() {
        let e = error.as_float().to_f64();
        let w = 1.0 / e / e;
        if k < ndm {
            p.dmoffsDM_weight[k] = Some(w);
            sums[0] += w;
        } else {
            p.dmoffsCM_weight[k - ndm] = Some(w);
            sums[1] += w;
        }
    }
    for (weights, n, sum) in [
        (&mut p.dmoffsDM_weight[..], ndm, sums[0]),
        (&mut p.dmoffsCM_weight[..], ncm, sums[1]),
    ] {
        for v in &mut weights[..n] {
            *v = Some(v.unwrap() / sum);
        }
    }
    Ok(())
}

/// autosetDMCM: source CM, DM, then CM grid generation/pruning order.
pub fn auto_set_dm_cm(
    context: &mut PulsarContext,
    p: usize,
    dm_step: f64,
    cm_step: f64,
    start: f64,
    end: f64,
    fixed_cm: bool,
) -> Result<(), TimingError> {
    if !(dm_step.is_finite()
        && dm_step > 0.0
        && cm_step.is_finite()
        && cm_step > 0.0
        && start.is_finite()
        && end.is_finite())
    {
        return Err(boundary("autosetDMCM finite range and positive steps"));
    }
    context.pulsars[p].dmoffsDMnum = Some(0);
    for (dm, step) in [(false, cm_step), (true, dm_step)] {
        let (psr, obs) = context.pulsar_observations_mut(p)?;
        let obs = &obs[..*read(&psr.nobs, "nobs")? as usize];
        let first = read(
            &obs.first()
                .ok_or_else(|| boundary("autosetDMCM empty observations"))?
                .sat,
            "sat",
        )?
        .to_f64()
            - step / 2.0;
        let last = read(&obs.last().unwrap().sat, "sat")?.to_f64() + step / 2.0;
        let (epochs, values, count) = if dm {
            (
                &mut psr.dmoffsDM_mjd[..],
                &mut psr.dmoffsDM[..],
                &mut psr.dmoffsDMnum,
            )
        } else {
            (
                &mut psr.dmoffsCM_mjd[..],
                &mut psr.dmoffsCM[..],
                &mut psr.dmoffsCMnum,
            )
        };
        let mut n = 0;
        let mut epoch = start;
        while epoch <= end {
            if epoch > first && epoch < last {
                if n == epochs.len() {
                    return Err(boundary("autosetDMCM MAX_IFUNC capacity"));
                }
                epochs[n] = Some(epoch);
                values[n] = Some(0.0);
                n += 1;
            }
            let next = epoch + step;
            if next == epoch {
                return Err(boundary("autosetDMCM step does not advance epoch"));
            }
            epoch = next;
        }
        *count = Some(n as i32);
        if dm || !fixed_cm {
            prune_grid(context, p, dm)?;
        }
    }
    if !fixed_cm {
        prune_grid(context, p, false)?;
    }
    Ok(())
}

/// Reused for the two CM passes and the DM pass in autosetDMCM.
fn prune_grid(context: &mut PulsarContext, p: usize, dm: bool) -> Result<(), TimingError> {
    loop {
        matrix_dm_weights(context, p)?;
        let psr = &mut context.pulsars[p];
        let (epochs, values, weights, count) = if dm {
            (
                &mut psr.dmoffsDM_mjd[..],
                &mut psr.dmoffsDM[..],
                &psr.dmoffsDM_weight[..],
                &mut psr.dmoffsDMnum,
            )
        } else {
            (
                &mut psr.dmoffsCM_mjd[..],
                &mut psr.dmoffsCM[..],
                &psr.dmoffsCM_weight[..],
                &mut psr.dmoffsCMnum,
            )
        };
        let n = *read(count, "DM/CM grid count")? as usize;
        if n == 0 {
            return Err(boundary("autosetDMCM source TKfindMin_d empty grid"));
        }
        let threshold = (if dm { 0.01 } else { 0.05 }) / n as f64;
        let mut min = *read(&weights[0], "grid weight")?;
        for w in &weights[..n] {
            let w = *read(w, "grid weight")?;
            if w < min {
                min = w;
            }
        }
        if min.partial_cmp(&threshold) != Some(std::cmp::Ordering::Less) {
            return Ok(());
        }
        let mut out = 0;
        let mut previous_minimum = false;
        for j in 0..n {
            epochs[out] = epochs[j];
            values[out] = Some(0.0);
            if *read(&weights[j], "grid weight")? > min {
                out += 1;
                previous_minimum = false;
            } else {
                if previous_minimum {
                    out += 1;
                }
                previous_minimum = true;
            }
        }
        *count = Some(out as i32);
    }
}
/// autoConstraints: source ordering and global-on-first-pulsar rules.
pub fn auto_constraints(p: &mut SourcePulsar, index: usize) -> Result<(), TimingError> {
    let fit = |l: Label, k: usize| p.param.slots[l as usize].cells[k].fit_flag;
    let dm = fit(param_dmmodel, 0) != 0 && *read(&p.dmoffsDMnum, "dmoffsDMnum")? > 1;
    let cm = fit(param_dmmodel, 0) != 0 && *read(&p.dmoffsCMnum, "dmoffsCMnum")? > 1;
    let bases = [
        (
            param_ifunc,
            [constraint_ifunc_0, constraint_ifunc_1, constraint_ifunc_2],
        ),
        (
            param_quad_ifunc_p,
            [
                constraint_quad_ifunc_p_0,
                constraint_quad_ifunc_p_1,
                constraint_quad_ifunc_p_2,
            ],
        ),
        (
            param_quad_ifunc_c,
            [
                constraint_quad_ifunc_c_0,
                constraint_quad_ifunc_c_1,
                constraint_quad_ifunc_c_2,
            ],
        ),
        (
            param_tel_dx,
            [
                constraint_tel_dx_0,
                constraint_tel_dx_1,
                constraint_tel_dx_2,
            ],
        ),
        (
            param_tel_dy,
            [
                constraint_tel_dy_0,
                constraint_tel_dy_1,
                constraint_tel_dy_2,
            ],
        ),
        (
            param_tel_dz,
            [
                constraint_tel_dz_0,
                constraint_tel_dz_1,
                constraint_tel_dz_2,
            ],
        ),
    ];
    let enabled = |l| fit(l, 0) == 1 || (index == 0 && fit(l, 0) == 2);
    let mut cs = vec![];
    if cm {
        cs.push(constraint_dmmodel_cw_0);
    }
    if dm {
        cs.push(constraint_dmmodel_mean);
    }
    for (l, c) in bases {
        if enabled(l) {
            cs.push(c[0]);
        }
    }
    for k in 0..2 {
        if fit(param_f, k) != 0 {
            if dm {
                cs.push(if k == 0 {
                    constraint_dmmodel_cw_1
                } else {
                    constraint_dmmodel_cw_2
                });
            }
            for (l, c) in bases {
                if enabled(l) {
                    cs.push(c[k + 1]);
                }
            }
        }
    }
    if fit(param_f, 2) != 0 && dm {
        cs.push(constraint_dmmodel_cw_3);
    }
    for (active, order) in [
        (fit(param_raj, 0) != 0 || fit(param_decj, 0) != 0, 0),
        (fit(param_pmra, 0) != 0 || fit(param_pmdec, 0) != 0, 1),
        (fit(param_px, 0) != 0, 2),
    ] {
        if !active {
            continue;
        }
        let families = [
            (
                dm,
                [
                    [
                        constraint_dmmodel_cw_year_sin,
                        constraint_dmmodel_cw_year_cos,
                    ],
                    [
                        constraint_dmmodel_cw_year_xsin,
                        constraint_dmmodel_cw_year_xcos,
                    ],
                    [
                        constraint_dmmodel_cw_year_sin2,
                        constraint_dmmodel_cw_year_cos2,
                    ],
                ],
            ),
            (
                enabled(param_ifunc),
                [
                    [constraint_ifunc_year_sin, constraint_ifunc_year_cos],
                    [constraint_ifunc_year_xsin, constraint_ifunc_year_xcos],
                    [constraint_ifunc_year_sin2, constraint_ifunc_year_cos2],
                ],
            ),
            (
                enabled(param_quad_ifunc_p) && order < 2,
                [
                    [constraint_qifunc_p_year_sin, constraint_qifunc_p_year_cos],
                    [constraint_qifunc_p_year_xsin, constraint_qifunc_p_year_xcos],
                    [constraint_qifunc_p_year_sin2, constraint_qifunc_p_year_cos2],
                ],
            ),
            (
                enabled(param_quad_ifunc_c) && order < 2,
                [
                    [constraint_qifunc_c_year_sin, constraint_qifunc_c_year_cos],
                    [constraint_qifunc_c_year_xsin, constraint_qifunc_c_year_xcos],
                    [constraint_qifunc_c_year_sin2, constraint_qifunc_c_year_cos2],
                ],
            ),
        ];
        for (on, c) in families {
            if on {
                cs.extend(c[order]);
            }
        }
    }
    p.nconstraints = Some(cs.len() as i32);
    for (i, c) in cs.into_iter().enumerate() {
        p.constraints[i] = Some(c);
    }
    Ok(())
}

pub fn standard_constraint(
    context: &mut PulsarContext,
    p: usize,
    c: Constraint,
    label: Label,
    k: usize,
) -> Result<f64, TimingError> {
    use crate::constraint_functions::*;
    type F = fn(
        &mut SourcePulsar,
        &mut [crate::observation::SourceObservation],
        i32,
        i32,
        i32,
        i32,
        &mut crate::clock_diagnostics::Diagnostics,
    ) -> Result<f64, TimingError>;
    let (f, order): (F, i32) = match c {
        constraint_dmmodel_mean => (consFunc_dmmodel_mean, 0),
        constraint_dmmodel_dm1 => (consFunc_dmmodel_dm1, 0),
        constraint_dmmodel_cw_0
        | constraint_dmmodel_cw_1
        | constraint_dmmodel_cw_2
        | constraint_dmmodel_cw_3 => (
            consFunc_dmmodel_cw,
            c as i32 - constraint_dmmodel_cw_0 as i32,
        ),
        constraint_ifunc_x0 => (consFunc_ifunc_X0, 0),
        constraint_ifunc_0 | constraint_ifunc_1 | constraint_ifunc_2 => {
            (consFunc_ifunc, c as i32 - constraint_ifunc_0 as i32)
        }
        constraint_tel_dx_0 | constraint_tel_dx_1 | constraint_tel_dx_2 => {
            (consFunc_tel_dx, c as i32 - constraint_tel_dx_0 as i32)
        }
        constraint_tel_dy_0 | constraint_tel_dy_1 | constraint_tel_dy_2 => {
            (consFunc_tel_dy, c as i32 - constraint_tel_dy_0 as i32)
        }
        constraint_tel_dz_0 | constraint_tel_dz_1 | constraint_tel_dz_2 => {
            (consFunc_tel_dz, c as i32 - constraint_tel_dz_0 as i32)
        }
        constraint_quad_ifunc_p_0 | constraint_quad_ifunc_p_1 | constraint_quad_ifunc_p_2 => (
            consFunc_quad_ifunc_p,
            c as i32 - constraint_quad_ifunc_p_0 as i32,
        ),
        constraint_quad_ifunc_c_0 | constraint_quad_ifunc_c_1 | constraint_quad_ifunc_c_2 => (
            consFunc_quad_ifunc_c,
            c as i32 - constraint_quad_ifunc_c_0 as i32,
        ),
        constraint_dmmodel_cw_year_sin
        | constraint_dmmodel_cw_year_cos
        | constraint_dmmodel_cw_year_xsin
        | constraint_dmmodel_cw_year_xcos
        | constraint_dmmodel_cw_year_sin2
        | constraint_dmmodel_cw_year_cos2
        | constraint_dmmodel_cw_px => (
            consFunc_dmmodel_cw_year,
            c as i32 - constraint_dmmodel_cw_year_sin as i32,
        ),
        constraint_ifunc_year_sin
        | constraint_ifunc_year_cos
        | constraint_ifunc_year_xsin
        | constraint_ifunc_year_xcos
        | constraint_ifunc_year_sin2
        | constraint_ifunc_year_cos2 => (
            consFunc_ifunc_year,
            c as i32 - constraint_ifunc_year_sin as i32,
        ),
        constraint_qifunc_p_year_sin
        | constraint_qifunc_p_year_cos
        | constraint_qifunc_p_year_xsin
        | constraint_qifunc_p_year_xcos
        | constraint_qifunc_p_year_sin2
        | constraint_qifunc_p_year_cos2 => (
            consFunc_qifunc_p_year,
            c as i32 - constraint_qifunc_p_year_sin as i32,
        ),
        constraint_qifunc_c_year_sin
        | constraint_qifunc_c_year_cos
        | constraint_qifunc_c_year_xsin
        | constraint_qifunc_c_year_xcos
        | constraint_qifunc_c_year_sin2
        | constraint_qifunc_c_year_cos2 => (
            consFunc_qifunc_c_year,
            c as i32 - constraint_qifunc_c_year_sin as i32,
        ),
        _ => return Ok(0.0),
    };
    crate::clkcorr::with_process(|s| {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        Ok(*read(&psr.constraint_efactor, "constraint_efactor")?
            * f(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                order,
                &mut s.diagnostics,
            )?)
    })
}

/// CONSTRAINTfuncs: additive legacy rows, ZERO at 0, then pulsar-zero globals
/// and locals. Jump columns are not part of this source enumerator.
pub fn legacy_constraint(
    context: &mut PulsarContext,
    index: usize,
    constraint: Constraint,
    out: &mut [f64],
) -> Result<(), TimingError> {
    let p = &context.pulsars[index];
    let mut coordinates = Vec::new();
    for flag in (1..=if index == 0 { 2 } else { 1 }).rev() {
        for (l, parameter) in p.param.slots.iter().enumerate() {
            if l == param_start as usize || l == param_finish as usize {
                continue;
            }
            for (k, cell) in parameter
                .cells
                .iter()
                .take(*read(&parameter.a_size, "parameter size")? as usize)
                .enumerate()
            {
                if cell.param_set == 1 && cell.fit_flag == flag {
                    let label = *crate::tempo2::PARAMETER_LABELS
                        .get(l)
                        .ok_or_else(|| boundary("legacy constraint unknown parameter"))?;
                    let n = match label {
                        param_dmmodel => {
                            *read(&p.dmoffsDMnum, "dmoffsDMnum")?
                                + *read(&p.dmoffsCMnum, "dmoffsCMnum")?
                        }
                        param_quad_ifunc_p => *read(&p.quad_ifuncN_p, "quad_ifuncN_p")?,
                        param_quad_ifunc_c => *read(&p.quad_ifuncN_c, "quad_ifuncN_c")?,
                        param_ifunc => *read(&p.ifuncN, "ifuncN")?,
                        _ => 1,
                    };
                    for j in 0..n {
                        coordinates.push((label, k + j as usize));
                    }
                }
            }
        }
    }
    if out.len() <= coordinates.len() {
        return Err(boundary("CONSTRAINTfuncs output capacity"));
    }
    for (i, (label, k)) in coordinates.into_iter().enumerate() {
        out[i + 1] += standard_constraint(context, index, constraint, label, k)?;
    }
    Ok(())
}
pub use crate::constraints_nestlike::qp_term_cutoff;

#[derive(Debug)]
pub struct ConstraintRows {
    pub labels: Vec<i32>,
    pub counters: Vec<i32>,
    pub matrix: Vec<Vec<f64>>,
    pub targets: Vec<f64>,
    pub inputs: Vec<crate::bats::ConsumedInput>,
    pub special: Vec<Option<ConstraintSpecial>>,
}
/// Typed equivalents of source void* constraint payloads; no native ABI layout.
#[derive(Clone, Debug)]
pub enum ConstraintSpecial {
    Parameter {
        label: Label,
        subindex: usize,
        error: f64,
    },
    Error(f64),
    CovarianceRow(Vec<f64>),
    DmCovarianceRow(Vec<f64>),
    CmCovarianceRow(Vec<f64>),
}
/// Installed constraint callback, using the same typed payload as row assembly.
#[allow(clippy::too_many_arguments)]
pub fn derivative(
    context: &mut PulsarContext,
    p: usize,
    constraint: Constraint,
    label: Label,
    constraint_k: usize,
    k: usize,
    special: Option<&ConstraintSpecial>,
) -> Result<f64, TimingError> {
    match special {
        Some(ConstraintSpecial::Parameter {
            label: selected,
            subindex,
            error,
        }) => {
            return Ok(if label == *selected && k == *subindex {
                1.0 / error
            } else {
                0.0
            });
        }
        Some(ConstraintSpecial::Error(error)) => {
            return Ok(if label == param_ne_sw_ifunc && k == constraint_k {
                1.0 / error
            } else {
                0.0
            });
        }
        Some(ConstraintSpecial::CovarianceRow(row)) => {
            return if label == param_ifunc {
                row.get(k)
                    .copied()
                    .ok_or_else(|| boundary("IFUNC constraint column"))
            } else {
                Ok(0.0)
            };
        }
        Some(ConstraintSpecial::DmCovarianceRow(row))
        | Some(ConstraintSpecial::CmCovarianceRow(row)) => {
            if label != param_dmmodel {
                return Ok(0.0);
            }
            let ndm = *read(&context.pulsars()[p].dmoffsDMnum, "dmoffsDMnum")? as usize;
            let column = match special {
                Some(ConstraintSpecial::DmCovarianceRow(_)) => (k < ndm).then_some(k),
                _ => k.checked_sub(ndm),
            };
            return match column {
                Some(k) => row
                    .get(k)
                    .copied()
                    .ok_or_else(|| boundary("DM/CM covariance column")),
                None => Ok(0.0),
            };
        }
        None => {}
    }
    use crate::constraints_nestlike::*;
    let f = match constraint {
        constraint_red_sin | constraint_red_cos => constraints_nestlike_red,
        constraint_red_dm_sin | constraint_red_dm_cos => constraints_nestlike_red_dm,
        constraint_red_chrom_sin | constraint_red_chrom_cos => constraints_nestlike_red_chrom,
        constraint_swgp_sin | constraint_swgp_cos => constraints_nestlike_swgp,
        constraint_band_red_sin | constraint_band_red_cos => constraints_nestlike_band,
        constraint_group_red_sin | constraint_group_red_cos => constraints_nestlike_group,
        constraint_jitter => constraints_nestlike_jitter,
        constraint_param
        | constraint_ne_sw_ifunc_sin
        | constraint_ne_sw_ifunc_sigma
        | constraint_ifunc_cov => return Err(boundary("missing constraint special payload")),
        _ => return standard_constraint(context, p, constraint, label, k),
    };
    crate::clkcorr::with_process(|s| {
        let (psr, obs) = context.pulsar_observations_mut(p)?;
        f(
            psr,
            obs,
            p as i32,
            constraint as i32,
            label as i32,
            constraint_k as i32,
            k as i32,
            &mut s.diagnostics,
        )
    })
}
/// t2Fit_fillFitInfo classical and parameter constraints; typed owned RHS rows.
pub fn build(
    context: &mut PulsarContext,
    index: usize,
    columns: &[Column],
) -> Result<ConstraintRows, TimingError> {
    let n = *read(&context.pulsars[index].nconstraints, "nconstraints")?;
    // fda6ad4 t2fit.C:235-237 computes weights only for a nonempty constraint
    // matrix. Unconditional preparation changed unconstrained DM/CM file weights.
    if n > 0 {
        compute_weights(&mut context.pulsars[index])?;
    }
    let mut out = ConstraintRows {
        labels: vec![],
        counters: vec![],
        matrix: vec![],
        targets: vec![],
        inputs: vec![],
        special: vec![],
    };
    for j in 0..n as usize {
        let p = &context.pulsars[index];
        let c = *read(&p.constraints[j], "constraints")?;
        let mut row = vec![0.0; columns.len()];
        let mut target = 0.0;
        let mut special = None;
        if matches!(
            c,
            constraint_param
                | constraint_ne_sw_ifunc_sin
                | constraint_ne_sw_ifunc_sigma
                | constraint_ifunc_cov
        ) {
            let view = p.constraint_special[j]
                .flatten()
                .ok_or_else(|| boundary("constraint payload missing"))?;
            let bytes = read_string(
                context
                    .payload(view)
                    .map_err(|e| boundary(&e.to_string()))?,
                "constraint payload",
            )?;
            let text =
                std::str::from_utf8(&bytes).map_err(|_| boundary("constraint text encoding"))?;
            let mut scanner = crate::source_text::SourceText::new(
                std::path::Path::new("<constraint payload>"),
                text.as_bytes().to_vec(),
            );
            if c == constraint_param {
                let name = scanner.required_token("constraint parameter")?;
                let prior = scanner.required_token("constraint value")?;
                let mut error = scanner.double("constraint error")?;
                let mut found = None;
                for (l, slot) in p.param.slots.iter().enumerate().take(param_LAST as usize) {
                    for (k, cell) in slot.cells.iter().enumerate() {
                        if crate::clock_text::c_bytes(&cell.shortlabel).eq_ignore_ascii_case(&name)
                        {
                            found = Some((l, k));
                        }
                    }
                }
                let (l, k) = found.ok_or_else(|| boundary("unknown constrained parameter"))?;
                let val = if l == param_raj as usize || l == param_decj as usize {
                    let v = if l == param_raj as usize {
                        crate::source_par::hms_turn(&mut prior.clone())
                    } else {
                        crate::source_par::dms_turn(&prior)?
                    };
                    error *= if l == param_raj as usize {
                        PI / 12.0 / 3600.0
                    } else {
                        PI / 180.0 / 3600.0
                    };
                    crate::tksvd::ld(crate::tempo2_util::turn_deg(v) * PI / 180.0)
                } else {
                    crate::source_text::extended(&prior)?
                };
                let mut value = (val - &p.param.slots[l].cells[k].val).to_f64();
                let (mut factor, mut sign) = (1.0, 1.0);
                let label = crate::tempo2::PARAMETER_LABELS[l];
                match label {
                    param_pmra | param_pmdec => {
                        factor =
                            180.0 / PI * 60.0 * 60.0 * 1000.0 * 86400.0 * 365.25 / 24.0 / 3600.0;
                        if label == param_pmra {
                            factor *= p.param.slots[param_decj as usize].cells[0]
                                .val
                                .clone()
                                .cos()
                                .to_f64();
                        }
                    }
                    param_pmra2 | param_pmdec2 => {
                        factor = 180.0 / PI
                            * 60.0
                            * 60.0
                            * 1000.0
                            * (86400.0f64 * 365.25 / 24.0 / 3600.0).powf(2.0);
                        if label == param_pmra2 {
                            factor *= p.param.slots[param_decj as usize].cells[0]
                                .val
                                .clone()
                                .cos()
                                .to_f64();
                        }
                    }
                    param_om | param_kom | param_kin => factor = 180.0 / PI,
                    param_f => {
                        sign = -1.0;
                        if k > 0 {
                            let scale: f64 = if k == 2 {
                                1e9
                            } else if k > 2 && k < 10 {
                                1e18
                            } else {
                                1.0
                            };
                            factor = (p.param.slots[param_f as usize].cells[0].val.clone()
                                / 86400.0f64.powf((k + 1) as f64)
                                / scale)
                                .to_f64();
                        }
                    }
                    _ => {}
                }
                value /= factor;
                error /= factor;
                value /= error;
                target = sign * value;
                special = Some(ConstraintSpecial::Parameter {
                    label,
                    subindex: k,
                    error,
                });
                for (i, col) in columns.iter().enumerate() {
                    if col.parameter_index == l && col.subindex == k {
                        row[i] = 1.0 / error;
                    }
                }
            } else if matches!(c, constraint_ne_sw_ifunc_sigma | constraint_ne_sw_ifunc_sin) {
                let count = if c == constraint_ne_sw_ifunc_sigma {
                    1
                } else {
                    5
                };
                let v = (0..count)
                    .map(|_| scanner.double("SW constraint number"))
                    .collect::<Result<Vec<_>, _>>()?;
                for k in 0..*read(&p.ne_sw_ifuncN, "ne_sw_ifuncN")? as usize {
                    let error = *v.last().unwrap();
                    let mut r = vec![0.0; columns.len()];
                    for (i, col) in columns.iter().enumerate() {
                        if col.parameter_index == param_ne_sw_ifunc as usize && col.subindex == k {
                            r[i] = 1.0 / error;
                        }
                    }
                    let target = if c == constraint_ne_sw_ifunc_sigma {
                        0.0
                    } else {
                        (v[0]
                            - v[1]
                                * (2.0 * PI * (*read(&p.ne_sw_ifuncT[k], "ne_sw_ifuncT")? - v[2])
                                    / v[3])
                                    .cos()
                            - *read(&p.ne_sw_ifuncV[k], "ne_sw_ifuncV")?)
                            / error
                    };
                    // t2fit.C omitted labels after the first solar IFUNC row.
                    // Every expanded row belongs to this configured constraint.
                    out.labels.push(c as i32);
                    out.counters.push(k as i32);
                    out.matrix.push(r);
                    out.targets.push(target);
                    out.special.push(Some(ConstraintSpecial::Error(error)));
                }
                continue;
            } else {
                // constraints_covar.C indexed every column at ifuncN (out of bounds).
                // check_callbacks.py uses a safely padded row to demonstrate it.
                let l = crate::cholesky::ifunc_cholesky(context, index, text, &mut out.inputs)?;
                for (k, weights) in l.iter().enumerate() {
                    let row = columns
                        .iter()
                        .map(|col| {
                            if col.parameter_index == param_ifunc as usize {
                                weights[col.subindex]
                            } else {
                                0.0
                            }
                        })
                        .collect();
                    // t2fit.C also used constraints[ii] instead of constraints[i].
                    out.labels.push(c as i32);
                    out.counters.push(k as i32);
                    out.matrix.push(row);
                    out.targets.push(0.0);
                    out.special
                        .push(Some(ConstraintSpecial::CovarianceRow(weights.clone())));
                }
                continue;
            }
        } else {
            for (i, col) in columns.iter().enumerate() {
                row[i] = standard_constraint(
                    context,
                    index,
                    c,
                    crate::tempo2::PARAMETER_LABELS[col.parameter_index],
                    col.subindex,
                )?;
            }
        }
        out.labels.push(c as i32);
        out.counters
            .push(if c == constraint_param { j as i32 } else { 0 });
        out.matrix.push(row);
        out.targets.push(target);
        out.special.push(special);
    }
    use crate::constraints_nestlike::*;
    type F = fn(
        &mut SourcePulsar,
        &mut [crate::observation::SourceObservation],
        i32,
        i32,
        i32,
        i32,
        i32,
        &mut crate::clock_diagnostics::Diagnostics,
    ) -> Result<f64, TimingError>;
    let mut priors: Vec<(Constraint, usize, F)> = vec![];
    for (sin, _, nh, nlog) in crate::t2fit_nestlike::families(&context.pulsars[index])? {
        let (a, b, f): (Constraint, Constraint, F) = match sin {
            param_red_sin => (
                constraint_red_sin,
                constraint_red_cos,
                constraints_nestlike_red,
            ),
            param_red_dm_sin => (
                constraint_red_dm_sin,
                constraint_red_dm_cos,
                constraints_nestlike_red_dm,
            ),
            param_red_chrom_sin => (
                constraint_red_chrom_sin,
                constraint_red_chrom_cos,
                constraints_nestlike_red_chrom,
            ),
            param_swgp_sin => (
                constraint_swgp_sin,
                constraint_swgp_cos,
                constraints_nestlike_swgp,
            ),
            param_band_red_sin => (
                constraint_band_red_sin,
                constraint_band_red_cos,
                constraints_nestlike_band,
            ),
            param_group_red_sin => (
                constraint_group_red_sin,
                constraint_group_red_cos,
                constraints_nestlike_group,
            ),
            _ => unreachable!(),
        };
        for k in 0..nh + nlog {
            priors.push((a, k, f));
            // t2fit.C repeats sine for DM/chromatic logarithmic pairs, omitting
            // the cosine prior. check_fitinfo.py witnesses both and the neighbor.
            priors.push((b, k, f));
        }
    }
    for c in columns {
        if c.parameter_index == param_jitter as usize {
            priors.push((constraint_jitter, c.subindex, constraints_nestlike_jitter));
        }
    }
    for (c, k, f) in priors {
        let mut row = vec![];
        for col in columns {
            row.push(crate::clkcorr::with_process(|s| {
                let (p, obs) = context.pulsar_observations_mut(index)?;
                f(
                    p,
                    obs,
                    index as i32,
                    c as i32,
                    col.parameter_index as i32,
                    k as i32,
                    col.subindex as i32,
                    &mut s.diagnostics,
                )
            })?);
        }
        out.labels.push(c as i32);
        out.counters.push(k as i32);
        out.matrix.push(row);
        out.targets.push(0.0);
        out.special.push(None);
    }
    Ok(out)
}

impl Constraint {
    pub fn from_index(index: i32) -> Option<Self> {
        const LABELS: [Constraint; 68] = [
            constraint_dmmodel_mean,
            constraint_dmmodel_dm1,
            constraint_dmmodel_cw_0,
            constraint_dmmodel_cw_1,
            constraint_dmmodel_cw_2,
            constraint_dmmodel_cw_3,
            constraint_ifunc_cov,
            constraint_ifunc_x0,
            constraint_ifunc_0,
            constraint_ifunc_1,
            constraint_ifunc_2,
            constraint_tel_dx_0,
            constraint_tel_dx_1,
            constraint_tel_dx_2,
            constraint_tel_dy_0,
            constraint_tel_dy_1,
            constraint_tel_dy_2,
            constraint_tel_dz_0,
            constraint_tel_dz_1,
            constraint_tel_dz_2,
            constraint_quad_ifunc_p_0,
            constraint_quad_ifunc_p_1,
            constraint_quad_ifunc_p_2,
            constraint_quad_ifunc_c_0,
            constraint_quad_ifunc_c_1,
            constraint_quad_ifunc_c_2,
            constraint_dmmodel_cw_year_sin,
            constraint_dmmodel_cw_year_cos,
            constraint_dmmodel_cw_year_xsin,
            constraint_dmmodel_cw_year_xcos,
            constraint_dmmodel_cw_year_sin2,
            constraint_dmmodel_cw_year_cos2,
            constraint_dmmodel_cw_px,
            constraint_ifunc_year_sin,
            constraint_ifunc_year_cos,
            constraint_ifunc_year_xsin,
            constraint_ifunc_year_xcos,
            constraint_ifunc_year_sin2,
            constraint_ifunc_year_cos2,
            constraint_qifunc_p_year_sin,
            constraint_qifunc_p_year_cos,
            constraint_qifunc_p_year_xsin,
            constraint_qifunc_p_year_xcos,
            constraint_qifunc_p_year_sin2,
            constraint_qifunc_p_year_cos2,
            constraint_qifunc_c_year_sin,
            constraint_qifunc_c_year_cos,
            constraint_qifunc_c_year_xsin,
            constraint_qifunc_c_year_xcos,
            constraint_qifunc_c_year_sin2,
            constraint_qifunc_c_year_cos2,
            constraint_red_sin,
            constraint_red_cos,
            constraint_band_red_sin,
            constraint_band_red_cos,
            constraint_red_dm_sin,
            constraint_red_dm_cos,
            constraint_red_chrom_sin,
            constraint_red_chrom_cos,
            constraint_swgp_sin,
            constraint_swgp_cos,
            constraint_group_red_sin,
            constraint_group_red_cos,
            constraint_jitter,
            constraint_param,
            constraint_ne_sw_ifunc_sin,
            constraint_ne_sw_ifunc_sigma,
            constraint_LAST,
        ];
        usize::try_from(index)
            .ok()
            .and_then(|i| LABELS.get(i))
            .copied()
    }
}

/// get_constraint_name, preserving source strings and its UNKNOWN fallback.
pub fn name(c: Constraint) -> &'static str {
    match c {
        constraint_dmmodel_mean => "DMMODEL mean(DM) = 0",
        constraint_dmmodel_dm1 => "DMMODEL mean(DM1) = 0",
        constraint_dmmodel_cw_0 => "DMMODEL mean(C) = 0",
        constraint_dmmodel_cw_1 => "DMMODEL linear(C) = 0",
        constraint_dmmodel_cw_2 => "DMMODEL quadratic(C) = 0",
        constraint_dmmodel_cw_3 => "DMMODEL cubic(C) = 0",
        constraint_ifunc_0 => "IFUNC mean(C) = 0",
        constraint_ifunc_1 => "IFUNC linear(C) = 0",
        constraint_ifunc_2 => "IFUNC quadratic(C) = 0",
        constraint_quad_ifunc_p_0 => "QIFUNC_p mean(C) = 0",
        constraint_quad_ifunc_p_1 => "QIFUNC_p linear(C) = 0",
        constraint_quad_ifunc_p_2 => "QIFUNC_p quadratic(C) = 0",
        constraint_quad_ifunc_c_0 => "QIFUNC_c mean(C) = 0",
        constraint_quad_ifunc_c_1 => "QIFUNC_c linear(C) = 0",
        constraint_quad_ifunc_c_2 => "QIFUNC_c quadratic(C) = 0",
        constraint_tel_dx_0 => "TEL_DX mean(C) = 0",
        constraint_tel_dx_1 => "TEL_DX linear(C) = 0",
        constraint_tel_dx_2 => "TEL_DX quadratic(C) = 0",
        constraint_tel_dy_0 => "TEL_DY mean(C) = 0",
        constraint_tel_dy_1 => "TEL_DY linear(C) = 0",
        constraint_tel_dy_2 => "TEL_DY quadratic(C) = 0",
        constraint_tel_dz_0 => "TEL_DZ mean(C) = 0",
        constraint_tel_dz_1 => "TEL_DZ linear(C) = 0",
        constraint_tel_dz_2 => "TEL_DZ quadratic(C) = 0",
        constraint_dmmodel_cw_year_sin => "DMMODEL_YEAR C.sin(t) = 0",
        constraint_dmmodel_cw_year_cos => "DMMODEL_YEAR C.cos(t) = 0",
        constraint_dmmodel_cw_year_xsin => "DMMODEL_YEAR C.t.sin(t) = 0",
        constraint_dmmodel_cw_year_xcos => "DMMODEL_YEAR C.t.cos(t) = 0",
        constraint_dmmodel_cw_px => "DMMODEL_YEAR C:PX = 0",
        constraint_dmmodel_cw_year_sin2 => "DMMODEL_YEAR C.sin(2t) = 0",
        constraint_dmmodel_cw_year_cos2 => "DMMODEL_YEAR C.cos(2t) = 0",
        constraint_ifunc_year_sin => "IFUNC_YEAR C.sin(t) = 0",
        constraint_ifunc_year_cos => "IFUNC_YEAR C.cos(t) = 0",
        constraint_ifunc_year_xsin => "IFUNC_YEAR C.t.sin(t) = 0",
        constraint_ifunc_year_xcos => "IFUNC_YEAR C.t.cos(t) = 0",
        constraint_ifunc_year_sin2 => "IFUNC_YEAR C.sin(2t) = 0",
        constraint_ifunc_year_cos2 => "IFUNC_YEAR C.cos(2t) = 0",
        constraint_qifunc_p_year_sin => "QIFUNC_p_YEAR C.sin(t) = 0",
        constraint_qifunc_p_year_cos => "QIFUNC_p_YEAR C.cos(t) = 0",
        constraint_qifunc_p_year_xsin => "QIFUNC_p_YEAR C.t.sin(t) = 0",
        constraint_qifunc_p_year_xcos => "QIFUNC_p_YEAR C.t.cos(t) = 0",
        constraint_qifunc_p_year_sin2 => "QIFUNC_p_YEAR C.sin(2t) = 0",
        constraint_qifunc_p_year_cos2 => "QIFUNC_p_YEAR C.cos(2t) = 0",
        constraint_qifunc_c_year_sin => "QIFUNC_c_YEAR C.sin(t) = 0",
        constraint_qifunc_c_year_cos => "QIFUNC_c_YEAR C.cos(t) = 0",
        constraint_qifunc_c_year_xsin => "QIFUNC_c_YEAR C.t.sin(t) = 0",
        constraint_qifunc_c_year_xcos => "QIFUNC_c_YEAR C.t.cos(t) = 0",
        constraint_qifunc_c_year_sin2 => "QIFUNC_c_YEAR C.sin(2t) = 0",
        constraint_qifunc_c_year_cos2 => "QIFUNC_c_YEAR C.cos(2t) = 0",
        constraint_param => "PARAMETER CONSTRAINT",
        constraint_ne_sw_ifunc_sin => "NE_SW IFUNC SIN CONSTRAINT",
        constraint_ne_sw_ifunc_sigma => "NE_SW IFUNC SIGMA CONSTRAINT",
        _ => "UNKNOWN!",
    }
}
