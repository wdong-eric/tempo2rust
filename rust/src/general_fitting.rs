//! t2fit.C modern local/global driver; tempo2.C NITS lifecycle; doFit.C adapters.
use crate::{
    RuntimeContext, TimingError,
    bats::{BatInput, Number},
    clock_diagnostics::boundary,
    observation::{read, read_string},
    pulsar::{Callback, FitInfo, MAX_FIT, SourcePulsar},
    pulsar_lifecycle::PulsarContext,
    tempo2::{Label, Label::*, PARAMETER_LABELS, PARAMETER_NAMES},
    tkfit::{FitNumber, LeastSquares},
    tksvd::ld,
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FitOptions {
    pub covariance: Option<String>,
    pub rescale_err_chisq: bool,
    pub force_always_fit_for_glitches: bool,
    pub acceleration: i32,
    pub diagnostics: Option<crate::tkfit::FitDiagnostics>,
}
impl Default for FitOptions {
    fn default() -> Self {
        Self {
            covariance: None,
            rescale_err_chisq: true,
            force_always_fit_for_glitches: false,
            acceleration: 0,
            diagnostics: None,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Column {
    /// -1 denotes a global coordinate.
    pub pulsar: i32,
    pub parameter: &'static str,
    pub parameter_index: usize,
    pub subindex: usize,
}
impl Column {
    pub(crate) fn new(p: i32, l: Label, k: usize) -> Self {
        Self {
            pulsar: p,
            parameter: PARAMETER_NAMES[l as usize],
            parameter_index: l as usize,
            subindex: k,
        }
    }
    fn label(&self) -> Label {
        PARAMETER_LABELS[self.parameter_index]
    }
}
#[derive(Debug, Serialize)]
pub struct FitRow {
    pub pulsar: usize,
    pub observation: usize,
    pub input_index: Option<usize>,
    pub x_days: FitNumber,
    pub residual_s: FitNumber,
    pub sigma_s: FitNumber,
}
#[derive(Debug, Serialize)]
pub struct PhysicalUpdate {
    pub pulsar: usize,
    pub column: Column,
    pub before_value: Option<Number>,
    pub before_error: Option<Number>,
    pub after_value: Option<Number>,
    pub after_error: Option<Number>,
    /// Canonical state after TN polynomial/SWGP postfit transfers.
    pub postfit_value: Option<Number>,
    pub postfit_error: Option<Number>,
    pub increment: FitNumber,
    pub fit_error: FitNumber,
}
#[derive(Debug, Serialize)]
pub struct FitSystem {
    pub rows: Vec<FitRow>,
    pub columns: Vec<Column>,
    pub data: Vec<FitNumber>,
    pub white_data: Vec<FitNumber>,
    pub matrix: Vec<Vec<FitNumber>>,
    pub white_matrix: Vec<Vec<FitNumber>>,
    pub constraints: Vec<Vec<FitNumber>>,
    pub constraint_targets: Vec<FitNumber>,
    pub solver: LeastSquares,
    pub updates: Vec<PhysicalUpdate>,
    pub whitening: Vec<Whitening>,
    pub covariance_inputs: Vec<crate::bats::ConsumedInput>,
    pub constraint_rows: Vec<ConstraintRow>,
    pub noise: Vec<NoiseState>,
    pub working_directory: std::path::PathBuf,
    pub solver_engine: &'static str,
    pub diagnostic_files: Vec<std::path::PathBuf>,
}
#[derive(Debug, Serialize)]
pub struct ConstraintRow {
    pub pulsar: usize,
    pub label: i32,
    pub subindex: i32,
}
#[derive(Debug, Serialize)]
pub struct NoiseState {
    pub pulsar: usize,
    pub observation: usize,
    pub red: FitNumber,
    pub red_error: FitNumber,
    pub dm: FitNumber,
    pub dm_error: FitNumber,
    pub chrom: FitNumber,
    pub chrom_error: FitNumber,
    pub solar_wind: FitNumber,
    pub solar_wind_error: FitNumber,
}
#[derive(Debug, Serialize)]
pub struct Whitening {
    pub pulsar: usize,
    pub fit_mode: i32,
    pub robust: u8,
    pub nits: i32,
    pub rescale_err_chisq: i32,
    pub sigma: Vec<FitNumber>,
    pub covariance: Option<Vec<Vec<FitNumber>>>,
    pub lower: Option<Vec<Vec<FitNumber>>>,
    pub log_determinant: FitNumber,
}
#[derive(Debug, Serialize)]
pub struct FitCycle {
    pub iteration: usize,
    pub prefit: crate::timing::TimingResult,
    pub systems: Vec<FitSystem>,
    pub postfit: crate::timing::TimingResult,
}
#[derive(Debug, Serialize)]
pub struct FitResult {
    pub schema_version: &'static str,
    pub options: FitOptions,
    pub remove_mean: bool,
    pub fit_performed: bool,
    pub covariance_coordinates: &'static str,
    pub cycles: Vec<FitCycle>,
}

/// Full source lifecycle on input already loaded/preprocessed by bats.
pub fn fit(
    input: &mut BatInput,
    runtime: &mut RuntimeContext,
    options: &FitOptions,
) -> Result<FitResult, TimingError> {
    let _workflow = crate::execution::enter();
    let nits = count(
        *read(
            &input
                .context
                .pulsars
                .first()
                .ok_or_else(|| boundary("fit_empty_context"))?
                .nits,
            "nits",
        )?,
        "nits",
    )?;
    let mut cycles = vec![];
    for iteration in 0..nits {
        if iteration > 0 {
            for p in &mut input.context.pulsars {
                for slot in &mut p.param.slots {
                    for c in &mut slot.cells {
                        c.prefit = Some(c.val.clone());
                        c.prefit_err = Some(c.err.clone());
                    }
                }
            }
        }
        let prefit = crate::timing::evaluate(input, runtime, true)?;
        let systems = t2_fit(&mut input.context, options)?;
        let postfit = crate::timing::evaluate(input, runtime, true)?;
        cycles.push(FitCycle {
            iteration: iteration + 1,
            prefit,
            systems,
            postfit,
        });
    }
    Ok(FitResult {
        schema_version: "tempo2-general-fit-v1",
        options: options.clone(),
        remove_mean: true,
        fit_performed: true,
        covariance_coordinates: "raw source fit coordinates; global columns then pulsar-local blocks",
        cycles,
    })
}
fn count(n: i32, field: &str) -> Result<usize, TimingError> {
    usize::try_from(n).map_err(|_| boundary(&format!("negative {field}")))
}

/// t2Fit_getFitData: snapshot all residuals before applying deletion and SAT/BAT cuts.
pub fn get_fit_data(context: &mut PulsarContext, index: usize) -> Result<Vec<FitRow>, TimingError> {
    let (p, obs) = context.pulsar_observations_mut(index)?;
    let a = &p.param.slots[param_start as usize].cells[0];
    let b = &p.param.slots[param_finish as usize].cells[0];
    let start_flag = if a.param_set == 1 { a.fit_flag } else { 0 };
    let finish_flag = if b.param_set == 1 { b.fit_flag } else { 0 };
    let cut_a = matches!(start_flag, 1 | 2);
    let cut_b = matches!(finish_flag, 1 | 2);
    crate::clkcorr::with_diagnostics(|d| {
        d.log(
            b'm',
            "t2fit.C",
            850,
            format!(
                "ss/fs {} {}",
                i32::from(start_flag == 1),
                i32::from(finish_flag == 1)
            )
            .as_bytes(),
        );
        Ok(())
    })?;
    let mut start = if cut_a { a.val.clone() } else { ld(1e10) };
    let mut finish = if cut_b { b.val.clone() } else { ld(0.0) };
    let n = count(*read(&p.nobs, "nobs")?, "nobs")?;
    let mut rows = vec![];
    for (i, o) in obs.iter_mut().take(n).enumerate() {
        o.prefitResidual = Some(read(&o.residual, "residual")?.clone());
        if !fit_observation_selected(p, o)? {
            continue;
        }
        let sat = read(&o.sat, "sat")?;
        if !cut_a && sat < &start {
            start = sat.clone();
        }
        if !cut_b && sat > &finish {
            finish = sat.clone();
        }
        rows.push(FitRow {
            pulsar: index,
            observation: i,
            input_index: o.input_index,
            x_days: (read(&o.bbat, "bbat")?.clone()
                - &p.param.slots[param_pepoch as usize].cells[0].val)
                .to_f64()
                .into(),
            residual_s: read(&o.residual, "residual")?.to_f64().into(),
            sigma_s: (*read(&o.toaErr, "toaErr")? * 1e-6).into(),
        });
    }
    crate::clkcorr::with_diagnostics(|d| {
        d.log(
            b'm',
            "t2fit.C",
            879,
            &crate::source_format::format(
                "START=%.18lg FINISH = %.18lg",
                &[start.to_f64().into(), finish.to_f64().into()],
            )?,
        );
        Ok(())
    })?;
    for (l, v) in [(param_start, start), (param_finish, finish)] {
        let c = &mut p.param.slots[l as usize].cells[0];
        // fda6ad4 t2fit.C::t2Fit_getFitData:880-883 activates these cells but
        // leaves prefit unwritten; textOutput:217 then reads it. Snapshot the
        // defined prior value on first activation, preserving supplied PAR state.
        if c.param_set == 0 && c.prefit.is_none() {
            c.prefit = Some(c.val.clone());
            c.prefit_err = Some(c.err.clone());
        }
        c.val = v;
        c.param_set = 1;
    }
    p.nFit = Some(rows.len() as i32);
    if rows.is_empty() {
        return Err(boundary("t2Fit_getFitData: source ndata>0 assertion"));
    }
    Ok(rows)
}

/// Shared source START/FINISH membership, without mutating epochs or residuals.
pub fn fit_observation_selected(
    p: &SourcePulsar,
    o: &crate::observation::SourceObservation,
) -> Result<bool, TimingError> {
    if *read(&o.deleted, "deleted")? != 0 {
        return Ok(false);
    }
    // tempo2.h casts the binary64 literal: longdouble(1e-11).
    let delta = ld(1e-11);
    for label in [param_start, param_finish] {
        let cell = &p.param.slots[label as usize].cells[0];
        if cell.param_set == 1 && matches!(cell.fit_flag, 1 | 2) {
            let epoch = read(
                if cell.fit_flag == 1 { &o.sat } else { &o.bat },
                "fit cut epoch",
            )?;
            if (label == param_start && epoch < &(cell.val.clone() - &delta))
                || (label == param_finish && epoch > &(cell.val.clone() + &delta))
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn parameter_columns(
    p: &SourcePulsar,
    index: i32,
    flag: i32,
    options: &FitOptions,
) -> Result<Vec<Column>, TimingError> {
    let mut columns = vec![];
    for (l, slot) in p.param.slots.iter().enumerate().take(param_LAST as usize) {
        let label = PARAMETER_LABELS[l];
        for (k, c) in slot.cells.iter().enumerate() {
            if c.param_set <= 0 || c.fit_flag != flag || matches!(label, param_start | param_finish)
            {
                continue;
            }
            if !crate::fit_callbacks::supported(label) {
                crate::clkcorr::with_process(|s| {
                    let _: () = s.diagnostics.log(
                        b'e',
                        "t2fit.C",
                        2041,
                        format!(
                            "ERROR: No methods for fitting parameter {} ({l})",
                            PARAMETER_NAMES[l]
                        )
                        .as_bytes(),
                    );
                    Ok(())
                })?;
                continue;
            }
            if matches!(
                label,
                param_glep
                    | param_glph
                    | param_glf0
                    | param_glf1
                    | param_glf2
                    | param_glf0d
                    | param_glf0d2
                    | param_glf0d3
                    | param_gltd
                    | param_gltd2
                    | param_gltd3
            ) && !options.force_always_fit_for_glitches
            {
                let e = &p.param.slots[param_glep as usize].cells[k].val;
                if !(e > &p.param.slots[param_start as usize].cells[0].val
                    && e < &p.param.slots[param_finish as usize].cells[0].val)
                {
                    continue;
                }
            }
            if matches!(
                label,
                param_expep | param_expph | param_exptau | param_expindex
            ) {
                let e = &p.param.slots[param_expep as usize].cells[k].val;
                if !(e > &p.param.slots[param_start as usize].cells[0].val
                    && e < &p.param.slots[param_finish as usize].cells[0].val)
                {
                    continue;
                }
            }
            if label == param_shapevent {
                for event in 0..count(
                    *read(&p.nTNShapeletEvents, "nTNShapeletEvents")?,
                    "shape events",
                )? {
                    if p.TNShapeletEvFitFlag[event] == Some(1) {
                        for coef in 0..count(
                            *read(&p.TNShapeletEvN[event], "shape order")?,
                            "shape order",
                        )? {
                            columns.push(Column::new(
                                index,
                                label,
                                event * crate::pulsar::MAX_TNShapeCoef + coef,
                            ));
                        }
                    }
                }
                continue;
            }
            if crate::t2fit_nestlike::is_noise(label)
                && let Some((_, _, nh, nlog)) = crate::t2fit_nestlike::families(p)?
                    .into_iter()
                    .find(|(sin, cos, _, _)| label == *sin || label == *cos)
            {
                columns.extend((0..nh + nlog).map(|i| Column::new(index, label, i)));
                continue;
            }
            let n = match label {
                param_wave_om => {
                    count(*read(&p.nWhite, "nWhite")?, "nWhite")?
                        * if p.waveScale == Some(2.0) { 4 } else { 2 }
                }
                param_wave_dm => count(*read(&p.nWhite_dm, "nWhite_dm")?, "nWhite_dm")? * 2,
                param_ifunc => count(*read(&p.ifuncN, "ifuncN")?, "ifuncN")?,
                param_clk_offs => count(*read(&p.clkOffsN, "clkOffsN")?, "clkOffsN")?,
                param_quad_ifunc_p => {
                    count(*read(&p.quad_ifuncN_p, "quad_ifuncN_p")?, "quad_ifuncN_p")?
                }
                param_quad_ifunc_c => {
                    count(*read(&p.quad_ifuncN_c, "quad_ifuncN_c")?, "quad_ifuncN_c")?
                }
                param_dmmodel => count(
                    *read(&p.dmoffsDMnum, "dmoffsDMnum")? + *read(&p.dmoffsCMnum, "dmoffsCMnum")?,
                    "dmmodel count",
                )?,
                param_ne_sw_ifunc => {
                    count(*read(&p.ne_sw_ifuncN, "ne_sw_ifuncN")?, "ne_sw_ifuncN")?
                }
                param_gwsingle => 4,
                // 0053: source omits labels after the first quadrupole column.
                // The common expansion below assigns the label to every one.
                param_quad_om => 4 * count(*read(&p.nQuad, "nQuad")?, "nQuad")?,
                param_tel_dx | param_tel_dy | param_tel_dz => {
                    let n = count(
                        *read(
                            match label {
                                param_tel_dx => &p.nTelDX,
                                param_tel_dy => &p.nTelDY,
                                _ => &p.nTelDZ,
                            },
                            "nTelD",
                        )?,
                        "nTelD",
                    )?;
                    if c.val == 2 {
                        n.checked_sub(1)
                            .ok_or_else(|| boundary("empty tel delta step basis"))?
                    } else {
                        n
                    }
                }
                _ => {
                    columns.push(Column::new(index, label, k));
                    continue;
                }
            };
            columns.extend((0..n).map(|i| Column::new(index, label, i)));
        }
    }
    Ok(columns)
}

fn callback_derivative(
    c: &mut PulsarContext,
    p: i32,
    x: f64,
    i: i32,
    l: i32,
    k: i32,
) -> Result<f64, TimingError> {
    crate::fit_callbacks::derivative(
        c,
        count(p, "pulsar")?,
        x,
        count(i, "observation")?,
        *PARAMETER_LABELS
            .get(count(l, "parameter")?)
            .ok_or_else(|| boundary("fit label"))?,
        count(k, "subindex")?,
    )
}
fn callback_update(
    c: &mut PulsarContext,
    p: i32,
    l: i32,
    k: i32,
    v: f64,
    e: f64,
) -> Result<(), TimingError> {
    crate::fit_callbacks::update(
        c,
        count(p, "pulsar")?,
        *PARAMETER_LABELS
            .get(count(l, "parameter")?)
            .ok_or_else(|| boundary("fit label"))?,
        count(k, "subindex")?,
        v,
        e,
    )
}
fn callback_constraint(
    context: &mut PulsarContext,
    p: i32,
    constraint: i32,
    label: i32,
    constraint_k: i32,
    k: i32,
    special: Option<&crate::constraints::ConstraintSpecial>,
) -> Result<f64, TimingError> {
    crate::constraints::derivative(
        context,
        count(p, "pulsar")?,
        crate::pulsar::Constraint::from_index(constraint)
            .ok_or_else(|| boundary("constraint label"))?,
        *PARAMETER_LABELS
            .get(count(label, "parameter")?)
            .ok_or_else(|| boundary("constraint parameter"))?,
        count(constraint_k, "constraint counter")?,
        count(k, "parameter counter")?,
        special,
    )
}
fn install_info(p: &mut SourcePulsar, columns: &[Column]) -> Result<(), TimingError> {
    if columns.len() > MAX_FIT {
        return Err(boundary("fit MAX_FIT capacity"));
    }
    let mut f = FitInfo {
        nParams: Some(columns.len() as u32),
        nConstraints: Some(0),
        ..FitInfo::default()
    };
    // t2Fit_fillFitInfo clears the complete struct, including inactive slots.
    // Keep this fit-time write separate from the lifecycle's unwritten default.
    f.paramIndex.fill(Some(0));
    f.constraintIndex.fill(Some(0));
    f.paramCounters.fill(Some(0));
    f.constraintCounters.fill(Some(0));
    f.paramDerivs.fill(Some(None));
    f.constraintDerivs.fill(Some(None));
    f.constraintSpecial.fill(Some(None));
    f.constraintValue.fill(Some(0.0));
    f.updateFunctions.fill(Some(None));
    f.output.parameterEstimates.fill(Some(0.0));
    f.output.errorEstimates.fill(Some(0.0));
    f.output.indexPsr.fill(Some(0));
    f.output.indexParam.fill(Some(0));
    f.output.indexCounter.fill(Some(0));
    f.output.totalNfit = Some(0);
    for (i, c) in columns.iter().enumerate() {
        f.paramIndex[i] = Some(c.parameter_index as i32);
        f.paramCounters[i] = Some(c.subindex as i32);
        f.paramDerivs[i] = Some(Some(Callback {
            identity: c.parameter_index as u64 + 1,
            callable: callback_derivative,
        }));
        f.updateFunctions[i] = Some(Some(Callback {
            identity: c.parameter_index as u64 + 1,
            callable: callback_update,
        }));
    }
    p.fitinfo = f;
    Ok(())
}
fn physical(p: &SourcePulsar, c: &Column) -> (Option<Number>, Option<Number>) {
    if c.label() == param_ZERO {
        return (p.offset.map(Number::double), p.offset_e.map(Number::double));
    }
    if c.label() == param_JUMP {
        return (
            p.jumpVal[c.subindex].map(Number::double),
            p.jumpValErr[c.subindex].map(Number::double),
        );
    }
    if c.label() == param_FDJUMP {
        return (
            p.fdjumpVal[c.subindex].map(Number::double),
            p.fdjumpValErr[c.subindex].map(Number::double),
        );
    }
    let k = c.subindex;
    let pair = match c.label() {
        param_ifunc => Some((p.ifuncV[k], p.ifuncE[k])),
        param_quad_ifunc_p => Some((p.quad_ifuncV_p[k], p.quad_ifuncE_p[k])),
        param_quad_ifunc_c => Some((p.quad_ifuncV_c[k], p.quad_ifuncE_c[k])),
        param_ne_sw_ifunc => Some((p.ne_sw_ifuncV[k], p.ne_sw_ifuncE[k])),
        param_tel_dx => Some((p.telDX_v[k], p.telDX_e[k])),
        param_tel_dy => Some((p.telDY_v[k], p.telDY_e[k])),
        param_tel_dz => Some((p.telDZ_v[k], p.telDZ_e[k])),
        param_quad_om => {
            let j = k / 4;
            Some(match k % 4 {
                0 => (p.quad_aplus_r[j], p.quad_aplus_r_e[j]),
                1 => (p.quad_aplus_i[j], p.quad_aplus_i_e[j]),
                2 => (p.quad_across_r[j], p.quad_across_r_e[j]),
                _ => (p.quad_across_i[j], p.quad_across_i_e[j]),
            })
        }
        // The gwsingle callback stores amplitudes but has no error fields.
        param_gwsingle => Some((
            match k {
                0 => p.gwsrc_aplus_r,
                1 => p.gwsrc_across_r,
                2 => p.gwsrc_aplus_i,
                3 => p.gwsrc_across_i,
                _ => None,
            },
            None,
        )),
        param_shapevent => {
            let e = k / crate::pulsar::MAX_TNShapeCoef;
            let j = k % crate::pulsar::MAX_TNShapeCoef;
            Some((p.TNShapeletEvCoef[e][j], p.TNShapeletEvCoefErr[e][j]))
        }
        param_dmmodel => {
            let n = p.dmoffsDMnum.unwrap_or(0) as usize;
            if k < n {
                Some((p.dmoffsDM[k], p.dmoffsDM_error[k]))
            } else {
                Some((p.dmoffsCM[k - n], p.dmoffsCM_error[k - n]))
            }
        }
        param_wave_dm | param_wave_om => {
            let dm = c.label() == param_wave_dm
                || (p.waveScale == Some(2.0) && k < (p.nWhite.unwrap_or(0) * 2) as usize);
            let j = if dm {
                k
            } else {
                k % (p.nWhite.unwrap_or(0) * 2) as usize
            } / 2;
            Some(if dm {
                if k.is_multiple_of(2) {
                    (p.wave_cos_dm[j], p.wave_cos_dm_err[j])
                } else {
                    (p.wave_sine_dm[j], p.wave_sine_dm_err[j])
                }
            } else if k.is_multiple_of(2) {
                (p.wave_cos[j], p.wave_cos_err[j])
            } else {
                (p.wave_sine[j], p.wave_sine_err[j])
            })
        }
        l if crate::t2fit_nestlike::is_noise(l) => return (None, None),
        _ => None,
    };
    if let Some((v, e)) = pair {
        return (v.map(Number::double), e.map(Number::double));
    }
    p.param.slots[c.parameter_index]
        .cells
        .get(c.subindex)
        .map(|v| {
            (
                Some(Number::extended(&v.val)),
                Some(Number::extended(&v.err)),
            )
        })
        .unwrap_or((None, None))
}

/// t2Fit_fillGlobalFitInfo: source-pulsar-zero jump and parameter coordinates.
pub fn global_fit_columns(
    pulsar: &SourcePulsar,
    options: &FitOptions,
) -> Result<Vec<Column>, TimingError> {
    let mut globals = vec![];
    for k in 1..=count(*read(&pulsar.nJumps, "nJumps")?, "nJumps")? {
        if pulsar.fitJump[k] == Some(2) {
            globals.push(Column::new(-1, param_JUMP, k));
        }
    }
    globals.extend(parameter_columns(pulsar, -1, 2, options)?);
    Ok(globals)
}

/// Install source local fit columns followed by supplied global columns.
/// Rows come from get_fit_data on this canonical timed pulsar. Constraint
/// payloads are installed separately by fill_constraint_info.
pub fn fill_fit_columns(
    context: &mut PulsarContext,
    i: usize,
    rows: &[FitRow],
    globals: &[Column],
    options: &FitOptions,
) -> Result<Vec<Column>, TimingError> {
    crate::t2fit_nestlike::prepare_parameters(&mut context.pulsars[i], true)?;
    let mut columns = vec![Column::new(i as i32, param_ZERO, 0)];
    let (p, obs) = context.pulsar_observations_mut(i)?;
    columns.extend(
        crate::t2fit_nestlike::jitter_epochs(p, obs, rows)?
            .into_iter()
            .map(|k| Column::new(i as i32, param_jitter, k)),
    );
    for (label, n) in [
        (param_JUMP, *read(&context.pulsars[i].nJumps, "nJumps")?),
        (
            param_FDJUMP,
            *read(&context.pulsars[i].nfdJumps, "nfdJumps")?,
        ),
    ] {
        for k in 1..=count(n, "jump count")? {
            let p = &context.pulsars[i];
            let selected = if label == param_JUMP {
                p.fitJump[k]
            } else {
                p.fitfdJump[k]
            };
            if selected != Some(1) {
                continue;
            }
            for row in rows {
                if crate::fit_callbacks::derivative(
                    context,
                    i,
                    row.x_days.0.to_f64(),
                    row.observation,
                    label,
                    k,
                )? != 0.0
                {
                    columns.push(Column::new(i as i32, label, k));
                    break;
                }
            }
        }
    }
    columns.extend(parameter_columns(
        &context.pulsars[i],
        i as i32,
        1,
        options,
    )?);
    columns.extend(globals.iter().cloned());
    crate::t2fit_nestlike::prepare_parameters(&mut context.pulsars[i], false)?;
    install_info(&mut context.pulsars[i], &columns)?;
    Ok(columns)
}

/// Install owned source constraint payloads and return their rows/targets.
pub fn fill_constraint_info(
    context: &mut PulsarContext,
    i: usize,
    columns: &[Column],
) -> Result<crate::constraints::ConstraintRows, TimingError> {
    let constraints = crate::constraints::build(context, i, columns)?;
    if context.pulsars[i].nconstraints == Some(0) && !constraints.matrix.is_empty() {
        // Noise-prior rows do not consume interpolation weights, but the source
        // still updates them whenever the resulting constraint matrix is nonempty.
        crate::constraints::compute_weights(&mut context.pulsars[i])?;
    }
    if constraints.matrix.len() > MAX_FIT {
        return Err(boundary("constraint MAX_FIT capacity"));
    }
    context.pulsars[i].fitinfo.nConstraints = Some(constraints.matrix.len() as u32);
    for j in 0..constraints.matrix.len() {
        let special = constraints.special[j]
            .clone()
            .map(|s| context.allocate_fit_constraint(s))
            .transpose()
            .map_err(|e| boundary(&e.to_string()))?;
        let f = &mut context.pulsars[i].fitinfo;
        f.constraintIndex[j] = Some(constraints.labels[j]);
        f.constraintCounters[j] = Some(constraints.counters[j]);
        f.constraintValue[j] = Some(constraints.targets[j]);
        f.constraintSpecial[j] = Some(special);
        f.constraintDerivs[j] = Some(Some(Callback {
            identity: constraints.labels[j] as u64 + 1,
            callable: callback_constraint,
        }));
    }
    Ok(constraints)
}

/// t2fit_prefit for one active pulsar, clearing the eight fitted-noise accumulators.
pub fn prefit(context: &mut PulsarContext, i: usize) -> Result<(), TimingError> {
    let (p, obs) = context.pulsar_observations_mut(i)?;
    // fda6ad4 t2fit.C::t2fit_prefit omits the nonglobal count; textOutput:1214
    // consumes it for quadrupole files. A new fit starts with zero global columns.
    p.globalNfit = Some(0);
    for o in obs.iter_mut().take(count(*read(&p.nobs, "nobs")?, "nobs")?) {
        o.TNRedSignal = Some(0.0);
        o.TNRedErr = Some(0.0);
        o.TNDMSignal = Some(0.0);
        o.TNDMErr = Some(0.0);
        o.TNChromSignal = Some(0.0);
        o.TNChromErr = Some(0.0);
        o.SWGPSignal = Some(0.0);
        o.SWGPErr = Some(0.0);
    }
    Ok(())
}

struct Prepared {
    rows: Vec<FitRow>,
    columns: Vec<Column>,
    matrix: Vec<Vec<f64>>,
    white_matrix: Vec<Vec<f64>>,
    data: Vec<f64>,
    white_data: Vec<f64>,
    constraints: crate::constraints::ConstraintRows,
    whitening: Whitening,
    inputs: Vec<crate::bats::ConsumedInput>,
    covariance_files: Vec<std::path::PathBuf>,
}
/// t2Fit built-in driver on canonical, already timed state.
pub fn t2_fit(
    context: &mut PulsarContext,
    options: &FitOptions,
) -> Result<Vec<FitSystem>, TimingError> {
    let _workflow = crate::execution::enter();
    crate::t2accel::with_acceleration(options.acceleration, || t2_fit_selected(context, options))
}
fn t2_fit_selected(
    context: &mut PulsarContext,
    options: &FitOptions,
) -> Result<Vec<FitSystem>, TimingError> {
    let p = context
        .pulsars
        .first()
        .ok_or_else(|| boundary("fit_empty_context"))?;
    if read_string(&p.fitFunc[..], "fitFunc")? != b"default" {
        return Err(boundary("native fit plugin ABI is not available"));
    }
    let covariance = options.covariance.as_deref().filter(|s| *s != "NULL");
    for i in 0..context.pulsars.len() {
        if context.pulsars[i].auto_constraints != Some(0) {
            crate::constraints::auto_constraints(&mut context.pulsars[i], i)?;
        }
    }
    let globals = global_fit_columns(&context.pulsars[0], options)?;
    let ng = globals.len();
    let mut prepared = vec![];
    for i in 0..context.pulsars.len() {
        prefit(context, i)?;
        let p = &mut context.pulsars[i];
        p.rescaleErrChisq = Some(i32::from(options.rescale_err_chisq));
        // t2fit.C captured row indices before sortToAs, pairing data with the wrong
        // observations on unordered direct input. Sort before capturing any rows.
        // Original/corrected witness: validation/general_fitting/order_probe.cc.
        if covariance.is_some() {
            crate::sort_toas::sort_toas(context, i)?;
            context.pulsars[i].fitMode = Some(1);
        }
        let rows = get_fit_data(context, i)?;
        let columns = fill_fit_columns(context, i, &rows, &globals, options)?;
        if let Some(io) = &options.diagnostics
            && io.write_residuals & 2 != 0
        {
            let bytes: String = columns
                .iter()
                .enumerate()
                .map(|(j, c)| format!("{j} {} {}\n", c.parameter, c.subindex))
                .collect();
            io.write("param.labels", bytes.as_bytes())
                .map_err(|e| boundary(&e))?;
        }
        let mut matrix = vec![];
        for row in &rows {
            matrix.push(design_matrix_row(
                context,
                i,
                row.x_days.0.to_f64(),
                row.observation,
            )?);
        }
        let constraints = fill_constraint_info(context, i, &columns)?;
        let data: Vec<_> = rows.iter().map(|r| r.residual_s.0.to_f64()).collect();
        let mode = *read(&context.pulsars[i].fitMode, "fitMode")?;
        let sigma: Vec<_> = rows
            .iter()
            .map(|r| if mode == 0 { 1.0 } else { r.sigma_s.0.to_f64() })
            .collect();
        let mut inputs = constraints.inputs.clone();
        let mut covariance_files = vec![];
        let (white_data, white_matrix, cm, lower) = if let Some(selector) = covariance {
            crate::clkcorr::with_diagnostics(|d| {
                d.log(b'm', "t2fit.C", 169, b"Doing a FULL COVARIANCE MATRIX fit");
                Ok(())
            })?;
            let x: Vec<_> = rows.iter().map(|r| r.x_days.0.to_f64()).collect();
            let indices: Vec<_> = rows.iter().map(|r| r.observation).collect();
            let result = crate::cholesky::get_cholesky_matrix(
                context,
                i,
                selector,
                &indices,
                &x,
                &sigma,
                0,
                &mut inputs,
                options.diagnostics.as_ref(),
            )?;
            covariance_files = result.diagnostic_files;
            let (cm, l) = (result.covariance, result.lower);
            (
                crate::tk_matrix::forward_sub_vec(&l, &data).map_err(|e| boundary(&e))?,
                crate::tk_matrix::forward_sub(&l, &matrix).map_err(|e| boundary(&e))?,
                Some(numbers(cm)),
                Some(numbers(l)),
            )
        } else {
            let mut det = 0.0;
            for s in &sigma {
                det += s.ln();
            }
            context.pulsars[i].detL = Some(det);
            (
                data.iter().zip(&sigma).map(|(v, s)| v / s).collect(),
                matrix
                    .iter()
                    .zip(&sigma)
                    .map(|(r, s)| r.iter().map(|v| v / s).collect())
                    .collect(),
                None,
                None,
            )
        };
        let whitening = Whitening {
            pulsar: i,
            fit_mode: mode,
            robust: *read(&context.pulsars[i].robust, "robust")?,
            nits: *read(&context.pulsars[i].nits, "nits")?,
            rescale_err_chisq: *read(&context.pulsars[i].rescaleErrChisq, "rescaleErrChisq")?,
            sigma: sigma.into_iter().map(Into::into).collect(),
            covariance: cm,
            lower,
            log_determinant: (*read(&context.pulsars[i].detL, "detL")?).into(),
        };
        prepared.push(Prepared {
            rows,
            columns,
            matrix,
            white_matrix,
            data,
            white_data,
            constraints,
            whitening,
            inputs,
            covariance_files,
        });
    }
    let groups = if ng == 0 {
        prepared.into_iter().map(|p| vec![p]).collect()
    } else {
        vec![prepared]
    };
    let mut systems = vec![];
    for group in groups {
        let locals: Vec<_> = group.iter().map(|p| p.columns.clone()).collect();
        let mut columns = globals.clone();
        for p in &group {
            columns.extend(p.columns[..p.columns.len() - ng].iter().cloned());
        }
        let ncols = columns.len();
        if ncols > MAX_FIT {
            return Err(boundary("combined fit MAX_FIT capacity"));
        }
        let mut rows = vec![];
        let mut matrix = vec![];
        let mut white_matrix = vec![];
        let mut data = vec![];
        let mut white_data = vec![];
        let mut offset = ng;
        let mut constraints = vec![];
        let mut constraint_targets = vec![];
        let mut whitening = vec![];
        let mut covariance_inputs = vec![];
        let mut covariance_files = vec![];
        let mut constraint_rows = vec![];
        for p in group {
            covariance_files.extend(p.covariance_files);
            let nl = p.columns.len() - ng;
            for (a, w) in p.matrix.into_iter().zip(p.white_matrix) {
                let mut m = vec![0.0; ncols];
                let mut wm = vec![0.0; ncols];
                m[..ng].copy_from_slice(&a[nl..]);
                wm[..ng].copy_from_slice(&w[nl..]);
                m[offset..offset + nl].copy_from_slice(&a[..nl]);
                wm[offset..offset + nl].copy_from_slice(&w[..nl]);
                matrix.push(m);
                white_matrix.push(wm);
            }
            constraint_rows.extend(
                p.constraints
                    .labels
                    .iter()
                    .zip(&p.constraints.counters)
                    .map(|(&label, &subindex)| ConstraintRow {
                        pulsar: p.rows[0].pulsar,
                        label,
                        subindex,
                    }),
            );
            for a in p.constraints.matrix {
                let mut c = vec![0.0; ncols];
                c[..ng].copy_from_slice(&a[nl..]);
                c[offset..offset + nl].copy_from_slice(&a[..nl]);
                constraints.push(c);
            }
            constraint_targets.extend(p.constraints.targets);
            whitening.push(p.whitening);
            covariance_inputs.extend(p.inputs);
            offset += nl;
            rows.extend(p.rows);
            data.extend(p.data);
            white_data.extend(p.white_data);
        }
        let control = rows[0].pulsar;
        let p = &context.pulsars[control];
        let mode = *read(&p.fitMode, "fitMode")?;
        let rescale = mode == 0 || (mode == 1 && p.rescaleErrChisq == Some(1));
        let solver = crate::tkrobust::robust_least_squares(
            &data,
            &white_data,
            &matrix,
            &white_matrix,
            &constraints,
            &constraint_targets,
            1e-27,
            rescale,
            *read(&p.robust, "robust")?,
            options.diagnostics.as_ref(),
        )
        .map_err(|e| boundary(&format!("fit: {e}")))?;
        let mut updates = vec![];
        let mut update_files = vec![];
        let mut offset = ng;
        for lc in &locals {
            let index = lc[0].pulsar as usize;
            let nl = lc.len() - ng;
            let p = &mut context.pulsars[index];
            p.fitChisq = Some(solver.chi_square.0.to_f64());
            p.fitNfree = Some(solver.dof as i32);
            p.nParam = Some(lc.len() as i32);
            p.nGlobal = Some(ng as i32);
            p.fitinfo.output.totalNfit = Some(ncols as i32);
            if ng > 0 {
                p.globalNfit = Some(ncols as i32);
            }
            for (j, c) in columns.iter().enumerate() {
                p.fitinfo.output.indexPsr[j] = Some(c.pulsar);
                p.fitinfo.output.indexParam[j] = Some(c.parameter_index as i32);
                p.fitinfo.output.indexCounter[j] = Some(c.subindex as i32);
                p.fitinfo.output.parameterEstimates[j] = Some(solver.increments[j].0.to_f64());
                p.fitinfo.output.errorEstimates[j] = Some(solver.errors[j].0.to_f64());
            }
            let owner = p
                .covar
                .flatten()
                .ok_or_else(|| boundary("fit covariance owner"))?;
            let cov = context
                .matrix_mut(owner)
                .map_err(|e| boundary(&e.to_string()))?;
            let stride = cov.get_blas_cols();
            for (j, r) in solver.covariance.iter().enumerate() {
                for (k, v) in r.iter().enumerate() {
                    cov.cells_mut()[j * stride + k] = v.0.to_f64();
                }
            }
            for (j, c) in lc.iter().enumerate() {
                let out = if j < nl { offset + j } else { j - nl };
                let (before_value, before_error) = physical(&context.pulsars[index], c);
                if let Some(io) = &options.diagnostics
                    && let Some(file) = crate::t2fit_nestlike::update_metadata(
                        &context.pulsars[index],
                        c.label(),
                        c.subindex,
                        io,
                    )?
                {
                    update_files.push(file);
                }
                crate::fit_callbacks::update(
                    context,
                    index,
                    c.label(),
                    c.subindex,
                    solver.increments[out].0.to_f64(),
                    solver.errors[out].0.to_f64(),
                )?;
                let (after_value, after_error) = physical(&context.pulsars[index], c);
                updates.push(PhysicalUpdate {
                    pulsar: index,
                    column: c.clone(),
                    before_value,
                    before_error,
                    after_value,
                    after_error,
                    postfit_value: None,
                    postfit_error: None,
                    increment: solver.increments[out].clone(),
                    fit_error: solver.errors[out].clone(),
                });
            }
            offset += nl;
        }
        let mut diagnostic_files = solver.diagnostic_files.clone();
        diagnostic_files.extend(covariance_files);
        diagnostic_files.extend(update_files);
        if let Some(io) = &options.diagnostics
            && io.write_residuals & 2 != 0
        {
            diagnostic_files.push(io.directory.join("param.labels"));
        }
        systems.push(FitSystem {
            rows,
            columns,
            data: data.into_iter().map(Into::into).collect(),
            white_data: white_data.into_iter().map(Into::into).collect(),
            matrix: numbers(matrix),
            white_matrix: numbers(white_matrix),
            constraints: numbers(constraints),
            constraint_targets: constraint_targets.into_iter().map(Into::into).collect(),
            solver_engine: if solver.solver == "QR" {
                "source QR (LAPACK)"
            } else {
                "source SVD (nonaccelerated)"
            },
            solver,
            updates,
            whitening,
            covariance_inputs,
            constraint_rows,
            noise: vec![],
            working_directory: std::env::current_dir().map_err(|e| boundary(&e.to_string()))?,
            diagnostic_files,
        });
    }
    let postfit_files = postfit(context, options.diagnostics.as_ref())?;
    for system in &mut systems {
        for update in &mut system.updates {
            (update.postfit_value, update.postfit_error) =
                physical(&context.pulsars[update.pulsar], &update.column);
        }
        system
            .diagnostic_files
            .extend(postfit_files.iter().cloned());
        for w in &system.whitening {
            let (p, obs) = context.pulsar_observations_mut(w.pulsar)?;
            for (i, o) in obs
                .iter()
                .take(*read(&p.nobs, "nobs")? as usize)
                .enumerate()
            {
                system.noise.push(NoiseState {
                    pulsar: w.pulsar,
                    observation: i,
                    red: (*read(&o.TNRedSignal, "TNRedSignal")?).into(),
                    red_error: (*read(&o.TNRedErr, "TNRedErr")?).into(),
                    dm: (*read(&o.TNDMSignal, "TNDMSignal")?).into(),
                    dm_error: (*read(&o.TNDMErr, "TNDMErr")?).into(),
                    chrom: (*read(&o.TNChromSignal, "TNChromSignal")?).into(),
                    chrom_error: (*read(&o.TNChromErr, "TNChromErr")?).into(),
                    solar_wind: (*read(&o.SWGPSignal, "SWGPSignal")?).into(),
                    solar_wind_error: (*read(&o.SWGPErr, "SWGPErr")?).into(),
                });
            }
        }
    }
    Ok(systems)
}
fn numbers(m: Vec<Vec<f64>>) -> Vec<Vec<FitNumber>> {
    m.into_iter()
        .map(|r| r.into_iter().map(Into::into).collect())
        .collect()
}
pub fn do_fit_all(
    context: &mut PulsarContext,
    options: &FitOptions,
) -> Result<Vec<FitSystem>, TimingError> {
    t2_fit(context, options)
}

/// t2Fit_buildDesignMatrix for the installed canonical FitInfo callbacks.
pub fn design_matrix_row(
    context: &mut PulsarContext,
    index: usize,
    x_days: f64,
    observation: usize,
) -> Result<Vec<f64>, TimingError> {
    let n = *read(&context.pulsars[index].fitinfo.nParams, "fit nParams")? as usize;
    let mut row = Vec::new();
    for j in 0..n {
        let info = &context.pulsars[index].fitinfo;
        let label = *read(&info.paramIndex[j], "fit parameter")?;
        let k = *read(&info.paramCounters[j], "fit parameter counter")?;
        let callback = info.paramDerivs[j]
            .flatten()
            .ok_or_else(|| boundary("null fit derivative"))?;
        let value =
            (callback.callable)(context, index as i32, x_days, observation as i32, label, k)?;
        if value.is_nan() {
            return Err(boundary(&format!(
                "NaN design matrix {index} {observation} {}",
                PARAMETER_NAMES[label as usize]
            )));
        }
        row.push(value);
    }
    Ok(row)
}

/// t2Fit_updateParameters on the installed local-then-global FitInfo order.
/// Postfit noise projection is an explicit separate operation.
pub fn update_parameters(
    context: &mut PulsarContext,
    index: usize,
    values: &[f64],
    errors: &[f64],
) -> Result<(), TimingError> {
    let n = *read(&context.pulsars[index].fitinfo.nParams, "fit nParams")? as usize;
    if values.len() != n || errors.len() != n {
        return Err(boundary("fit update dimensions"));
    }
    for j in 0..n {
        let info = &context.pulsars[index].fitinfo;
        let label = *read(&info.paramIndex[j], "fit parameter")?;
        let k = *read(&info.paramCounters[j], "fit parameter counter")?;
        let callback = info.updateFunctions[j]
            .flatten()
            .ok_or_else(|| boundary("null fit update"))?;
        (callback.callable)(context, index as i32, label, k, values[j], errors[j])?;
    }
    Ok(())
}
/// t2Fit_getParamMatrixRow; -1 output owners apply to every pulsar.
pub fn get_param_matrix_row(info: &FitInfo, pulsar: i32, label: Label, k: i32) -> Option<usize> {
    (0..info.output.totalNfit.unwrap_or(0).max(0) as usize).find(|&i| {
        matches!(info.output.indexPsr[i],Some(p) if p==pulsar||p == -1)
            && info.output.indexParam[i] == Some(label as i32)
            && info.output.indexCounter[i] == Some(k)
    })
}
/// t2Fit_buildConstraintMatrix row: invoke installed canonical callbacks.
pub fn constraint_matrix_row(
    context: &mut PulsarContext,
    p: usize,
    row: usize,
) -> Result<Vec<f64>, TimingError> {
    let f = &context.pulsars[p].fitinfo;
    if row >= *read(&f.nConstraints, "fitinfo.nConstraints")? as usize {
        return Err(boundary("constraint matrix row"));
    }
    let c = *read(&f.constraintIndex[row], "constraintIndex")?;
    let ck = *read(&f.constraintCounters[row], "constraintCounters")?;
    let callback = f.constraintDerivs[row]
        .flatten()
        .ok_or_else(|| boundary("constraint callback"))?;
    let special = f.constraintSpecial[row]
        .flatten()
        .map(|v| context.fit_constraint(v).cloned())
        .transpose()
        .map_err(|e| boundary(&e.to_string()))?;
    let params = (0..*read(&f.nParams, "fitinfo.nParams")? as usize)
        .map(|j| {
            Ok((
                *read(&f.paramIndex[j], "paramIndex")?,
                *read(&f.paramCounters[j], "paramCounters")?,
            ))
        })
        .collect::<Result<Vec<_>, TimingError>>()?;
    params
        .into_iter()
        .map(|(label, k)| (callback.callable)(context, p as i32, c, label, ck, k, special.as_ref()))
        .collect()
}

/// t2fit_postfit: source family-dependent error square roots and deterministic
/// projections. Band/group accumulators follow the source red-family condition.
pub fn postfit(
    context: &mut PulsarContext,
    diagnostics: Option<&crate::tkfit::FitDiagnostics>,
) -> Result<Vec<std::path::PathBuf>, TimingError> {
    let mut files = vec![];
    for i in 0..context.pulsars.len() {
        let families = crate::t2fit_nestlike::families(&context.pulsars[i])?;
        for (sin, _, _, _) in families {
            let label = match sin {
                param_red_sin => param_f,
                param_red_dm_sin => param_dm,
                param_red_chrom_sin => param_cm,
                param_swgp_sin => param_ne_sw,
                _ => continue,
            };
            let (p, obs) = context.pulsar_observations_mut(i)?;
            for o in obs.iter_mut().take(*read(&p.nobs, "nobs")? as usize) {
                let err = match label {
                    param_f => &mut o.TNRedErr,
                    param_dm => &mut o.TNDMErr,
                    param_cm => &mut o.TNChromErr,
                    _ => &mut o.SWGPErr,
                };
                *err = Some(read(err, "noise squared error")?.sqrt());
            }
            if label == param_ne_sw {
                if p.param.slots[label as usize].cells[0].fit_flag != 0 {
                    project_noise(context, i, label, diagnostics)?;
                }
            } else if *read(&p.TNsubtractPoly, "TNsubtractPoly")? & 1 != 0
                && let Some(path) = project_noise(context, i, label, diagnostics)?
            {
                files.push(path);
            }
        }
    }
    Ok(files)
}
/// subtractTNPoly / subtractSWGPMeanSW share data selection and the canonical
/// solver; raw TOA errors here are microseconds, as in the pinned mini-fits.
fn project_noise(
    context: &mut PulsarContext,
    index: usize,
    label: Label,
    diagnostics: Option<&crate::tkfit::FitDiagnostics>,
) -> Result<Option<std::path::PathBuf>, TimingError> {
    let rows = get_fit_data(context, index)?;
    let ks: Vec<_> = if label == param_ne_sw {
        vec![0]
    } else {
        context.pulsars[index].param.slots[label as usize]
            .cells
            .iter()
            .enumerate()
            .filter_map(|(k, c)| (c.fit_flag == 1).then_some(k))
            .collect()
    };
    // t2fit.C leaves white_y unwritten for the constant-only red fit.
    // There is no deterministic parameter to update, and its constant is never
    // subtracted from the noise signal. Omit that unused, undefined solve.
    if ks.is_empty() {
        return Ok(None);
    }
    let mut matrix = vec![];
    let mut data = vec![];
    let mut sigma = vec![];
    for r in &rows {
        let mut m = vec![];
        for &k in &ks {
            m.push(crate::fit_callbacks::derivative(
                context,
                index,
                r.x_days.0.to_f64(),
                r.observation,
                label,
                k,
            )?);
        }
        if label == param_f {
            m.push(1.0);
        }
        matrix.push(m);
        let (_, obs) = context.pulsar_observations_mut(index)?;
        let o = &obs[r.observation];
        data.push(*read(
            match label {
                param_f => &o.TNRedSignal,
                param_dm => &o.TNDMSignal,
                param_cm => &o.TNChromSignal,
                _ => &o.SWGPSignal,
            },
            "noise signal",
        )?);
        sigma.push(*read(&o.toaErr, "toaErr")?);
    }
    let white_data: Vec<_> = data.iter().zip(&sigma).map(|(v, s)| v / s).collect();
    let white_matrix: Vec<Vec<_>> = matrix
        .iter()
        .zip(&sigma)
        .map(|(r, s)| r.iter().map(|v| v / s).collect())
        .collect();
    let fit = crate::tkfit::constrained_least_squares(
        &data,
        &white_data,
        &matrix,
        &white_matrix,
        &[],
        &[],
        1e-27,
        false,
    )
    .map_err(|e| boundary(&e))?;
    let mut metadata = vec![];
    for (j, &k) in ks.iter().enumerate() {
        let v = fit.increments[j].0.to_f64();
        crate::clkcorr::with_diagnostics(|d| {
            d.log(
                b'm',
                "t2fit.C",
                716,
                &crate::source_format::format(
                    "TNsubtractPoly -- Updating %s k=%d : %g +/- %g",
                    &[
                        PARAMETER_NAMES[label as usize].into(),
                        (k as i32).into(),
                        v.into(),
                        fit.errors[j].0.to_f64().into(),
                    ],
                )?,
            );
            Ok(())
        })?;
        use crate::clock_text::{NumberFormat, format_number};
        metadata.extend(
            [
                format!("{} {k} ", PARAMETER_NAMES[label as usize]).as_bytes(),
                &format_number(v, NumberFormat::GeneralSix),
                b" ",
                &format_number(fit.errors[j].0.to_f64(), NumberFormat::GeneralSix),
                b"\n",
            ]
            .concat(),
        );
        crate::fit_callbacks::update(context, index, label, k, v, -1.0)?;
        let (_, obs) = context.pulsar_observations_mut(index)?;
        for (i, r) in rows.iter().enumerate() {
            let o = &mut obs[r.observation];
            let signal = match label {
                param_f => &mut o.TNRedSignal,
                param_dm => &mut o.TNDMSignal,
                param_cm => &mut o.TNChromSignal,
                _ => &mut o.SWGPSignal,
            };
            *signal = Some(*read(signal, "noise signal")? - matrix[i][j] * v);
        }
    }
    if label != param_ne_sw
        && let Some(io) = diagnostics
        && io.write_residuals & 4 != 0
    {
        let name = match label {
            param_dm => "tnreddm_subtract.meta",
            param_cm => "tnredcm_subtract.meta",
            _ => "tnred_subtract.meta",
        };
        return io
            .write(name, &metadata)
            .map(Some)
            .map_err(|e| boundary(&e));
    }
    Ok(None)
}
