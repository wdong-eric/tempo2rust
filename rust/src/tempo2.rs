//! TEMPO2 tempo2.h::label, param_label and parameter (not C ABI layout).
//! Source array-valued storage; lib.rs::Parameter is a derived scalar output view.
#![allow(non_camel_case_types)]
use crate::PRECISION_BITS;
use rug::Float;

pub type param_label = i32;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Label {
    param_raj = 0,
    param_decj = 1,
    param_f = 2,
    param_pepoch = 3,
    param_posepoch = 4,
    param_dmepoch = 5,
    param_dm = 6,
    param_cm = 7,
    param_pmra = 8,
    param_pmdec = 9,
    param_pmra2 = 10,
    param_pmdec2 = 11,
    param_px = 12,
    param_sini = 13,
    param_pb = 14,
    param_fb = 15,
    param_t0 = 16,
    param_a1 = 17,
    param_om = 18,
    param_pmrv = 19,
    param_ecc = 20,
    param_edot = 21,
    param_e2dot = 22,
    param_xpbdot = 23,
    param_pbdot = 24,
    param_pb2dot = 25,
    param_a1dot = 26,
    param_a2dot = 27,
    param_omdot = 28,
    param_om2dot = 29,
    param_orbpx = 30,
    param_tasc = 31,
    param_eps1 = 32,
    param_eps2 = 33,
    param_m2 = 34,
    param_gamma = 35,
    param_mtot = 36,
    param_glep = 37,
    param_glph = 38,
    param_glf0 = 39,
    param_glf1 = 40,
    param_glf2 = 41,
    param_expep = 42,
    param_expph = 43,
    param_exptau = 44,
    param_expindex = 45,
    param_gausep = 46,
    param_gausamp = 47,
    param_gaussig = 48,
    param_gausindex = 49,
    param_glf0d = 50,
    param_gltd = 51,
    param_start = 52,
    param_finish = 53,
    param_track = 54,
    param_bp = 55,
    param_bpp = 56,
    param_tzrmjd = 57,
    param_tzrfrq = 58,
    param_fddc = 59,
    param_fddi = 60,
    param_fd = 61,
    param_dr = 62,
    param_dtheta = 63,
    param_tspan = 64,
    param_bpjep = 65,
    param_bpjph = 66,
    param_bpja1 = 67,
    param_bpjec = 68,
    param_bpjom = 69,
    param_bpjpb = 70,
    param_wave_om = 71,
    param_kom = 72,
    param_kin = 73,
    param_shapmax = 74,
    param_dth = 75,
    param_a0 = 76,
    param_b0 = 77,
    param_xomdot = 78,
    param_afac = 79,
    param_eps1dot = 80,
    param_eps2dot = 81,
    param_tres = 82,
    param_trestn = 83,
    param_wave_dm = 84,
    param_waveepoch_dm = 85,
    param_dshk = 86,
    param_ephver = 87,
    param_daop = 88,
    param_iperharm = 89,
    param_dmassplanet = 90,
    param_dphaseplanet = 91,
    param_waveepoch = 92,
    param_ifunc = 93,
    param_clk_offs = 94,
    param_dmx = 95,
    param_dmxr1 = 96,
    param_dmxr2 = 97,
    param_dmmodel = 98,
    param_gwsingle = 99,
    param_cgw = 100,
    param_quad_om = 101,
    param_h3 = 102,
    param_h4 = 103,
    param_nharm = 104,
    param_stig = 105,
    param_telx = 106,
    param_tely = 107,
    param_telz = 108,
    param_telEpoch = 109,
    param_quad_ifunc_p = 110,
    param_quad_ifunc_c = 111,
    param_tel_dx = 112,
    param_tel_dy = 113,
    param_tel_dz = 114,
    param_tel_vx = 115,
    param_tel_vy = 116,
    param_tel_vz = 117,
    param_tel_x0 = 118,
    param_tel_y0 = 119,
    param_tel_z0 = 120,
    param_gwm_amp = 121,
    param_gwcs_amp = 122,
    param_gwecc = 123,
    param_gwb_amp = 124,
    param_dm_sin1yr = 125,
    param_dm_cos1yr = 126,
    param_brake = 127,
    param_stateSwitchT = 128,
    param_df1 = 129,
    param_red_sin = 130,
    param_red_cos = 131,
    param_jitter = 132,
    param_red_dm_sin = 133,
    param_red_dm_cos = 134,
    param_red_chrom_sin = 135,
    param_red_chrom_cos = 136,
    param_swgp_sin = 137,
    param_swgp_cos = 138,
    param_band_red_sin = 139,
    param_band_red_cos = 140,
    param_sx = 141,
    param_sxr1 = 142,
    param_sxr2 = 143,
    param_sxer = 144,
    param_group_red_sin = 145,
    param_group_red_cos = 146,
    param_ne_sw = 147,
    param_shapevent = 148,
    param_orbifunc = 149,
    param_gltd2 = 150,
    param_gltd3 = 151,
    param_glf0d2 = 152,
    param_glf0d3 = 153,
    param_ne_sw_sin = 154,
    param_ne_sw_ifunc = 155,
    param_chromx = 156,
    param_chromxr1 = 157,
    param_chromxr2 = 158,
    param_nudot_epoch = 159,
    param_nudot_amp = 160,
    param_nudot_transition_time = 161,
    param_LAST = 162,
    param_ZERO = 163,
    param_JUMP = 164,
    param_FDJUMP = 165,
}

