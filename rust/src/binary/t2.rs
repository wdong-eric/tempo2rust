//! T2model.C model, derivatives, updates and private helpers.
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
/// T2model.C::T2model (pinned source line 85).
pub(super) fn T2model(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    mut arr: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: Option<f64> = None;
    let mut pb: Option<f64> = None;
    let mut omdot: Option<f64> = None;
    let mut rad2deg: f64 = (180.0f64 / std::f64::consts::PI);
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut tt0: Float;
    let mut t0: Option<Float> = None;
    let mut ct: Float;
    let mut t0asc: Option<Float> = None;
    let mut m2: Option<f64> = None;
    let mut x: Option<f64> = None;
    let mut ecc: Option<f64> = None;
    let mut er: Option<f64> = None;
    let mut xdot: Option<f64> = None;
    let mut x2dot: Option<f64> = None;
    let mut edot: Option<f64> = None;
    let mut dr: Option<f64> = None;
    let mut dth: Option<f64> = None;
    let mut eth: Option<f64> = None;
    let mut pbdot: Option<f64> = None;
    let mut pb2dot: Option<f64> = None;
    let mut xpbdot: Option<f64> = None;
    let mut phase: f64;
    let mut u: Option<f64> = None;
    let mut gamma: Option<f64> = None;
    let mut orbits: f64;
    let mut norbits: i32;
    let mut cu: Option<f64> = None;
    let mut onemecu: f64 = (0i32) as f64;
    let mut cae: f64;
    let mut sae: f64;
    let mut ae: Option<f64> = None;
    let mut omega: f64;
    let mut omz: Option<f64> = None;
    let mut sw: Option<f64> = None;
    let mut cw: Option<f64> = None;
    let mut alpha: f64;
    let mut beta: f64;
    let mut bg: f64;
    let mut dre: f64;
    let mut drep: f64;
    let mut drepp: f64;
    let mut anhat: f64;
    let mut su: f64 = (0i32) as f64;
    let mut sqr1me2: Option<f64> = None;
    let mut cume: Option<f64> = None;
    let mut brace: f64;
    let mut si: Option<f64> = None;
    let mut dlogbr: f64;
    let mut ds: f64;
    let mut da: f64;
    let mut a0: Option<f64> = None;
    let mut b0: Option<f64> = None;
    let mut d2bar: f64;
    let mut torb: Option<f64> = None;
    let mut csigma: Option<f64> = None;
    let mut ce: Option<f64> = None;
    let mut cx: Option<f64> = None;
    let mut comega: Option<f64> = None;
    let mut cgamma: Option<f64> = None;
    let mut cm2: Option<f64> = None;
    let mut csi: Option<f64> = None;
    let mut ckom: f64;
    let mut ckin: f64;
    let mut eps1: Option<f64> = None;
    let mut eps2: Option<f64> = None;
    let mut eps1dot: Option<f64> = None;
    let mut eps2dot: Option<f64> = None;
    let mut ceps1: Option<f64> = None;
    let mut ceps2: Option<f64> = None;
    let mut shapmax: Option<f64> = None;
    let mut cshapmax: Option<f64> = None;
    let mut sdds: Option<f64> = None;
    let mut com: i32;
    let mut com1: i32;
    let mut com2: i32;
    let mut allTerms: i32 = 1i32;
    let mut dpara: Option<f64> = None;
    let mut pmra: Option<f64> = None;
    let mut pmdec: Option<f64> = None;
    let mut sin_omega: Option<f64> = None;
    let mut cos_omega: Option<f64> = None;
    let mut ki: Option<f64> = None;
    let mut mtot: Option<f64> = None;
    let mut m1: Option<f64> = None;
    let mut xk: Option<f64> = None;
    let mut xomdot: Option<f64> = None;
    let mut afac: Option<f64> = None;
    let mut kom: Option<f64> = None;
    let mut DK011: Option<Float> = None;
    let mut DK012: Option<Float> = None;
    let mut DK021: Option<Float> = None;
    let mut DK022: Option<Float> = None;
    let mut DK031: Option<Float> = None;
    let mut DK032: Option<Float> = None;
    let mut DK041: Option<Float> = None;
    let mut DK042: Option<Float> = None;
    let mut C: Option<Float> = None;
    let mut S: Option<Float> = None;
    let mut DK013: Option<Float> = None;
    let mut DK014: Option<Float> = None;
    let mut DK023: Option<Float> = None;
    let mut DK024: Option<Float> = None;
    let mut DK033: Option<Float> = None;
    let mut DK034: Option<Float> = None;
    let mut DK043: Option<Float> = None;
    let mut DK044: Option<Float> = None;
    let mut DAOP: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut DSR: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut DOP: Float = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
    let mut daop: Option<f64> = None;
    let mut h3: Option<Float> = None;
    let mut h4: Option<Float> = None;
    let mut stig: Option<Float> = None;
    let mut lgf: Option<Float> = None;
    let mut TrueAnom: Option<Float> = None;
    let mut lsc: Option<Float> = None;
    let mut fs: Option<Float> = None;
    let mut nharm: i32 = 4i32;
    let mut mode: i32 = (-1i32);
    torb = Some(0.0f64);
    d.version("T2model.C", "T2model()");
    if (param == (-1i32)) {
        {
            com1 = 0i32;
            com2 = *read(&psr.nCompanion, "psr[p].nCompanion")?;
        }
    } else {
        {
            com1 = arr;
            com2 = *read(&psr.nCompanion, "psr[p].nCompanion")?;
        }
    }
    let mut outer_delay: f64 = 0.0f64;
    let mut inner_derivative_dT0: f64 = 0.0f64;
    {
        com = (com2 - 1i32);
        while (com >= com1) {
            {
                getKeplerian(
                    psr,
                    rows,
                    com,
                    &mut pb,
                    &mut t0,
                    &mut ecc,
                    &mut omz,
                    &mut x,
                    &mut eps1,
                    &mut eps2,
                    &mut t0asc,
                    &mut shapmax,
                    &mut kom,
                    &mut ki,
                    d,
                )?;
                addKeplerianJumps(
                    psr, rows, ipos, &mut torb, &mut x, &mut ecc, &mut omz, &mut pb, d,
                )?;
                deriveKeplerian(
                    *read(&pb, "pb")?,
                    *read(&kom, "kom")?,
                    &mut an,
                    &mut sin_omega,
                    &mut cos_omega,
                    d,
                )?;
                getPostKeplerian(
                    psr,
                    rows,
                    com,
                    *read(&an, "an")?,
                    &mut si,
                    &mut m2,
                    &mut mtot,
                    &mut omdot,
                    &mut gamma,
                    &mut xdot,
                    &mut x2dot,
                    &mut xpbdot,
                    &mut pbdot,
                    &mut pb2dot,
                    &mut edot,
                    &mut pmra,
                    &mut pmdec,
                    &mut dpara,
                    &mut dr,
                    &mut dth,
                    &mut a0,
                    &mut b0,
                    &mut xomdot,
                    &mut afac,
                    &mut eps1dot,
                    &mut eps2dot,
                    &mut daop,
                    d,
                )?;
                if ((psr.param.slots[param_bp as usize].cells[0].param_set == 1i32)
                    && (psr.param.slots[param_bpp as usize].cells[0].param_set == 1i32))
                {
                    {
                        d.stdout(b"Beta model not implemented yet\n");
                    }
                }
                if (companion_set(psr, param_mtot, com)) {
                    {
                        calcGR(
                            *read(&mtot, "mtot")?,
                            *read(&m2, "m2")?,
                            *read(&x, "x")?,
                            *read(&ecc, "ecc")?,
                            *read(&an, "an")?,
                            *read(&afac, "afac")?,
                            ((psr.param.slots[param_f as usize].cells[0].val).clone()).to_f64(),
                            &mut dr,
                            &mut dth,
                            &mut er,
                            &mut eth,
                            &mut xk,
                            &mut si,
                            &mut gamma,
                            &mut pbdot,
                            &mut a0,
                            &mut b0,
                            d,
                        )?;
                        omdot = Some(*read(&xomdot, "xomdot")? + *read(&xk, "xk")?);
                    }
                }
                derivePostKeplerian(
                    *read(&mtot, "mtot")?,
                    *read(&m2, "m2")?,
                    *read(&dr, "dr")?,
                    *read(&dth, "dth")?,
                    *read(&ecc, "ecc")?,
                    &mut m1,
                    &mut er,
                    &mut eth,
                    d,
                )?;
                ct = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                    - Float::with_val(PRECISION_BITS, (outer_delay / 86400.0f64)));
                if (companion_set(psr, param_t0, com)) {
                    tt0 = (((ct).clone() - (read(&t0, "t0")?).clone())
                        * Float::with_val(PRECISION_BITS, 86400.0f64));
                } else {
                    if (companion_set(psr, param_tasc, com)) {
                        tt0 = (((ct).clone() - (read(&t0asc, "t0asc")?).clone())
                            * Float::with_val(PRECISION_BITS, 86400.0f64));
                    } else {
                        {
                            d.stdout(
                                b"ERROR [T2] T0 or TASC needs to be set in the parameter file\n",
                            );
                            return Err(boundary("T2model.C::T2model: source exit"));
                        }
                    }
                }
                updateParameters(
                    *read(&edot, "edot")?,
                    *read(&xdot, "xdot")?,
                    *read(&x2dot, "x2dot")?,
                    *read(&eps1dot, "eps1dot")?,
                    *read(&eps2dot, "eps2dot")?,
                    (tt0).clone(),
                    &mut ecc,
                    &mut x,
                    &mut eps1,
                    &mut eps2,
                    d,
                )?;
                if ((*read(&ecc, "ecc")? < 0.0f64) || (*read(&ecc, "ecc")? > 1.0f64)) {
                    {
                        d.display(
                            1i32,
                            "BIN2".as_bytes(),
                            "Eccentricity of orbit < 0, setting to 0".as_bytes(),
                            "".as_bytes(),
                            *read(&psr.noWarnings, "psr[p].noWarnings")?,
                        )?;
                        psr.param.slots[param_ecc as usize].cells[(com) as usize].val =
                            Float::with_val(PRECISION_BITS, 0.0f64);
                        ecc = Some(0.0f64);
                    }
                }
                orbits = ((((tt0).clone() / Float::with_val(PRECISION_BITS, *read(&pb, "pb")?))
                    - (Float::with_val(
                        PRECISION_BITS,
                        (0.5f64 * (*read(&pbdot, "pbdot")? + *read(&xpbdot, "xpbdot")?)),
                    ) * ((tt0).clone() / Float::with_val(PRECISION_BITS, *read(&pb, "pb")?))
                        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
                    - (Float::with_val(
                        PRECISION_BITS,
                        ((1.0f64 / 6.0f64) * *read(&pb2dot, "pb2dot")?),
                    ) * ((tt0).clone() / Float::with_val(PRECISION_BITS, *read(&pb, "pb")?))
                        .pow(&(Float::with_val(PRECISION_BITS, 3i32)))))
                .to_f64();
                norbits = source_i32(orbits)?;
                if (orbits < 0.0f64) {
                    norbits = norbits
                        .checked_sub(1)
                        .ok_or_else(|| boundary("binary orbit count overflow"))?;
                }
                phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
                if (companion_set(psr, param_ecc, com)) {
                    {
                        computeU(phase, *read(&ecc, "ecc")?, &mut u, d)?;
                        su = (*read(&u, "u")?).sin();
                        cu = Some((*read(&u, "u")?).cos());
                        onemecu = (1.0f64 - (*read(&ecc, "ecc")? * *read(&cu, "cu")?));
                        cae = ((*read(&cu, "cu")? - *read(&ecc, "ecc")?) / onemecu);
                        sae = (((1.0f64 - (*read(&ecc, "ecc")?).powf((2i32) as f64)).sqrt() * su)
                            / onemecu);
                        ae = Some((sae).atan2(cae));
                        if (*read(&ae, "ae")? < 0.0f64) {
                            ae = Some(*read(&ae, "ae")? + (2.0f64 * std::f64::consts::PI));
                        }
                        ae = Some(
                            (((2.0f64 * std::f64::consts::PI) * orbits) + *read(&ae, "ae")?)
                                - phase,
                        );
                        omega = ((*read(&omz, "omz")? / rad2deg)
                            + (*read(&omdot, "omdot")? * *read(&ae, "ae")?));
                        sw = Some((omega).sin());
                        cw = Some((omega).cos());
                        sqr1me2 = Some(
                            ((1i32) as f64 - (*read(&ecc, "ecc")?).powf((2i32) as f64)).sqrt(),
                        );
                        cume = Some(*read(&cu, "cu")? - *read(&ecc, "ecc")?);
                        if (((companion_set(psr, param_kin, com))
                            && (companion_set(psr, param_kom, com)))
                            && ((companion_set(psr, param_pmra, com))
                                || (companion_set(psr, param_pmdec, com))))
                        {
                            {
                                KopeikinTerms(
                                    psr,
                                    rows,
                                    ipos,
                                    *read(&ki, "ki")?,
                                    *read(&pmra, "pmra")?,
                                    *read(&sin_omega, "sin_omega")?,
                                    *read(&pmdec, "pmdec")?,
                                    *read(&cos_omega, "cos_omega")?,
                                    (tt0).clone(),
                                    *read(&dpara, "dpara")?,
                                    *read(&daop, "daop")?,
                                    *read(&si, "si")?,
                                    &mut x,
                                    &mut DK011,
                                    &mut DK012,
                                    &mut DK021,
                                    &mut DK022,
                                    &mut DK031,
                                    &mut DK032,
                                    &mut DK041,
                                    &mut DK042,
                                    &mut DK013,
                                    &mut DK014,
                                    &mut DK023,
                                    &mut DK024,
                                    &mut DK033,
                                    &mut DK034,
                                    &mut DK043,
                                    &mut DK044,
                                    d,
                                )?;
                                C = Some(Float::with_val(
                                    PRECISION_BITS,
                                    ((*read(&cw, "cw")? * (*read(&cu, "cu")? - *read(&er, "er")?))
                                        - (((1.0f64 - (*read(&eth, "eth")?).powf(2.0f64)).sqrt()
                                            * *read(&sw, "sw")?)
                                            * su)),
                                ));
                                S = Some(Float::with_val(
                                    PRECISION_BITS,
                                    ((*read(&sw, "sw")? * (*read(&cu, "cu")? - *read(&er, "er")?))
                                        + ((*read(&cw, "cw")?
                                            * (1.0f64 - (*read(&eth, "eth")?).powf(2.0f64))
                                                .sqrt())
                                            * su)),
                                ));
                                DAOP = ((((read(&DK011, "DK011")?).clone()
                                    + (read(&DK012, "DK012")?).clone())
                                    * (read(&C, "C")?).clone())
                                    - (((read(&DK021, "DK021")?).clone()
                                        + (read(&DK022, "DK022")?).clone())
                                        * (read(&S, "S")?).clone()));
                                DSR = ((((read(&DK031, "DK031")?).clone()
                                    + (read(&DK032, "DK032")?).clone())
                                    * (read(&C, "C")?).clone())
                                    + (((read(&DK041, "DK041")?).clone()
                                        + (read(&DK042, "DK042")?).clone())
                                        * (read(&S, "S")?).clone()));
                                DOP = Float::with_val(
                                    PRECISION_BITS,
                                    ((((*read(&dpara, "dpara")? / 499.00478364f64) / 2.0f64)
                                        * (*read(&x, "x")?).powf(2.0f64))
                                        * (((((((*read(&ki, "ki")?).sin()).powf(-2.0f64)
                                            - 0.5f64)
                                            + ((0.5f64 * (*read(&ecc, "ecc")?).powf(2.0f64))
                                                * (((1i32) as f64
                                                    + (*read(&sw, "sw")?).powf(2.0f64))
                                                    - (3.0f64
                                                        / ((*read(&ki, "ki")?).sin())
                                                            .powf(2.0f64)))))
                                            - (((2.0f64 * *read(&ecc, "ecc")?)
                                                * (((*read(&ki, "ki")?).sin()).powf(-2.0f64)
                                                    - (*read(&sw, "sw")?).powf(2.0f64)))
                                                * *read(&cume, "cume")?))
                                            - (((((*read(&sqr1me2, "sqr1me2")?
                                                * (2i32) as f64)
                                                * *read(&sw, "sw")?)
                                                * *read(&cw, "cw")?)
                                                * su)
                                                * *read(&cume, "cume")?))
                                            + ((0.5f64
                                                * ((2.0f64 * omega).cos()
                                                    + ((*read(&ecc, "ecc")?).powf(2.0f64)
                                                        * (((*read(&ki, "ki")?).sin())
                                                            .powf(-2.0f64)
                                                            + (*read(&cu, "cu")?)
                                                                .powf(2.0f64)))))
                                                * (2.0f64 * *read(&u, "u")?).cos()))),
                                );
                            }
                        }
                        if (companion_set(psr, param_shapmax, com)) {
                            {
                                sdds =
                                    Some(1.0f64 - ((-1.0f64) * *read(&shapmax, "shapmax")?).exp());
                                brace = (onemecu
                                    - (*read(&sdds, "sdds")?
                                        * ((*read(&sw, "sw")? * *read(&cume, "cume")?)
                                            + ((*read(&sqr1me2, "sqr1me2")?
                                                * *read(&cw, "cw")?)
                                                * su))));
                            }
                        } else {
                            brace = (onemecu
                                - (*read(&si, "si")?
                                    * ((*read(&sw, "sw")? * *read(&cume, "cume")?)
                                        + ((*read(&sqr1me2, "sqr1me2")? * *read(&cw, "cw")?)
                                            * su))));
                        }
                        da = ((*read(&a0, "a0")?
                            * ((omega + *read(&ae, "ae")?).sin()
                                + (*read(&ecc, "ecc")? * *read(&sw, "sw")?)))
                            + (*read(&b0, "b0")?
                                * ((omega + *read(&ae, "ae")?).cos()
                                    + (*read(&ecc, "ecc")? * *read(&cw, "cw")?))));
                        alpha = (*read(&x, "x")? * *read(&sw, "sw")?);
                        beta = ((*read(&x, "x")?
                            * ((1i32) as f64 - (*read(&eth, "eth")?).powf((2i32) as f64)).sqrt())
                            * *read(&cw, "cw")?);
                        bg = (beta + *read(&gamma, "gamma")?);
                        dre = ((alpha * (*read(&cu, "cu")? - *read(&er, "er")?)) + (bg * su));
                        drep = (((-alpha) * su) + (bg * *read(&cu, "cu")?));
                        drepp = (((-alpha) * *read(&cu, "cu")?) - (bg * su));
                        anhat = (*read(&an, "an")? / onemecu);
                        dlogbr = (brace).ln();
                        ds = (((-2i32) as f64 * *read(&m2, "m2")?) * dlogbr);
                    }
                } else {
                    if (companion_set(psr, param_eps1, com)) {
                        {
                            dre = (*read(&x, "x")?
                                * ((phase).sin()
                                    - (0.5f64
                                        * ((*read(&eps1, "eps1")? * (2.0f64 * phase).cos())
                                            - (*read(&eps2, "eps2")? * (2.0f64 * phase).sin())))));
                            drep = (*read(&x, "x")? * (phase).cos());
                            drepp = ((-*read(&x, "x")?) * (phase).sin());
                            if (((companion_set(psr, param_kin, com))
                                && (companion_set(psr, param_kom, com)))
                                && ((companion_set(psr, param_pmra, com))
                                    || (companion_set(psr, param_pmdec, com))))
                            {
                                {
                                    S = Some(Float::with_val(
                                        PRECISION_BITS,
                                        ((phase).sin()
                                            - (0.5f64
                                                * ((*read(&eps1, "eps1")?
                                                    * (2.0f64 * phase).cos())
                                                    - (*read(&eps2, "eps2")?
                                                        * (2.0f64 * phase).sin())))),
                                    ));
                                    C = Some(Float::with_val(
                                        PRECISION_BITS,
                                        ((phase).cos()
                                            + (0.5f64
                                                * ((*read(&eps2, "eps2")?
                                                    * (2.0f64 * phase).cos())
                                                    + (*read(&eps1, "eps1")?
                                                        * (2.0f64 * phase).sin())))),
                                    ));
                                    KopeikinTerms(
                                        psr,
                                        rows,
                                        ipos,
                                        *read(&ki, "ki")?,
                                        *read(&pmra, "pmra")?,
                                        *read(&sin_omega, "sin_omega")?,
                                        *read(&pmdec, "pmdec")?,
                                        *read(&cos_omega, "cos_omega")?,
                                        (tt0).clone(),
                                        *read(&dpara, "dpara")?,
                                        *read(&daop, "daop")?,
                                        *read(&si, "si")?,
                                        &mut x,
                                        &mut DK011,
                                        &mut DK012,
                                        &mut DK021,
                                        &mut DK022,
                                        &mut DK031,
                                        &mut DK032,
                                        &mut DK041,
                                        &mut DK042,
                                        &mut DK013,
                                        &mut DK014,
                                        &mut DK023,
                                        &mut DK024,
                                        &mut DK033,
                                        &mut DK034,
                                        &mut DK043,
                                        &mut DK044,
                                        d,
                                    )?;
                                    DAOP = ((((read(&DK011, "DK011")?).clone()
                                        + (read(&DK012, "DK012")?).clone())
                                        * (read(&C, "C")?).clone())
                                        - (((read(&DK021, "DK021")?).clone()
                                            + (read(&DK022, "DK022")?).clone())
                                            * (read(&S, "S")?).clone()));
                                    DSR = ((((read(&DK031, "DK031")?).clone()
                                        + (read(&DK032, "DK032")?).clone())
                                        * (read(&C, "C")?).clone())
                                        + (((read(&DK041, "DK041")?).clone()
                                            + (read(&DK042, "DK042")?).clone())
                                            * (read(&S, "S")?).clone()));
                                }
                            }
                            brace = ((1i32) as f64 - (*read(&si, "si")? * (phase).sin()));
                            da = ((*read(&a0, "a0")? * (phase).sin())
                                + (*read(&b0, "b0")? * (phase).cos()));
                            anhat = *read(&an, "an")?;
                            ecc = Some(0.0f64);
                            if (((psr.param.slots[param_h3 as usize].cells[0].param_set
                                * psr.param.slots[param_stig as usize].cells[0].param_set)
                                == 1i32)
                                || ((psr.param.slots[param_h3 as usize].cells[0].param_set
                                    * psr.param.slots[param_h4 as usize].cells[0].param_set)
                                    == 1i32))
                            {
                                {
                                    h3 = Some(crate::tempo2_util::get_parameter_value_inner(
                                        psr,
                                        param_h3 as usize,
                                        (0i32) as usize,
                                        d,
                                    )?);
                                    if (psr.param.slots[param_h4 as usize].cells[0].param_set
                                        == 1i32)
                                    {
                                        {
                                            h4 = Some(
                                                crate::tempo2_util::get_parameter_value_inner(
                                                    psr,
                                                    param_h4 as usize,
                                                    (0i32) as usize,
                                                    d,
                                                )?,
                                            );
                                            if (psr.param.slots[param_nharm as usize].cells[0]
                                                .param_set
                                                == 1i32)
                                            {
                                                {
                                                    nharm = crate::ifteph::extended_i32(
                                                        &((psr.param.slots[param_nharm as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()),
                                                        "binary_integer",
                                                    )?;
                                                    if (nharm > 4i32) {
                                                        mode = 3i32;
                                                    } else {
                                                        mode = 2i32;
                                                    }
                                                }
                                            }
                                            if (psr.param.slots[param_stig as usize].cells[0]
                                                .param_set
                                                == 1i32)
                                            {
                                                {
                                                    d.log(b'e', "T2model.C", 353, b"You specified both H4 and STIG - Ignoring STIG");
                                                    d.log(
                                                        b'm',
                                                        "T2model.C",
                                                        354,
                                                        b"WARNING! You specified both H4 and STIG.",
                                                    );
                                                    d.log(b'm', "T2model.C", 355, b"We will ignore STIG and perform the approx. H4 fit instead.");
                                                    d.log(b'm', "T2model.C", 356, b"If you want to perform the exact fit for H3 and STIG, then ");
                                                    d.log(b'm', "T2model.C", 357, b"please remove H4 from your parameter file.");
                                                }
                                            }
                                            mode = 2i32;
                                        }
                                    } else {
                                        {
                                            if (psr.param.slots[param_stig as usize].cells[0]
                                                .param_set
                                                == 1i32)
                                            {
                                                {
                                                    stig = Some(crate::tempo2_util::get_parameter_value_inner(psr, param_stig as usize, (0i32) as usize, d)?);
                                                    mode = 1i32;
                                                }
                                            } else {
                                                {
                                                    mode = 0i32;
                                                    h4 =
                                                        Some(Float::with_val(PRECISION_BITS, 0i32));
                                                    nharm = 3i32;
                                                }
                                            }
                                        }
                                    }
                                    if (mode == 1i32) {
                                        {
                                            si = Some(
                                                ((Float::with_val(PRECISION_BITS, 2.0f64)
                                                    * (read(&stig, "stig")?).clone())
                                                    / (Float::with_val(PRECISION_BITS, 1.0f64)
                                                        + ((read(&stig, "stig")?).clone()).pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2.0f64,
                                                            )),
                                                        )))
                                                .to_f64(),
                                            );
                                            m2 = Some(
                                                ((read(&h3, "h3")?).clone()
                                                    / ((read(&stig, "stig")?).clone()).pow(
                                                        &(Float::with_val(PRECISION_BITS, 3.0f64)),
                                                    ))
                                                .to_f64(),
                                            );
                                            if (*read(&si, "si")? > 1.0f64) {
                                                {
                                                    d.display(1i32, "BIN1".as_bytes(), "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(), "".as_bytes(), *read(&psr.noWarnings, "psr[p].noWarnings")?)?;
                                                    si = Some(1.0f64);
                                                    psr.param.slots[param_sini as usize].cells[0]
                                                        .val = Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if ((mode == 2i32) || (mode == 3i32)) {
                                            {
                                                si = Some(
                                                    (((Float::with_val(PRECISION_BITS, 2.0f64)
                                                        * (read(&h3, "h3")?).clone())
                                                        * (read(&h4, "h4")?).clone())
                                                        / (((read(&h3, "h3")?).clone()
                                                            * (read(&h3, "h3")?).clone())
                                                            + ((read(&h4, "h4")?).clone()
                                                                * (read(&h4, "h4")?).clone())))
                                                    .to_f64(),
                                                );
                                                m2 = Some(
                                                    (((read(&h3, "h3")?).clone()).pow(
                                                        &(Float::with_val(PRECISION_BITS, 4.0f64)),
                                                    ) / ((read(&h4, "h4")?).clone()).pow(
                                                        &(Float::with_val(PRECISION_BITS, 3.0f64)),
                                                    ))
                                                    .to_f64(),
                                                );
                                                if (*read(&si, "si")? > 1.0f64) {
                                                    {
                                                        d.display(1i32, "BIN1".as_bytes(), "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(), "".as_bytes(), *read(&psr.noWarnings, "psr[p].noWarnings")?)?;
                                                        si = Some(1.0f64);
                                                        psr.param.slots[param_sini as usize]
                                                            .cells[0]
                                                            .val = Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if (mode == 0i32) {
                                                {}
                                            } else {
                                                {
                                                    d.stdout(
                                                        b"This should not be possible. Go Away.\n",
                                                    );
                                                    d.stdout(b"And tell someone about it: joris.verbiest@gmail.com, e.g.\n");
                                                }
                                            }
                                        }
                                    }
                                    brace = ((1i32) as f64 - (*read(&si, "si")? * (phase).sin()));
                                    dlogbr = (brace).ln();
                                    ecc = Some(
                                        ((*read(&eps1, "eps1")? * *read(&eps1, "eps1")?)
                                            + (*read(&eps2, "eps2")? * *read(&eps2, "eps2")?))
                                        .sqrt(),
                                    );
                                    TrueAnom = Some(Float::with_val(PRECISION_BITS, phase));
                                    omega = (*read(&eps1, "eps1")?).atan2(*read(&eps2, "eps2")?);
                                    // T2model.C:394-396: STIG intermediates are consumed only in mode 1
                                    // or its derivative; H4 modes do not define STIG.
                                    if mode == 1 || param == param_stig as i32 {
                                        fs = Some(
                                            (Float::with_val(PRECISION_BITS, 1.0f64)
                                                + ((read(&stig, "stig")?).clone()
                                                    * (read(&stig, "stig")?).clone()))
                                                - ((Float::with_val(PRECISION_BITS, 2.0f64)
                                                    * (read(&stig, "stig")?).clone())
                                                    * ((read(&TrueAnom, "TrueAnom")?).clone())
                                                        .sin()),
                                        );
                                        lgf = Some(((read(&fs, "fs")?).clone()).ln());
                                        lsc = Some(
                                            ((read(&lgf, "lgf")?).clone()
                                                + ((Float::with_val(PRECISION_BITS, 2.0f64)
                                                    * (read(&stig, "stig")?).clone())
                                                    * ((read(&TrueAnom, "TrueAnom")?).clone())
                                                        .sin()))
                                                - (((read(&stig, "stig")?).clone()
                                                    * (read(&stig, "stig")?).clone())
                                                    * (Float::with_val(PRECISION_BITS, 2.0f64)
                                                        * (read(&TrueAnom, "TrueAnom")?).clone())
                                                    .cos()),
                                        );
                                    }
                                    if (mode == 0i32) {
                                        {
                                            ds = ((Float::with_val(
                                                PRECISION_BITS,
                                                ((-4.0f64) / 3.0f64),
                                            ) * (read(&h3, "h3")?).clone())
                                                * (Float::with_val(PRECISION_BITS, 3.0f64)
                                                    * (read(&TrueAnom, "TrueAnom")?).clone())
                                                .sin())
                                            .to_f64();
                                        }
                                    } else {
                                        if (mode == 1i32) {
                                            {
                                                ds = (Float::with_val(
                                                    PRECISION_BITS,
                                                    ((-2.0f64) * *read(&m2, "m2")?),
                                                ) * (read(&lsc, "lsc")?).clone())
                                                .to_f64();
                                            }
                                        } else {
                                            {
                                                ds = calcDH(
                                                    ((read(&TrueAnom, "TrueAnom")?).clone())
                                                        .to_f64(),
                                                    ((read(&h3, "h3")?).clone()).to_f64(),
                                                    ((read(&h4, "h4")?).clone()).to_f64(),
                                                    nharm,
                                                    0i32,
                                                    d,
                                                )?;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    dlogbr = (brace).ln();
                                    ds = (((-2i32) as f64 * *read(&m2, "m2")?) * dlogbr);
                                }
                            }
                        }
                    } else {
                        {
                            d.stdout(&[b"Require eccentricity set or EPS1/EPS2 parameters for companion ".to_vec(),(com + 1i32).to_string().into_bytes(),b"\n".to_vec()].concat());
                            return Err(boundary("T2model.C::T2model: source exit"));
                        }
                    }
                }
                if (onemecu != 0.0f64) {
                    {
                        d2bar = (Float::with_val(
                            PRECISION_BITS,
                            (dre * (((1i32) as f64 - (anhat * drep))
                                + (((allTerms) as f64 * (anhat).powf((2i32) as f64))
                                    * (((drep).powf((2i32) as f64) + ((0.5f64 * dre) * drepp))
                                        - (((((0.5f64 * *read(&ecc, "ecc")?) * su) * dre)
                                            * drep)
                                            / onemecu))))),
                        ) + (Float::with_val(PRECISION_BITS, allTerms)
                            * (((Float::with_val(PRECISION_BITS, (ds + da)) + (DAOP).clone())
                                + (DSR).clone())
                                + (DOP).clone())))
                        .to_f64();
                    }
                } else {
                    {
                        d2bar = (Float::with_val(
                            PRECISION_BITS,
                            (dre * (((1i32) as f64 - (anhat * drep))
                                + (((allTerms) as f64 * (anhat).powf((2i32) as f64))
                                    * ((drep).powf((2i32) as f64) + ((0.5f64 * dre) * drepp))))),
                        ) + (Float::with_val(PRECISION_BITS, allTerms)
                            * (((Float::with_val(PRECISION_BITS, (ds + da)) + (DAOP).clone())
                                + (DSR).clone())
                                + (DOP).clone())))
                        .to_f64();
                    }
                }
                torb = Some(*read(&torb, "torb")? - d2bar);
                outer_delay += d2bar;
                if ((param == (-1i32)) && (com == 0i32)) {
                    return Ok(*read(&torb, "torb")?);
                } else {
                    if ((param != (-1i32)) && (com == arr)) {
                        {
                            inner_derivative_dT0 = 0.0f64;
                            let mut com_inner: i32;
                            {
                                com_inner = 0i32;
                                while (com_inner < arr) {
                                    {
                                        inner_derivative_dT0 += (T2model(
                                            psr,
                                            rows,
                                            ipos,
                                            (param_t0 as i32),
                                            com_inner,
                                            d,
                                        )? / 86400.0f64);
                                    }
                                    com_inner += 1;
                                }
                            }
                            if (companion_set(psr, param_ecc, com)) {
                                {
                                    csigma = Some(
                                        (*read(&x, "x")?
                                            * (((-*read(&sw, "sw")?) * su)
                                                + ((*read(&sqr1me2, "sqr1me2")?
                                                    * *read(&cw, "cw")?)
                                                    * *read(&cu, "cu")?)))
                                            / onemecu,
                                    );
                                    ce = Some(
                                        ((su * *read(&csigma, "csigma")?)
                                            - (*read(&x, "x")? * *read(&sw, "sw")?))
                                            - ((((*read(&ecc, "ecc")? * *read(&x, "x")?)
                                                * *read(&cw, "cw")?)
                                                * su)
                                                / *read(&sqr1me2, "sqr1me2")?),
                                    );
                                    cx = Some(
                                        (*read(&sw, "sw")? * *read(&cume, "cume")?)
                                            + ((*read(&sqr1me2, "sqr1me2")? * *read(&cw, "cw")?)
                                                * su),
                                    );
                                    comega = Some(
                                        *read(&x, "x")?
                                            * ((*read(&cw, "cw")? * *read(&cume, "cume")?)
                                                - ((*read(&sqr1me2, "sqr1me2")?
                                                    * *read(&sw, "sw")?)
                                                    * su)),
                                    );
                                    cgamma = Some(su);
                                    cm2 = Some((-2i32) as f64 * dlogbr);
                                    if (companion_set(psr, param_shapmax, com)) {
                                        cshapmax = Some(
                                            ((((2i32) as f64 * *read(&m2, "m2")?)
                                                * ((*read(&sw, "sw")? * *read(&cume, "cume")?)
                                                    + ((*read(&sqr1me2, "sqr1me2")?
                                                        * *read(&cw, "cw")?)
                                                        * su)))
                                                / brace)
                                                * (1.0f64 - *read(&sdds, "sdds")?),
                                        );
                                    } else {
                                        if (*read(
                                            &psr.param.slots[param_sini as usize].n_link_to,
                                            "psr[p].param[param_sini].nLinkTo",
                                        )? > 0i32)
                                        {
                                            {
                                                csi = Some(
                                                    ((((2i32) as f64 * *read(&m2, "m2")?)
                                                        * ((*read(&sw, "sw")?
                                                            * *read(&cume, "cume")?)
                                                            + ((*read(&sqr1me2, "sqr1me2")?
                                                                * *read(&cw, "cw")?)
                                                                * su)))
                                                        * (*read(&ki, "ki")?).cos())
                                                        / brace,
                                                );
                                            }
                                        } else {
                                            csi = Some(
                                                (((2i32) as f64 * *read(&m2, "m2")?)
                                                    * ((*read(&sw, "sw")?
                                                        * *read(&cume, "cume")?)
                                                        + ((*read(&sqr1me2, "sqr1me2")?
                                                            * *read(&cw, "cw")?)
                                                            * su)))
                                                    / brace,
                                            );
                                        }
                                    }
                                }
                            } else {
                                if (companion_set(psr, param_eps1, com)) {
                                    {
                                        csigma = Some(*read(&x, "x")? * (phase).cos());
                                        cx = Some((phase).sin());
                                        ceps1 = Some(
                                            ((-0.5f64) * *read(&x, "x")?)
                                                * ((2i32) as f64 * phase).cos(),
                                        );
                                        ceps2 = Some(
                                            (0.5f64 * *read(&x, "x")?)
                                                * ((2i32) as f64 * phase).sin(),
                                        );
                                        cm2 = Some((-2i32) as f64 * dlogbr);
                                        csi = Some(
                                            (((2i32) as f64 * *read(&m2, "m2")?) * (phase).sin())
                                                / brace,
                                        );
                                    }
                                }
                            }
                            let mut fac: f64 = (1.0f64 + inner_derivative_dT0);
                            if (param == (param_pb as i32)) {
                                return Ok((((Float::with_val(
                                    PRECISION_BITS,
                                    (((-*read(&csigma, "csigma")?) * *read(&an, "an")?)
                                        * 86400.0f64),
                                ) * (tt0).clone())
                                    / Float::with_val(PRECISION_BITS, *read(&pb, "pb")?))
                                    * Float::with_val(PRECISION_BITS, fac))
                                .to_f64());
                            } else {
                                if (param == (param_a1 as i32)) {
                                    return Ok(*read(&cx, "cx")? * fac);
                                } else {
                                    if (param == (param_ecc as i32)) {
                                        return Ok(*read(&ce, "ce")? * fac);
                                    } else {
                                        if (param == (param_edot as i32)) {
                                            return Ok(((Float::with_val(
                                                PRECISION_BITS,
                                                *read(&ce, "ce")?,
                                            ) * (tt0).clone())
                                                * Float::with_val(PRECISION_BITS, fac))
                                            .to_f64());
                                        } else {
                                            if (param == (param_om as i32)) {
                                                return Ok(*read(&comega, "comega")? * fac);
                                            } else {
                                                if (param == (param_omdot as i32)) {
                                                    return Ok(((*read(&ae, "ae")?
                                                        * *read(&comega, "comega")?)
                                                        / ((((*read(&an, "an")? * 360.0f64)
                                                            / (2.0f64 * std::f64::consts::PI))
                                                            * 365.25f64)
                                                            * 86400.0f64))
                                                        * fac);
                                                } else {
                                                    if (param == (param_t0 as i32)) {
                                                        return Ok((((-*read(
                                                            &csigma, "csigma",
                                                        )?) * *read(&an, "an")?)
                                                            * 86400.0f64)
                                                            * fac);
                                                    } else {
                                                        if (param == (param_pbdot as i32)) {
                                                            {
                                                                if (*read(
                                                                    &psr.param.slots
                                                                        [param_pbdot as usize]
                                                                        .n_link_from,
                                                                    "psr[p].param[param_pbdot].nLinkFrom",
                                                                )? > 0i32)
                                                                {
                                                                    {
                                                                        return Ok((((Float::with_val(PRECISION_BITS, 0.5f64) * (tt0).clone()) * ((Float::with_val(PRECISION_BITS, (((-*read(&csigma, "csigma")?) * *read(&an, "an")?) * 86400.0f64)) * (tt0).clone()) / Float::with_val(PRECISION_BITS, (*read(&pb, "pb")? * 86400.0f64)))) * Float::with_val(PRECISION_BITS, fac)).to_f64());
                                                                    }
                                                                } else {
                                                                    return Ok((((Float::with_val(
                                                                        PRECISION_BITS,
                                                                        0.5f64,
                                                                    )
                                                                        * (tt0).clone())
                                                                        * ((Float::with_val(
                                                                            PRECISION_BITS,
                                                                            (((-*read(
                                                                                &csigma, "csigma",
                                                                            )?) * *read(
                                                                                &an, "an",
                                                                            )?) * 86400.0f64),
                                                                        ) * (tt0).clone())
                                                                            / Float::with_val(
                                                                                PRECISION_BITS,
                                                                                (*read(&pb, "pb")?
                                                                                    * 86400.0f64),
                                                                            )))
                                                                        * Float::with_val(
                                                                            PRECISION_BITS,
                                                                            fac,
                                                                        ))
                                                                    .to_f64());
                                                                }
                                                            }
                                                        } else {
                                                            if (param == (param_pb2dot as i32)) {
                                                                {
                                                                    return Ok(
                                                                        ((((Float::with_val(
                                                                            PRECISION_BITS,
                                                                            (1.0f64 / 6.0f64),
                                                                        ) * (tt0).clone())
                                                                            * (tt0).clone())
                                                                            * ((Float::with_val(
                                                                                PRECISION_BITS,
                                                                                ((-*read(
                                                                                    &csigma,
                                                                                    "csigma",
                                                                                )?) * *read(
                                                                                    &an, "an",
                                                                                )?),
                                                                            ) * (tt0).clone())
                                                                                / Float::with_val(
                                                                                    PRECISION_BITS,
                                                                                    *read(
                                                                                        &pb, "pb",
                                                                                    )?,
                                                                                )))
                                                                            * Float::with_val(
                                                                                PRECISION_BITS,
                                                                                fac,
                                                                            ))
                                                                        .to_f64(),
                                                                    );
                                                                }
                                                            } else {
                                                                if (param == (param_sini as i32)) {
                                                                    return Ok(
                                                                        *read(&csi, "csi")? * fac
                                                                    );
                                                                } else {
                                                                    if (param
                                                                        == (param_gamma as i32))
                                                                    {
                                                                        return Ok(*read(
                                                                            &cgamma, "cgamma",
                                                                        )? * fac);
                                                                    } else {
                                                                        if (param
                                                                            == (param_m2 as i32))
                                                                        {
                                                                            return Ok((*read(
                                                                                &cm2, "cm2",
                                                                            )?
                                                                                * SUNMASS)
                                                                                * fac);
                                                                        } else {
                                                                            if (param
                                                                                == (param_a1dot
                                                                                    as i32))
                                                                            {
                                                                                return Ok(((Float::with_val(PRECISION_BITS, *read(&cx, "cx")?) * (tt0).clone()) * Float::with_val(PRECISION_BITS, fac)).to_f64());
                                                                            } else {
                                                                                if (param
                                                                                    == (param_a2dot
                                                                                        as i32))
                                                                                {
                                                                                    return Ok((((Float::with_val(PRECISION_BITS, (0.5f64 * *read(&cx, "cx")?)) * (tt0).clone()) * (tt0).clone()) * Float::with_val(PRECISION_BITS, fac)).to_f64());
                                                                                } else {
                                                                                    if (param == (param_eps1 as i32)) {
return Ok(*read(&ceps1, "ceps1")? * fac);
} else {
if (param == (param_eps1dot as i32)) {
return Ok(((Float::with_val(PRECISION_BITS, *read(&ceps1, "ceps1")?) * (tt0).clone()) * Float::with_val(PRECISION_BITS, fac)).to_f64());
} else {
if (param == (param_eps2dot as i32)) {
return Ok(((Float::with_val(PRECISION_BITS, *read(&ceps2, "ceps2")?) * (tt0).clone()) * Float::with_val(PRECISION_BITS, fac)).to_f64());
} else {
if (param == (param_eps2 as i32)) {
return Ok(*read(&ceps2, "ceps2")? * fac);
} else {
if (param == (param_tasc as i32)) {
return Ok((((-*read(&csigma, "csigma")?) * *read(&an, "an")?) * 86400.0f64) * fac);
} else {
if (param == (param_shapmax as i32)) {
return Ok(*read(&cshapmax, "cshapmax")? * fac);
} else {
if (param == (param_stig as i32)) {
{
return Ok((((Float::with_val(PRECISION_BITS, ((-2.0f64) * *read(&m2, "m2")?)) / (read(&stig, "stig")?).clone()) * ((Float::with_val(PRECISION_BITS, 1.0f64) - (Float::with_val(PRECISION_BITS, 3.0f64) * (read(&lgf, "lgf")?).clone())) - ((Float::with_val(PRECISION_BITS, 1.0f64) - ((read(&stig, "stig")?).clone() * (read(&stig, "stig")?).clone())) / (read(&fs, "fs")?).clone()))) + (Float::with_val(PRECISION_BITS, (2.0f64 * *read(&m2, "m2")?)) * ((Float::with_val(PRECISION_BITS, 4.0f64) * ((read(&TrueAnom, "TrueAnom")?).clone()).sin()) - ((read(&stig, "stig")?).clone() * (Float::with_val(PRECISION_BITS, 2.0f64) * (read(&TrueAnom, "TrueAnom")?).clone()).cos())))).to_f64());
}
} else {
if (param == (param_h3 as i32)) {
{
if ((mode == 0i32) || (mode == 2i32)) {
return Ok((Float::with_val(PRECISION_BITS, ((-4.0f64) / 3.0f64)) * (Float::with_val(PRECISION_BITS, 3.0f64) * (read(&TrueAnom, "TrueAnom")?).clone()).sin()).to_f64());
} else {
if (mode == 1i32) {
{
return Ok(((Float::with_val(PRECISION_BITS, (-2.0f64)) * (read(&lsc, "lsc")?).clone()) / ((read(&stig, "stig")?).clone()).pow(&(Float::with_val(PRECISION_BITS, 3.0f64)))).to_f64());
}
} else {
if (mode == 3i32) {
return calcDH(((read(&TrueAnom, "TrueAnom")?).clone()).to_f64(), ((read(&h3, "h3")?).clone()).to_f64(), ((read(&h4, "h4")?).clone()).to_f64(), nharm, 3i32, d);
} else {
{
d.stdout(b"ERROR in ELLH model in T2. This really shouldn't happen.\n");
}
}
}
}
}
} else {
if (param == (param_h4 as i32)) {
{
if (mode == 2i32) {
return Ok(((Float::with_val(PRECISION_BITS, 4.0f64) * (read(&TrueAnom, "TrueAnom")?).clone()).cos()).to_f64());
} else {
return calcDH(((read(&TrueAnom, "TrueAnom")?).clone()).to_f64(), ((read(&h3, "h3")?).clone()).to_f64(), ((read(&h4, "h4")?).clone()).to_f64(), nharm, 4i32, d);
}
}
} else {
if (param == (param_kom as i32)) {
{
ckom = (((read(&C, "C")?).clone() * ((((read(&DK033, "DK033")?).clone() + (read(&DK034, "DK034")?).clone()) + (read(&DK013, "DK013")?).clone()) + (read(&DK014, "DK014")?).clone())) + ((read(&S, "S")?).clone() * ((((read(&DK043, "DK043")?).clone() + (read(&DK044, "DK044")?).clone()) + (read(&DK023, "DK023")?).clone()) + (read(&DK024, "DK024")?).clone()))).to_f64();
return Ok(ckom);
}
} else {
if (param == (param_kin as i32)) {
{
ckin = ((((read(&C, "C")?).clone() / Float::with_val(PRECISION_BITS, (*read(&ki, "ki")?).sin())) * ((((read(&DK043, "DK043")?).clone() + (read(&DK044, "DK044")?).clone()) + (read(&DK023, "DK023")?).clone()) + (read(&DK024, "DK024")?).clone())) - (((read(&S, "S")?).clone() / Float::with_val(PRECISION_BITS, (*read(&ki, "ki")?).sin())) * ((((read(&DK013, "DK013")?).clone() + (read(&DK014, "DK014")?).clone()) + (read(&DK033, "DK033")?).clone()) + (read(&DK034, "DK034")?).clone()))).to_f64();
if (companion_set(psr, param_ecc, com)) {
ckin += ((((((*read(&dpara, "dpara")? / 499.00478364f64) / 2.0f64) * (*read(&x, "x")?).powf(2.0f64)) * (*read(&ki, "ki")?).cos()) * ((*read(&ki, "ki")?).sin()).powf(-3.0f64)) * ((((*read(&ecc, "ecc")?).powf(2.0f64) * (3.0f64 - (2.0f64 * *read(&u, "u")?).cos())) + ((4.0f64 * *read(&ecc, "ecc")?) * *read(&cume, "cume")?)) - 2.0f64));
}
if (*read(&psr.param.slots[param_kin as usize].n_link_from, "psr[p].param[param_kin].nLinkFrom")? > 0i32) {
ckin += *read(&csi, "csi")?;
}
return Ok(ckin);
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
                                }
                            }
                            if ((psr.param.slots[param_bpjep as usize].cells[(arr) as usize]
                                .param_set
                                == 1i32)
                                && ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?)
                                    .clone()
                                    > (psr.param.slots[param_bpjep as usize].cells
                                        [(arr) as usize]
                                        .val)
                                        .clone()))
                            {
                                {
                                    if (param == (param_bpjph as i32)) {
                                        return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                                            / (psr.param.slots[param_f as usize].cells[0].val)
                                                .clone())
                                        .to_f64());
                                    } else {
                                        if (param == (param_bpja1 as i32)) {
                                            return Ok(*read(&cx, "cx")?);
                                        } else {
                                            if (param == (param_bpjec as i32)) {
                                                return Ok(*read(&ce, "ce")?);
                                            } else {
                                                if (param == (param_bpjom as i32)) {
                                                    return Ok(*read(&comega, "comega")?);
                                                } else {
                                                    if (param == (param_bpjpb as i32)) {
                                                        return Ok(((Float::with_val(
                                                            PRECISION_BITS,
                                                            (((-*read(&csigma, "csigma")?)
                                                                * *read(&an, "an")?)
                                                                * 86400.0f64),
                                                        ) * (tt0).clone())
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                (*read(&pb, "pb")? * 86400.0f64),
                                                            ))
                                                        .to_f64());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                return Ok(0.0f64);
                            }
                        }
                    }
                }
            }
            com -= 1;
        }
    }
    Ok(0.0f64)
}

/// T2model.C::updateT2 (pinned source line 598).
pub(super) fn updateT2(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut val: f64,
    mut err: f64,
    mut pos: i32,
    mut arr: i32,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if ((((((((((((((((((((pos == (param_pb as i32))
        || (pos == (param_a1 as i32)))
        || (pos == (param_ecc as i32)))
        || (pos == (param_t0 as i32)))
        || (pos == (param_sini as i32)))
        || (pos == (param_m2 as i32)))
        || (pos == (param_gamma as i32)))
        || (pos == (param_eps1 as i32)))
        || (pos == (param_eps2 as i32)))
        || (pos == (param_tasc as i32)))
        || (pos == (param_bpjph as i32)))
        || (pos == (param_bpja1 as i32)))
        || (pos == (param_bpjec as i32)))
        || (pos == (param_bpjom as i32)))
        || (pos == (param_bpjpb as i32)))
        || (pos == (param_shapmax as i32)))
        || (pos == (param_h3 as i32)))
        || (pos == (param_h4 as i32)))
        || (pos == (param_stig as i32)))
        || (pos == (param_edot as i32)))
    {
        {
            psr.param.slots[(pos) as usize].cells[(arr) as usize].val =
                ((psr.param.slots[(pos) as usize].cells[(arr) as usize].val).clone()
                    + Float::with_val(PRECISION_BITS, val));
            psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
                Float::with_val(PRECISION_BITS, err);
        }
    } else {
        if (((pos == (param_om as i32)) || (pos == (param_kom as i32)))
            || (pos == (param_kin as i32)))
        {
            {
                psr.param.slots[(pos) as usize].cells[(arr) as usize].val = ((psr.param.slots
                    [(pos) as usize]
                    .cells[(arr) as usize]
                    .val)
                    .clone()
                    + Float::with_val(PRECISION_BITS, ((val * 180.0f64) / std::f64::consts::PI)));
                psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
                    Float::with_val(PRECISION_BITS, ((err * 180.0f64) / std::f64::consts::PI));
            }
        } else {
            if (pos == (param_pbdot as i32)) {
                {
                    psr.param.slots[(pos) as usize].cells[(arr) as usize].val =
                        ((psr.param.slots[(pos) as usize].cells[(arr) as usize].val).clone()
                            + Float::with_val(PRECISION_BITS, val));
                    psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
                        Float::with_val(PRECISION_BITS, err);
                }
            } else {
                if (pos == (param_pb2dot as i32)) {
                    {
                        psr.param.slots[(pos) as usize].cells[(arr) as usize].val =
                            ((psr.param.slots[(pos) as usize].cells[(arr) as usize].val).clone()
                                + Float::with_val(PRECISION_BITS, val));
                        psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
                            Float::with_val(PRECISION_BITS, err);
                    }
                } else {
                    if ((((pos == (param_a1dot as i32)) || (pos == (param_a2dot as i32)))
                        || (pos == (param_eps1dot as i32)))
                        || (pos == (param_eps2dot as i32)))
                    {
                        {
                            psr.param.slots[(pos) as usize].cells[(arr) as usize].val =
                                ((psr.param.slots[(pos) as usize].cells[(arr) as usize].val)
                                    .clone()
                                    + Float::with_val(PRECISION_BITS, val));
                            psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
                                Float::with_val(PRECISION_BITS, err);
                        }
                    } else {
                        if (pos == (param_omdot as i32)) {
                            {
                                psr.param.slots[(pos) as usize].cells[(arr) as usize].val =
                                    ((psr.param.slots[(pos) as usize].cells[(arr) as usize].val)
                                        .clone()
                                        + Float::with_val(PRECISION_BITS, val));
                                psr.param.slots[(pos) as usize].cells[(arr) as usize].err =
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

/// T2model.C::getParameter (pinned source line 634).
pub(super) fn getParameter(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut p: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<Float, TimingError> {
    // T2model.C:638: k == aSize is outside the allocated parameter cells.
    // Preserve the source zero default for absent companions, including this boundary.
    if (k >= *read(&psr.param.slots[(p) as usize].a_size, "psr->param[p].aSize")?) {
        return Ok(Float::with_val(PRECISION_BITS, 0.0f64));
    }
    if (psr.param.slots[(p) as usize].cells[(k) as usize].param_set == 1i32) {
        {
            if (*read(
                &psr.param.slots[(p) as usize].n_link_to,
                "psr->param[p].nLinkTo",
            )? > 0i32)
            {
                return crate::tempo2_util::get_parameter_value_inner(
                    psr,
                    (p) as usize,
                    (k) as usize,
                    d,
                );
            } else {
                return Ok((psr.param.slots[(p) as usize].cells[(k) as usize].val).clone());
            }
        }
    }
    Ok(Float::with_val(PRECISION_BITS, 0.0f64))
}

/// T2model.C::calcGR (pinned source line 655).
pub(super) fn calcGR(
    mut mtot: f64,
    mut m2: f64,
    mut x: f64,
    mut ecc: f64,
    mut an: f64,
    mut afac: f64,
    mut f0: f64,
    mut dr: &mut Option<f64>,
    mut dth: &mut Option<f64>,
    mut er: &mut Option<f64>,
    mut eth: &mut Option<f64>,
    mut xk: &mut Option<f64>,
    mut si: &mut Option<f64>,
    mut gamma: &mut Option<f64>,
    mut pbdot: &mut Option<f64>,
    mut a0: &mut Option<f64>,
    mut b0: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut ARRTOL: f64 = 1.0e-10f64;
    let mut m1: f64;
    let mut arr0: f64;
    let mut arrold: f64;
    let mut arr: f64;
    let mut ar: f64;
    let mut a0aligned: f64;
    m1 = (mtot - m2);
    if (mtot < (0i32) as f64) {
        {
            d.stdout(b"ERROR: problem in DDGR model (mtot < 0)\n");
            return Err(boundary("T2model.C::calcGR: source exit"));
        }
    }
    arr0 = (mtot / (an * an)).powf(1.0f64 / 3.0f64);
    arr = arr0;
    loop {
        {
            arrold = arr;
            arr = (arr0
                * (1.0f64
                    + ((((((m1 * m2) / (mtot).powf((2i32) as f64)) - 9.0f64) * 0.5f64) * mtot)
                        / arr))
                    .powf(2.0f64 / 3.0f64));
        }
        if !(((arr - arrold) / arr).abs() > ARRTOL) {
            break;
        }
    }
    arr = (arr0
        * (1.0f64
            + ((((((m1 * m2) / (mtot).powf((2i32) as f64)) - 9.0f64) * 0.5f64) * mtot) / arr))
            .powf(2.0f64 / 3.0f64));
    ar = ((arr * m2) / mtot);
    *si = Some(x / ar);
    *xk = Some((3.0f64 * mtot) / (arr * (1.0f64 - (ecc * ecc))));
    *gamma = Some(((ecc * m2) * (m1 + ((2i32) as f64 * m2))) / ((an * arr) * mtot));
    *pbdot = Some(
        ((((((-(((96.0f64 * 2.0f64) * std::f64::consts::PI) / 5.0f64))
            * (an).powf(5.0f64 / 3.0f64))
            * (1.0f64 - (ecc).powf((2i32) as f64)).powf(-3.5f64))
            * (((1i32) as f64 + ((73.0f64 / (24i32) as f64) * (ecc).powf((2i32) as f64)))
                + ((37.0f64 / (96i32) as f64) * (ecc).powf((4i32) as f64))))
            * m1)
            * m2)
            * (mtot).powf((-1.0f64) / 3.0f64),
    );
    *dr = Some(
        (((3.0f64 * (m1).powf((2i32) as f64)) + ((6.0f64 * m1) * m2))
            + (2.0f64 * (m2).powf((2i32) as f64)))
            / (arr * mtot),
    );
    *er = Some(ecc * (1.0f64 + *read(&*dr, "*dr")?));
    *dth = Some(
        ((((3.5f64 * m1) * m1) + (((6i32) as f64 * m1) * m2)) + (((2i32) as f64 * m2) * m2))
            / (arr * mtot),
    );
    *eth = Some(ecc * (1.0f64 + *read(&*dth, "*dth")?));
    a0aligned = ((an * ar)
        / ((((2.0f64 * std::f64::consts::PI) * f0) * *read(&*si, "*si")?)
            * ((1i32) as f64 - (ecc * ecc)).sqrt()));
    *a0 = Some(afac * a0aligned);
    *b0 = Some((0i32) as f64);
    Ok(())
}

/// T2model.C::getKeplerian (pinned source line 715).
pub(super) fn getKeplerian(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut com: i32,
    mut pb: &mut Option<f64>,
    mut t0: &mut Option<Float>,
    mut ecc: &mut Option<f64>,
    mut omz: &mut Option<f64>,
    mut x: &mut Option<f64>,
    mut eps1: &mut Option<f64>,
    mut eps2: &mut Option<f64>,
    mut t0asc: &mut Option<Float>,
    mut shapmax: &mut Option<f64>,
    mut kom: &mut Option<f64>,
    mut kin: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    *pb = Some(
        (getParameter(psr, rows, (param_pb as i32), com, d)?
            * Float::with_val(PRECISION_BITS, 86400.0f64))
        .to_f64(),
    );
    *t0 = Some(getParameter(psr, rows, (param_t0 as i32), com, d)?);
    *ecc = Some((getParameter(psr, rows, (param_ecc as i32), com, d)?).to_f64());
    *omz = Some((getParameter(psr, rows, (param_om as i32), com, d)?).to_f64());
    *x = Some((getParameter(psr, rows, (param_a1 as i32), com, d)?).to_f64());
    *eps1 = Some((getParameter(psr, rows, (param_eps1 as i32), com, d)?).to_f64());
    *eps2 = Some((getParameter(psr, rows, (param_eps2 as i32), com, d)?).to_f64());
    *t0asc = Some(getParameter(psr, rows, (param_tasc as i32), com, d)?);
    *shapmax = Some((getParameter(psr, rows, (param_shapmax as i32), com, d)?).to_f64());
    *kom = Some(
        ((getParameter(psr, rows, (param_kom as i32), com, d)?
            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            / Float::with_val(PRECISION_BITS, 180.0f64))
        .to_f64(),
    );
    *kin = Some(
        ((getParameter(psr, rows, (param_kin as i32), com, d)?
            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            / Float::with_val(PRECISION_BITS, 180.0f64))
        .to_f64(),
    );
    Ok(())
}

/// T2model.C::addKeplerianJumps (pinned source line 740).
pub(super) fn addKeplerianJumps(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut torb: &mut Option<f64>,
    mut x: &mut Option<f64>,
    mut ecc: &mut Option<f64>,
    mut omz: &mut Option<f64>,
    mut pb: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut i: i32;
    {
        i = 0i32;
        while (i < *read(
            &psr.param.slots[param_bpjep as usize].a_size,
            "psr->param[param_bpjep].aSize",
        )?) {
            {
                if ((psr.param.slots[param_bpjep as usize].cells[(i) as usize].param_set == 1i32)
                    && ((read(&rows[(ipos) as usize].bbat, "psr->obsn[ipos].bbat")?).clone()
                        > (psr.param.slots[param_bpjep as usize].cells[(i) as usize].val).clone()))
                {
                    {
                        *torb = Some(
                            *read(&*torb, "*torb")?
                                - ((psr.param.slots[param_bpjph as usize].cells[(i) as usize].val)
                                    .clone()
                                    / (psr.param.slots[param_f as usize].cells[0].val).clone())
                                .to_f64(),
                        );
                        *x = Some(
                            *read(&*x, "*x")?
                                + ((psr.param.slots[param_bpja1 as usize].cells[(i) as usize].val)
                                    .clone())
                                .to_f64(),
                        );
                        *ecc = Some(
                            *read(&*ecc, "*ecc")?
                                + ((psr.param.slots[param_bpjec as usize].cells[(i) as usize].val)
                                    .clone())
                                .to_f64(),
                        );
                        *omz = Some(
                            *read(&*omz, "*omz")?
                                + ((psr.param.slots[param_bpjom as usize].cells[(i) as usize].val)
                                    .clone())
                                .to_f64(),
                        );
                        *pb = Some(
                            *read(&*pb, "*pb")?
                                + (((psr.param.slots[param_bpjpb as usize].cells[(i) as usize]
                                    .val)
                                    .clone())
                                .to_f64()
                                    * 86400.0f64),
                        );
                    }
                }
            }
            i += 1;
        }
    }
    Ok(())
}

/// T2model.C::getPostKeplerian (pinned source line 776).
pub(super) fn getPostKeplerian(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut com: i32,
    mut an: f64,
    mut si: &mut Option<f64>,
    mut m2: &mut Option<f64>,
    mut mtot: &mut Option<f64>,
    mut omdot: &mut Option<f64>,
    mut gamma: &mut Option<f64>,
    mut xdot: &mut Option<f64>,
    mut x2dot: &mut Option<f64>,
    mut xpbdot: &mut Option<f64>,
    mut pbdot: &mut Option<f64>,
    mut pb2dot: &mut Option<f64>,
    mut edot: &mut Option<f64>,
    mut pmra: &mut Option<f64>,
    mut pmdec: &mut Option<f64>,
    mut dpara: &mut Option<f64>,
    mut dr: &mut Option<f64>,
    mut dth: &mut Option<f64>,
    mut a0: &mut Option<f64>,
    mut b0: &mut Option<f64>,
    mut xomdot: &mut Option<f64>,
    mut afac: &mut Option<f64>,
    mut eps1dot: &mut Option<f64>,
    mut eps2dot: &mut Option<f64>,
    mut daop: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut rad2deg: f64 = (180.0f64 / std::f64::consts::PI);
    let mut pxConv: f64 = (((std::f64::consts::PI / 180.0f64) / (3600i32) as f64) * 1e-3f64);
    *si = Some((getParameter(psr, rows, (param_sini as i32), com, d)?).to_f64());
    if (*read(&*si, "*si")? > 1.0f64) {
        {
            d.display(
                1i32,
                "BIN1".as_bytes(),
                "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(),
                "".as_bytes(),
                *read(&psr.noWarnings, "psr[0].noWarnings")?,
            )?;
            *si = Some(1.0f64);
            psr.param.slots[param_sini as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, 1.0f64);
        }
    }
    if (*read(&*si, "*si")? < (-1.0f64)) {
        {
            d.display(
                1i32,
                "BIN1".as_bytes(),
                "SIN I < -1.0, setting to -1: should probably use DDS model".as_bytes(),
                "".as_bytes(),
                *read(&psr.noWarnings, "psr[0].noWarnings")?,
            )?;
            *si = Some(-1.0f64);
            psr.param.slots[param_sini as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, (-1.0f64));
        }
    }
    *m2 = Some(
        (getParameter(psr, rows, (param_m2 as i32), com, d)?
            * Float::with_val(PRECISION_BITS, SUNMASS))
        .to_f64(),
    );
    *mtot = Some(
        (getParameter(psr, rows, (param_mtot as i32), com, d)?
            * Float::with_val(PRECISION_BITS, SUNMASS))
        .to_f64(),
    );
    *omdot = Some(
        (getParameter(psr, rows, (param_omdot as i32), com, d)?
            / Float::with_val(PRECISION_BITS, (((rad2deg * 365.25f64) * 86400.0f64) * an)))
        .to_f64(),
    );
    *gamma = Some((getParameter(psr, rows, (param_gamma as i32), com, d)?).to_f64());
    *xdot = Some((getParameter(psr, rows, (param_a1dot as i32), com, d)?).to_f64());
    *x2dot = Some((getParameter(psr, rows, (param_a2dot as i32), com, d)?).to_f64());
    *xpbdot = Some((getParameter(psr, rows, (param_xpbdot as i32), com, d)?).to_f64());
    *pbdot = Some((getParameter(psr, rows, (param_pbdot as i32), com, d)?).to_f64());
    *pb2dot = Some((getParameter(psr, rows, (param_pb2dot as i32), com, d)?).to_f64());
    *edot = Some((getParameter(psr, rows, (param_edot as i32), com, d)?).to_f64());
    *pmra = Some(
        (((getParameter(psr, rows, (param_pmra as i32), com, d)?
            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            / Float::with_val(PRECISION_BITS, (180.0f64 * 3600.0e3f64)))
            / Float::with_val(PRECISION_BITS, (365.25f64 * 86400.0f64)))
        .to_f64(),
    );
    *pmdec = Some(
        (((getParameter(psr, rows, (param_pmdec as i32), com, d)?
            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            / Float::with_val(PRECISION_BITS, (180.0f64 * 3600.0e3f64)))
            / Float::with_val(PRECISION_BITS, (365.25f64 * 86400.0f64)))
        .to_f64(),
    );
    *dpara = Some(
        (getParameter(psr, rows, (param_px as i32), com, d)?
            * Float::with_val(PRECISION_BITS, pxConv))
        .to_f64(),
    );
    *dr = Some((getParameter(psr, rows, (param_dr as i32), com, d)?).to_f64());
    *dth = Some((getParameter(psr, rows, (param_dth as i32), com, d)?).to_f64());
    *a0 = Some((getParameter(psr, rows, (param_a0 as i32), com, d)?).to_f64());
    *b0 = Some((getParameter(psr, rows, (param_b0 as i32), com, d)?).to_f64());
    *xomdot = Some(
        (getParameter(psr, rows, (param_xomdot as i32), com, d)?
            / Float::with_val(PRECISION_BITS, (((an * rad2deg) * 365.25f64) * 86400.0f64)))
        .to_f64(),
    );
    *afac = Some((getParameter(psr, rows, (param_afac as i32), com, d)?).to_f64());
    *eps1dot = Some((getParameter(psr, rows, (param_eps1dot as i32), com, d)?).to_f64());
    *eps2dot = Some((getParameter(psr, rows, (param_eps2dot as i32), com, d)?).to_f64());
    *daop = Some(
        ((getParameter(psr, rows, (param_daop as i32), com, d)?
            * Float::with_val(PRECISION_BITS, 1e-3f64))
            / Float::with_val(PRECISION_BITS, pxConv))
        .to_f64(),
    );
    Ok(())
}

/// T2model.C::updateParameters (pinned source line 829).
pub(super) fn updateParameters(
    mut edot: f64,
    mut xdot: f64,
    mut x2dot: f64,
    mut eps1dot: f64,
    mut eps2dot: f64,
    mut tt0: Float,
    mut ecc: &mut Option<f64>,
    mut x: &mut Option<f64>,
    mut eps1: &mut Option<f64>,
    mut eps2: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    *ecc = Some(
        (Float::with_val(PRECISION_BITS, *read(&*ecc, "*ecc")?)
            + (Float::with_val(PRECISION_BITS, edot) * (tt0).clone()))
        .to_f64(),
    );
    *x = Some(
        (Float::with_val(PRECISION_BITS, *read(&*x, "*x")?)
            + ((Float::with_val(PRECISION_BITS, xdot) * (tt0).clone())
                + ((Float::with_val(PRECISION_BITS, (0.5f64 * x2dot)) * (tt0).clone())
                    * (tt0).clone())))
        .to_f64(),
    );
    *eps1 = Some(
        (Float::with_val(PRECISION_BITS, *read(&*eps1, "*eps1")?)
            + (Float::with_val(PRECISION_BITS, eps1dot) * (tt0).clone()))
        .to_f64(),
    );
    *eps2 = Some(
        (Float::with_val(PRECISION_BITS, *read(&*eps2, "*eps2")?)
            + (Float::with_val(PRECISION_BITS, eps2dot) * (tt0).clone()))
        .to_f64(),
    );
    Ok(())
}

/// T2model.C::deriveKeplerian (pinned source line 838).
pub(super) fn deriveKeplerian(
    mut pb: f64,
    mut kom: f64,
    mut an: &mut Option<f64>,
    mut sin_omega: &mut Option<f64>,
    mut cos_omega: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    *an = Some((2.0f64 * std::f64::consts::PI) / pb);
    *sin_omega = Some((kom).sin());
    *cos_omega = Some((kom).cos());
    Ok(())
}

/// T2model.C::derivePostKeplerian (pinned source line 845).
pub(super) fn derivePostKeplerian(
    mut mtot: f64,
    mut m2: f64,
    mut dr: f64,
    mut dth: f64,
    mut ecc: f64,
    mut m1: &mut Option<f64>,
    mut er: &mut Option<f64>,
    mut eth: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    *m1 = Some(mtot - m2);
    *er = Some(ecc * (1.0f64 + dr));
    *eth = Some(ecc * (1.0f64 + dth));
    Ok(())
}

/// T2model.C::KopeikinTerms (pinned source line 853).
pub(super) fn KopeikinTerms(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut ki: f64,
    mut pmra: f64,
    mut sin_omega: f64,
    mut pmdec: f64,
    mut cos_omega: f64,
    mut tt0: Float,
    mut dpara: f64,
    mut daop: f64,
    mut si: f64,
    mut x: &mut Option<f64>,
    mut DK011: &mut Option<Float>,
    mut DK012: &mut Option<Float>,
    mut DK021: &mut Option<Float>,
    mut DK022: &mut Option<Float>,
    mut DK031: &mut Option<Float>,
    mut DK032: &mut Option<Float>,
    mut DK041: &mut Option<Float>,
    mut DK042: &mut Option<Float>,
    mut DK013: &mut Option<Float>,
    mut DK014: &mut Option<Float>,
    mut DK023: &mut Option<Float>,
    mut DK024: &mut Option<Float>,
    mut DK033: &mut Option<Float>,
    mut DK034: &mut Option<Float>,
    mut DK043: &mut Option<Float>,
    mut DK044: &mut Option<Float>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut sini: f64;
    let mut cosi: f64;
    let mut tani: f64;
    let mut sin_delta: f64;
    let mut cos_delta: f64;
    let mut sin_alpha: f64;
    let mut cos_alpha: f64;
    let mut delta_i0: f64;
    let mut delta_j0: f64;
    si = (ki).sin();
    sini = (ki).sin();
    cosi = (ki).cos();
    tani = (sini / cosi);
    sin_delta = *read(
        &rows[(ipos) as usize].psrPos[2],
        "psr->obsn[ipos].psrPos[2]",
    )?;
    cos_delta = ((sin_delta).asin()).cos();
    sin_alpha = (*read(
        &rows[(ipos) as usize].psrPos[1],
        "psr->obsn[ipos].psrPos[1]",
    )? / cos_delta);
    cos_alpha = (*read(
        &rows[(ipos) as usize].psrPos[0],
        "psr->obsn[ipos].psrPos[0]",
    )? / cos_delta);
    delta_i0 = ((((-*read(
        &rows[(ipos) as usize].earth_ssb[0],
        "psr->obsn[ipos].earth_ssb[0]",
    )?) / 499.00478364f64)
        * sin_alpha)
        + ((*read(
            &rows[(ipos) as usize].earth_ssb[1],
            "psr->obsn[ipos].earth_ssb[1]",
        )? / 499.00478364f64)
            * cos_alpha));
    delta_j0 = ((((((-*read(
        &rows[(ipos) as usize].earth_ssb[0],
        "psr->obsn[ipos].earth_ssb[0]",
    )?) / 499.00478364f64)
        * sin_delta)
        * cos_alpha)
        - (((*read(
            &rows[(ipos) as usize].earth_ssb[1],
            "psr->obsn[ipos].earth_ssb[1]",
        )? / 499.00478364f64)
            * sin_delta)
            * sin_alpha))
        + ((*read(
            &rows[(ipos) as usize].earth_ssb[2],
            "psr->obsn[ipos].earth_ssb[2]",
        )? / 499.00478364f64)
            * cos_delta));
    if (psr.param.slots[param_daop as usize].cells[0].param_set == 1i32) {
        {
            *DK011 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) / daop) / si) * delta_i0) * sin_omega),
            ));
            *DK012 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) / daop) / si) * delta_j0) * cos_omega),
            ));
            *DK013 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) / daop) / si) * delta_i0) * cos_omega),
            ));
            *DK014 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? / daop) / si) * delta_j0) * sin_omega),
            ));
            *DK021 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? / daop) / tani) * delta_i0) * cos_omega),
            ));
            *DK022 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) / daop) / tani) * delta_j0) * sin_omega),
            ));
            *DK023 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? / daop) / tani) * delta_i0) * sin_omega),
            ));
            *DK024 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? / daop) / tani) * delta_j0) * cos_omega),
            ));
        }
    } else {
        {
            *DK011 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) * dpara) / si) * delta_i0) * sin_omega),
            ));
            *DK012 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) * dpara) / si) * delta_j0) * cos_omega),
            ));
            *DK013 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) * dpara) / si) * delta_i0) * cos_omega),
            ));
            *DK014 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? * dpara) / si) * delta_j0) * sin_omega),
            ));
            *DK021 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? * dpara) / tani) * delta_i0) * cos_omega),
            ));
            *DK022 = Some(Float::with_val(
                PRECISION_BITS,
                (((((-*read(&*x, "*x")?) * dpara) / tani) * delta_j0) * sin_omega),
            ));
            *DK023 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? * dpara) / tani) * delta_i0) * sin_omega),
            ));
            *DK024 = Some(Float::with_val(
                PRECISION_BITS,
                ((((*read(&*x, "*x")? * dpara) / tani) * delta_j0) * cos_omega),
            ));
        }
    }
    *DK031 = Some(
        (((Float::with_val(PRECISION_BITS, *read(&*x, "*x")?) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, si))
            * Float::with_val(PRECISION_BITS, pmra))
            * Float::with_val(PRECISION_BITS, sin_omega),
    );
    *DK032 = Some(
        (((Float::with_val(PRECISION_BITS, *read(&*x, "*x")?) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, si))
            * Float::with_val(PRECISION_BITS, pmdec))
            * Float::with_val(PRECISION_BITS, cos_omega),
    );
    *DK033 = Some(
        (((Float::with_val(PRECISION_BITS, *read(&*x, "*x")?) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, si))
            * Float::with_val(PRECISION_BITS, pmra))
            * Float::with_val(PRECISION_BITS, cos_omega),
    );
    *DK034 = Some(
        (((Float::with_val(PRECISION_BITS, (-*read(&*x, "*x")?)) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, si))
            * Float::with_val(PRECISION_BITS, pmdec))
            * Float::with_val(PRECISION_BITS, sin_omega),
    );
    *DK041 = Some(
        (((Float::with_val(PRECISION_BITS, *read(&*x, "*x")?) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, tani))
            * Float::with_val(PRECISION_BITS, pmra))
            * Float::with_val(PRECISION_BITS, cos_omega),
    );
    *DK042 = Some(
        (((Float::with_val(PRECISION_BITS, (-*read(&*x, "*x")?)) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, tani))
            * Float::with_val(PRECISION_BITS, pmdec))
            * Float::with_val(PRECISION_BITS, sin_omega),
    );
    *DK043 = Some(
        (((Float::with_val(PRECISION_BITS, (-*read(&*x, "*x")?)) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, tani))
            * Float::with_val(PRECISION_BITS, pmra))
            * Float::with_val(PRECISION_BITS, sin_omega),
    );
    *DK044 = Some(
        (((Float::with_val(PRECISION_BITS, (-*read(&*x, "*x")?)) * (tt0).clone())
            / Float::with_val(PRECISION_BITS, tani))
            * Float::with_val(PRECISION_BITS, pmdec))
            * Float::with_val(PRECISION_BITS, cos_omega),
    );
    Ok(())
}

