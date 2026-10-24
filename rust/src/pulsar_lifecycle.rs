//! Owned counterparts of initialise.C and tempo2Util.C::copyPSR.
//! Handles are context-qualified and never reused; moving a context is harmless.
use crate::{
    pulsar::SourcePulsar,
    tempo2::{MAX_PARAMS, ParameterStore, SourceObservation},
    tk_matrix::BlasMatrix,
};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct LifecycleBoundary {
    pub operation: &'static str,
    pub field: String,
    pub kind: &'static str,
}
pub(crate) fn fail(
    operation: &'static str,
    field: impl Into<String>,
    kind: &'static str,
) -> LifecycleBoundary {
    LifecycleBoundary {
        operation,
        field: field.into(),
        kind,
    }
}
impl std::fmt::Display for LifecycleBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.operation, self.field, self.kind)
    }
}
impl std::error::Error for LifecycleBoundary {}
pub(crate) fn read<T: Clone>(
    value: &Option<T>,
    operation: &'static str,
    field: &str,
) -> Result<T, LifecycleBoundary> {
    value
        .clone()
        .ok_or_else(|| fail(operation, field, "unwritten"))
}
macro_rules! owner {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            pub(crate) context: u64,
            pub(crate) index: usize,
        }
    };
}
owner!(ObservationOwner);
owner!(MatrixOwner);
owner!(PayloadOwner);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservationView {
    pub owner: ObservationOwner,
    pub offset: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PayloadView {
    pub owner: PayloadOwner,
    pub offset: usize,
}
static NEXT_CONTEXT: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct PulsarContext {
    pub(crate) pulsars: Vec<SourcePulsar>,
    identity: u64,
    observations: Vec<Option<Vec<SourceObservation>>>,
    matrices: Vec<Option<BlasMatrix>>,
    payloads: Vec<Option<Vec<Option<u8>>>>,
    boundary: Option<LifecycleBoundary>,
    /// Logical release observations, not native allocator instrumentation.
    pub(crate) releases: Vec<String>,
}
impl Clone for PulsarContext {
    fn clone(&self) -> Self {
        let identity = NEXT_CONTEXT.fetch_add(1, Ordering::Relaxed);
        let mut out = Self {
            pulsars: self
                .pulsars
                .iter()
                .map(SourcePulsar::snapshot_fields)
                .collect(),
            identity,
            observations: self.observations.clone(),
            matrices: self.matrices.clone(),
            payloads: self.payloads.clone(),
            boundary: self.boundary.clone(),
            releases: self.releases.clone(),
        };
        // Remap each handle, retaining internal shared/interior alias relations.
        for p in &mut out.pulsars {
            if let Some(Some(v)) = &mut p.obsn
                && v.owner.context == self.identity
            {
                v.owner.context = identity;
            }
            for v in [&mut p.covar, &mut p.ToAextraCovar]
                .into_iter()
                .flatten()
                .flatten()
            {
                if v.context == self.identity {
                    v.context = identity;
                }
            }
            for v in p
                .constraint_special
                .iter_mut()
                .chain(p.fitinfo.constraintSpecial.iter_mut())
                .flatten()
                .flatten()
            {
                if v.owner.context == self.identity {
                    v.owner.context = identity;
                }
            }
        }
        out
    }
}
impl PulsarContext {
    pub fn unwritten(count: usize) -> Self {
        Self {
            pulsars: (0..count).map(|_| SourcePulsar::default()).collect(),
            identity: NEXT_CONTEXT.fetch_add(1, Ordering::Relaxed),
            observations: Vec::new(),
            matrices: Vec::new(),
            payloads: Vec::new(),
            boundary: None,
            releases: Vec::new(),
        }
    }
    pub fn pulsars(&self) -> &[SourcePulsar] {
        &self.pulsars
    }
    pub fn boundary(&self) -> Option<&LifecycleBoundary> {
        self.boundary.as_ref()
    }
    pub fn release_trace(&self) -> &[String] {
        &self.releases
    }
    fn run<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, LifecycleBoundary>,
    ) -> Result<T, LifecycleBoundary> {
        if let Some(e) = &self.boundary {
            return Err(e.clone());
        }
        let result = operation(self);
        if let Err(e) = &result {
            self.boundary = Some(e.clone());
        }
        result
    }
    pub fn allocate_observations(
        &mut self,
        count: usize,
    ) -> Result<ObservationView, LifecycleBoundary> {
        self.run(|s| {
            count
                .checked_mul(std::mem::size_of::<SourceObservation>())
                .filter(|n| *n <= isize::MAX as usize)
                .ok_or_else(|| fail("allocate_observations", "capacity", "size_overflow"))?;
            let mut rows = Vec::new();
            rows.try_reserve_exact(count)
                .map_err(|_| fail("allocate_observations", "capacity", "allocation_failure"))?;
            rows.resize_with(count, SourceObservation::default);
            let owner = ObservationOwner {
                context: s.identity,
                index: s.observations.len(),
            };
            s.observations.push(Some(rows));
            Ok(ObservationView { owner, offset: 0 })
        })
    }
    pub fn observations(
        &self,
        view: ObservationView,
    ) -> Result<&[SourceObservation], LifecycleBoundary> {
        if view.owner.context != self.identity {
            return Err(fail("observation_access", "owner", "foreign_owner"));
        }
        self.observations
            .get(view.owner.index)
            .and_then(Option::as_ref)
            .ok_or_else(|| fail("observation_access", "owner", "released_owner"))?
            .get(view.offset..)
            .ok_or_else(|| fail("observation_access", "offset", "capacity"))
    }
    pub fn observations_mut(
        &mut self,
        view: ObservationView,
    ) -> Result<&mut [SourceObservation], LifecycleBoundary> {
        if let Some(e) = &self.boundary {
            return Err(e.clone());
        }
        if view.owner.context != self.identity {
            return Err(fail("observation_access", "owner", "foreign_owner"));
        }
        self.observations
            .get_mut(view.owner.index)
            .and_then(Option::as_mut)
            .ok_or_else(|| fail("observation_access", "owner", "released_owner"))?
            .get_mut(view.offset..)
            .ok_or_else(|| fail("observation_access", "offset", "capacity"))
    }
    pub fn malloc_blas(
        &mut self,
        rows: usize,
        cols: usize,
    ) -> Result<MatrixOwner, LifecycleBoundary> {
        self.run(|s| {
            diagnostic(|d| {
                d.log(
                    b'd',
                    "TKmatrix.C",
                    169,
                    format!(
                        "Allocate {rows} x {cols} double array ({:.3} kb)",
                        rows as f64 * cols as f64 * 8.0 / 1024.0
                    )
                    .as_bytes(),
                )
            })?;
            let matrix = BlasMatrix::malloc_blas(rows, cols)?;
            let owner = MatrixOwner {
                context: s.identity,
                index: s.matrices.len(),
            };
            // Host address/header presentation is projected explicitly: Rust has
            // typed metadata and one dense owner, no fabricated native pointer table.
            diagnostic(|d| {
                d.log(
                    b'd',
                    "TKmatrix.C",
                    184,
                    format!("Allocated owned matrix {}", owner.index).as_bytes(),
                );
                d.log(
                    b'd',
                    "TKmatrix.C",
                    192,
                    format!("Typed metadata rows={rows} cols={cols}").as_bytes(),
                );
                d.log(
                    b'd',
                    "TKmatrix.C",
                    194,
                    format!("Accessible owned cells 0 -> {}", matrix.cells().len()).as_bytes(),
                );
            })?;
            s.matrices.push(Some(matrix));
            Ok(owner)
        })
    }
    pub fn malloc_uinv(&mut self, n: usize) -> Result<MatrixOwner, LifecycleBoundary> {
        self.malloc_blas(n, n)
    }
    pub fn matrix(&self, owner: MatrixOwner) -> Result<&BlasMatrix, LifecycleBoundary> {
        if owner.context != self.identity {
            return Err(fail("matrix_access", "owner", "foreign_owner"));
        }
        self.matrices
            .get(owner.index)
            .and_then(Option::as_ref)
            .ok_or_else(|| fail("matrix_access", "owner", "released_owner"))
    }
    pub fn matrix_mut(&mut self, owner: MatrixOwner) -> Result<&mut BlasMatrix, LifecycleBoundary> {
        if let Some(e) = &self.boundary {
            return Err(e.clone());
        }
        if owner.context != self.identity {
            return Err(fail("matrix_access", "owner", "foreign_owner"));
        }
        self.matrices
            .get_mut(owner.index)
            .and_then(Option::as_mut)
            .ok_or_else(|| fail("matrix_access", "owner", "released_owner"))
    }
    pub fn get_blas_rows(&self, owner: MatrixOwner) -> Result<usize, LifecycleBoundary> {
        Ok(self.matrix(owner)?.get_blas_rows())
    }
    pub fn get_blas_cols(&self, owner: MatrixOwner) -> Result<usize, LifecycleBoundary> {
        Ok(self.matrix(owner)?.get_blas_cols())
    }
    pub fn allocate_payload(
        &mut self,
        bytes: Vec<Option<u8>>,
    ) -> Result<PayloadView, LifecycleBoundary> {
        self.run(|s| {
            let owner = PayloadOwner {
                context: s.identity,
                index: s.payloads.len(),
            };
            s.payloads.push(Some(bytes));
            Ok(PayloadView { owner, offset: 0 })
        })
    }
    pub fn payload(&self, view: PayloadView) -> Result<&[Option<u8>], LifecycleBoundary> {
        if view.owner.context != self.identity {
            return Err(fail("payload_access", "owner", "foreign_owner"));
        }
        self.payloads
            .get(view.owner.index)
            .and_then(Option::as_ref)
            .ok_or_else(|| fail("payload_access", "owner", "released_owner"))?
            .get(view.offset..)
            .ok_or_else(|| fail("payload_access", "offset", "capacity"))
    }
    pub fn free_observations(&mut self, view: ObservationView) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            if view.offset != 0 {
                return Err(fail("free_observations", "offset", "interior_owner"));
            }
            s.observations(view)?;
            s.observations[view.owner.index] = None;
            s.releases
                .push(format!("observations:{}", view.owner.index));
            Ok(())
        })
    }
    pub fn free_blas(&mut self, owner: MatrixOwner) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            let m = s.matrix(owner)?;
            diagnostic(|d| {
                d.log(
                    b'd',
                    "TKmatrix.C",
                    203,
                    format!("free owned matrix {}", owner.index).as_bytes(),
                );
                d.log(
                    b'd',
                    "TKmatrix.C",
                    204,
                    format!("m was {} x {}", m.get_blas_rows(), m.get_blas_cols()).as_bytes(),
                );
            })?;
            s.matrices[owner.index] = None;
            s.releases.push(format!("matrix:{}", owner.index));
            diagnostic(|d| {
                d.log(
                    b'd',
                    "TKmatrix.C",
                    208,
                    format!("free owned metadata {}", owner.index).as_bytes(),
                );
                d.log(b'd', "TKmatrix.C", 211, b"leaving free_blas");
            })?;
            Ok(())
        })
    }
    pub fn free_uinv(&mut self, owner: MatrixOwner) -> Result<(), LifecycleBoundary> {
        self.free_blas(owner)
    }
    pub fn free_payload(&mut self, view: PayloadView) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            if view.offset != 0 {
                return Err(fail("free_payload", "offset", "interior_owner"));
            }
            s.payload(view)?;
            s.payloads[view.owner.index] = None;
            s.releases.push(format!("payload:{}", view.owner.index));
            Ok(())
        })
    }
    /// Returns actual retired buffers, not reconstructed copies. Embedded links
    /// remain in the current object. Dropping an unretained bundle reclaims the
    /// source's unreachable leaks; retained bundles own their original allocations.
    pub fn allocate_memory(
        &mut self,
        p: usize,
        realloc: i32,
    ) -> Result<ParameterStore, LifecycleBoundary> {
        self.run(|s| {
            let psr = s
                .pulsars
                .get_mut(p)
                .ok_or_else(|| fail("allocateMemory", "pulsar", "capacity"))?;
            let mut replacement = crate::initialise::allocate_memory(realloc);
            for (new, old) in replacement.slots.iter_mut().zip(&psr.param.slots) {
                new.link_to = old.link_to;
                new.link_from = old.link_from;
            }
            Ok(std::mem::replace(&mut psr.param, replacement))
        })
    }
    pub fn destroy_memory(&mut self, p: usize) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            let psr = s
                .pulsars
                .get_mut(p)
                .ok_or_else(|| fail("destroyMemory", "pulsar", "capacity"))?;
            for (i, param) in psr.param.slots.iter_mut().enumerate() {
                let count = read(&param.a_size, "destroyMemory", &format!("param[{i}].aSize"))?;
                if let Some(e) = &param.boundary {
                    return Err(fail("destroyMemory", format!("param[{i}]"), e.kind));
                }
                for j in 0..count {
                    let cell = param.cells.get_mut(j as usize).ok_or_else(|| {
                        fail(
                            "destroyMemory",
                            format!("param[{i}].label[{j}]"),
                            "capacity",
                        )
                    })?;
                    cell.label.clear();
                    s.releases.push(format!("param[{i}].label[{j}]"));
                    cell.shortlabel.clear();
                    s.releases.push(format!("param[{i}].shortlabel[{j}]"));
                }
                for field in [
                    "val",
                    "err",
                    "prefit",
                    "prefitErr",
                    "fitFlag",
                    "paramSet",
                    "label",
                    "shortlabel",
                ] {
                    s.releases.push(format!("param[{i}].{field}"));
                }
                // The owned cell allocation groups buffers. Drop also reclaims label
                // tails beyond a shortened aSize (O's accepted ownership departure).
                param.cells.clear();
                param.boundary = Some(crate::tempo2::ParameterBoundary {
                    cell: 0,
                    field: "owner",
                    kind: "released_owner",
                });
            }
            Ok(())
        })
    }
    pub fn destroy_one(&mut self, p: usize) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            let psr = s
                .pulsars
                .get(p)
                .ok_or_else(|| fail("destroyOne", "pulsar", "capacity"))?;
            if let Some(view) = read(&psr.obsn, "destroyOne", "obsn")? {
                s.free_observations(view)?;
            }
            let matrix = read(&s.pulsars[p].covar, "destroyOne", "covar")?
                .ok_or_else(|| fail("destroyOne", "covar", "null_owner"))?;
            s.free_blas(matrix)?;
            for i in 0..MAX_PARAMS {
                if let Some(view) = read(
                    &s.pulsars[p].constraint_special[i],
                    "destroyOne",
                    &format!("constraint_special[{i}]"),
                )? {
                    s.free_payload(view)?;
                }
            }
            s.destroy_memory(p)
        })
    }
}

