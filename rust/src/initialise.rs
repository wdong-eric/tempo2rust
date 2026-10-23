//! Parameter-only initialise.C::allocateMemory / initialiseOne / destroyMemory.
//! All other pulsar/observation/covariance initialization is deferred.
// GPL-3.0-or-later; TEMPO2 contributors. Source pin in O's projection.
use crate::PRECISION_BITS;
use crate::tempo2::{
    Label::*, MAX_BPJ_JUMPS, MAX_COMPANIONS, MAX_DM_DERIVATIVES, MAX_DMX, MAX_FREQ_DERIVATIVES,
    MAX_PARAMS, MAX_SX, MAX_TEL_DX, MAX_TEL_DY, MAX_TEL_DZ, ParameterStore, SourceParameter,
};
use rug::Float;

/// realloc is unused by the source; Rust replacement drops the previous owner.
pub(crate) fn allocate_memory(_realloc: i32) -> ParameterStore {
    let slots = (0..MAX_PARAMS)
        .map(|i| {
            let capacity = match i as i32 {
                x if x == param_dm as i32 => MAX_DM_DERIVATIVES,
                x if x == param_cm as i32 => MAX_DM_DERIVATIVES,
                x if x == param_f as i32 => MAX_FREQ_DERIVATIVES,
                x if x == param_pb as i32
                    || x == param_ecc as i32
                    || x == param_om as i32
                    || x == param_t0 as i32
                    || x == param_a1 as i32
                    || x == param_a1dot as i32
                    || x == param_a2dot as i32
                    || x == param_pbdot as i32 =>
                {
                    MAX_COMPANIONS
                }
                x if x == param_fb as i32 => 20,
                x if x == param_bpjep as i32
                    || x == param_bpjph as i32
                    || x == param_bpja1 as i32
                    || x == param_bpjec as i32
                    || x == param_bpjom as i32
                    || x == param_bpjpb as i32 =>
                {
                    MAX_BPJ_JUMPS
                }
                x if x == param_glep as i32
                    || x == param_glph as i32
                    || x == param_glf0 as i32
                    || x == param_glf1 as i32
                    || x == param_stateSwitchT as i32
                    || x == param_glf2 as i32
                    || x == param_glf0d as i32
                    || x == param_gltd as i32
                    || x == param_glf0d2 as i32
                    || x == param_glf0d3 as i32
                    || x == param_gltd2 as i32
                    || x == param_gltd3 as i32 =>
                {
                    40
                }
                x if x == param_expep as i32
                    || x == param_expph as i32
                    || x == param_exptau as i32
                    || x == param_expindex as i32 =>
                {
                    40
                }
                x if x == param_gausep as i32
                    || x == param_gausamp as i32
                    || x == param_gaussig as i32
                    || x == param_gausindex as i32 =>
                {
                    40
                }
                x if x == param_dmassplanet as i32 => 9,
                x if x == param_dphaseplanet as i32 => 9,
                x if x == param_dmx as i32
                    || x == param_dmxr1 as i32
                    || x == param_dmxr2 as i32
                    || x == param_chromx as i32
                    || x == param_chromxr1 as i32
                    || x == param_chromxr2 as i32 =>
                {
                    MAX_DMX
                }
                x if x == param_sx as i32
                    || x == param_sxr1 as i32
                    || x == param_sxr2 as i32
                    || x == param_sxer as i32 =>
                {
                    MAX_SX
                }
                x if x == param_fd as i32 => 9,
                x if x == param_telx as i32 => 4,
                x if x == param_tely as i32 => 4,
                x if x == param_telz as i32 => 4,
                x if x == param_tel_dx as i32 => MAX_TEL_DX,
                x if x == param_tel_dy as i32 => MAX_TEL_DY,
                x if x == param_tel_dz as i32 => MAX_TEL_DZ,
                x if x == param_raj as i32 || x == param_decj as i32 => 2,
                x if x == param_gwm_amp as i32 => 2,
                x if x == param_gwcs_amp as i32 => 2,
                x if x == param_gwb_amp as i32 => 2,
                x if x == param_gwecc as i32 => 2,
                x if x == param_nudot_epoch as i32
                    || x == param_nudot_amp as i32
                    || x == param_nudot_transition_time as i32 =>
                {
                    32
                }
                _ => 1,
            };
            SourceParameter::allocated(capacity, 100)
        })
        .collect();
    ParameterStore { slots }
}

// Known source literals/formats fit the allocated buffers; retain bytes after NUL.
fn write_label(destination: &mut [u8], value: &[u8]) {
    destination[..value.len()].copy_from_slice(value);
    destination[value.len()] = 0;
}