pub const PARAMETER_NAMES: [&str; 166] = [
    "param_raj",
    "param_decj",
    "param_f",
    "param_pepoch",
    "param_posepoch",
    "param_dmepoch",
    "param_dm",
    "param_cm",
    "param_pmra",
    "param_pmdec",
    "param_pmra2",
    "param_pmdec2",
    "param_px",
    "param_sini",
    "param_pb",
    "param_fb",
    "param_t0",
    "param_a1",
    "param_om",
    "param_pmrv",
    "param_ecc",
    "param_edot",
    "param_e2dot",
    "param_xpbdot",
    "param_pbdot",
    "param_pb2dot",
    "param_a1dot",
    "param_a2dot",
    "param_omdot",
    "param_om2dot",
    "param_orbpx",
    "param_tasc",
    "param_eps1",
    "param_eps2",
    "param_m2",
    "param_gamma",
    "param_mtot",
    "param_glep",
    "param_glph",
    "param_glf0",
    "param_glf1",
    "param_glf2",
    "param_expep",
    "param_expph",
    "param_exptau",
    "param_expindex",
    "param_gausep",
    "param_gausamp",
    "param_gaussig",
    "param_gausindex",
    "param_glf0d",
    "param_gltd",
    "param_start",
    "param_finish",
    "param_track",
    "param_bp",
    "param_bpp",
    "param_tzrmjd",
    "param_tzrfrq",
    "param_fddc",
    "param_fddi",
    "param_fd",
    "param_dr",
    "param_dtheta",
    "param_tspan",
    "param_bpjep",
    "param_bpjph",
    "param_bpja1",
    "param_bpjec",
    "param_bpjom",
    "param_bpjpb",
    "param_wave_om",
    "param_kom",
    "param_kin",
    "param_shapmax",
    "param_dth",
    "param_a0",
    "param_b0",
    "param_xomdot",
    "param_afac",
    "param_eps1dot",
    "param_eps2dot",
    "param_tres",
    "param_trestn",
    "param_wave_dm",
    "param_waveepoch_dm",
    "param_dshk",
    "param_ephver",
    "param_daop",
    "param_iperharm",
    "param_dmassplanet",
    "param_dphaseplanet",
    "param_waveepoch",
    "param_ifunc",
    "param_clk_offs",
    "param_dmx",
    "param_dmxr1",
    "param_dmxr2",
    "param_dmmodel",
    "param_gwsingle",
    "param_cgw",
    "param_quad_om",
    "param_h3",
    "param_h4",
    "param_nharm",
    "param_stig",
    "param_telx",
    "param_tely",
    "param_telz",
    "param_telEpoch",
    "param_quad_ifunc_p",
    "param_quad_ifunc_c",
    "param_tel_dx",
    "param_tel_dy",
    "param_tel_dz",
    "param_tel_vx",
    "param_tel_vy",
    "param_tel_vz",
    "param_tel_x0",
    "param_tel_y0",
    "param_tel_z0",
    "param_gwm_amp",
    "param_gwcs_amp",
    "param_gwecc",
    "param_gwb_amp",
    "param_dm_sin1yr",
    "param_dm_cos1yr",
    "param_brake",
    "param_stateSwitchT",
    "param_df1",
    "param_red_sin",
    "param_red_cos",
    "param_jitter",
    "param_red_dm_sin",
    "param_red_dm_cos",
    "param_red_chrom_sin",
    "param_red_chrom_cos",
    "param_swgp_sin",
    "param_swgp_cos",
    "param_band_red_sin",
    "param_band_red_cos",
    "param_sx",
    "param_sxr1",
    "param_sxr2",
    "param_sxer",
    "param_group_red_sin",
    "param_group_red_cos",
    "param_ne_sw",
    "param_shapevent",
    "param_orbifunc",
    "param_gltd2",
    "param_gltd3",
    "param_glf0d2",
    "param_glf0d3",
    "param_ne_sw_sin",
    "param_ne_sw_ifunc",
    "param_chromx",
    "param_chromxr1",
    "param_chromxr2",
    "param_nudot_epoch",
    "param_nudot_amp",
    "param_nudot_transition_time",
    "param_LAST",
    "param_ZERO",
    "param_JUMP",
    "param_FDJUMP",
];
pub const PARAMETER_LABELS: [Label; 166] = [
    Label::param_raj,
    Label::param_decj,
    Label::param_f,
    Label::param_pepoch,
    Label::param_posepoch,
    Label::param_dmepoch,
    Label::param_dm,
    Label::param_cm,
    Label::param_pmra,
    Label::param_pmdec,
    Label::param_pmra2,
    Label::param_pmdec2,
    Label::param_px,
    Label::param_sini,
    Label::param_pb,
    Label::param_fb,
    Label::param_t0,
    Label::param_a1,
    Label::param_om,
    Label::param_pmrv,
    Label::param_ecc,
    Label::param_edot,
    Label::param_e2dot,
    Label::param_xpbdot,
    Label::param_pbdot,
    Label::param_pb2dot,
    Label::param_a1dot,
    Label::param_a2dot,
    Label::param_omdot,
    Label::param_om2dot,
    Label::param_orbpx,
    Label::param_tasc,
    Label::param_eps1,
    Label::param_eps2,
    Label::param_m2,
    Label::param_gamma,
    Label::param_mtot,
    Label::param_glep,
    Label::param_glph,
    Label::param_glf0,
    Label::param_glf1,
    Label::param_glf2,
    Label::param_expep,
    Label::param_expph,
    Label::param_exptau,
    Label::param_expindex,
    Label::param_gausep,
    Label::param_gausamp,
    Label::param_gaussig,
    Label::param_gausindex,
    Label::param_glf0d,
    Label::param_gltd,
    Label::param_start,
    Label::param_finish,
    Label::param_track,
    Label::param_bp,
    Label::param_bpp,
    Label::param_tzrmjd,
    Label::param_tzrfrq,
    Label::param_fddc,
    Label::param_fddi,
    Label::param_fd,
    Label::param_dr,
    Label::param_dtheta,
    Label::param_tspan,
    Label::param_bpjep,
    Label::param_bpjph,
    Label::param_bpja1,
    Label::param_bpjec,
    Label::param_bpjom,
    Label::param_bpjpb,
    Label::param_wave_om,
    Label::param_kom,
    Label::param_kin,
    Label::param_shapmax,
    Label::param_dth,
    Label::param_a0,
    Label::param_b0,
    Label::param_xomdot,
    Label::param_afac,
    Label::param_eps1dot,
    Label::param_eps2dot,
    Label::param_tres,
    Label::param_trestn,
    Label::param_wave_dm,
    Label::param_waveepoch_dm,
    Label::param_dshk,
    Label::param_ephver,
    Label::param_daop,
    Label::param_iperharm,
    Label::param_dmassplanet,
    Label::param_dphaseplanet,
    Label::param_waveepoch,
    Label::param_ifunc,
    Label::param_clk_offs,
    Label::param_dmx,
    Label::param_dmxr1,
    Label::param_dmxr2,
    Label::param_dmmodel,
    Label::param_gwsingle,
    Label::param_cgw,
    Label::param_quad_om,
    Label::param_h3,
    Label::param_h4,
    Label::param_nharm,
    Label::param_stig,
    Label::param_telx,
    Label::param_tely,
    Label::param_telz,
    Label::param_telEpoch,
    Label::param_quad_ifunc_p,
    Label::param_quad_ifunc_c,
    Label::param_tel_dx,
    Label::param_tel_dy,
    Label::param_tel_dz,
    Label::param_tel_vx,
    Label::param_tel_vy,
    Label::param_tel_vz,
    Label::param_tel_x0,
    Label::param_tel_y0,
    Label::param_tel_z0,
    Label::param_gwm_amp,
    Label::param_gwcs_amp,
    Label::param_gwecc,
    Label::param_gwb_amp,
    Label::param_dm_sin1yr,
    Label::param_dm_cos1yr,
    Label::param_brake,
    Label::param_stateSwitchT,
    Label::param_df1,
    Label::param_red_sin,
    Label::param_red_cos,
    Label::param_jitter,
    Label::param_red_dm_sin,
    Label::param_red_dm_cos,
    Label::param_red_chrom_sin,
    Label::param_red_chrom_cos,
    Label::param_swgp_sin,
    Label::param_swgp_cos,
    Label::param_band_red_sin,
    Label::param_band_red_cos,
    Label::param_sx,
    Label::param_sxr1,
    Label::param_sxr2,
    Label::param_sxer,
    Label::param_group_red_sin,
    Label::param_group_red_cos,
    Label::param_ne_sw,
    Label::param_shapevent,
    Label::param_orbifunc,
    Label::param_gltd2,
    Label::param_gltd3,
    Label::param_glf0d2,
    Label::param_glf0d3,
    Label::param_ne_sw_sin,
    Label::param_ne_sw_ifunc,
    Label::param_chromx,
    Label::param_chromxr1,
    Label::param_chromxr2,
    Label::param_nudot_epoch,
    Label::param_nudot_amp,
    Label::param_nudot_transition_time,
    Label::param_LAST,
    Label::param_ZERO,
    Label::param_JUMP,
    Label::param_FDJUMP,
];
// Owned source storage.
pub const MAX_PARAMS: usize = 2000;
pub const MAX_FREQ_DERIVATIVES: usize = 13;
pub const MAX_DM_DERIVATIVES: usize = 10;
pub const MAX_COMPANIONS: usize = 4;
pub const MAX_BPJ_JUMPS: usize = 5;
pub const MAX_DMX: usize = 512;
pub const MAX_SX: usize = 512;
pub const MAX_TEL_DX: usize = 500;
pub const MAX_TEL_DY: usize = 500;
pub const MAX_TEL_DZ: usize = 500;