// Source-ordered nonparameter writes from initialise.C:106-258.
fn initialise_fields(p: &mut SourcePulsar, no_warnings: i32) {
    write_literal(&mut p.eopc04_file[..], b"/earth/eopc04_IAU2000.62-now");
    write_literal(&mut p.filterStr[..], b"");
    write_literal(&mut p.passStr[..], b"");
    write_literal(&mut p.fitFunc[..], b"default");
    write_literal(&mut p.whiteNoiseModelFile[..], b"NULL");
    p.simflag = Some(0);
    p.nits = Some(1);
    p.clockFromOverride[0] = Some(0);
    p.nCompanion = Some(0);
    p.bootStrap = Some(0);
    p.units = Some(1);
    p.setUnits = Some(0);
    p.ne_sw = Some(4.0);
    p.nWhite = Some(0);
    p.nQuad = Some(0);
    p.ifuncN = Some(0);
    p.orbifuncN = Some(0);
    p.clkOffsN = Some(0);
    p.nTelDX = Some(0);
    p.nTelDY = Some(0);
    p.nTelDZ = Some(0);
    p.quad_ifuncN_p = Some(0);
    p.quad_ifuncN_c = Some(0);
    p.quad_ifunc_geom_p = Some(0.0);
    p.quad_ifunc_geom_c = Some(0.0);
    p.cgw_mc = Some(0.0);
    p.timeEphemeris = Some(1);
    p.useCalceph = Some(0);
    p.dilateFreq = Some(1);
    p.planetShapiro = Some(1);
    p.correctTroposphere = Some(1);
    p.t2cMethod = Some(1);
    p.fixedFormat = Some(0);
    p.nStorePrecision = Some(0);
    write_literal(&mut p.fjumpID[..], b"");
    p.fdjump_pint_format = Some(0);
    write_literal(&mut p.deleteFileName[..], b"NONE");
    write_literal(&mut p.tzrsite[..], b"NULL");
    p.refphs = Some(0);
    p.ToAextraCovar = Some(None);
    p.dmoffsDMnum = Some(0);
    p.dmoffsCMnum = Some(0);
    p.calcShapiro = Some(1);
    p.dmOffset = Some(0.0);
    p.ipm = Some(1);
    p.swm = Some(0);
    p.nPhaseJump = Some(0);
    p.eclCoord = Some(0);
    p.noWarnings = Some(no_warnings);
    p.fitMode = Some(0);
    p.robust = Some(0);
    p.rescaleErrChisq = Some(1);
    write_literal(&mut p.name[..], b"NOT SET");
    write_literal(&mut p.binaryModel[..], b"NONE");
    p.nJumps = Some(0);
    p.nfdJumps = Some(0);
    p.nToffset = Some(0);
    p.ndmx = Some(0);
    p.nconstraints = Some(0);
    p.constraint_efactor = Some(1e15);
    p.auto_constraints = Some(0);
    p.jboFormat = Some(0);
    p.dmOffset = Some(0.0);
    for i in 0..crate::pulsar::MAX_JUMPS {
        p.jumpVal[i] = Some(0.0);
        p.jumpSAT[i] = Some(0);
        p.jumpScaled[i] = Some(0);
        p.jumpValErr[i] = Some(0.0);
    }
    for i in 0..crate::pulsar::MAX_JUMPS {
        p.fdjumpVal[i] = Some(0.0);
        p.fdjumpIdx[i] = Some(0);
        p.fdjumpValErr[i] = Some(0.0);
    }
    p.fdjump_log = Some(1);
    p.nT2efac = Some(0);
    p.nT2equad = Some(0);
    p.T2globalEfac = Some(1.0);
    p.nSx = Some(0);
    p.nTNEF = Some(0);
    p.nTNEQ = Some(0);
    p.nTNECORR = Some(0);
    p.nTNSECORR = Some(0);
    p.TNGlobalEF = Some(0.0);
    p.TNGlobalEQ = Some(0.0);
    p.nTNSQ = Some(0);
    p.TNRedAmp = Some(0.0);
    p.TNRedGam = Some(0.0);
    p.TNRedAmp2 = Some(0.0);
    p.TNRedGam2 = Some(0.0);
    p.TNRedC = Some(0);
    p.TNRedFLow = Some(0.0);
    p.TNRedCorner = Some(0.0);
    p.TNRed_log_freqs = Some(0);
    p.TNRed_log_factor = Some(2.0);
    p.TNDMAmp = Some(0.0);
    p.TNDMGam = Some(0.0);
    p.TNDMC = Some(0);
    p.SWGPAmp = Some(0.0);
    p.SWGPGam = Some(0.0);
    p.SWGPC = Some(0);
    p.TNDM_log_freqs = Some(0);
    p.TNDM_log_factor = Some(2.0);
    p.TNChromAmp = Some(0.0);
    p.TNChromGam = Some(0.0);
    p.TNChromC = Some(0);
    p.TNChromIdx = Some(0.0);
    p.TNChrom_log_freqs = Some(0);
    p.TNChrom_log_factor = Some(2.0);
    p.TNBandDMAmp = Some(0.0);
    p.TNBandDMGam = Some(0.0);
    p.TNBandDMC = Some(0);
    p.nTNBandNoise = Some(0);
    p.nTNGroupNoise = Some(0);
    p.TNsubtractDM = Some(0);
    p.TNsubtractRed = Some(0);
    p.TNsubtractChrom = Some(0);
    p.TNsubtractSWGP = Some(0);
    p.TNsubtractPoly = Some(crate::t2fit_nestlike::t2EncodeTNsubtractPoly(1, 2, 2, 2));
    p.TN_QpRatio = Some(0.0);
    p.TN_QpPeriod = Some(0.0);
    p.TN_QpSig = Some(0.0);
    p.TN_QpLam = Some(0.0);
    p.AverageResiduals = Some(0);
    p.useTNOrth = Some(0);
    p.nDMEvents = Some(0);
    p.nTNShapeletEvents = Some(0);
    p.sorted = Some(0);
    p.detL = Some(0.0);
}
fn write_literal(target: &mut [Option<u8>], bytes: &[u8]) {
    for (dst, byte) in target
        .iter_mut()
        .zip(bytes.iter().copied().chain(std::iter::once(0)))
    {
        *dst = Some(byte);
    }
}