/// T2model.C::computeU (pinned source line 948).
pub(super) fn computeU(
    mut phase: f64,
    mut ecc: f64,
    mut u: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut du: f64;
    *u = Some(
        phase
            + ((ecc * (phase).sin())
                / ((1.0f64 - (((2i32) as f64 * ecc) * (phase).cos())) + (ecc * ecc)).sqrt()),
    );
    loop {
        {
            du = ((phase - (*read(&*u, "*u")? - (ecc * (*read(&*u, "*u")?).sin())))
                / (1.0f64 - (ecc * (*read(&*u, "*u")?).cos())));
            *u = Some(*read(&*u, "*u")? + du);
        }
        if !((du).abs() > 1.0e-14f64) {
            break;
        }
    }
    Ok(())
}

/// T2model.C::calcDH (pinned source line 1022).
pub(super) fn calcDH(
    mut ae: f64,
    mut h3: f64,
    mut h4: f64,
    mut nharm: i32,
    mut sel: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut s: f64 = (h4 / h3);
    let mut FirstH3: f64 = (((-4.0f64) / 3.0f64) * (3.0f64 * ae).sin());
    let mut SecondH3: f64 = 0.0f64;
    let mut FirstH4: f64 = (4.0f64 * ae).cos();
    let mut SecondH4: f64 = 0.0f64;
    let mut sd3: f64 = ((((-4.0f64) / 3.0f64) * h3) * (3.0f64 * ae).sin());
    let mut sd4: f64 = (h4 * (4.0f64 * ae).cos());
    let mut sd5: f64 = 0.0f64;
    let mut fs: f64 = ((s * (5.0f64 * ae).sin()) / 5.0f64);
    let mut dfsds: f64 = ((5.0f64 * ae).sin() / 5.0f64);
    let mut count: f64;
    if (nharm > 5i32) {
        {
            let mut EvenCt: i32 = 6i32;
            while (EvenCt <= nharm) {
                {
                    count = (EvenCt) as f64;
                    fs += ((((-1.0f64).powf(count / 2.0f64) / count) * (s).powf(count - 4.0f64))
                        * (count * ae).cos());
                    dfsds += (((((-1.0f64).powf(count / 2.0f64) / count)
                        * (s).powf(count - 5.0f64))
                        * (count * ae).cos())
                        * (count - 4.0f64));
                }
                EvenCt = EvenCt
                    .checked_add(2)
                    .ok_or_else(|| boundary("harmonic count overflow"))?;
            }
        }
    }
    if (nharm > 6i32) {
        {
            let mut OddCt: i32 = 7i32;
            while (OddCt <= nharm) {
                {
                    count = (OddCt) as f64;
                    fs += ((((-1.0f64).powf((count - 1.0f64) / 2.0f64) / count)
                        * (s).powf(count - 4.0f64))
                        * (count * ae).sin());
                    dfsds += (((((-1.0f64).powf((count - 1.0f64) / 2.0f64) / count)
                        * (s).powf(count - 5.0f64))
                        * (count * ae).sin())
                        * (count - 4.0f64));
                }
                OddCt = OddCt
                    .checked_add(2)
                    .ok_or_else(|| boundary("harmonic count overflow"))?;
            }
        }
    }
    if (nharm > 4i32) {
        {
            SecondH3 = ((((-4.0f64) * dfsds) * s) * s);
            SecondH4 = (4.0f64 * (fs + (dfsds * s)));
            sd5 = ((4.0f64 * h4) * fs);
        }
    }
    if (sel == 3i32) {
        Ok(FirstH3 + SecondH3)
    } else {
        if (sel == 4i32) {
            Ok(FirstH4 + SecondH4)
        } else {
            if (sel == 0i32) {
                Ok((sd3 + sd4) + sd5)
            } else {
                {
                    d.stdout(b"ERROR in ELL1Hmodel! This shouldn't be happening.\n");
                    Ok((0i32) as f64)
                }
            }
        }
    }
}

// T2model.C:192 and subsequent per-companion predicates also index scalar
// optional arrays (MTOT, KIN, SHAPMAX, ...). As in getParameter, a companion
// beyond that parameter's allocation is absent, not an out-of-bounds read.
fn companion_set(psr: &SourcePulsar, label: Label, companion: i32) -> bool {
    psr.param.slots[label as usize]
        .cells
        .get(companion as usize)
        .is_some_and(|cell| cell.param_set == 1)
}