/// An operation-local undefined-source boundary; completed writes are retained.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ParameterBoundary {
    pub cell: usize,
    pub field: &'static str,
    pub kind: &'static str,
}

impl std::fmt::Display for ParameterBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "parameter cell {} {}: {}",
            self.cell, self.field, self.kind
        )
    }
}
impl std::error::Error for ParameterBoundary {}

/// One backing cell. None means source-unwritten, never a numeric sentinel.
#[derive(Clone, Debug)]
pub struct ParameterCell {
    pub(crate) label: Vec<u8>,
    pub(crate) shortlabel: Vec<u8>,
    pub(crate) val: Float,
    pub(crate) err: Float,
    pub(crate) fit_flag: i32,
    pub(crate) param_set: i32,
    pub(crate) prefit: Option<Float>,
    pub(crate) prefit_err: Option<Float>,
}
impl ParameterCell {
    pub fn label_bytes(&self) -> &[u8] {
        &self.label
    }
    pub fn shortlabel_bytes(&self) -> &[u8] {
        &self.shortlabel
    }
    pub fn value(&self) -> &Float {
        &self.val
    }
    pub fn error(&self) -> &Float {
        &self.err
    }
    pub fn fit_flag(&self) -> i32 {
        self.fit_flag
    }
    pub fn param_set(&self) -> i32 {
        self.param_set
    }
    pub fn prefit(&self) -> Option<&Float> {
        self.prefit.as_ref()
    }
    pub fn prefit_error(&self) -> Option<&Float> {
        self.prefit_err.as_ref()
    }
}

