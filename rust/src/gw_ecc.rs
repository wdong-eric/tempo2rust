//! GWsim.C: eccRes, Rs, dadt, dedt, dtdt and Fe only. GPL-3.0-or-later.
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens
)]
// Source predicates retain NaN semantics; signatures, constants and explicit
// promotion boundaries follow the pinned routine for numerical comparison.
#![allow(
    clippy::manual_range_contains,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::if_same_then_else,
    clippy::too_many_arguments,
    clippy::excessive_precision,
    clippy::neg_multiply,
    clippy::unnecessary_cast
)]
use crate::binary::source_i32;
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::Diagnostics,
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::Float;
/// GWsim.C::eccRes (pinned source line 351).
pub(super) fn eccRes(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut i: i32,
    mut coalesceFlag: &mut Option<i32>,
    mut prev_p: &mut Option<f64>,
    mut prev_e: &mut Option<f64>,
    mut prev_a: &mut Option<f64>,
    mut prev_epoch: &mut Option<f64>,
    mut prev_theta: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<Float, TimingError> {
    ecc_res_inner(
        psr,
        rows,
        i,
        coalesceFlag,
        prev_p,
        prev_e,
        prev_a,
        prev_epoch,
        prev_theta,
        d,
        None,
    )
}

/// Both source bodies are arithmetically identical through the residual; the
/// energy variant adds its own f32 writes and diagnostic at the original sites.
#[allow(clippy::needless_option_as_deref)]
pub(super) fn ecc_res_inner(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    i: i32,
    coalesceFlag: &mut Option<i32>,
    prev_p: &mut Option<f64>,
    prev_e: &mut Option<f64>,
    prev_a: &mut Option<f64>,
    prev_epoch: &mut Option<f64>,
    prev_theta: &mut Option<f64>,
    d: &mut Diagnostics,
    mut energy: Option<&mut f32>,
) -> Result<Float, TimingError> {
    let mut pert: Float;
    if (*read(&*coalesceFlag, "*coalesceFlag")? == 1i32) {
        {
            pert = Float::with_val(PRECISION_BITS, 0.0f64);
            if let Some(e) = energy.as_deref_mut() {
                *e = 0.0;
            }
            Ok((pert).clone())
        }
    } else {
        {
            let mut ra_p: f64;
            let mut dec_p: f64;
            let mut ra_g: f64;
            let mut dec_g: f64;
            let mut m1: f64;
            let mut m2: f64;
            let mut inc: f64;
            let mut theta_n: f64;
            let mut phi: f64;
            let mut z: f64;
            let mut D: f64;
            let mut t_0: f64;
            let mut ih: i32;
            let mut G: f64 = 6.67e-11f64;
            let mut c: f64 = 2.998e8f64;
            let mut msun: f64 = 1.9891e30f64;
            let mut Mpc_to_m: f64 = 3.08568e22f64;
            ra_p = ((psr.param.slots[param_raj as usize].cells[0].val).clone()).to_f64();
            dec_p = ((psr.param.slots[param_decj as usize].cells[0].val).clone()).to_f64();
            ra_g = *read(&psr.gwecc_ra, "psr->gwecc_ra")?;
            dec_g = *read(&psr.gwecc_dec, "psr->gwecc_dec")?;
            m1 = (*read(&psr.gwecc_m1, "psr->gwecc_m1")? * msun);
            m2 = (*read(&psr.gwecc_m2, "psr->gwecc_m2")? * msun);
            inc = *read(&psr.gwecc_inc, "psr->gwecc_inc")?;
            theta_n = *read(&psr.gwecc_theta_nodes, "psr->gwecc_theta_nodes")?;
            phi = *read(&psr.gwecc_nodes_orientation, "psr->gwecc_nodes_orientation")?;
            z = *read(&psr.gwecc_redshift, "psr->gwecc_redshift")?;
            D = (*read(&psr.gwecc_distance, "psr->gwecc_distance")? * Mpc_to_m);
            t_0 = *read(&psr.gwecc_epoch, "psr->gwecc_epoch")?;
            let mut rs_m1: f64 = Rs(m1, d)?;
            let mut time: f64 = ((((read(&rows[(i) as usize].sat, "psr->obsn[i].sat")?).clone()
                - ((read(&rows[(i) as usize].residual, "psr->obsn[i].residual")?).clone()
                    / Float::with_val(PRECISION_BITS, 86400.0f64)))
            .to_f64()
                - *read(&*prev_epoch, "*prev_epoch")?)
                / (1.0f64 + z));
            let mut tint: f64 = ((86400.0f64 * time) / (100.0f64 * (time).floor()).abs());
            let mut nt: i32 = source_i32((100.0f64 * (time).floor()).abs())?;
            if (nt > 1000000i32) {
                {
                    tint = ((86400.0f64 * time) / 1000000.0f64);
                    nt = 1000000i32;
                    d.stdout(
                        &[
                            b"running ".to_vec(),
                            (nt).to_string().into_bytes(),
                            b" ints with tint ".to_vec(),
                            crate::clock_text::format_number(
                                (tint / 86400.0f64),
                                crate::clock_text::NumberFormat::GeneralSix,
                            ),
                            b"\n".to_vec(),
                        ]
                        .concat(),
                    );
                }
            }
            let mut theta: f64 = *read(&*prev_theta, "*prev_theta")?;
            let mut ec: f64 = *read(&*prev_e, "*prev_e")?;
            let mut ax: f64 = *read(&*prev_a, "*prev_a")?;
            let mut k1: f64;
            let mut k2: f64;
            let mut k3: f64;
            let mut k4: f64;
            let mut l1: f64;
            let mut l2: f64;
            let mut l3: f64;
            let mut l4: f64;
            let mut o1: f64;
            let mut o2: f64;
            let mut o3: f64;
            let mut o4: f64;
            ih = 0i32;
            while ((ih < nt) && (*read(&*coalesceFlag, "*coalesceFlag")? == 0i32)) {
                k1 = (tint * dadt(ec, ax, m1, m2, d)?);
                l1 = (tint * dedt(ec, ax, m1, m2, d)?);
                o1 = (tint * dtdt(ec, theta, *read(&*prev_p, "*prev_p")?, d)?);
                if ((ax + (k1 / 2.0f64)) < (1.5f64 * rs_m1)) {
                    {
                        *coalesceFlag = Some(1i32);
                        d.stdout(b"***********SOURCE HAS COALESCED***********\n");
                    }
                }
                k2 = (tint * dadt((ec + (l1 / 2.0f64)), (ax + (k1 / 2.0f64)), m1, m2, d)?);
                l2 = (tint * dedt((ec + (l1 / 2.0f64)), (ax + (k1 / 2.0f64)), m1, m2, d)?);
                o2 = (tint
                    * dtdt(
                        (ec + (l1 / 2.0f64)),
                        (theta + (o1 / 2.0f64)),
                        *read(&*prev_p, "*prev_p")?,
                        d,
                    )?);
                if ((ax + (k2 / 2.0f64)) < (1.5f64 * rs_m1)) {
                    {
                        *coalesceFlag = Some(1i32);
                        d.stdout(b"***********SOURCE HAS COALESCED***********\n");
                    }
                }
                k3 = (tint * dadt((ec + (l2 / 2.0f64)), (ax + (k2 / 2.0f64)), m1, m2, d)?);
                l3 = (tint * dedt((ec + (l2 / 2.0f64)), (ax + (k2 / 2.0f64)), m1, m2, d)?);
                o3 = (tint
                    * dtdt(
                        (ec + (l2 / 2.0f64)),
                        (theta + (o2 / 2.0f64)),
                        *read(&*prev_p, "*prev_p")?,
                        d,
                    )?);
                if ((ax + k3) < (1.5f64 * rs_m1)) {
                    {
                        *coalesceFlag = Some(1i32);
                        d.stdout(b"***********SOURCE HAS COALESCED***********\n");
                    }
                }
                k4 = (tint * dadt((ec + l3), (ax + k3), m1, m2, d)?);
                l4 = (tint * dedt((ec + l3), (ax + k3), m1, m2, d)?);
                o4 = (tint * dtdt((ec + l3), (theta + o3), *read(&*prev_p, "*prev_p")?, d)?);
                ax += ((((k1 + (2.0f64 * k2)) + (2.0f64 * k3)) + k4) / 6.0f64);
                ec += ((((l1 + (2.0f64 * l2)) + (2.0f64 * l3)) + l4) / 6.0f64);
                theta += ((((o1 + (2.0f64 * o2)) + (2.0f64 * o3)) + o4) / 6.0f64);
                if (ax < (1.5f64 * rs_m1)) {
                    {
                        *coalesceFlag = Some(1i32);
                        d.stdout(b"***********SOURCE HAS COALESCED***********\n");
                    }
                }
                *prev_p = Some(
                    1.0f64
                        / (((G * (m1 + m2)) * (ax).powf(-3.0f64)).sqrt()
                            / (2.0f64 * std::f64::consts::PI)),
                );
                ih += 1;
            }
            d.stdout(
                &[
                    b"Have time ".to_vec(),
                    crate::clock_text::format_number(
                        (((((read(&rows[(i) as usize].sat, "psr->obsn[i].sat")?).clone()
                            - ((read(&rows[(i) as usize].residual, "psr->obsn[i].residual")?)
                                .clone()
                                / Float::with_val(PRECISION_BITS, 86400.0f64)))
                            - Float::with_val(PRECISION_BITS, t_0))
                        .to_f64()
                            / 365.25f64)
                            / (*read(&*prev_p, "*prev_p")? / (86400.0f64 * 365.25f64))),
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b" theta ".to_vec(),
                    crate::clock_text::format_number(
                        (theta / (2.0f64 * std::f64::consts::PI)),
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b"\n".to_vec(),
                ]
                .concat(),
            );
            if (*read(&*coalesceFlag, "*coalesceFlag")? == 1i32) {
                {
                    pert = Float::with_val(PRECISION_BITS, 0.0f64);
                    if let Some(e) = energy.as_deref_mut() {
                        *e = 0.0;
                    }
                }
            } else {
                {
                    *prev_e = Some(ec);
                    *prev_epoch = Some(
                        ((read(&rows[(i) as usize].sat, "psr->obsn[i].sat")?).clone()
                            - ((read(&rows[(i) as usize].residual, "psr->obsn[i].residual")?)
                                .clone()
                                / Float::with_val(PRECISION_BITS, 86400.0f64)))
                        .to_f64(),
                    );
                    *prev_theta = Some(theta);
                    *prev_a = Some(ax);
                    let mut Ag: f64 = (((((4i32) as f64 * (G).powf(2.0f64)) * m1) * m2)
                        / ((((c).powf(4.0f64) * D) * ((1i32) as f64 - (ec * ec))) * ax));
                    let mut frontTerm: f64 = (((((-Ag)
                        * ((1i32) as f64 - (ec * ec)).powf(3.0f64 / 2.0f64))
                        * *read(&*prev_p, "*prev_p")?)
                        * (theta).sin())
                        / ((32.0f64 * std::f64::consts::PI)
                            * ((1i32) as f64 + (ec * (theta).cos()))));
                    let mut rplus: f64 = (frontTerm
                        * (((((ec * (2.0f64 * (inc - phi)).cos())
                            + (ec * (2.0f64 * (inc + phi)).cos()))
                            - (((16.0f64 * (inc).cos()) * (2.0f64 * phi).sin())
                                * (theta - (2.0f64 * theta_n)).sin()))
                            + ((((8.0f64 * ec) * (inc).cos()) * (2.0f64 * phi).sin())
                                * (2.0f64 * theta_n).sin()))
                            + ((2.0f64 * (2.0f64 * phi).cos())
                                * (((-ec)
                                    + (((3.0f64 + (2.0f64 * inc).cos())
                                        * (ec + (2.0f64 * (theta).cos())))
                                        * (2.0f64 * theta_n).cos()))
                                    + (((2.0f64 * (3.0f64 + (2.0f64 * inc).cos()))
                                        * (theta).sin())
                                        * (2.0f64 * theta_n).sin())))));
                    let mut rcross: f64 = (frontTerm
                        * ((((ec * (2.0f64 * (inc - phi)).sin())
                            - (ec * (2.0f64 * (inc + phi)).sin()))
                            + (((8.0f64 * (inc).cos()) * (2.0f64 * phi).cos())
                                * (((-2.0f64) * (theta - (2.0f64 * theta_n)).sin())
                                    + (ec * (2.0f64 * theta_n).sin()))))
                            - ((2.0f64 * (2.0f64 * phi).sin())
                                * (((-ec)
                                    + (((3.0f64 + (2.0f64 * inc).cos())
                                        * (ec + (2.0f64 * (theta).cos())))
                                        * (2.0f64 * theta_n).cos()))
                                    + (((2.0f64 * (3.0f64 + (2.0f64 * inc).cos()))
                                        * (theta).sin())
                                        * (2.0f64 * theta_n).sin())))));
                    let mut dalpha: f64 = (ra_g - ra_p);
                    let mut gplus: f64 = (((((-(1.0f64 + (3.0f64 * (2.0f64 * dec_p).cos())))
                        * ((dec_g).cos()).powf(2.0f64))
                        + (((2.0f64 * dalpha).cos() * ((-3.0f64) + (2.0f64 * dec_g).cos()))
                            * ((dec_p).sin()).powf(2.0f64)))
                        + (((2.0f64 * (dalpha).cos()) * (2.0f64 * dec_g).sin())
                            * (2.0f64 * dec_p).sin()))
                        / (8.0f64
                            * (((-1.0f64) + ((dec_p).cos() * (dec_g).sin()))
                                + (((dalpha).cos() * (dec_g).cos()) * (dec_p).sin()))));
                    let mut gcross: f64 = ((((((-(dec_g).cos()) * (dec_p).cos())
                        + (((dalpha).cos() * (dec_g).sin()) * (dec_p).sin()))
                        * (dalpha).sin())
                        * (dec_p).sin())
                        / (((-1.0f64) + ((dec_p).cos() * (dec_g).sin()))
                            + (((dalpha).cos() * (dec_g).cos()) * (dec_p).sin())));
                    pert = Float::with_val(
                        PRECISION_BITS,
                        (1e9f64 * ((rplus * gplus) + (rcross * gcross))),
                    );
                    if let Some(e) = energy.as_deref_mut() {
                        // GWsim.C:664: the prefactor rounds to float BEFORE the
                        // double product, then the final assignment rounds again.
                        // c^3/G has units kg/s; Ag/P^2 makes eOut kg/s^3 (W/m^2).
                        // This is the source estimate, not a validated total energy.
                        *e = ((c.powf(3.0) / (16.0 * std::f64::consts::PI * G)) as f32 as f64
                            * ((Ag
                                * std::f64::consts::PI.powf(3.0)
                                * (1.0 + ec * theta.cos()).powf(2.0))
                                / (prev_p.unwrap().powf(2.0) * (ec * ec - 1.0).powf(3.0)))
                            * (-1536.0
                                - 6184.0 * ec * ec
                                - 1847.0 * ec.powf(4.0)
                                - 96.0 * ec.powf(6.0)
                                + 256.0 * (4.0 * theta).cos()
                                + ec * (-4.0
                                    * (1488.0 + 1689.0 * ec * ec + 116.0 * ec.powf(4.0))
                                    * theta.cos()
                                    + 2.0
                                        * ec
                                        * (-1270.0 - 243.0 * ec * ec + 48.0 * ec.powf(4.0))
                                        * (2.0 * theta).cos()
                                    + 4.0
                                        * (32.0 + 267.0 * ec * ec + 76.0 * ec.powf(4.0))
                                        * (3.0 * theta).cos()
                                    + 2.0 * ec * (200.0 + 479.0 * ec * ec) * (4.0 * theta).cos()
                                    + 4.0
                                        * (176.0 + 87.0 * ec * ec + 40.0 * ec.powf(4.0))
                                        * (5.0 * theta).cos()
                                    + 14.0 * ec * (46.0 + 5.0 * ec * ec) * (6.0 * theta).cos()
                                    + 220.0 * ec * ec * (7.0 * theta).cos()
                                    + 25.0 * ec.powf(3.0) * (8.0 * theta).cos())))
                            as f32;
                    }
                }
            }
            d.stdout(
                &[
                    b"Have pert ".to_vec(),
                    crate::clock_text::extended_general_six(&((pert).clone())),
                    if let Some(e) = energy.as_deref() {
                        format!(" energy {:.6}", e).into_bytes()
                    } else {
                        vec![]
                    },
                    b"\n".to_vec(),
                ]
                .concat(),
            );
            Ok((pert).clone())
        }
    }
}

/// GWsim.C::Rs (pinned source line 682).
pub(super) fn Rs(mut m1: f64, d: &mut Diagnostics) -> Result<f64, TimingError> {
    let mut G: f64 = 6.67e-11f64;
    let mut c: f64 = 2.998e8f64;
    let mut retval: f64 = ((((2i32) as f64 * G) * m1) / (c * c));
    Ok(retval)
}

/// GWsim.C::dadt (pinned source line 695).
pub(super) fn dadt(
    mut ec: f64,
    mut a: f64,
    mut m1: f64,
    mut m2: f64,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut G: f64 = 6.67e-11f64;
    let mut c: f64 = 2.998e8f64;
    let mut retval: f64 = ((((((((-64.0f64) / 5.0f64) * (G / a).powf(3.0f64)) * m1) * m2)
        * (m1 + m2))
        * (c).powf(-5.0f64))
        * Fe(ec, d)?);
    Ok(retval)
}

/// GWsim.C::dedt (pinned source line 707).
pub(super) fn dedt(
    mut ec: f64,
    mut a: f64,
    mut m1: f64,
    mut m2: f64,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut G: f64 = 6.67e-11f64;
    let mut c: f64 = 2.998e8f64;
    let mut retval: f64 = (((((((((((-304.0f64) / 15.0f64) * (G).powf(3.0f64)) * m1) * m2)
        * (m1 + m2))
        * (c).powf(-5.0f64))
        * (a).powf(-4.0f64))
        * (1.0f64 - (ec * ec)).powf((-5.0f64) / 2.0f64))
        * ec)
        * (1.0f64 + (((121.0f64 * ec) * ec) / 304.0f64)));
    Ok(retval)
}

/// GWsim.C::dtdt (pinned source line 719).
pub(super) fn dtdt(
    mut ec: f64,
    mut t: f64,
    mut p: f64,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut retval: f64 = ((2.0f64 * std::f64::consts::PI) / p);
    Ok(retval)
}

/// GWsim.C::Fe (pinned source line 732).
pub(super) fn Fe(mut ec: f64, d: &mut Diagnostics) -> Result<f64, TimingError> {
    let mut retval: f64 = ((1.0f64 - (ec * ec)).powf((-7.0f64) / 2.0f64)
        * ((1.0f64 + ((73.0f64 * (ec).powf(2.0f64)) / 24.0f64))
            + ((37.0f64 * (ec).powf(4.0f64)) / 96.0f64)));
    Ok(retval)
}
