//! t2fit.C callback dispatch and source family selection.
use crate::{
    TimingError,
    clock_diagnostics::boundary,
    observation::read,
    pulsar_lifecycle::PulsarContext,
    tempo2::{Label, Label::*},
};
pub fn derivative(
    context: &mut PulsarContext,
    p: usize,
    x: f64,
    i: usize,
    label: Label,
    k: usize,
) -> Result<f64, TimingError> {
    if binary_label(label) {
        return crate::binary::fit_function(context, p, i, label, k);
    }
    if crate::t2fit_nestlike::is_noise(label) {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        return crate::t2fit_nestlike::derivative(psr, rows, x, i, label, k);
    }
    crate::clkcorr::with_process(|s| {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        if i >= rows.len() {
            return Err(boundary("fit_observation_index"));
        }
        let d = &mut s.diagnostics;
        match label {
            param_raj | param_decj | param_pmra | param_pmdec | param_pmra2 | param_pmdec2
            | param_px | param_pmrv | param_dshk => crate::t2fit_position::t2FitFunc_stdPosition(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_f => crate::t2fit_stdFitFuncs::t2FitFunc_stdFreq(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_brake => crate::t2fit_stdFitFuncs::t2FitFunc_stdFreq(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_dm => crate::t2fit_stdFitFuncs::t2FitFunc_stdDm(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_cm => crate::t2fit_stdFitFuncs::t2FitFunc_stdCm(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_fddc => crate::t2fit_dmother::t2FitFunc_fddc(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_fd => crate::t2fit_dmother::t2FitFunc_fd(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_dm_sin1yr | param_dm_cos1yr => crate::t2fit_dmother::t2FitFunc_dmsinusoids(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_fddi => crate::t2fit_stdFitFuncs::t2FitFunc_notImplemented(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_dmx => crate::t2fit_dmother::t2FitFunc_dmx(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_chromx => crate::t2fit_dmother::t2FitFunc_chromx(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_wave_om => crate::t2fit_fitwaves::t2FitFunc_fitwaves(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_wave_dm => crate::t2fit_fitwaves::t2FitFunc_fitwaves(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_glep | param_glph | param_glf0 | param_glf1 | param_glf0d | param_glf0d2
            | param_glf0d3 | param_gltd | param_gltd2 | param_gltd3 | param_glf2 => {
                crate::t2fit_glitch::t2FitFunc_stdGlitch(
                    psr,
                    rows,
                    p as i32,
                    x,
                    i as i32,
                    label as i32,
                    k as i32,
                    d,
                )
            }
            param_expph | param_exptau | param_expindex | param_expep => {
                crate::t2fit_glitch::t2FitFunc_expdip(
                    psr,
                    rows,
                    p as i32,
                    x,
                    i as i32,
                    label as i32,
                    k as i32,
                    d,
                )
            }
            param_gausep | param_gausamp | param_gaussig | param_gausindex => {
                crate::t2fit_glitch::t2FitFunc_gausdip(
                    psr,
                    rows,
                    p as i32,
                    x,
                    i as i32,
                    label as i32,
                    k as i32,
                    d,
                )
            }
            param_telx | param_tely | param_telz => crate::t2fit_stdFitFuncs::t2FitFunc_telPos(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_ne_sw => crate::t2fit_solarwind::t2FitFunc_ne_sw(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_ne_sw_sin => crate::t2fit_solarwind::t2FitFunc_ne_sw_sin(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_ne_sw_ifunc => crate::t2fit_solarwind::t2FitFunc_ne_sw_ifunc(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_gwm_amp => crate::t2fit_gw::t2FitFunc_gwm_amp(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_gwcs_amp => crate::t2fit_gw::t2FitFunc_gwcs_amp(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_quad_om => crate::t2fit_gw::t2FitFunc_quad_om(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_gwsingle => crate::t2fit_gw::t2FitFunc_gwsingle(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_dmassplanet => crate::t2fit_stdFitFuncs::planet(psr, rows, i, k),
            param_tel_dx | param_tel_dy | param_tel_dz => {
                crate::t2fit_stdFitFuncs::tel_delta(psr, rows, i, label as i32, k, d)
            }
            param_gwecc => Err(boundary("GWECC fit: source derivative pointer is null")),
            param_ZERO => crate::t2fit_stdFitFuncs::t2FitFunc_zero(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_JUMP => crate::t2fit_stdFitFuncs::t2FitFunc_jump(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_FDJUMP => crate::t2fit_stdFitFuncs::t2FitFunc_fdjump(
                psr,
                rows,
                p as i32,
                x,
                i as i32,
                label as i32,
                k as i32,
                d,
            ),
            param_ifunc | param_quad_ifunc_p | param_quad_ifunc_c | param_clk_offs => {
                let f = if label == param_ifunc && psr.param.slots[label as usize].cells[0].val == 1
                {
                    crate::t2fit_ifunc::t2FitFunc_sifunc
                } else {
                    crate::t2fit_ifunc::t2FitFunc_ifunc
                };
                f(psr, rows, p as i32, x, i as i32, label as i32, k as i32, d)
            }
            param_dmmodel => {
                let f = if k < *read(&psr.dmoffsDMnum, "dmoffsDMnum")? as usize {
                    crate::t2fit_dmmodel::t2FitFunc_dmmodelDM
                } else {
                    crate::t2fit_dmmodel::t2FitFunc_dmmodelCM
                };
                f(psr, rows, p as i32, x, i as i32, label as i32, k as i32, d)
            }
            _ => Err(boundary(&format!(
                "source has no fit callback: {label:?}[{k}]"
            ))),
        }
    })
}
pub fn update(
    context: &mut PulsarContext,
    p: usize,
    label: Label,
    k: usize,
    value: f64,
    error: f64,
) -> Result<(), TimingError> {
    if binary_label(label) {
        return crate::binary::update(context, p, label, k, value, error);
    }
    if crate::t2fit_nestlike::is_noise(label) {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        return crate::t2fit_nestlike::update(psr, rows, label, k, value, error);
    }
    crate::clkcorr::with_process(|s| {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        let d = &mut s.diagnostics;
        match label {
            param_raj | param_decj | param_pmra | param_pmdec | param_pmra2 | param_pmdec2
            | param_px | param_pmrv | param_dshk => {
                crate::t2fit_position::t2UpdateFunc_stdPosition(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_f => crate::t2fit_stdFitFuncs::t2UpdateFunc_stdFreq(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_brake => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_dm => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_cm => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_fddc => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_fd => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_dm_sin1yr | param_dm_cos1yr => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_fddi => crate::t2fit_stdFitFuncs::t2UpdateFunc_notImplemented(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_dmx => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_chromx => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_wave_om => crate::t2fit_fitwaves::t2UpdateFunc_fitwaves(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_wave_dm => crate::t2fit_fitwaves::t2UpdateFunc_fitwaves(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_glf2 => crate::t2fit_glitch::t2UpdateFunc_stdGlitch(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_glep | param_glph | param_glf0 | param_glf1 | param_glf0d | param_glf0d2
            | param_glf0d3 | param_gltd | param_gltd2 | param_gltd3 => {
                crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_expph | param_exptau | param_expindex | param_expep => {
                crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_gausep | param_gausamp | param_gaussig | param_gausindex => {
                crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_telx | param_tely | param_telz => {
                crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_ne_sw => crate::t2fit_solarwind::t2UpdateFunc_ne_sw(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_ne_sw_sin => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_ne_sw_ifunc => crate::t2fit_solarwind::t2UpdateFunc_ne_sw_ifunc(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_gwm_amp => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_gwcs_amp => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleMinus(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_quad_om => crate::t2fit_gw::t2UpdateFunc_quad_om(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_gwsingle => crate::t2fit_gw::t2UpdateFunc_gwsingle(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_dmassplanet => crate::t2fit_stdFitFuncs::t2UpdateFunc_simpleAdd(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_tel_dx | param_tel_dy | param_tel_dz => {
                crate::t2fit_stdFitFuncs::update_tel_delta(psr, label as i32, k, value, error)
            }
            param_ZERO => crate::t2fit_stdFitFuncs::t2UpdateFunc_zero(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_JUMP => crate::t2fit_stdFitFuncs::t2UpdateFunc_jump(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_FDJUMP => crate::t2fit_stdFitFuncs::t2UpdateFunc_fdjump(
                psr,
                rows,
                p as i32,
                label as i32,
                k as i32,
                value,
                error,
                d,
            ),
            param_ifunc | param_quad_ifunc_p | param_quad_ifunc_c | param_clk_offs => {
                crate::t2fit_ifunc::t2UpdateFunc_ifunc(
                    psr,
                    rows,
                    p as i32,
                    label as i32,
                    k as i32,
                    value,
                    error,
                    d,
                )
            }
            param_dmmodel => {
                let f = if k < *read(&psr.dmoffsDMnum, "dmoffsDMnum")? as usize {
                    crate::t2fit_dmmodel::t2UpdateFunc_dmmodelDM
                } else {
                    crate::t2fit_dmmodel::t2UpdateFunc_dmmodelCM
                };
                f(psr, rows, p as i32, label as i32, k as i32, value, error, d)
            }
            _ => Err(boundary(&format!(
                "source has no fit update: {label:?}[{k}]"
            ))),
        }
    })
}

pub(crate) fn binary_label(label: Label) -> bool {
    matches!(
        label,
        param_sini
            | param_pb
            | param_fb
            | param_t0
            | param_a1
            | param_om
            | param_ecc
            | param_edot
            | param_e2dot
            | param_xpbdot
            | param_pbdot
            | param_pb2dot
            | param_a1dot
            | param_a2dot
            | param_omdot
            | param_om2dot
            | param_orbpx
            | param_tasc
            | param_eps1
            | param_eps2
            | param_eps1dot
            | param_eps2dot
            | param_m2
            | param_gamma
            | param_mtot
            | param_bp
            | param_bpp
            | param_dr
            | param_dtheta
            | param_bpjep
            | param_bpjph
            | param_bpja1
            | param_bpjec
            | param_bpjom
            | param_bpjpb
            | param_h3
            | param_h4
            | param_stig
            | param_kin
            | param_kom
    )
}

/// Labels selected by t2fit_fillOneParameterFitInfo (including explicit source failures).
pub fn supported(label: Label) -> bool {
    matches!(
        label,
        param_raj
            | param_decj
            | param_pmra
            | param_pmdec
            | param_pmra2
            | param_pmdec2
            | param_px
            | param_pmrv
            | param_dshk
            | param_start
            | param_finish
            | param_f
            | param_brake
            | param_sini
            | param_pb
            | param_fb
            | param_t0
            | param_a1
            | param_om
            | param_ecc
            | param_edot
            | param_e2dot
            | param_xpbdot
            | param_pbdot
            | param_pb2dot
            | param_a1dot
            | param_a2dot
            | param_omdot
            | param_om2dot
            | param_orbpx
            | param_tasc
            | param_eps1
            | param_eps2
            | param_eps1dot
            | param_eps2dot
            | param_m2
            | param_gamma
            | param_mtot
            | param_bp
            | param_bpp
            | param_dr
            | param_dtheta
            | param_bpjep
            | param_bpjph
            | param_bpja1
            | param_bpjec
            | param_bpjom
            | param_bpjpb
            | param_h3
            | param_h4
            | param_stig
            | param_kin
            | param_kom
            | param_dm
            | param_cm
            | param_fddc
            | param_fd
            | param_dm_sin1yr
            | param_dm_cos1yr
            | param_fddi
            | param_dmx
            | param_chromx
            | param_dmassplanet
            | param_wave_om
            | param_wave_dm
            | param_glep
            | param_glph
            | param_glf0
            | param_glf1
            | param_glf0d
            | param_glf0d2
            | param_glf0d3
            | param_gltd
            | param_gltd2
            | param_gltd3
            | param_glf2
            | param_expph
            | param_exptau
            | param_expindex
            | param_expep
            | param_gausep
            | param_gausamp
            | param_gaussig
            | param_gausindex
            | param_telx
            | param_tely
            | param_telz
            | param_tel_dx
            | param_tel_dy
            | param_tel_dz
            | param_clk_offs
            | param_ifunc
            | param_quad_ifunc_p
            | param_quad_ifunc_c
            | param_dmmodel
            | param_red_sin
            | param_red_cos
            | param_band_red_sin
            | param_band_red_cos
            | param_group_red_sin
            | param_group_red_cos
            | param_red_dm_sin
            | param_red_dm_cos
            | param_red_chrom_sin
            | param_red_chrom_cos
            | param_swgp_sin
            | param_swgp_cos
            | param_shapevent
            | param_ne_sw
            | param_ne_sw_sin
            | param_ne_sw_ifunc
            | param_gwecc
            | param_gwm_amp
            | param_gwcs_amp
            | param_quad_om
            | param_gwsingle
    )
}

/// Standalone t2FitFunc_gwb_amp; the pinned default dispatcher has no route to it.
pub fn gwb_amplitude(
    context: &mut PulsarContext,
    p: usize,
    x: f64,
    i: usize,
    k: usize,
) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| {
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        crate::t2fit_gw::t2FitFunc_gwb_amp(
            psr,
            rows,
            p as i32,
            x,
            i as i32,
            param_gwb_amp as i32,
            k as i32,
            &mut s.diagnostics,
        )
    })
}