/// Source `parameter`, with independent signed logical size and owned backing.
/// Clone clones the entire owner; it is deliberately different from copyParam.
#[derive(Clone, Debug)]
pub struct SourceParameter {
    pub(crate) a_size: i32,
    pub(crate) cells: Vec<ParameterCell>,
    pub(crate) link_from: [Option<i32>; 5],
    pub(crate) link_to: [Option<i32>; 5],
    pub(crate) n_link_to: i32,
    pub(crate) n_link_from: i32,
    pub(crate) boundary: Option<ParameterBoundary>,
}
impl SourceParameter {
    pub(crate) fn allocated(capacity: usize, label_bytes: usize) -> Self {
        Self {
            a_size: capacity as i32,
            cells: (0..capacity)
                .map(|_| ParameterCell {
                    label: vec![0; label_bytes],
                    shortlabel: vec![0; label_bytes],
                    val: Float::with_val(PRECISION_BITS, 0),
                    err: Float::with_val(PRECISION_BITS, 0),
                    fit_flag: 0,
                    param_set: 0,
                    prefit: None,
                    prefit_err: None,
                })
                .collect(),
            link_from: [None; 5],
            link_to: [None; 5],
            n_link_to: 0,
            n_link_from: 0,
            boundary: None,
        }
    }
    pub fn logical_size(&self) -> i32 {
        self.a_size
    }
    pub fn backing_size(&self) -> usize {
        self.cells.len()
    }
    pub fn link_from(&self) -> &[Option<i32>; 5] {
        &self.link_from
    }
    pub fn link_to(&self) -> &[Option<i32>; 5] {
        &self.link_to
    }
    pub fn n_link_from(&self) -> i32 {
        self.n_link_from
    }
    pub fn n_link_to(&self) -> i32 {
        self.n_link_to
    }
    pub fn boundary(&self) -> Option<&ParameterBoundary> {
        self.boundary.as_ref()
    }
    pub fn cell(&self, i: usize) -> Result<&ParameterCell, ParameterBoundary> {
        if let Some(error) = &self.boundary {
            return Err(error.clone());
        }
        self.cells.get(i).ok_or(ParameterBoundary {
            cell: i,
            field: "cell",
            kind: "missing_cell",
        })
    }
    /// Defined prefix/backing only. After a boundary these cells are diagnostic data,
    /// unavailable for scientific reads through cell() or source copy.
    pub fn diagnostic_cells(&self) -> &[ParameterCell] {
        &self.cells
    }
}

/// Source-indexed parameter owner. Public access is read-only; it is not a PAR map.
#[derive(Clone, Debug)]
pub struct ParameterStore {
    pub(crate) slots: Vec<SourceParameter>,
}
impl ParameterStore {
    pub fn slots(&self) -> &[SourceParameter] {
        &self.slots
    }
    pub fn parameter(&self, label: Label) -> &SourceParameter {
        &self.slots[label as usize]
    }
}

// Complete observation storage uses source names; no C layout equivalence.
pub use crate::observation::{
    MAX_CLK_CORR, MAX_FILELEN, MAX_FLAG_LEN, MAX_FLAGS, SourceClockCorrection, SourceObservation,
};