impl PulsarContext {
    /// fullSetup is ignored by the source, including negative values.
    pub fn initialise_one(
        &mut self,
        p: usize,
        max_obsn: i32,
        no_warnings: i32,
        _full_setup: i32,
    ) -> Result<ParameterStore, LifecycleBoundary> {
        self.run(|s| {
            s.pulsars
                .get_mut(p)
                .ok_or_else(|| fail("initialiseOne", "pulsar", "capacity"))?
                .nobs = Some(0);
            let count = usize::try_from(max_obsn)
                .map_err(|_| fail("initialiseOne", "MAX_OBSN", "invalid_capacity"))?;
            let view = s.allocate_observations(count)?;
            s.pulsars[p].obsn = Some(Some(view));
            let matrix = s.malloc_blas(crate::pulsar::MAX_FIT, crate::pulsar::MAX_FIT)?;
            s.pulsars[p].covar = Some(Some(matrix));
            initialise_fields(&mut s.pulsars[p], no_warnings);
            let retired = s.allocate_memory(p, 0)?;
            let psr = &mut s.pulsars[p];
            crate::initialise::label_parameters(&mut psr.param, &mut psr.dm_series_type);
            psr.constraint_special.fill(Some(None));
            Ok(retired)
        })
    }
    /// Source wrapper's signed range: zero and negative MAX_PSR do no work.
    pub fn initialise(
        &mut self,
        max_psr: i32,
        max_obsn: i32,
        no_warnings: i32,
    ) -> Result<Vec<ParameterStore>, LifecycleBoundary> {
        self.run(|s| {
            diagnostic(|d| {
                d.version("initialise.C", "initialise()");
                d.log(b'd', "initialise.C", 49, b"Initialise");
            })?;
            let mut retired = Vec::new();
            for p in 0..max_psr {
                retired.push(s.initialise_one(p as usize, max_obsn, no_warnings, 1)?);
            }
            Ok(retired)
        })
    }
    pub fn copy_psr(&mut self, source: usize, destination: usize) -> Result<(), LifecycleBoundary> {
        self.run(|s| {
            const OP: &str = "copyPSR";
            if source == destination {
                return Err(fail(OP, "name", "overlapping_strcpy"));
            }
            if source >= s.pulsars.len() || destination >= s.pulsars.len() {
                return Err(fail(OP, "pulsar", "capacity"));
            }
            macro_rules! copy {
                ($field:ident) => {{
                    let v = read(&s.pulsars[source].$field, OP, stringify!($field))?;
                    s.pulsars[destination].$field = Some(v);
                }};
                ($field:ident,$i:expr) => {{
                    let i = $i;
                    let field = format!("{}[{i}]", stringify!($field));
                    let v = read(
                        s.pulsars[source]
                            .$field
                            .get(i)
                            .ok_or_else(|| fail(OP, &field, "source_capacity"))?,
                        OP,
                        &field,
                    )?;
                    *s.pulsars[destination]
                        .$field
                        .get_mut(i)
                        .ok_or_else(|| fail(OP, &field, "destination_capacity"))? = Some(v);
                }};
            }
            macro_rules! string {
                ($field:ident) => {{
                    let (a, b) = two_mut(&mut s.pulsars, source, destination);
                    copy_string(&a.$field[..], &mut b.$field[..], stringify!($field))?;
                }};
                ($field:ident,$i:expr) => {{
                    let i = $i;
                    let field = format!("{}[{i}]", stringify!($field));
                    let (a, b) = two_mut(&mut s.pulsars, source, destination);
                    copy_string(
                        a.$field
                            .get(i)
                            .ok_or_else(|| fail(OP, &field, "source_capacity"))?,
                        b.$field
                            .get_mut(i)
                            .ok_or_else(|| fail(OP, &field, "destination_capacity"))?,
                        &field,
                    )?;
                }};
            }
            string!(name);
            string!(binaryModel);
            copy!(fitMode);
            copy!(nobs);
            copy!(nits);
            for i in 0..read(&s.pulsars[source].nobs, OP, "nobs")? {
                let src = read(&s.pulsars[source].obsn, OP, "source.obsn")?
                    .ok_or_else(|| fail(OP, "source.obsn", "null_owner"))?;
                // One current record per forward iteration: overlapping source reads
                // must observe the writes made by previous iterations.
                let record = s
                    .observations(src)?
                    .get(i as usize)
                    .ok_or_else(|| fail(OP, format!("source.obsn[{i}]"), "capacity"))?
                    .clone();
                let dst = read(&s.pulsars[destination].obsn, OP, "destination.obsn")?
                    .ok_or_else(|| fail(OP, "destination.obsn", "null_owner"))?;
                *s.observations_mut(dst)?
                    .get_mut(i as usize)
                    .ok_or_else(|| fail(OP, format!("destination.obsn[{i}]"), "capacity"))? =
                    record;
            }
            for i in 0..3 {
                copy!(posPulsar, i);
                copy!(velPulsar, i);
            }
            copy!(eclCoord);
            copy!(nJumps);
            for i in 0..=read(&s.pulsars[destination].nJumps, OP, "nJumps")? {
                let i = i as usize;
                copy!(jumpVal, i);
                copy!(fitJump, i);
                copy!(jumpValErr, i);
                string!(jumpStr, i);
            }
            // Native defect retained: no nfdJumps assignment; destination sets the bound.
            for i in 0..=read(&s.pulsars[destination].nfdJumps, OP, "nfdJumps")? {
                let i = i as usize;
                copy!(fdjumpVal, i);
                copy!(fitfdJump, i);
                copy!(fdjumpValErr, i);
                string!(fdjumpStr, i);
            }
            copy!(nFit);
            copy!(fitMode);
            copy!(offset);
            copy!(calcShapiro);
            copy!(units);
            copy!(tempo1);
            copy!(dilateFreq);
            copy!(timeEphemeris);
            copy!(t2cMethod);
            copy!(correctTroposphere);
            string!(clock);
            string!(clockFromOverride);
            string!(JPL_EPHEMERIS);
            string!(ephemeris);
            copy!(dm_series_type);
            Ok(())
        })
    }
}
fn two_mut<T>(rows: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    if a < b {
        let (left, right) = rows.split_at_mut(b);
        (&mut left[a], &mut right[0])
    } else {
        let (left, right) = rows.split_at_mut(a);
        (&mut right[0], &mut left[b])
    }
}
fn copy_string(
    source: &[Option<u8>],
    destination: &mut [Option<u8>],
    field: &str,
) -> Result<(), LifecycleBoundary> {
    for i in 0.. {
        let byte = read(
            source
                .get(i)
                .ok_or_else(|| fail("copyPSR", format!("{field}[{i}]"), "missing_terminator"))?,
            "copyPSR",
            &format!("{field}[{i}]"),
        )?;
        *destination
            .get_mut(i)
            .ok_or_else(|| fail("copyPSR", format!("{field}[{i}]"), "destination_capacity"))? =
            Some(byte);
        if byte == 0 {
            return Ok(());
        }
    }
    unreachable!()
}

// Use the existing process diagnostic owner. Lifecycle boundaries remain local
// to their context; no second warning/cache state or terminal clock fault is made.
fn diagnostic(
    f: impl FnOnce(&mut crate::clock_diagnostics::Diagnostics),
) -> Result<(), LifecycleBoundary> {
    crate::clkcorr::lifecycle_diagnostics(f)
        .map_err(|e| fail("diagnostics", e.field, "session_boundary"))
}
