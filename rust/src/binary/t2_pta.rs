//! T2-PTAmodel.C model, derivatives, updates and private helpers.
//! TEMPO2 fda6ad4; GPL-3.0-or-later; original TEMPO2 contributors.
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
use super::*;
/// T2-PTAmodel.C::T2_PTAmodel (pinned source line 40).
pub(super) fn T2_PTAmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    mut arr: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut SUNMASS: Float = Float::with_val(PRECISION_BITS, 4.925490947e-6f64);
    let mut T0: Float;
    let mut tbbat: Float;
    let mut dt: Float;
    let mut e: Float;
    let mut e0: Float;
    let mut edot: Float;
    let mut pb: Float;
    let mut nval: Float;
    let mut orbits: Float;
    let mut u: Float;
    let mut norbits: i32;
    let mut phase: Float;
    let mut Ae: Float;
    let mut kval: Float;
    let mut omega: Float;
    let mut Sval: Float;
    let mut basicRoemer: Float;
    let mut x: Float;
    let mut pbdot: Float;
    let mut deltaB: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut dDB: Float;
    let mut deltaSB: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut model: Float;
    let mut r: Option<Float> = None;
    let mut s: Option<Float> = None;
    let mut deltaSR: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut deltaAOP: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut kom: Option<Float> = None;
    let mut kin: Option<Float> = None;
    let mut pmra: Option<Float> = None;
    let mut pmdec: Option<Float> = None;
    let mut dt0: Option<Float> = None;
    let mut Cval: Option<Float> = None;
    d.version("T2-PTAmodel.C", "T2_PTAmodel()");
    if (psr.param.slots[param_t0 as usize].cells[0].param_set == 0i32) {
        {
            d.stdout(b"Must use T0 in the binary model\n");
            return Err(boundary("T2-PTAmodel.C::T2_PTAmodel: source exit"));
        }
    }
    T0 = (psr.param.slots[param_t0 as usize].cells[0].val).clone();
    loop {
        {
            tbbat = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                - ((deltaB).clone()
                    / Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap())));
            dt = (((tbbat).clone() - (T0).clone())
                * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()));
            if (psr.param.slots[param_ecc as usize].cells[0].param_set == 0i32) {
                {
                    d.stdout(b"Must use ECC in the binary model\n");
                    return Err(boundary("T2-PTAmodel.C::T2_PTAmodel: source exit"));
                }
            }
            e0 = (psr.param.slots[param_ecc as usize].cells[0].val).clone();
            if (psr.param.slots[param_edot as usize].cells[0].param_set == 1i32) {
                edot = (psr.param.slots[param_edot as usize].cells[0].val).clone();
            } else {
                edot = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
            }
            e = ((e0).clone() + ((edot).clone() * (dt).clone()));
            if (psr.param.slots[param_pb as usize].cells[0].param_set == 0i32) {
                {
                    d.stdout(b"Must use PB in the binary model\n");
                    return Err(boundary("T2-PTAmodel.C::T2_PTAmodel: source exit"));
                }
            }
            pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
                * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()));
            nval = ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                / (pb).clone());
            pbdot = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
            if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
                {
                    pbdot = (psr.param.slots[param_pbdot as usize].cells[0].val).clone();
                    nval = ((nval).clone()
                        + ((((Float::with_val(PRECISION_BITS, std::f64::consts::PI)
                            * (pbdot).clone())
                            * (dt).clone())
                            / (pb).clone())
                            / (pb).clone()));
                }
            }
            orbits = ((((nval).clone()
                / Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap()))
                / Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                * (dt).clone());
            norbits = crate::ifteph::extended_i32(&((orbits).clone()), "binary_integer")?;
            if ((orbits).clone() < Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap())) {
                norbits = norbits
                    .checked_sub(1)
                    .ok_or_else(|| boundary("binary orbit count overflow"))?;
            }
            phase = ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                * ((orbits).clone() - Float::with_val(PRECISION_BITS, norbits)));
            u = computeU((phase).clone(), (e).clone(), d)?;
            Ae = (Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                * (((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                    + (e).clone())
                    * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                        + (e).clone()))
                    * ((u).clone() / Float::with_val(PRECISION_BITS, 2.0f64)).tan())
                .atan2(
                    &((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                        - (e).clone())
                        * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                            - (e).clone())),
                ));
            if ((Ae).clone() < Float::with_val(PRECISION_BITS, 0.0f64)) {
                Ae = ((Ae).clone()
                    + Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)));
            }
            if (psr.param.slots[param_omdot as usize].cells[0].param_set == 1i32) {
                {
                    kval = (((((psr.param.slots[param_omdot as usize].cells[0].val).clone()
                        * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                        / Float::with_val(PRECISION_BITS, 180.0f64))
                        / (Float::with_val(PRECISION_BITS, 365.25f64)
                            * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap())))
                        / (nval).clone());
                }
            } else {
                kval = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
            }
            if (psr.param.slots[param_om as usize].cells[0].param_set == 0i32) {
                {
                    d.stdout(b"Must use OM in the binary model\n");
                    return Err(boundary("T2-PTAmodel.C::T2_PTAmodel: source exit"));
                }
            }
            omega = ((((psr.param.slots[param_om as usize].cells[0].val).clone()
                * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                / Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                + ((kval).clone() * (Ae).clone()));
            Sval = ((((omega).clone()).sin() * (((u).clone()).cos() - (e).clone()))
                + ((((omega).clone()).cos()
                    * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                        - ((e).clone() * (e).clone()))
                    .sqrt())
                    * ((u).clone()).sin()));
            if (psr.param.slots[param_a1 as usize].cells[0].param_set == 0i32) {
                {
                    d.stdout(b"Must use A1 in the binary model\n");
                    return Err(boundary("T2-PTAmodel.C::T2_PTAmodel: source exit"));
                }
            }
            x = (psr.param.slots[param_a1 as usize].cells[0].val).clone();
            if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
                x = ((x).clone()
                    + ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()
                        * (dt).clone()));
            }
            basicRoemer = ((x).clone() * (Sval).clone());
            if ((psr.param.slots[param_kin as usize].cells[0].param_set == 1i32)
                && (psr.param.slots[param_m2 as usize].cells[0].param_set == 1i32))
            {
                {
                    kin = Some(
                        ((psr.param.slots[param_kin as usize].cells[0].val).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()),
                    );
                    s = Some(((read(&kin, "kin")?).clone()).sin());
                    r = Some(
                        (psr.param.slots[param_m2 as usize].cells[0].val).clone()
                            * (SUNMASS).clone(),
                    );
                    deltaSB = ((Float::with_val(PRECISION_BITS, (-2i32))
                        * (read(&r, "r")?).clone())
                        * ((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                            - ((e).clone() * ((u).clone()).cos()))
                            - ((read(&s, "s")?).clone()
                                * ((((omega).clone()).sin()
                                    * (((u).clone()).cos() - (e).clone()))
                                    + (((Float::with_val(PRECISION_BITS, 1.0f64)
                                        - ((e).clone() * (e).clone()))
                                    .sqrt()
                                        * ((omega).clone()).cos())
                                        * ((u).clone()).sin()))))
                        .ln());
                }
            }
            if ((((psr.param.slots[param_pmra as usize].cells[0].param_set == 1i32)
                && (psr.param.slots[param_pmdec as usize].cells[0].param_set == 1i32))
                && (psr.param.slots[param_kin as usize].cells[0].param_set == 1i32))
                && (psr.param.slots[param_kom as usize].cells[0].param_set == 1i32))
            {
                {
                    dt0 = Some(
                        ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                            - (psr.param.slots[param_posepoch as usize].cells[0].val).clone())
                            * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()),
                    );
                    kin = Some(
                        ((psr.param.slots[param_kin as usize].cells[0].val).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, 180.0f64),
                    );
                    kom = Some(
                        ((psr.param.slots[param_kom as usize].cells[0].val).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, 180.0f64),
                    );
                    pmra = Some(
                        (((psr.param.slots[param_pmra as usize].cells[0].val).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, (180.0f64 * 3600.0e3f64)))
                            / (Float::with_val(PRECISION_BITS, 365.25f64)
                                * Float::with_val(
                                    PRECISION_BITS,
                                    Float::parse("86400.0").unwrap(),
                                )),
                    );
                    pmdec = Some(
                        (((psr.param.slots[param_pmdec as usize].cells[0].val).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, (180.0f64 * 3600.0e3f64)))
                            / (Float::with_val(PRECISION_BITS, 365.25f64)
                                * Float::with_val(
                                    PRECISION_BITS,
                                    Float::parse("86400.0").unwrap(),
                                )),
                    );
                    Cval = Some(
                        (((omega).clone()).cos() * (((u).clone()).cos() - (e).clone()))
                            - ((((omega).clone()).sin()
                                * Float::with_val(
                                    PRECISION_BITS,
                                    ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - ((e).clone() * (e).clone()))
                                    .to_f64())
                                    .sqrt(),
                                ))
                                * ((u).clone()).sin()),
                    );
                    deltaSR = (((x).clone() * (read(&dt0, "dt0")?).clone())
                        * ((((((read(&pmra, "pmra")?).clone()
                            * ((read(&kom, "kom")?).clone()).sin())
                            + ((read(&pmdec, "pmdec")?).clone()
                                * ((read(&kom, "kom")?).clone()).cos()))
                            * (read(&Cval, "Cval")?).clone())
                            / ((read(&kin, "kin")?).clone()).sin())
                            + (((((read(&pmra, "pmra")?).clone()
                                * ((read(&kom, "kom")?).clone()).cos())
                                - ((read(&pmdec, "pmdec")?).clone()
                                    * ((read(&kom, "kom")?).clone()).sin()))
                                * (Sval).clone())
                                / ((read(&kin, "kin")?).clone()).tan())));
                    {
                        let mut pi_daop: Float;
                        let mut rdote1: Float;
                        let mut rdote2: Float;
                        let mut sin_delta: Float;
                        let mut cos_delta: Float;
                        let mut sin_alpha: Float;
                        let mut cos_alpha: Float;
                        let mut px: Float;
                        sin_delta = Float::with_val(
                            PRECISION_BITS,
                            *read(
                                &rows[(ipos) as usize].psrPos[2],
                                "psr->obsn[ipos].psrPos[2]",
                            )?,
                        );
                        cos_delta = (((sin_delta).clone()).asin()).cos();
                        sin_alpha = (Float::with_val(
                            PRECISION_BITS,
                            *read(
                                &rows[(ipos) as usize].psrPos[1],
                                "psr->obsn[ipos].psrPos[1]",
                            )?,
                        ) / (cos_delta).clone());
                        cos_alpha = (Float::with_val(
                            PRECISION_BITS,
                            *read(
                                &rows[(ipos) as usize].psrPos[0],
                                "psr->obsn[ipos].psrPos[0]",
                            )?,
                        ) / (cos_delta).clone());
                        rdote1 = ((Float::with_val(
                            PRECISION_BITS,
                            ((-*read(
                                &rows[(ipos) as usize].earth_ssb[0],
                                "psr->obsn[ipos].earth_ssb[0]",
                            )?) / 499.00478364f64),
                        ) * (sin_alpha).clone())
                            + (Float::with_val(
                                PRECISION_BITS,
                                (*read(
                                    &rows[(ipos) as usize].earth_ssb[1],
                                    "psr->obsn[ipos].earth_ssb[1]",
                                )? / 499.00478364f64),
                            ) * (cos_alpha).clone()));
                        rdote2 = ((((Float::with_val(
                            PRECISION_BITS,
                            ((-*read(
                                &rows[(ipos) as usize].earth_ssb[0],
                                "psr->obsn[ipos].earth_ssb[0]",
                            )?) / 499.00478364f64),
                        ) * (sin_delta).clone())
                            * (cos_alpha).clone())
                            - ((Float::with_val(
                                PRECISION_BITS,
                                (*read(
                                    &rows[(ipos) as usize].earth_ssb[1],
                                    "psr->obsn[ipos].earth_ssb[1]",
                                )? / 499.00478364f64),
                            ) * (sin_delta).clone())
                                * (sin_alpha).clone()))
                            + (Float::with_val(
                                PRECISION_BITS,
                                (*read(
                                    &rows[(ipos) as usize].earth_ssb[2],
                                    "psr->obsn[ipos].earth_ssb[2]",
                                )? / 499.00478364f64),
                            ) * (cos_delta).clone()));
                        px = (psr.param.slots[param_px as usize].cells[0].val).clone();
                        pi_daop = (((((px).clone()
                            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                            / Float::with_val(PRECISION_BITS, 180.0f64))
                            / Float::with_val(PRECISION_BITS, 3600.0f64))
                            * Float::with_val(PRECISION_BITS, 1e-3f64));
                        deltaAOP = (((-(x).clone()) * (pi_daop).clone())
                            * ((((((rdote1).clone() * ((read(&kom, "kom")?).clone()).sin())
                                + ((rdote2).clone() * ((read(&kom, "kom")?).clone()).cos()))
                                * (read(&Cval, "Cval")?).clone())
                                / ((read(&kin, "kin")?).clone()).sin())
                                + (((((rdote1).clone() * ((read(&kom, "kom")?).clone()).cos())
                                    - ((rdote2).clone()
                                        * ((read(&kom, "kom")?).clone()).sin()))
                                    * (Sval).clone())
                                    / ((read(&kin, "kin")?).clone()).tan())));
                    }
                }
            }
            model = ((((basicRoemer).clone() + (deltaSB).clone()) + (deltaSR).clone())
                + (deltaAOP).clone());
            dDB = ((model).clone() - (deltaB).clone());
            deltaB = (model).clone();
        }
        if !(((dDB).clone()).abs() > Float::with_val(PRECISION_BITS, 1e-14f64)) {
            break;
        }
    }
    d.stdout(
        &[
            b"basicRoemer = ".to_vec(),
            crate::clock_text::format_number(
                ((basicRoemer).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b"\n".to_vec(),
        ]
        .concat(),
    );
    if (param == (-1i32)) {
        return Ok((-(model).clone()).to_f64());
    }
    let mut du_dpb: Float;
    let mut dD_dpb: Float;
    let mut dD_da1: Float;
    let mut dD_domega: Float;
    let mut du_dT0: Float;
    let mut dD_dT0: Float;
    let mut du_de: Float;
    let mut dD_de: Float;
    let mut du_dpdot: Float;
    let mut dD_dpdot: Float;
    let mut dD_domegaDot: Float;
    let mut domega_domegaDot: Float;
    let mut dD_dkom: Option<Float> = None;
    let mut dD_di: Float;
    let mut dD_dm2: Float;
    du_dpb = ((((((-Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap()))
        * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
        * (dt).clone())
        / (pb).clone())
        / (pb).clone())
        / (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
            - ((e).clone() * ((u).clone()).cos())));
    du_dT0 = ((((-Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap()))
        * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
        / (pb).clone())
        / (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
            - ((e).clone() * ((u).clone()).cos())));
    du_de = (((u).clone()).sin()
        / (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
            - ((e).clone() * ((u).clone()).cos())));
    du_dpdot = (((((Float::with_val(PRECISION_BITS, std::f64::consts::PI) * (dt).clone())
        * (dt).clone())
        / (pb).clone())
        / (pb).clone())
        / (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
            - ((e).clone() * ((u).clone()).cos())));
    domega_domegaDot = ((Ae).clone()
        / (((((nval).clone() * Float::with_val(PRECISION_BITS, 180.0f64))
            / Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            * Float::with_val(PRECISION_BITS, 365.25f64))
            * Float::with_val(PRECISION_BITS, 86400.0f64)));
    dD_da1 = ((((omega).clone()).sin() * (((u).clone()).cos() - (e).clone()))
        + ((((omega).clone()).cos()
            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * ((u).clone()).sin()));
    dD_de = (((((((-(x).clone()) * ((omega).clone()).sin()) * ((u).clone()).sin())
        * (du_de).clone())
        - ((x).clone() * ((omega).clone()).sin()))
        - (((((x).clone() * ((omega).clone()).cos())
            / (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * (e).clone())
            * ((u).clone()).sin()))
        + (((((x).clone() * ((omega).clone()).cos())
            * (Float::with_val(PRECISION_BITS, 1i32) - ((e).clone() * (e).clone())).sqrt())
            * ((u).clone()).cos())
            * (du_de).clone()));
    dD_dT0 = (((((-(x).clone()) * ((omega).clone()).sin()) * ((u).clone()).sin())
        + ((((x).clone() * ((omega).clone()).cos())
            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * ((u).clone()).cos()))
        * (du_dT0).clone());
    dD_dpb = (((((-(x).clone()) * ((omega).clone()).sin()) * ((u).clone()).sin())
        + ((((x).clone() * ((omega).clone()).cos())
            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * ((u).clone()).cos()))
        * (du_dpb).clone());
    dD_dpdot = (((((-(x).clone()) * ((omega).clone()).sin()) * ((u).clone()).sin())
        + ((((x).clone() * ((omega).clone()).cos())
            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * ((u).clone()).cos()))
        * (du_dpdot).clone());
    dD_domega = ((((x).clone() * (((u).clone()).cos() - (e).clone())) * ((omega).clone()).cos())
        - ((((x).clone()
            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                - ((e).clone() * (e).clone()))
            .sqrt())
            * ((u).clone()).sin())
            * ((omega).clone()).sin()));
    dD_domegaDot = ((dD_domega).clone() * (domega_domegaDot).clone());
    dD_dm2 = ((deltaSB).clone() / (read(&r, "r")?).clone());
    dD_di = (((Float::with_val(PRECISION_BITS, (-2i32)) * (read(&r, "r")?).clone())
        / ((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
            - ((e).clone() * ((u).clone()).cos()))
            - ((read(&s, "s")?).clone()
                * ((((omega).clone()).sin() * (((u).clone()).cos() - (e).clone()))
                    + (((Float::with_val(PRECISION_BITS, 1.0f64)
                        - ((e).clone() * (e).clone()))
                    .sqrt()
                        * ((omega).clone()).cos())
                        * ((u).clone()).sin())))))
        * ((-((read(&kin, "kin")?).clone()).cos())
            * ((((omega).clone()).sin() * (((u).clone()).cos() - (e).clone()))
                + (((Float::with_val(PRECISION_BITS, 1.0f64) - ((e).clone() * (e).clone()))
                    .sqrt()
                    * ((omega).clone()).cos())
                    * ((u).clone()).sin()))));
    if (param == (param_kom as i32)) {
        dD_dkom = Some(
            ((x).clone() * (read(&dt0, "dt0")?).clone())
                * ((((read(&Cval, "Cval")?).clone() / ((read(&kin, "kin")?).clone()).sin())
                    * (((read(&pmra, "pmra")?).clone() * ((read(&kom, "kom")?).clone()).cos())
                        - ((read(&pmdec, "pmdec")?).clone()
                            * ((read(&kom, "kom")?).clone()).sin())))
                    + (((Sval).clone() / ((read(&kin, "kin")?).clone()).tan())
                        * (((-(read(&pmra, "pmra")?).clone())
                            * ((read(&kom, "kom")?).clone()).sin())
                            - ((read(&pmdec, "pmdec")?).clone()
                                * ((read(&kom, "kom")?).clone()).cos())))),
        );
    }
    d.stdout(
        &[
            b"output: ".to_vec(),
            crate::clock_text::format_number(
                ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b" ".to_vec(),
            crate::clock_text::format_number(
                ((u).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b" ".to_vec(),
            crate::clock_text::format_number(
                ((Ae).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b" ".to_vec(),
            crate::clock_text::format_number(
                ((nval).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b" ".to_vec(),
            crate::clock_text::format_number(
                ((deltaSR).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b" ".to_vec(),
            crate::clock_text::format_number(
                ((deltaAOP).clone()).to_f64(),
                crate::clock_text::NumberFormat::GeneralSix,
            ),
            b"\n".to_vec(),
        ]
        .concat(),
    );
    if (param == (param_pb as i32)) {
        Ok(((dD_dpb).clone()).to_f64())
    } else {
        if (param == (param_pbdot as i32)) {
            Ok(((dD_dpdot).clone()).to_f64())
        } else {
            if (param == (param_a1 as i32)) {
                Ok(((dD_da1).clone()).to_f64())
            } else {
                if (param == (param_a1dot as i32)) {
                    Ok(((dD_da1).clone() * (dt).clone()).to_f64())
                } else {
                    if (param == (param_om as i32)) {
                        Ok(((dD_domega).clone()).to_f64())
                    } else {
                        if (param == (param_omdot as i32)) {
                            Ok(((dD_domegaDot).clone()).to_f64())
                        } else {
                            if (param == (param_kom as i32)) {
                                Ok(((read(&dD_dkom, "dD_dkom")?).clone()).to_f64())
                            } else {
                                if (param == (param_kin as i32)) {
                                    Ok(((dD_di).clone()).to_f64())
                                } else {
                                    if (param == (param_m2 as i32)) {
                                        Ok(((dD_dm2).clone()).to_f64())
                                    } else {
                                        if (param == (param_t0 as i32)) {
                                            {
                                                d.stdout(&[b"dT0 = ".to_vec(),crate::clock_text::format_number(((dD_dT0).clone()).to_f64(), crate::clock_text::NumberFormat::GeneralSix),b"\n".to_vec()].concat());
                                                Ok(((dD_dT0).clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("86400.0").unwrap(),
                                                    ))
                                                .to_f64())
                                            }
                                        } else {
                                            if (param == (param_ecc as i32)) {
                                                Ok(((dD_de).clone()).to_f64())
                                            } else {
                                                Ok((0i32) as f64)
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
    }
}

/// T2-PTAmodel.C::computeU (pinned source line 283).
pub(super) fn computeU(
    mut phase: Float,
    mut ecc: Float,
    d: &mut Diagnostics,
) -> Result<Float, TimingError> {
    let mut du: Float;
    let mut u: Float;
    u = ((phase).clone()
        + (((ecc).clone() * ((phase).clone()).sin())
            * (Float::with_val(PRECISION_BITS, 1.0f64)
                + ((ecc).clone() * ((phase).clone()).cos()))));
    loop {
        {
            du = (((phase).clone() - ((u).clone() - ((ecc).clone() * ((u).clone()).sin())))
                / (Float::with_val(PRECISION_BITS, 1.0f64)
                    - ((ecc).clone() * ((u).clone()).cos())));
            u = ((u).clone() + (du).clone());
        }
        if !(((du).clone()).abs() > Float::with_val(PRECISION_BITS, 1.0e-14f64)) {
            break;
        }
    }
    Ok((u).clone())
}

/// T2-PTAmodel.C::updateT2_PTA (pinned source line 298).
pub(super) fn updateT2_PTA(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut val: f64,
    mut err: f64,
    mut pos: i32,
    mut arr: i32,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut SUNMASS: Float = Float::with_val(PRECISION_BITS, 4.925490947e-6f64);
    if (pos == (param_pb as i32)) {
        {
            d.stdout(
                &[
                    b"In here with param_pb: ".to_vec(),
                    crate::clock_text::format_number(
                        ((psr.param.slots[(pos) as usize].cells[0].val).clone()).to_f64(),
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b" ".to_vec(),
                    crate::clock_text::format_number(
                        val,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b" ".to_vec(),
                    crate::clock_text::format_number(
                        err,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b"\n".to_vec(),
                ]
                .concat(),
            );
            psr.param.slots[(pos) as usize].cells[0].val =
                ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                    + Float::with_val(PRECISION_BITS, (val / 86400.0f64)));
            psr.param.slots[(pos) as usize].cells[0].err =
                Float::with_val(PRECISION_BITS, (err / 86400.0f64));
        }
    } else {
        if (pos == (param_om as i32)) {
            {
                psr.param.slots[(pos) as usize].cells[0].val =
                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                        + ((Float::with_val(PRECISION_BITS, val)
                            * Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                            / Float::with_val(PRECISION_BITS, std::f64::consts::PI)));
                psr.param.slots[(pos) as usize].cells[0].err =
                    ((Float::with_val(PRECISION_BITS, err)
                        * Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                        / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
            }
        } else {
            if (pos == (param_kom as i32)) {
                {
                    psr.param.slots[(pos) as usize].cells[0].val = ((psr.param.slots
                        [(pos) as usize]
                        .cells[0]
                        .val)
                        .clone()
                        + ((Float::with_val(PRECISION_BITS, val)
                            * Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                            / Float::with_val(PRECISION_BITS, std::f64::consts::PI)));
                    psr.param.slots[(pos) as usize].cells[0].err =
                        ((Float::with_val(PRECISION_BITS, err)
                            * Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                            / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                }
            } else {
                if (pos == (param_kin as i32)) {
                    {
                        psr.param.slots[(pos) as usize].cells[0].val =
                            ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                + ((Float::with_val(PRECISION_BITS, val)
                                    * Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("180.0").unwrap(),
                                    ))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI)));
                        psr.param.slots[(pos) as usize].cells[0].err =
                            ((Float::with_val(PRECISION_BITS, err)
                                * Float::with_val(PRECISION_BITS, Float::parse("180.0").unwrap()))
                                / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                    }
                } else {
                    if (pos == (param_m2 as i32)) {
                        {
                            psr.param.slots[(pos) as usize].cells[0].val =
                                ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                    + (Float::with_val(PRECISION_BITS, val) / (SUNMASS).clone()));
                            psr.param.slots[(pos) as usize].cells[0].err =
                                (Float::with_val(PRECISION_BITS, err) / (SUNMASS).clone());
                        }
                    } else {
                        if (pos == (param_omdot as i32)) {
                            {
                                psr.param.slots[(pos) as usize].cells[0].val =
                                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                        + Float::with_val(PRECISION_BITS, val));
                                psr.param.slots[(pos) as usize].cells[0].err =
                                    Float::with_val(PRECISION_BITS, err);
                            }
                        } else {
                            {
                                psr.param.slots[(pos) as usize].cells[0].val =
                                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                        + Float::with_val(PRECISION_BITS, val));
                                psr.param.slots[(pos) as usize].cells[0].err =
                                    Float::with_val(PRECISION_BITS, err);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
