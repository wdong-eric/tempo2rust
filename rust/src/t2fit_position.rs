//! Source fitting callbacks, TEMPO2 fda6ad4, GPL-3.0-or-later.
#![allow(
    clippy::double_parens,
    clippy::assign_op_pattern,
    clippy::redundant_guards,
    clippy::useless_format
)]
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens
)]
#![allow(
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::manual_range_contains,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::neg_multiply,
    clippy::unnecessary_cast,
    clippy::needless_return
)]
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// t2fit_position.C::t2FitFunc_stdPosition (pinned source line 6).
pub(crate) fn t2FitFunc_stdPosition(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut i: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut rce: [f64; 3] = [0.0; 3];
    let mut re: f64;
    let mut deltae: f64;
    let mut alphae: f64;
    let mut psrra: f64;
    let mut psrdec: f64;
    let mut axy: f64;
    let mut s: f64;
    let mut l: i32;
    let mut conv_epoch: f64 =
        ((psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val).clone()
            - (psr.param.slots[(param_posepoch as i32) as usize].cells[(0i32) as usize].val)
                .clone())
        .to_f64();
    {
        l = 0i32;
        while (l < 3i32) {
            {
                rce[(l) as usize] = *read(
                    &rows[(ipos) as usize].earth_ssb[(l) as usize],
                    "psr[p].obsn[ipos].earth_ssb[l]",
                )?;
            }
            l = (l + 1);
        }
    }
    re = (crate::tempo2_util::dot_product([rce[0], rce[1], rce[2]], [rce[0], rce[1], rce[2]]))
        .sqrt();
    axy = (rce[(2i32) as usize] / 499.00478364f64);
    s = (axy / (re / 499.00478364f64));
    deltae = (s).atan2((1.0f64 - (s * s)).sqrt());
    alphae = (rce[(1i32) as usize]).atan2(rce[(0i32) as usize]);
    psrra = ((psr.param.slots[(param_raj as i32) as usize].cells[(0i32) as usize].val).clone())
        .to_f64();
    psrdec = ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize].val).clone())
        .to_f64();
    if (i == (param_raj as i32)) {
        {
            return Ok((((re * (deltae).cos()) * (psrdec).cos()) * (psrra - alphae).sin()));
        }
    } else {
        if (i == (param_decj as i32)) {
            {
                return Ok((re
                    * ((((deltae).cos() * (psrdec).sin()) * (psrra - alphae).cos())
                        - ((deltae).sin() * (psrdec).cos()))));
            }
        } else {
            if (i == (param_pmra as i32)) {
                {
                    let mut t0: Float = Float::with_val(PRECISION_BITS, (x + conv_epoch));
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        (((re * (deltae).cos()) * (psrdec).cos()) * (psrra - alphae).sin()),
                    ) * (t0).clone())
                    .to_f64());
                }
            } else {
                if (i == (param_pmdec as i32)) {
                    {
                        let mut t0: Float = Float::with_val(PRECISION_BITS, (x + conv_epoch));
                        return Ok((Float::with_val(
                            PRECISION_BITS,
                            (re * ((((deltae).cos() * (psrdec).sin()) * (psrra - alphae).cos())
                                - ((deltae).sin() * (psrdec).cos()))),
                        ) * (t0).clone())
                        .to_f64());
                    }
                } else {
                    if (i == (param_pmra2 as i32)) {
                        {
                            let mut t0: Float = Float::with_val(PRECISION_BITS, (x + conv_epoch));
                            return Ok((((Float::with_val(
                                PRECISION_BITS,
                                (((re * (deltae).cos()) * (psrdec).cos()) * (psrra - alphae).sin()),
                            ) * (t0).clone())
                                * (t0).clone())
                                / Float::with_val(PRECISION_BITS, 2.0f64))
                            .to_f64());
                        }
                    } else {
                        if (i == (param_pmdec2 as i32)) {
                            {
                                let mut t0: Float =
                                    Float::with_val(PRECISION_BITS, (x + conv_epoch));
                                return Ok((((Float::with_val(
                                    PRECISION_BITS,
                                    (re * ((((deltae).cos() * (psrdec).sin())
                                        * (psrra - alphae).cos())
                                        - ((deltae).sin() * (psrdec).cos()))),
                                ) * (t0).clone())
                                    * (t0).clone())
                                    / Float::with_val(PRECISION_BITS, 2.0f64))
                                .to_f64());
                            }
                        } else {
                            if (i == (param_px as i32)) {
                                {
                                    let mut l: i32;
                                    let mut pxConv: f64 = (1.74532925199432958E-2f64 / 3600.0e3f64);
                                    let mut rca: [f64; 3] = [0.0; 3];
                                    let mut rr: f64;
                                    let mut rcos1: f64;
                                    {
                                        l = 0i32;
                                        while (l < 3i32) {
                                            rca[(l) as usize] = (*read(
                                                &rows[(ipos) as usize].earth_ssb[(l) as usize],
                                                "psr[p].obsn[ipos].earth_ssb[l]",
                                            )? + *read(
                                                &rows[(ipos) as usize].observatory_earth
                                                    [(l) as usize],
                                                "psr[p].obsn[ipos].observatory_earth[l]",
                                            )?);
                                            l = (l + 1);
                                        }
                                    }
                                    rr = crate::tempo2_util::dot_product(
                                        [rca[0], rca[1], rca[2]],
                                        [rca[0], rca[1], rca[2]],
                                    );
                                    rcos1 = crate::tempo2_util::dot_product(
                                        [
                                            *read(&psr.posPulsar[0], "vector")?,
                                            *read(&psr.posPulsar[1], "vector")?,
                                            *read(&psr.posPulsar[2], "vector")?,
                                        ],
                                        [rca[0], rca[1], rca[2]],
                                    );
                                    let mut afunc: Float = Float::with_val(
                                        PRECISION_BITS,
                                        (((0.5f64 * pxConv) * (rr - (rcos1 * rcos1)))
                                            / 499.00478364f64),
                                    );
                                    if ((*read(
                                        &psr.param.slots[(i) as usize].n_link_from,
                                        "psr[p].param[i].nLinkFrom",
                                    )? == 1i32)
                                        && (*read(
                                            &psr.param.slots[(i) as usize].link_from
                                                [(0i32) as usize],
                                            "psr[p].param[i].linkFrom[0]",
                                        )? == (param_dshk as i32)))
                                    {
                                        {
                                            let mut kpc2m: Float =
                                                Float::with_val(PRECISION_BITS, 3.08568025e19f64);
                                            let mut mas_yr2rad_s: Float =
                                                Float::with_val(PRECISION_BITS, 1.536281850e-16f64);
                                            let mut t0: Float;
                                            let mut afunc2: Float;
                                            t0 = Float::with_val(PRECISION_BITS, (x + conv_epoch));
                                            afunc2 = ((((((t0).clone() * (t0).clone())
                                                / Float::with_val(PRECISION_BITS, 2.0f64))
                                                / Float::with_val(
                                                    PRECISION_BITS,
                                                    299792458.0f64,
                                                ))
                                                * (((((psr.param.slots
                                                    [(param_pmra as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (psr.param.slots
                                                        [(param_pmra as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    * (mas_yr2rad_s).clone())
                                                    * (mas_yr2rad_s).clone())
                                                    + ((((psr.param.slots
                                                        [(param_pmdec as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        * (psr.param.slots
                                                            [(param_pmdec as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone())
                                                        * (mas_yr2rad_s).clone())
                                                        * (mas_yr2rad_s).clone())))
                                                * (kpc2m).clone());
                                            afunc = ((afunc).clone()
                                                + (((-(afunc2).clone())
                                                    / (psr.param.slots
                                                        [(param_px as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    / (psr.param.slots
                                                        [(param_px as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()));
                                        }
                                    }
                                    return Ok(((afunc).clone()).to_f64());
                                }
                            } else {
                                if (i == (param_pmrv as i32)) {
                                    {
                                        let mut j: i32;
                                        let mut delt: f64;
                                        let mut etat: f64;
                                        let mut dt_SSB: f64;
                                        let mut pmtrans_rcos2: f64;
                                        let mut rca: [f64; 4] = [0.0; 4];
                                        {
                                            j = 0i32;
                                            while (j < 3i32) {
                                                rca[(j) as usize] = (*read(
                                                    &rows[(ipos) as usize].earth_ssb[(j) as usize],
                                                    "psr[p].obsn[ipos].earth_ssb[j]",
                                                )? + *read(
                                                    &rows[(ipos) as usize].observatory_earth
                                                        [(j) as usize],
                                                    "psr[p].obsn[ipos].observatory_earth[j]",
                                                )?);
                                                j = (j + 1);
                                            }
                                        }
                                        pmtrans_rcos2 = crate::tempo2_util::dot_product(
                                            [
                                                *read(&psr.velPulsar[0], "vector")?,
                                                *read(&psr.velPulsar[1], "vector")?,
                                                *read(&psr.velPulsar[2], "vector")?,
                                            ],
                                            [rca[0], rca[1], rca[2]],
                                        );
                                        dt_SSB = 0.0f64;
                                        etat = 32.149618300000f64;
                                        delt = ((((read(
                                            &rows[(ipos) as usize].sat,
                                            "psr[p].obsn[ipos].sat",
                                        )?)
                                        .clone()
                                            - (psr.param.slots
                                                [(param_posepoch as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone())
                                            + Float::with_val(
                                                PRECISION_BITS,
                                                ((etat + dt_SSB) / 86400.0f64),
                                            ))
                                            / Float::with_val(PRECISION_BITS, 36525.0f64))
                                        .to_f64();
                                        return Ok(((delt).powf((2i32) as f64) * pmtrans_rcos2));
                                    }
                                } else {
                                    if (i == (param_dshk as i32)) {
                                        {
                                            let mut kpc2m: Float =
                                                Float::with_val(PRECISION_BITS, 3.08568025e19f64);
                                            let mut mas_yr2rad_s: Float =
                                                Float::with_val(PRECISION_BITS, 1.536281850e-16f64);
                                            let mut t0: Float;
                                            t0 = (((Float::with_val(PRECISION_BITS, x)
                                                + (psr.param.slots
                                                    [(param_pepoch as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                - (psr.param.slots
                                                    [(param_posepoch as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64));
                                            return Ok(((((((t0).clone() * (t0).clone())
                                                / Float::with_val(PRECISION_BITS, 2.0f64))
                                                / Float::with_val(
                                                    PRECISION_BITS,
                                                    299792458.0f64,
                                                ))
                                                * (((((psr.param.slots
                                                    [(param_pmra as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (psr.param.slots
                                                        [(param_pmra as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    * (mas_yr2rad_s).clone())
                                                    * (mas_yr2rad_s).clone())
                                                    + ((((psr.param.slots
                                                        [(param_pmdec as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        * (psr.param.slots
                                                            [(param_pmdec as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone())
                                                        * (mas_yr2rad_s).clone())
                                                        * (mas_yr2rad_s).clone())))
                                                * (kpc2m).clone())
                                            .to_f64());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return Err(boundary(
        "t2fit_position.C::t2FitFunc_stdPosition: assertion",
    ));
}

/// t2fit_position.C::t2UpdateFunc_stdPosition (pinned source line 110).
pub(crate) fn t2UpdateFunc_stdPosition(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    match label {
        source_case if source_case == (param_raj as i32) => {
            psr.param.slots[(param_raj as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_raj as i32) as usize].cells[(k) as usize].val).clone()
                    + Float::with_val(PRECISION_BITS, val));
            psr.param.slots[(param_raj as i32) as usize].cells[(k) as usize].err =
                Float::with_val(PRECISION_BITS, err);
            let retstr = crate::update_epoch::turn_hms(
                ((psr.param.slots[(param_raj as i32) as usize].cells[(k) as usize].val).clone()
                    / Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)))
                .to_f64(),
            )?;
            crate::observation::write_string(&mut psr.rajStrPost[..], &retstr, "rajStrPost")?;
        }
        source_case if source_case == (param_decj as i32) => {
            psr.param.slots[(param_decj as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_decj as i32) as usize].cells[(k) as usize].val).clone()
                    + Float::with_val(PRECISION_BITS, val));
            psr.param.slots[(param_decj as i32) as usize].cells[(k) as usize].err =
                Float::with_val(PRECISION_BITS, err);
            let retstr = crate::update_epoch::turn_dms(
                ((psr.param.slots[(param_decj as i32) as usize].cells[(k) as usize].val).clone()
                    / Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)))
                .to_f64(),
            )?;
            crate::observation::write_string(&mut psr.decjStrPost[..], &retstr, "decjStrPost")?;
        }
        source_case if source_case == (param_pmra as i32) => {
            psr.param.slots[(param_pmra as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_pmra as i32) as usize].cells[(k) as usize].val).clone()
                    + (Float::with_val(
                        PRECISION_BITS,
                        (((((((((val * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                            * 1000.0f64)
                            * 86400.0f64)
                            * 365.25f64)
                            / 24.0f64)
                            / 3600.0f64),
                    ) * ((psr.param.slots[(param_decj as i32) as usize].cells
                        [(0i32) as usize]
                        .val)
                        .clone())
                    .cos()));
            psr.param.slots[(param_pmra as i32) as usize].cells[(k) as usize].err = (Float::with_val(
                PRECISION_BITS,
                (((((((((err * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                    * 1000.0f64)
                    * 86400.0f64)
                    * 365.25f64)
                    / 24.0f64)
                    / 3600.0f64),
            )
                * ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize].val)
                    .clone())
                .cos());
        }
        source_case if source_case == (param_pmdec as i32) => {
            psr.param.slots[(param_pmdec as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_pmdec as i32) as usize].cells[(k) as usize].val).clone()
                    + Float::with_val(
                        PRECISION_BITS,
                        (((((((((val * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                            * 1000.0f64)
                            * 86400.0f64)
                            * 365.25f64)
                            / 24.0f64)
                            / 3600.0f64),
                    ));
            psr.param.slots[(param_pmdec as i32) as usize].cells[(k) as usize].err =
                Float::with_val(
                    PRECISION_BITS,
                    (((((((((err * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                        * 1000.0f64)
                        * 86400.0f64)
                        * 365.25f64)
                        / 24.0f64)
                        / 3600.0f64),
                );
        }
        source_case if source_case == (param_pmra2 as i32) => {
            psr.param.slots[(param_pmra2 as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_pmra2 as i32) as usize].cells[(k) as usize].val).clone()
                    + ((Float::with_val(
                        PRECISION_BITS,
                        (((((val * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                            * 1000.0f64),
                    ) * (Float::with_val(
                        PRECISION_BITS,
                        (((86400.0f64 * 365.25f64) / 24.0f64) / 3600.0f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                        * ((psr.param.slots[(param_decj as i32) as usize].cells
                            [(0i32) as usize]
                            .val)
                            .clone())
                        .cos()));
            psr.param.slots[(param_pmra2 as i32) as usize].cells[(k) as usize].err =
                ((Float::with_val(
                    PRECISION_BITS,
                    (((((err * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64) * 1000.0f64),
                ) * (Float::with_val(
                    PRECISION_BITS,
                    (((86400.0f64 * 365.25f64) / 24.0f64) / 3600.0f64),
                ))
                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                    * ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                    .cos());
        }
        source_case if source_case == (param_pmdec2 as i32) => {
            psr.param.slots[(param_pmdec2 as i32) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(param_pmdec2 as i32) as usize].cells[(k) as usize].val).clone()
                    + (Float::with_val(
                        PRECISION_BITS,
                        (((((val * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64)
                            * 1000.0f64),
                    ) * (Float::with_val(
                        PRECISION_BITS,
                        (((86400.0f64 * 365.25f64) / 24.0f64) / 3600.0f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))));
            psr.param.slots[(param_pmdec2 as i32) as usize].cells[(k) as usize].err =
                (Float::with_val(
                    PRECISION_BITS,
                    (((((err * 180.0f64) / std::f64::consts::PI) * 60.0f64) * 60.0f64) * 1000.0f64),
                ) * (Float::with_val(
                    PRECISION_BITS,
                    (((86400.0f64 * 365.25f64) / 24.0f64) / 3600.0f64),
                ))
                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))));
        }
        source_case if source_case == (param_pmrv as i32) => {
            psr.param.slots[(label) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
                    + Float::with_val(
                        PRECISION_BITS,
                        (((((10.0f64 * val) * 360.0f64) * 60.0f64) * 60.0f64)
                            / (2.0f64 * std::f64::consts::PI)),
                    ));
            psr.param.slots[(label) as usize].cells[(k) as usize].err = Float::with_val(
                PRECISION_BITS,
                (((((10.0f64 * err) * 360.0f64) * 60.0f64) * 60.0f64)
                    / (2.0f64 * std::f64::consts::PI)),
            );
        }
        _ => {
            psr.param.slots[(label) as usize].cells[(k) as usize].val =
                ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
                    + Float::with_val(PRECISION_BITS, val));
            psr.param.slots[(label) as usize].cells[(k) as usize].err =
                Float::with_val(PRECISION_BITS, err);
        }
    }
    Ok(())
}