pub(crate) fn initialise_parameters() -> ParameterStore {
    let mut psr = allocate_memory(0);
    let mut temp: String;

    for j in 0..psr.slots[param_f as usize].cells.len() {
        psr.slots[param_f as usize].cells[j].val = Float::with_val(PRECISION_BITS, 0);
        temp = format!("F{} (s^-{})", j, j + 1);
        write_label(
            &mut psr.slots[param_f as usize].cells[j].label,
            temp.as_bytes(),
        );
        temp = format!("F{}", j);
        write_label(
            &mut psr.slots[param_f as usize].cells[j].shortlabel,
            temp.as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_raj as usize].cells[0].label,
        b"RAJ (rad)",
    );
    write_label(
        &mut psr.slots[param_raj as usize].cells[0].shortlabel,
        b"RAJ",
    );
    write_label(
        &mut psr.slots[param_decj as usize].cells[0].label,
        b"DECJ (rad)",
    );
    write_label(
        &mut psr.slots[param_decj as usize].cells[0].shortlabel,
        b"DECJ",
    );
    write_label(&mut psr.slots[param_fddi as usize].cells[0].label, b"FDDI");
    write_label(
        &mut psr.slots[param_fddi as usize].cells[0].shortlabel,
        b"FDDI",
    );
    write_label(&mut psr.slots[param_fddc as usize].cells[0].label, b"FDDC");
    write_label(
        &mut psr.slots[param_fddc as usize].cells[0].shortlabel,
        b"FDDC",
    );
    for j in 0..psr.slots[param_fd as usize].cells.len() {
        write_label(
            &mut psr.slots[param_fd as usize].cells[j].shortlabel,
            format!("FD{}", j + 1).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_fd as usize].cells[j].label,
            format!("FD{}", j + 1).as_bytes(),
        );
    }

    for k in 0..psr.slots[param_dm as usize].cells.len() {
        if k > 0 {
            temp = format!("DM{} (cm^-3 pc yr^-{})", k, k);
            write_label(
                &mut psr.slots[param_dm as usize].cells[k].label,
                temp.as_bytes(),
            );
            temp = format!("DM{}", k);
            write_label(
                &mut psr.slots[param_dm as usize].cells[k].shortlabel,
                temp.as_bytes(),
            );
        } else {
            write_label(
                &mut psr.slots[param_dm as usize].cells[0].label,
                b"DM (cm^-3 pc)",
            );
            write_label(&mut psr.slots[param_dm as usize].cells[0].shortlabel, b"DM");
        }
    }
    for k in 0..psr.slots[param_cm as usize].cells.len() {
        if k > 0 {
            temp = format!("CM{} (arb yr^-{})", k, k);
            write_label(
                &mut psr.slots[param_cm as usize].cells[k].label,
                temp.as_bytes(),
            );
            temp = format!("CM{}", k);
            write_label(
                &mut psr.slots[param_cm as usize].cells[k].shortlabel,
                temp.as_bytes(),
            );
        } else {
            write_label(
                &mut psr.slots[param_cm as usize].cells[0].label,
                b"CM (arb)",
            );
            write_label(&mut psr.slots[param_cm as usize].cells[0].shortlabel, b"CM");
        }
    }

    write_label(
        &mut psr.slots[param_px as usize].cells[0].label,
        b"PX (mas)",
    );
    write_label(&mut psr.slots[param_px as usize].cells[0].shortlabel, b"PX");

    write_label(
        &mut psr.slots[param_dm_sin1yr as usize].cells[0].label,
        b"DM_S1YR (cm^-3 pc)",
    );
    write_label(
        &mut psr.slots[param_dm_sin1yr as usize].cells[0].shortlabel,
        b"DM_S1YR",
    );

    write_label(
        &mut psr.slots[param_dm_cos1yr as usize].cells[0].label,
        b"DM_C1YR (cm^-3 pc)",
    );
    write_label(
        &mut psr.slots[param_dm_cos1yr as usize].cells[0].shortlabel,
        b"DM_C1YR",
    );

    write_label(
        &mut psr.slots[param_daop as usize].cells[0].label,
        b"AOP dist. (kpc)",
    );
    write_label(
        &mut psr.slots[param_daop as usize].cells[0].shortlabel,
        b"D_AOP",
    );
    // Source defect: D_AOP is overwritten; param_iperharm is also labeled later.
    write_label(
        &mut psr.slots[param_daop as usize].cells[0].label,
        b"IPERHARM",
    );
    write_label(
        &mut psr.slots[param_daop as usize].cells[0].shortlabel,
        b"IPERHARM",
    );
    write_label(
        &mut psr.slots[param_pmrv as usize].cells[0].label,
        b"PMRV (mas/yr)",
    );
    write_label(
        &mut psr.slots[param_pmrv as usize].cells[0].shortlabel,
        b"PMRV",
    );
    for k in 0..psr.slots[param_dphaseplanet as usize].cells.len() {
        write_label(
            &mut psr.slots[param_dphaseplanet as usize].cells[k].label,
            format!("DPHASEPLANET{} ", k + 1).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_dphaseplanet as usize].cells[k].shortlabel,
            format!("DPHASEPLANET{}", k + 1).as_bytes(),
        );
    }
    for k in 0..psr.slots[param_dmassplanet as usize].cells.len() {
        write_label(
            &mut psr.slots[param_dmassplanet as usize].cells[k].label,
            format!("DMASSPLANET{} (Msun)", k + 1).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_dmassplanet as usize].cells[k].shortlabel,
            format!("DMASSPLANET{}", k + 1).as_bytes(),
        );
    }
    write_label(&mut psr.slots[param_tres as usize].cells[0].label, b"TRES");
    write_label(
        &mut psr.slots[param_tres as usize].cells[0].shortlabel,
        b"TRES",
    );
    write_label(
        &mut psr.slots[param_ephver as usize].cells[0].label,
        b"EPHVER",
    );
    write_label(
        &mut psr.slots[param_ephver as usize].cells[0].shortlabel,
        b"EPHVER",
    );
    write_label(
        &mut psr.slots[param_pmra as usize].cells[0].label,
        b"PMRA (mas/yr)",
    );
    write_label(
        &mut psr.slots[param_pmra as usize].cells[0].shortlabel,
        b"PMRA",
    );
    write_label(
        &mut psr.slots[param_pmdec as usize].cells[0].label,
        b"PMDEC (mas/yr)",
    );
    write_label(
        &mut psr.slots[param_pmdec as usize].cells[0].shortlabel,
        b"PMDEC",
    );
    write_label(
        &mut psr.slots[param_pmra2 as usize].cells[0].label,
        b"PMRA2 (mas/yr^2)",
    );
    write_label(
        &mut psr.slots[param_pmra2 as usize].cells[0].shortlabel,
        b"PMRA2",
    );
    write_label(
        &mut psr.slots[param_pmdec2 as usize].cells[0].label,
        b"PMDEC2 (mas/yr^2)",
    );
    write_label(
        &mut psr.slots[param_pmdec2 as usize].cells[0].shortlabel,
        b"PMDEC2",
    );

    write_label(
        &mut psr.slots[param_posepoch as usize].cells[0].label,
        b"POSEPOCH (MJD)",
    );
    write_label(
        &mut psr.slots[param_posepoch as usize].cells[0].shortlabel,
        b"POSEPOCH",
    );
    write_label(
        &mut psr.slots[param_waveepoch as usize].cells[0].label,
        b"WAVEEPOCH (MJD)",
    );
    write_label(
        &mut psr.slots[param_waveepoch as usize].cells[0].shortlabel,
        b"WAVEEPOCH",
    );
    write_label(
        &mut psr.slots[param_waveepoch_dm as usize].cells[0].label,
        b"WAVEEPOCHD (MJD)",
    );
    write_label(
        &mut psr.slots[param_waveepoch_dm as usize].cells[0].shortlabel,
        b"WAVEEPOCHD",
    );

    write_label(
        &mut psr.slots[param_gwm_amp as usize].cells[0].label,
        b"GWM_AMP",
    );
    write_label(
        &mut psr.slots[param_gwm_amp as usize].cells[0].shortlabel,
        b"GWM_AMP",
    );
    write_label(
        &mut psr.slots[param_gwm_amp as usize].cells[1].label,
        b"GWM_AMP_2",
    );
    write_label(
        &mut psr.slots[param_gwm_amp as usize].cells[1].shortlabel,
        b"GWM_AMP_2",
    );

    write_label(
        &mut psr.slots[param_gwcs_amp as usize].cells[0].label,
        b"GWCS_AMP1",
    );
    write_label(
        &mut psr.slots[param_gwcs_amp as usize].cells[0].shortlabel,
        b"GWCS_AMP1",
    );
    write_label(
        &mut psr.slots[param_gwcs_amp as usize].cells[1].label,
        b"GWCS_AMP2",
    );
    write_label(
        &mut psr.slots[param_gwcs_amp as usize].cells[1].shortlabel,
        b"GWCS_AMP2",
    );

    write_label(
        &mut psr.slots[param_gwb_amp as usize].cells[0].label,
        b"GWB_AMP",
    );
    write_label(
        &mut psr.slots[param_gwb_amp as usize].cells[0].shortlabel,
        b"GWB_AMP",
    );
    write_label(
        &mut psr.slots[param_gwb_amp as usize].cells[1].label,
        b"GWB_AMP_2",
    );
    write_label(
        &mut psr.slots[param_gwb_amp as usize].cells[1].shortlabel,
        b"GWB_AMP_2",
    );
    write_label(
        &mut psr.slots[param_gwecc as usize].cells[0].label,
        b"GWECC_AMP",
    );
    write_label(
        &mut psr.slots[param_gwecc as usize].cells[0].shortlabel,
        b"GWECC_AMP",
    );
    write_label(
        &mut psr.slots[param_tel_dx as usize].cells[0].label,
        b"TEL_DX",
    );
    write_label(
        &mut psr.slots[param_tel_dy as usize].cells[0].label,
        b"TEL_DY",
    );
    write_label(
        &mut psr.slots[param_tel_dz as usize].cells[0].label,
        b"TEL_DZ",
    );
    write_label(
        &mut psr.slots[param_tel_vx as usize].cells[0].label,
        b"TEL_VX (km/s)",
    );
    write_label(
        &mut psr.slots[param_tel_vy as usize].cells[0].label,
        b"TEL_VY (km/s)",
    );
    write_label(
        &mut psr.slots[param_tel_vz as usize].cells[0].label,
        b"TEL_VZ (km/s)",
    );
    write_label(
        &mut psr.slots[param_tel_x0 as usize].cells[0].label,
        b"TEL_X0 (km)",
    );
    write_label(
        &mut psr.slots[param_tel_y0 as usize].cells[0].label,
        b"TEL_Y0 (km)",
    );
    write_label(
        &mut psr.slots[param_tel_z0 as usize].cells[0].label,
        b"TEL_Z0 (km)",
    );
    write_label(
        &mut psr.slots[param_tel_dx as usize].cells[0].shortlabel,
        b"TEL_DX",
    );
    write_label(
        &mut psr.slots[param_tel_dy as usize].cells[0].shortlabel,
        b"TEL_DY",
    );
    write_label(
        &mut psr.slots[param_tel_dz as usize].cells[0].shortlabel,
        b"TEL_DZ",
    );
    write_label(
        &mut psr.slots[param_tel_vx as usize].cells[0].shortlabel,
        b"TEL_VX",
    );
    write_label(
        &mut psr.slots[param_tel_vy as usize].cells[0].shortlabel,
        b"TEL_VY",
    );
    write_label(
        &mut psr.slots[param_tel_vz as usize].cells[0].shortlabel,
        b"TEL_VZ",
    );
    write_label(
        &mut psr.slots[param_tel_x0 as usize].cells[0].shortlabel,
        b"TEL_X0",
    );
    write_label(
        &mut psr.slots[param_tel_y0 as usize].cells[0].shortlabel,
        b"TEL_Y0",
    );
    write_label(
        &mut psr.slots[param_tel_z0 as usize].cells[0].shortlabel,
        b"TEL_Z0",
    );
    write_label(
        &mut psr.slots[param_ifunc as usize].cells[0].label,
        b"IFUNC",
    );
    write_label(
        &mut psr.slots[param_ifunc as usize].cells[0].shortlabel,
        b"IFUNC",
    );
    write_label(
        &mut psr.slots[param_orbifunc as usize].cells[0].label,
        b"ORBIFUNC",
    );
    write_label(
        &mut psr.slots[param_orbifunc as usize].cells[0].shortlabel,
        b"ORBIFUNC",
    );
    write_label(&mut psr.slots[param_df1 as usize].cells[0].label, b"DF1");
    write_label(
        &mut psr.slots[param_df1 as usize].cells[0].shortlabel,
        b"DF1",
    );
    write_label(
        &mut psr.slots[param_clk_offs as usize].cells[0].label,
        b"CLK_OFFS",
    );
    write_label(
        &mut psr.slots[param_clk_offs as usize].cells[0].shortlabel,
        b"CLK_OFFS",
    );
    write_label(
        &mut psr.slots[param_quad_ifunc_p as usize].cells[0].label,
        b"QIFUNC_p",
    );
    write_label(
        &mut psr.slots[param_quad_ifunc_p as usize].cells[0].shortlabel,
        b"QIFUNC_p",
    );
    write_label(
        &mut psr.slots[param_quad_ifunc_c as usize].cells[0].label,
        b"QIFUNC_c",
    );
    write_label(
        &mut psr.slots[param_quad_ifunc_c as usize].cells[0].shortlabel,
        b"QIFUNC_c",
    );
    write_label(
        &mut psr.slots[param_pepoch as usize].cells[0].label,
        b"PEPOCH (MJD)",
    );
    write_label(
        &mut psr.slots[param_pepoch as usize].cells[0].shortlabel,
        b"PEPOCH",
    );
    write_label(
        &mut psr.slots[param_dmepoch as usize].cells[0].label,
        b"DMEPOCH (MJD)",
    );
    write_label(
        &mut psr.slots[param_dmepoch as usize].cells[0].shortlabel,
        b"DMEPOCH",
    );
    write_label(
        &mut psr.slots[param_start as usize].cells[0].label,
        b"START (MJD)",
    );
    write_label(
        &mut psr.slots[param_start as usize].cells[0].shortlabel,
        b"START",
    );
    write_label(
        &mut psr.slots[param_finish as usize].cells[0].label,
        b"FINISH (MJD)",
    );
    write_label(
        &mut psr.slots[param_finish as usize].cells[0].shortlabel,
        b"FINISH",
    );
    write_label(
        &mut psr.slots[param_track as usize].cells[0].label,
        b"TRACK (MJD)",
    );
    write_label(
        &mut psr.slots[param_track as usize].cells[0].shortlabel,
        b"TRACK",
    );
    write_label(
        &mut psr.slots[param_dshk as usize].cells[0].label,
        b"DSHK (kpc)",
    );
    write_label(
        &mut psr.slots[param_dshk as usize].cells[0].shortlabel,
        b"DSHK",
    );

    write_label(
        &mut psr.slots[param_iperharm as usize].cells[0].label,
        b"IPERHARM",
    );
    write_label(
        &mut psr.slots[param_iperharm as usize].cells[0].shortlabel,
        b"IPERHARM",
    );

    write_label(
        &mut psr.slots[param_telx as usize].cells[0].shortlabel,
        b"TELX",
    );
    write_label(
        &mut psr.slots[param_telx as usize].cells[0].label,
        b"TELX (lt-s)",
    );
    for k in 1..psr.slots[param_telx as usize].cells.len() {
        write_label(
            &mut psr.slots[param_telx as usize].cells[k].label,
            format!("TELX{} (lt-s)", k).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_telx as usize].cells[k].shortlabel,
            format!("TELX{}", k).as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_tely as usize].cells[0].shortlabel,
        b"TELY",
    );
    write_label(
        &mut psr.slots[param_tely as usize].cells[0].label,
        b"TELY (lt-s)",
    );
    for k in 1..psr.slots[param_tely as usize].cells.len() {
        write_label(
            &mut psr.slots[param_tely as usize].cells[k].label,
            format!("TELY{} (lt-s)", k).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_tely as usize].cells[k].shortlabel,
            format!("TELY{}", k).as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_telz as usize].cells[0].shortlabel,
        b"TELZ",
    );
    write_label(
        &mut psr.slots[param_telz as usize].cells[0].label,
        b"TELZ (lt-s)",
    );
    for k in 1..psr.slots[param_telz as usize].cells.len() {
        write_label(
            &mut psr.slots[param_telz as usize].cells[k].label,
            format!("TELZ{} (lt-s)", k).as_bytes(),
        );
        write_label(
            &mut psr.slots[param_telz as usize].cells[k].shortlabel,
            format!("TELZ{}", k).as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_telEpoch as usize].cells[0].shortlabel,
        b"TELEPOCH",
    );
    write_label(
        &mut psr.slots[param_telEpoch as usize].cells[0].label,
        b"TEL EPOCH (MJD)",
    );

    for k in 0..psr.slots[param_glep as usize].cells.len() {
        temp = format!("GLEP_{}", k + 1);
        write_label(
            &mut psr.slots[param_glep as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glep as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLPH_{}", k + 1);
        write_label(
            &mut psr.slots[param_glph as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glph as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF0_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf0 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf0 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF1_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF2_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF0D_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf0d as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf0d as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLTD_{}", k + 1);
        write_label(
            &mut psr.slots[param_gltd as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gltd as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF0D2_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf0d2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf0d2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLTD2_{}", k + 1);
        write_label(
            &mut psr.slots[param_gltd2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gltd2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLF0D3_{}", k + 1);
        write_label(
            &mut psr.slots[param_glf0d3 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_glf0d3 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("GLTD3_{}", k + 1);
        write_label(
            &mut psr.slots[param_gltd3 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gltd3 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
        temp = format!("SWITCH_{}", k + 1);
        write_label(
            &mut psr.slots[param_stateSwitchT as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_stateSwitchT as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    for k in 0..psr.slots[param_expep as usize].cells.len() {
        temp = format!("EXPEP_{}", k + 1);
        write_label(
            &mut psr.slots[param_expep as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_expep as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("EXPPH_{}", k + 1);
        write_label(
            &mut psr.slots[param_expph as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_expph as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("EXPTAU_{}", k + 1);
        write_label(
            &mut psr.slots[param_exptau as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_exptau as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("EXPINDEX_{}", k + 1);
        write_label(
            &mut psr.slots[param_expindex as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_expindex as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    for k in 0..psr.slots[param_gausep as usize].cells.len() {
        temp = format!("GAUSEP_{}", k + 1);
        write_label(
            &mut psr.slots[param_gausep as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gausep as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("GAUSAMP_{}", k + 1);
        write_label(
            &mut psr.slots[param_gausamp as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gausamp as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("GAUSSIG_{}", k + 1);
        write_label(
            &mut psr.slots[param_gaussig as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gaussig as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("GAUSINDEX_{}", k + 1);
        write_label(
            &mut psr.slots[param_gausindex as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_gausindex as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_t0 as usize].cells[0].label,
        b"T0 (MJD)",
    );
    write_label(&mut psr.slots[param_t0 as usize].cells[0].shortlabel, b"T0");
    for k in 0..psr.slots[param_fb as usize].cells.len() {
        temp = format!("FB{}", k);
        write_label(
            &mut psr.slots[param_fb as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("FB{}", k);
        write_label(
            &mut psr.slots[param_fb as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    for k in 1..psr.slots[param_pb as usize].cells.len() {
        temp = format!("PB_{} (d)", k + 1);
        write_label(
            &mut psr.slots[param_pb as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("PB_{}", k + 1);
        write_label(
            &mut psr.slots[param_pb as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("ECC_{}", k + 1);
        write_label(
            &mut psr.slots[param_ecc as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("ECC_{}", k + 1);
        write_label(
            &mut psr.slots[param_ecc as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("OM_{} (deg)", k + 1);
        write_label(
            &mut psr.slots[param_om as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("OM_{}", k + 1);
        write_label(
            &mut psr.slots[param_om as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("A1_{} (lt-s)", k + 1);
        write_label(
            &mut psr.slots[param_a1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("A1_{}", k + 1);
        write_label(
            &mut psr.slots[param_a1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("T0_{} (mjd)", k + 1);
        write_label(
            &mut psr.slots[param_t0 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("T0_{}", k + 1);
        write_label(
            &mut psr.slots[param_t0 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("XDOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_a1dot as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("XDOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_a1dot as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("X2DOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_a2dot as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("X2DOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_a2dot as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("PBDOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_pbdot as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("PBDOT_{}", k + 1);
        write_label(
            &mut psr.slots[param_pbdot as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    write_label(&mut psr.slots[param_pb as usize].cells[0].label, b"PB (d)");
    write_label(&mut psr.slots[param_pb as usize].cells[0].shortlabel, b"PB");
    write_label(
        &mut psr.slots[param_a1 as usize].cells[0].label,
        b"A1 (lt-s)",
    );
    write_label(&mut psr.slots[param_a1 as usize].cells[0].shortlabel, b"A1");
    write_label(
        &mut psr.slots[param_om as usize].cells[0].label,
        b"OM (deg)",
    );
    write_label(&mut psr.slots[param_om as usize].cells[0].shortlabel, b"OM");
    write_label(&mut psr.slots[param_ecc as usize].cells[0].label, b"ECC");
    write_label(
        &mut psr.slots[param_e2dot as usize].cells[0].shortlabel,
        b"E2DOT",
    );
    write_label(
        &mut psr.slots[param_e2dot as usize].cells[0].label,
        b"E2DOT",
    );
    write_label(
        &mut psr.slots[param_edot as usize].cells[0].shortlabel,
        b"EDOT",
    );
    write_label(&mut psr.slots[param_edot as usize].cells[0].label, b"EDOT");
    write_label(
        &mut psr.slots[param_ecc as usize].cells[0].shortlabel,
        b"ECC",
    );
    write_label(&mut psr.slots[param_kom as usize].cells[0].label, b"KOM");
    write_label(
        &mut psr.slots[param_kom as usize].cells[0].shortlabel,
        b"KOM",
    );
    write_label(&mut psr.slots[param_kin as usize].cells[0].label, b"KIN");
    write_label(
        &mut psr.slots[param_kin as usize].cells[0].shortlabel,
        b"KIN",
    );
    write_label(
        &mut psr.slots[param_shapmax as usize].cells[0].label,
        b"SHAPMAX",
    );
    write_label(
        &mut psr.slots[param_shapmax as usize].cells[0].shortlabel,
        b"SHAPMAX",
    );
    write_label(&mut psr.slots[param_m2 as usize].cells[0].label, b"M2");
    write_label(&mut psr.slots[param_m2 as usize].cells[0].shortlabel, b"M2");
    write_label(&mut psr.slots[param_mtot as usize].cells[0].label, b"MTOT");
    write_label(
        &mut psr.slots[param_mtot as usize].cells[0].shortlabel,
        b"MTOT",
    );
    write_label(&mut psr.slots[param_dr as usize].cells[0].label, b"DR");
    write_label(&mut psr.slots[param_dr as usize].cells[0].shortlabel, b"DR");
    write_label(&mut psr.slots[param_dth as usize].cells[0].label, b"DTH");
    write_label(
        &mut psr.slots[param_dth as usize].cells[0].shortlabel,
        b"DTH",
    );
    write_label(&mut psr.slots[param_a0 as usize].cells[0].label, b"A0");
    write_label(&mut psr.slots[param_a0 as usize].cells[0].shortlabel, b"A0");
    write_label(&mut psr.slots[param_b0 as usize].cells[0].label, b"B0");
    write_label(&mut psr.slots[param_b0 as usize].cells[0].shortlabel, b"B0");
    write_label(&mut psr.slots[param_bp as usize].cells[0].label, b"BP");
    write_label(&mut psr.slots[param_bp as usize].cells[0].shortlabel, b"BP");
    write_label(&mut psr.slots[param_bpp as usize].cells[0].label, b"BPP");
    write_label(
        &mut psr.slots[param_bpp as usize].cells[0].shortlabel,
        b"BPP",
    );
    write_label(
        &mut psr.slots[param_dtheta as usize].cells[0].label,
        b"DTHETA",
    );
    write_label(
        &mut psr.slots[param_dtheta as usize].cells[0].shortlabel,
        b"DTHETA",
    );
    write_label(&mut psr.slots[param_sini as usize].cells[0].label, b"SINI");
    write_label(
        &mut psr.slots[param_sini as usize].cells[0].shortlabel,
        b"SINI",
    );

    write_label(&mut psr.slots[param_h3 as usize].cells[0].label, b"H3");
    write_label(&mut psr.slots[param_h3 as usize].cells[0].shortlabel, b"H3");
    write_label(&mut psr.slots[param_stig as usize].cells[0].label, b"STIG");
    write_label(
        &mut psr.slots[param_stig as usize].cells[0].shortlabel,
        b"STIG",
    );
    write_label(&mut psr.slots[param_h4 as usize].cells[0].label, b"H4");
    write_label(&mut psr.slots[param_h4 as usize].cells[0].shortlabel, b"H4");
    write_label(
        &mut psr.slots[param_nharm as usize].cells[0].label,
        b"Number of Shapiro delay harmonics",
    );
    write_label(
        &mut psr.slots[param_nharm as usize].cells[0].shortlabel,
        b"NHARM",
    );

    write_label(
        &mut psr.slots[param_gamma as usize].cells[0].label,
        b"GAMMA",
    );
    write_label(
        &mut psr.slots[param_gamma as usize].cells[0].shortlabel,
        b"GAMMA",
    );
    write_label(
        &mut psr.slots[param_pbdot as usize].cells[0].label,
        b"PBDOT",
    );
    write_label(
        &mut psr.slots[param_pbdot as usize].cells[0].shortlabel,
        b"PBDOT",
    );
    write_label(
        &mut psr.slots[param_pb2dot as usize].cells[0].label,
        b"PB2DOT (s^-1)",
    );
    write_label(
        &mut psr.slots[param_pb2dot as usize].cells[0].shortlabel,
        b"PB2DOT",
    );
    write_label(
        &mut psr.slots[param_xpbdot as usize].cells[0].label,
        b"XPBDOT",
    );
    write_label(
        &mut psr.slots[param_xpbdot as usize].cells[0].shortlabel,
        b"XPBDOT",
    );
    write_label(&mut psr.slots[param_a1dot as usize].cells[0].label, b"XDOT");
    write_label(
        &mut psr.slots[param_a1dot as usize].cells[0].shortlabel,
        b"XDOT",
    );
    write_label(
        &mut psr.slots[param_a2dot as usize].cells[0].label,
        b"X2DOT",
    );
    write_label(
        &mut psr.slots[param_a2dot as usize].cells[0].shortlabel,
        b"X2DOT",
    );
    write_label(
        &mut psr.slots[param_xomdot as usize].cells[0].label,
        b"XOMDOT",
    );
    write_label(
        &mut psr.slots[param_xomdot as usize].cells[0].shortlabel,
        b"XOMDOT",
    );
    write_label(&mut psr.slots[param_afac as usize].cells[0].label, b"AFAC");
    write_label(
        &mut psr.slots[param_afac as usize].cells[0].shortlabel,
        b"AFAC",
    );
    write_label(
        &mut psr.slots[param_omdot as usize].cells[0].label,
        b"OMDOT (deg/yr)",
    );
    write_label(
        &mut psr.slots[param_omdot as usize].cells[0].shortlabel,
        b"OMDOT",
    );
    write_label(
        &mut psr.slots[param_om2dot as usize].cells[0].label,
        b"OM2DOT(1e-20 rad/yr^2)",
    );
    write_label(
        &mut psr.slots[param_om2dot as usize].cells[0].shortlabel,
        b"OM2DOT",
    );
    write_label(
        &mut psr.slots[param_orbpx as usize].cells[0].label,
        b"ORBPX (kpc^-1)",
    );
    write_label(
        &mut psr.slots[param_orbpx as usize].cells[0].shortlabel,
        b"ORBPX",
    );
    write_label(
        &mut psr.slots[param_tasc as usize].cells[0].label,
        b"TASC (MJD)",
    );
    write_label(
        &mut psr.slots[param_tasc as usize].cells[0].shortlabel,
        b"TASC",
    );
    write_label(&mut psr.slots[param_eps1 as usize].cells[0].label, b"EPS1");
    write_label(
        &mut psr.slots[param_eps1 as usize].cells[0].shortlabel,
        b"EPS1",
    );
    write_label(
        &mut psr.slots[param_eps1dot as usize].cells[0].label,
        b"EPS1DOT",
    );
    write_label(
        &mut psr.slots[param_eps1dot as usize].cells[0].shortlabel,
        b"EPS1DOT",
    );
    write_label(&mut psr.slots[param_eps2 as usize].cells[0].label, b"EPS2");
    write_label(
        &mut psr.slots[param_eps2 as usize].cells[0].shortlabel,
        b"EPS2",
    );
    write_label(
        &mut psr.slots[param_eps2dot as usize].cells[0].label,
        b"EPS2DOT",
    );
    write_label(
        &mut psr.slots[param_eps2dot as usize].cells[0].shortlabel,
        b"EPS2DOT",
    );
    write_label(
        &mut psr.slots[param_tzrmjd as usize].cells[0].label,
        b"TZRMJD",
    );
    write_label(
        &mut psr.slots[param_tzrmjd as usize].cells[0].shortlabel,
        b"TZRMJD",
    );
    write_label(
        &mut psr.slots[param_tzrfrq as usize].cells[0].label,
        b"TZRFRQ (MHz)",
    );
    write_label(
        &mut psr.slots[param_tzrfrq as usize].cells[0].shortlabel,
        b"TZRFRQ",
    );
    write_label(
        &mut psr.slots[param_tspan as usize].cells[0].label,
        b"TSPAN (min)",
    );
    write_label(
        &mut psr.slots[param_tspan as usize].cells[0].shortlabel,
        b"TSPAN",
    );
    write_label(
        &mut psr.slots[param_brake as usize].cells[0].label,
        b"BRAKING INDEX",
    );
    write_label(
        &mut psr.slots[param_brake as usize].cells[0].shortlabel,
        b"BRAKE",
    );

    write_label(
        &mut psr.slots[param_ne_sw as usize].cells[0].label,
        b"NE_SW (cm^-3)",
    );
    write_label(
        &mut psr.slots[param_ne_sw as usize].cells[0].shortlabel,
        b"NE_SW",
    );

    write_label(
        &mut psr.slots[param_ne_sw_sin as usize].cells[0].label,
        b"NE_SW_SIN (cm^-3)",
    );
    write_label(
        &mut psr.slots[param_ne_sw_sin as usize].cells[0].shortlabel,
        b"NE_SW_SIN",
    );

    write_label(
        &mut psr.slots[param_ne_sw_ifunc as usize].cells[0].label,
        b"NE_SW_IFUNC",
    );
    write_label(
        &mut psr.slots[param_ne_sw_ifunc as usize].cells[0].shortlabel,
        b"NE_SW_IFUNC",
    );

    for k in 0..psr.slots[param_bpjep as usize].cells.len() {
        temp = format!("BPJEP_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpjep as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpjep as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("BPJPH_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpjph as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpjph as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("BPJA1_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpja1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpja1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("BPJEC_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpjec as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpjec as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("BPJOM_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpjom as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpjom as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("BPJPB_{}", k + 1);
        write_label(
            &mut psr.slots[param_bpjpb as usize].cells[k].label,
            temp.as_bytes(),
        );
        write_label(
            &mut psr.slots[param_bpjpb as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    write_label(
        &mut psr.slots[param_wave_om as usize].cells[0].label,
        b"WAVE_OM",
    );
    write_label(
        &mut psr.slots[param_wave_om as usize].cells[0].shortlabel,
        b"WAVE_OM",
    );
    write_label(
        &mut psr.slots[param_wave_dm as usize].cells[0].label,
        b"WAVE_DM",
    );
    write_label(
        &mut psr.slots[param_wave_dm as usize].cells[0].shortlabel,
        b"WAVE_DM",
    );
    write_label(
        &mut psr.slots[param_quad_om as usize].cells[0].label,
        b"QUAD_OM",
    );
    write_label(
        &mut psr.slots[param_quad_om as usize].cells[0].shortlabel,
        b"QUAD_OM",
    );
    write_label(
        &mut psr.slots[param_dmmodel as usize].cells[0].label,
        b"DMMODEL",
    );
    write_label(
        &mut psr.slots[param_dmmodel as usize].cells[0].shortlabel,
        b"DMMODEL",
    );
    write_label(
        &mut psr.slots[param_gwsingle as usize].cells[0].label,
        b"GW_OMEGA",
    );
    write_label(
        &mut psr.slots[param_gwsingle as usize].cells[0].shortlabel,
        b"GW_OMEGA",
    );

    for k in 0..psr.slots[param_dmx as usize].cells.len() {
        temp = format!("DMX_{:04} (cm^-3 pc)", k + 1);
        write_label(
            &mut psr.slots[param_dmx as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("DMX_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_dmx as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("DMXR1_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_dmxr1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("DMXR1_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_dmxr1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("DMXR2_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_dmxr2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("DMXR2_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_dmxr2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    for k in 0..psr.slots[param_chromx as usize].cells.len() {
        temp = format!("CHROMX_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_chromx as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("CHROMX_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_chromx as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("CHROMXR1_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_chromxr1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("CHROMXR1_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_chromxr1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("CHROMXR2_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_chromxr2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("CHROMXR2_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_chromxr2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    for k in 0..psr.slots[param_sx as usize].cells.len() {
        temp = format!("SX_{:04} (cm^-3 pc)", k + 1);
        write_label(
            &mut psr.slots[param_sx as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("SX_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_sx as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("SXR1_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_sxr1 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("SXR1_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_sxr1 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("SXR2_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_sxr2 as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("SXR2_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_sxr2 as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );

        temp = format!("SXER_{:04} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_sxer as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("SXER_{:04}", k + 1);
        write_label(
            &mut psr.slots[param_sxer as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    for k in 0..psr.slots[param_nudot_epoch as usize].cells.len() {
        temp = format!("NUDOT_EPOCH_{} (MJD)", k + 1);
        write_label(
            &mut psr.slots[param_nudot_epoch as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("NUDOT_EPOCH_{}", k + 1);
        write_label(
            &mut psr.slots[param_nudot_epoch as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    for k in 0..psr.slots[param_nudot_amp as usize].cells.len() {
        temp = format!("NUDOT_AMP_{} (Hz^2)", k + 1);
        write_label(
            &mut psr.slots[param_nudot_amp as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("NUDOT_AMP_{}", k + 1);
        write_label(
            &mut psr.slots[param_nudot_amp as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }
    for k in 0..psr.slots[param_nudot_transition_time as usize].cells.len() {
        temp = format!("NUDOT_TIME_{} (days)", k + 1);
        write_label(
            &mut psr.slots[param_nudot_transition_time as usize].cells[k].label,
            temp.as_bytes(),
        );
        temp = format!("NUDOT_TIME_{}", k + 1);
        write_label(
            &mut psr.slots[param_nudot_transition_time as usize].cells[k].shortlabel,
            temp.as_bytes(),
        );
    }

    psr
}
