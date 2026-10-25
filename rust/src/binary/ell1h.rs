//! ELL1Hmodel.C model, derivatives, updates and private helpers.
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
/// ELL1Hmodel.C::ELL1Hmodel (pinned source line 34).
pub(super) fn ELL1Hmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: f64;
    let mut x0: f64;
    let mut m2: Option<f64> = None;
    let mut tt0: f64;
    let mut orbits: f64;
    let mut phase: f64;
    let mut e1: f64;
    let mut e2: f64;
    let mut dre: f64;
    let mut drep: f64;
    let mut drepp: f64;
    let mut brace: Option<f64> = None;
    let mut dlogbr: Option<f64> = None;
    let mut ds: f64;
    let mut da: f64;
    let mut pb: f64;
    let mut eps1: f64;
    let mut eps2: f64;
    let mut eps1dot: f64;
    let mut eps2dot: f64;
    let mut si: Option<f64> = None;
    let mut a0: f64;
    let mut b0: f64;
    let mut d2bar: f64;
    let mut torb: f64;
    let mut Csigma: f64;
    let mut Cx: f64;
    let mut Ceps1: f64;
    let mut Ceps2: f64;
    let mut Cm2: Option<f64> = None;
    let mut Csi: Option<f64> = None;
    let mut ct: f64;
    let mut t0asc: f64;
    let mut pbdot: f64;
    let mut xpbdot: f64;
    let mut x: f64;
    let mut xdot: f64;
    let mut norbits: i32;
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut TrueAnom: f64;
    let mut h3: f64;
    let mut h4: f64;
    let mut stig: Option<f64> = None;
    let mut lgf: Option<f64> = None;
    let mut nharm: i32 = 4i32;
    let mut mode: i32 = (-1i32);
    a0 = 0.0f64;
    b0 = 0.0f64;
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = ((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        pbdot = 0.0f64;
    }
    an = ((2.0f64 * std::f64::consts::PI) / pb);
    if (psr.param.slots[param_h3 as usize].cells[0].param_set != 1i32) {
        {
            d.log(
                b'e',
                "ELL1Hmodel.C",
                78,
                b"Cannot use the ELL1H model without H3.",
            );
        }
    }
    h3 = ((psr.param.slots[param_h3 as usize].cells[0].val).clone()).to_f64();
    h4 = ((psr.param.slots[param_h4 as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_h4 as usize].cells[0].param_set == 1i32) {
        {
            if (psr.param.slots[param_nharm as usize].cells[0].param_set == 1i32) {
                {
                    nharm = crate::ifteph::extended_i32(
                        &((psr.param.slots[param_nharm as usize].cells[0].val).clone()),
                        "binary_integer",
                    )?;
                    if (nharm > 4i32) {
                        mode = 3i32;
                    } else {
                        mode = 2i32;
                    }
                }
            }
            if (psr.param.slots[param_stig as usize].cells[0].param_set == 1i32) {
                {
                    d.log(
                        b'e',
                        "ELL1Hmodel.C",
                        97,
                        b"You specified both H4 and STIG - Ignoring STIG",
                    );
                    d.log(
                        b'm',
                        "ELL1Hmodel.C",
                        98,
                        b"WARNING! You specified both H4 and STIG.",
                    );
                    d.log(
                        b'm',
                        "ELL1Hmodel.C",
                        99,
                        b"We will ignore STIG and perform the approx. H4 fit instead.",
                    );
                    d.log(
                        b'm',
                        "ELL1Hmodel.C",
                        100,
                        b"If you want to perform the exact fit for H3 and STIG, then ",
                    );
                    d.log(
                        b'm',
                        "ELL1Hmodel.C",
                        101,
                        b"please remove H4 from your parameter file.",
                    );
                }
            }
            mode = 2i32;
        }
    } else {
        {
            if (psr.param.slots[param_nharm as usize].cells[0].param_set == 1i32) {
                {
                    d.log(
                        b'e',
                        "ELL1Hmodel.C",
                        107,
                        b"ELL1H - Cannot use NHARM without H4. Reverting to H3 only and nharm=3",
                    );
                }
            }
            if (psr.param.slots[param_stig as usize].cells[0].param_set == 1i32) {
                {
                    stig = Some(
                        ((psr.param.slots[param_stig as usize].cells[0].val).clone()).to_f64(),
                    );
                    mode = 1i32;
                }
            } else {
                {
                    mode = 0i32;
                    h4 = (0i32) as f64;
                    nharm = 3i32;
                }
            }
        }
    }
    if (mode == 1i32) {
        {
            si = Some(
                (2.0f64 * *read(&stig, "stig")?) / (1.0f64 + (*read(&stig, "stig")?).powf(2.0f64)),
            );
            m2 = Some(h3 / (*read(&stig, "stig")?).powf(3.0f64));
            if (*read(&si, "si")? > 1.0f64) {
                {
                    d.display(
                        1i32,
                        "BIN1".as_bytes(),
                        "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(),
                        "".as_bytes(),
                        *read(&psr.noWarnings, "psr[p].noWarnings")?,
                    )?;
                    si = Some(1.0f64);
                    psr.param.slots[param_sini as usize].cells[0].val =
                        Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap());
                }
            }
        }
    } else {
        if ((mode == 2i32) || (mode == 3i32)) {
            {
                si = Some(((2.0f64 * h3) * h4) / ((h3 * h3) + (h4 * h4)));
                m2 = Some((h3).powf(4.0f64) / (h4).powf(3.0f64));
                if (*read(&si, "si")? > 1.0f64) {
                    {
                        d.display(
                            1i32,
                            "BIN1".as_bytes(),
                            "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(),
                            "".as_bytes(),
                            *read(&psr.noWarnings, "psr[p].noWarnings")?,
                        )?;
                        si = Some(1.0f64);
                        psr.param.slots[param_sini as usize].cells[0].val =
                            Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap());
                    }
                }
            }
        } else {
            if (mode == 0i32) {
                {}
            } else {
                {
                    d.stdout(b"This should not be possible. Go Away.\n");
                    d.stdout(b"And tell someone about it: joris.verbiest@gmail.com, e.g.\n");
                }
            }
        }
    }
    x0 = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        xdot = 0.0f64;
    }
    t0asc = ((psr.param.slots[param_tasc as usize].cells[0].val).clone()).to_f64();
    xpbdot = 0.0f64;
    eps1 = ((psr.param.slots[param_eps1 as usize].cells[0].val).clone()).to_f64();
    eps2 = ((psr.param.slots[param_eps2 as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_eps1dot as usize].cells[0].param_set == 1i32) {
        eps1dot = ((psr.param.slots[param_eps1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        eps1dot = (0i32) as f64;
    }
    if (psr.param.slots[param_eps2dot as usize].cells[0].param_set == 1i32) {
        eps2dot = ((psr.param.slots[param_eps2dot as usize].cells[0].val).clone()).to_f64();
    } else {
        eps2dot = (0i32) as f64;
    }
    ct = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()).to_f64();
    tt0 = ((ct - t0asc) * 86400.0f64);
    orbits = ((tt0 / pb) - ((0.5f64 * (pbdot + xpbdot)) * (tt0 / pb).powf((2i32) as f64)));
    norbits = source_i32(orbits)?;
    if (orbits < 0.0f64) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
    x = (x0 + (xdot * tt0));
    e1 = (eps1 + (eps1dot * tt0));
    e2 = (eps2 + (eps2dot * tt0));
    dre = (x
        * ((phase).sin()
            - (0.5f64 * ((e1 * (2.0f64 * phase).cos()) - (e2 * (2.0f64 * phase).sin())))));
    drep = (x * (phase).cos());
    drepp = ((-x) * (phase).sin());
    // ELL1Hmodel.C:198-206,246-247: H3-only does not define SINI/M2;
    // unused conventional derivatives and STIG terms must not read them.
    if param == param_sini as i32 || param == param_m2 as i32 {
        brace = Some(1.0f64 - (*read(&si, "si")? * (phase).sin()));
        dlogbr = Some((*read(&brace, "brace")?).ln());
    }
    TrueAnom = phase;
    let mut lsc: Option<f64> = None;
    let mut fs: Option<f64> = None;
    if mode == 1 || param == param_stig as i32 {
        fs = Some(
            (1.0f64 + (*read(&stig, "stig")? * *read(&stig, "stig")?))
                - ((2.0f64 * *read(&stig, "stig")?) * (TrueAnom).sin()),
        );
        lgf = Some((*read(&fs, "fs")?).ln());
        lsc = Some(
            (*read(&lgf, "lgf")? + ((2.0f64 * *read(&stig, "stig")?) * (TrueAnom).sin()))
                - ((*read(&stig, "stig")? * *read(&stig, "stig")?) * (2.0f64 * TrueAnom).cos()),
        );
    }
    if (mode == 0i32) {
        {
            ds = ((((-4.0f64) / 3.0f64) * h3) * (3.0f64 * TrueAnom).sin());
        }
    } else {
        if (mode == 1i32) {
            {
                ds = (((-2.0f64) * *read(&m2, "m2")?) * *read(&lsc, "lsc")?);
            }
        } else {
            {
                ds = calcDH(TrueAnom, h3, h4, nharm, 0i32, d)?;
            }
        }
    }
    da = ((a0 * (phase).sin()) + (b0 * (phase).cos()));
    d2bar = (((dre
        * ((((1i32) as f64 - (an * drep)) + (an * drep).powf((2i32) as f64))
            + (((0.5f64 * (an).powf((2i32) as f64)) * dre) * drepp)))
        + ds)
        + da);
    torb = (-d2bar);
    if (param == (-1i32)) {
        return Ok(torb);
    }
    Csigma = (x * (phase).cos());
    Cx = (phase).sin();
    Ceps1 = (((-0.5f64) * x) * ((2i32) as f64 * phase).cos());
    Ceps2 = ((0.5f64 * x) * ((2i32) as f64 * phase).sin());
    if param == param_m2 as i32 {
        Cm2 = Some((-2i32) as f64 * *read(&dlogbr, "dlogbr")?);
    }
    if param == param_sini as i32 {
        Csi = Some((((2i32) as f64 * *read(&m2, "m2")?) * (phase).sin()) / *read(&brace, "brace")?);
    }
    if (param == (param_pb as i32)) {
        {
            return Ok(((((-Csigma) * an) * 86400.0f64) * tt0) / (pb * 86400.0f64));
        }
    } else {
        if (param == (param_a1 as i32)) {
            {
                return Ok(Cx);
            }
        } else {
            if (param == (param_eps1 as i32)) {
                {
                    return Ok(Ceps1);
                }
            } else {
                if (param == (param_tasc as i32)) {
                    {
                        return Ok(((-Csigma) * an) * 86400.0f64);
                    }
                } else {
                    if (param == (param_eps2 as i32)) {
                        {
                            return Ok(Ceps2);
                        }
                    } else {
                        if (param == (param_eps1dot as i32)) {
                            {
                                return Ok(Ceps1 * tt0);
                            }
                        } else {
                            if (param == (param_eps2dot as i32)) {
                                {
                                    return Ok(Ceps2 * tt0);
                                }
                            } else {
                                if (param == (param_pbdot as i32)) {
                                    {
                                        return Ok((0.5f64 * tt0)
                                            * (((((-Csigma) * an) * 86400.0f64) * tt0)
                                                / (pb * 86400.0f64)));
                                    }
                                } else {
                                    if (param == (param_a1dot as i32)) {
                                        {
                                            return Ok(Cx * tt0);
                                        }
                                    } else {
                                        if (param == (param_sini as i32)) {
                                            {
                                                return Ok(*read(&Csi, "Csi")?);
                                            }
                                        } else {
                                            if (param == (param_m2 as i32)) {
                                                {
                                                    return Ok(*read(&Cm2, "Cm2")? * SUNMASS);
                                                }
                                            } else {
                                                if (param == (param_stig as i32)) {
                                                    {
                                                        return Ok(((((-2.0f64)
                                                            * *read(&m2, "m2")?)
                                                            / *read(&stig, "stig")?)
                                                            * ((1.0f64
                                                                - (3.0f64
                                                                    * *read(&lgf, "lgf")?))
                                                                - ((1.0f64
                                                                    - (*read(&stig, "stig")?
                                                                        * *read(
                                                                            &stig, "stig",
                                                                        )?))
                                                                    / *read(&fs, "fs")?)))
                                                            + ((2.0f64 * *read(&m2, "m2")?)
                                                                * ((4.0f64 * (TrueAnom).sin())
                                                                    - (*read(&stig, "stig")?
                                                                        * (2.0f64 * TrueAnom)
                                                                            .cos()))));
                                                    }
                                                } else {
                                                    if (param == (param_h3 as i32)) {
                                                        {
                                                            if ((mode == 0i32) || (mode == 2i32)) {
                                                                return Ok(((-4.0f64) / 3.0f64)
                                                                    * (3.0f64 * TrueAnom).sin());
                                                            } else {
                                                                if (mode == 1i32) {
                                                                    {
                                                                        return Ok(((-2.0f64)
                                                                            * *read(
                                                                                &lsc, "lsc",
                                                                            )?)
                                                                            / (*read(
                                                                                &stig, "stig",
                                                                            )?)
                                                                            .powf(3.0f64));
                                                                    }
                                                                } else {
                                                                    if (mode == 3i32) {
                                                                        return calcDH(
                                                                            TrueAnom, h3, h4,
                                                                            nharm, 3i32, d,
                                                                        );
                                                                    } else {
                                                                        {
                                                                            d.stdout(b"ERROR in ELL1H. This really shouldn't happen.\n");
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if (param == (param_h4 as i32)) {
                                                            {
                                                                if (mode == 2i32) {
                                                                    return Ok(
                                                                        (4.0f64 * TrueAnom).cos()
                                                                    );
                                                                } else {
                                                                    return calcDH(
                                                                        TrueAnom, h3, h4, nharm,
                                                                        4i32, d,
                                                                    );
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
    Ok(0.0f64)
}

/// ELL1Hmodel.C::updateELL1H (pinned source line 306).
pub(super) fn updateELL1H(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut val: f64,
    mut err: f64,
    mut pos: i32,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if (pos == (param_pb as i32)) {
        {
            psr.param.slots[param_pb as usize].cells[0].val =
                ((psr.param.slots[param_pb as usize].cells[0].val).clone()
                    + Float::with_val(PRECISION_BITS, (val / 86400.0f64)));
            psr.param.slots[param_pb as usize].cells[0].err =
                Float::with_val(PRECISION_BITS, (err / 86400.0f64));
        }
    } else {
        if (((((((((((pos == (param_a1 as i32)) || (pos == (param_eps1 as i32)))
            || (pos == (param_eps2 as i32)))
            || (pos == (param_tasc as i32)))
            || (pos == (param_sini as i32)))
            || (pos == (param_m2 as i32)))
            || (pos == (param_eps1dot as i32)))
            || (pos == (param_eps2dot as i32)))
            || (pos == (param_h3 as i32)))
            || (pos == (param_h4 as i32)))
            || (pos == (param_stig as i32)))
        {
            {
                psr.param.slots[(pos) as usize].cells[0].val =
                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                        + Float::with_val(PRECISION_BITS, val));
                psr.param.slots[(pos) as usize].cells[0].err = Float::with_val(PRECISION_BITS, err);
            }
        } else {
            if (pos == (param_pbdot as i32)) {
                {
                    psr.param.slots[(pos) as usize].cells[0].val =
                        ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                            + Float::with_val(PRECISION_BITS, val));
                    psr.param.slots[(pos) as usize].cells[0].err =
                        Float::with_val(PRECISION_BITS, err);
                }
            } else {
                if (pos == (param_a1dot as i32)) {
                    {
                        psr.param.slots[(pos) as usize].cells[0].val =
                            ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                + Float::with_val(PRECISION_BITS, val));
                        psr.param.slots[(pos) as usize].cells[0].err =
                            ((psr.param.slots[(pos) as usize].cells[0].err).clone()
                                + Float::with_val(PRECISION_BITS, err));
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
                    }
                }
            }
        }
    }
    Ok(())
}

/// ELL1Hmodel.C::calcDH (pinned source line 330).
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
