//! t2fit.C legacy interfaces. Their diagonal argument multiplies rows;
//! the modern driver instead divides by sigma. Full matrices are lower L.
use crate::{
    TimingError, clock_diagnostics::boundary, observation::read, pulsar_lifecycle::PulsarContext,
    tkfit::LeastSquares,
};
use serde::Serialize;

pub type Basis = fn(&mut PulsarContext, usize, f64, usize, &mut [f64]) -> Result<(), TimingError>;

#[derive(Debug, Serialize)]
pub struct Design {
    #[serde(serialize_with = "crate::tkfit::serialize_matrix")]
    pub matrix: Vec<Vec<f64>>,
    #[serde(serialize_with = "crate::tkfit::serialize_matrix")]
    pub white_matrix: Vec<Vec<f64>>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub data: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub white_data: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct GlobalFit {
    pub design: Design,
    #[serde(serialize_with = "crate::tkfit::serialize_matrix")]
    pub constraints: Vec<Vec<f64>>,
    pub solver: LeastSquares,
}

/// TKfit_getPulsarDesignMatrix. The caller supplies the legacy column ordering.
#[allow(clippy::too_many_arguments)]
pub fn design_matrix(
    context: &mut PulsarContext,
    p: usize,
    x: &[f64],
    y: &[f64],
    observations: &[usize],
    nparams: usize,
    lower: &[Vec<f64>],
    basis: Basis,
) -> Result<Design, TimingError> {
    let n = x.len();
    if n == 0
        || y.len() != n
        || observations.len() != n
        || nparams == 0
        || !(lower.len() == 1 || lower.len() == n)
        || lower.iter().any(|r| r.len() != n)
    {
        return Err(boundary("TKfit_getPulsarDesignMatrix dimensions"));
    }
    let mut matrix = vec![vec![0.0; nparams]; n];
    for i in 0..n {
        basis(context, p, x[i], observations[i], &mut matrix[i])?;
    }
    let (white_data, white_matrix) = if lower.len() == 1 {
        (
            y.iter().zip(&lower[0]).map(|(v, w)| v * w).collect(),
            matrix
                .iter()
                .zip(&lower[0])
                .map(|(r, w)| r.iter().map(|v| v * w).collect())
                .collect(),
        )
    } else {
        (
            crate::tk_matrix::forward_sub_vec(lower, y).map_err(|e| boundary(&e))?,
            crate::tk_matrix::forward_sub(lower, &matrix).map_err(|e| boundary(&e))?,
        )
    };
    Ok(Design {
        matrix,
        white_matrix,
        data: y.to_vec(),
        white_data,
    })
}

/// TKleastSquares_single_pulsar, including legacy additive constraints.
#[allow(clippy::too_many_arguments)]
pub fn single_pulsar(
    context: &mut PulsarContext,
    x: &[f64],
    y: &[f64],
    observations: &[usize],
    nparams: usize,
    lower: &[Vec<f64>],
    basis: Basis,
    tolerance: f64,
    rescale: bool,
) -> Result<LeastSquares, TimingError> {
    let design = design_matrix(context, 0, x, y, observations, nparams, lower, basis)?;
    let p = &mut context.pulsars[0];
    let nc = *read(&p.nconstraints, "nconstraints")? as usize;
    if nc > 0 {
        crate::constraints::compute_weights(p)?;
    }
    let mut constraints = vec![vec![0.0; nparams]; nc];
    for (i, row) in constraints.iter_mut().enumerate() {
        let c = *read(&context.pulsars[0].constraints[i], "constraint")?;
        crate::constraints::legacy_constraint(context, 0, c, row)?;
    }
    crate::tkrobust::robust_least_squares(
        &design.data,
        &design.white_data,
        &design.matrix,
        &design.white_matrix,
        &constraints,
        &vec![0.0; nc],
        tolerance,
        rescale,
        *read(&context.pulsars[0].robust, "robust")?,
        None,
    )
    .map_err(|e| boundary(&e))
}

/// TKleastSquares_global_pulsar. Each callback row starts with the shared
/// globals, followed by this pulsar's ZERO/local columns. `local_counts` excludes
/// globals. Unlike t2Fit's local-first callback rows, this is the legacy layout.
#[allow(clippy::too_many_arguments)]
pub fn global_pulsar(
    context: &mut PulsarContext,
    x: &[Vec<f64>],
    y: &[Vec<f64>],
    observations: &[Vec<usize>],
    local_counts: &[usize],
    globals: usize,
    lower: &[Vec<Vec<f64>>],
    basis: Basis,
    tolerance: f64,
    rescale: bool,
    diagnostics: Option<&crate::tkfit::FitDiagnostics>,
) -> Result<GlobalFit, TimingError> {
    let np = context.pulsars.len();
    if np == 0
        || [
            x.len(),
            y.len(),
            observations.len(),
            local_counts.len(),
            lower.len(),
        ]
        .iter()
        .any(|n| *n != np)
    {
        return Err(boundary("TKleastSquares_global_pulsar dimensions"));
    }
    let nrows: usize = x.iter().map(Vec::len).sum();
    let ncols = globals + local_counts.iter().sum::<usize>();
    let mut design = Design {
        matrix: vec![vec![0.0; ncols]; nrows],
        white_matrix: vec![vec![0.0; ncols]; nrows],
        data: vec![0.0; nrows],
        white_data: vec![0.0; nrows],
    };
    let mut constraints = vec![];
    let (mut row_offset, mut local_offset) = (0, globals);
    for p in 0..np {
        let nf = local_counts[p];
        let local = design_matrix(
            context,
            p,
            &x[p],
            &y[p],
            &observations[p],
            nf + globals,
            &lower[p],
            basis,
        )?;
        for i in 0..x[p].len() {
            design.matrix[row_offset + i][..globals].copy_from_slice(&local.matrix[i][..globals]);
            design.white_matrix[row_offset + i][..globals]
                .copy_from_slice(&local.white_matrix[i][..globals]);
            design.matrix[row_offset + i][local_offset..local_offset + nf]
                .copy_from_slice(&local.matrix[i][globals..]);
            design.white_matrix[row_offset + i][local_offset..local_offset + nf]
                .copy_from_slice(&local.white_matrix[i][globals..]);
            design.data[row_offset + i] = local.data[i];
            design.white_data[row_offset + i] = local.white_data[i];
        }
        let nc = *read(&context.pulsars[p].nconstraints, "nconstraints")? as usize;
        if nc > 0 {
            if nf == 0 {
                return Err(boundary("legacy constraints require local ZERO column"));
            }
            crate::constraints::compute_weights(&mut context.pulsars[p])?;
        }
        for i in 0..nc {
            // Pinned t2fit.C uses psr[0]'s label and writes a ZERO-first row
            // into a globals-first matrix at off_f. AE's legacy-layout witness
            // demonstrates both mismatches. Remap the existing enumerator once.
            let g = if p == 0 { globals } else { 0 };
            let mut row = vec![0.0; nf + g];
            let label = *read(&context.pulsars[p].constraints[i], "constraint")?;
            crate::constraints::legacy_constraint(context, p, label, &mut row)?;
            let mut combined = vec![0.0; ncols];
            combined[..g].copy_from_slice(&row[1..g + 1]);
            combined[local_offset] = row[0];
            combined[local_offset + 1..local_offset + nf].copy_from_slice(&row[g + 1..]);
            constraints.push(combined);
        }
        row_offset += x[p].len();
        local_offset += nf;
    }
    // The source allocates a zero-row constraints matrix and asserts when there
    // are no constraints. An empty owned matrix directly represents that case.
    let solver = crate::tkrobust::robust_least_squares(
        &design.data,
        &design.white_data,
        &design.matrix,
        &design.white_matrix,
        &constraints,
        &vec![0.0; constraints.len()],
        tolerance,
        rescale,
        *read(&context.pulsars[0].robust, "robust")?,
        diagnostics,
    )
    .map_err(|e| boundary(&e))?;
    Ok(GlobalFit {
        design,
        constraints,
        solver,
    })
}

/// TKleastSquares_svd_psr_dcm. `sig` is unused by the source adapter.
#[allow(clippy::too_many_arguments)]
pub fn svd_psr_dcm(
    context: &mut PulsarContext,
    x: &[f64],
    y: &[f64],
    observations: &[usize],
    nparams: usize,
    lower: &[Vec<f64>],
    basis: Basis,
    tolerance: f64,
    weight: i32,
) -> Result<LeastSquares, TimingError> {
    let rescale = weight == 0 || (weight == 1 && context.pulsars[0].rescaleErrChisq == Some(1));
    single_pulsar(
        context,
        x,
        y,
        observations,
        nparams,
        lower,
        basis,
        tolerance,
        rescale,
    )
}

/// TKleastSquares_svd_psr: source weight==1 multiplies by supplied sigma.
#[allow(clippy::too_many_arguments)]
pub fn svd_psr(
    context: &mut PulsarContext,
    x: &[f64],
    y: &[f64],
    sigma: &[f64],
    observations: &[usize],
    nparams: usize,
    basis: Basis,
    tolerance: f64,
    weight: i32,
) -> Result<LeastSquares, TimingError> {
    let diagonal = if weight == 1 {
        sigma.to_vec()
    } else {
        vec![1.0; x.len()]
    };
    svd_psr_dcm(
        context,
        x,
        y,
        observations,
        nparams,
        &[diagonal],
        basis,
        tolerance,
        weight,
    )
}

/// Pinned TKleastSquares_svd_passN explicitly exits without an implementation.
pub fn svd_pass_n() -> Result<(), TimingError> {
    Err(boundary(
        "TKleastSquares_svd_passN: source method no longer implemented",
    ))
}
