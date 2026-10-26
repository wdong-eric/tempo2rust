//! textOutput.C full report/PAR/helper transfer at fda6ad4; GPL-3.0-or-later.
//! Lint allowances preserve source grouping, constants, branches and first-knot loop.
//! Explicit source-order arithmetic and canonical state; no native ABI layout.
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unreachable_code,
    clippy::assign_op_pattern,
    clippy::double_parens,
    clippy::needless_borrow,
    clippy::unnecessary_to_owned,
    clippy::excessive_precision,
    clippy::needless_return,
    clippy::too_many_arguments,
    clippy::get_first,
    clippy::unnecessary_cast,
    clippy::if_same_then_else,
    clippy::never_loop
)]
use super::*;
use crate::tempo2::Label::*;
use rug::ops::Pow;
/// textOutput.C:60::textOutput.
pub(crate) fn text_output_inner(
    context: &mut PulsarContext,
    globalParameter: f64,
    nGlobal: i32,
    outRes: i32,
    newpar: i32,
    fname: &[u8],
    io: &mut OutputIo<'_>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut chisqr: f64;
    let mut i: i32;
    let mut p: i32;
    let mut k: i32;
    let mut fout = None::<std::fs::File>;
    d.version("textOutput.C", "textOutput()");
    d.log(
        b'd',
        "textOutput.C",
        71,
        &crate::source_format::format("In textOutput", &[])?,
    );
    io.drain(d)?;
    {
        p = 0i32;
        while (p < context.pulsars.len() as i32) {
            {
                // fda6ad4 textOutput.C:150,1441: unweighted PAR read had no producer; use fitChisq.
                chisqr = *read(&context.pulsars[(p) as usize].fitChisq, "psr[p].fitChisq")?;
                if (outRes == 1i32) {
                    {
                        fout = io.create(&"residuals.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Unable to open file residuals.dat for writing\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nobs,
                                        "psr[p].nobs",
                                    )?) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.10g %.10g %.10g\n",
                                                &[
                                                    Argument::from(
                                                        ((read(
                                                            &output_rows(context, (p) as usize)?
                                                                [(i) as usize]
                                                                .bat,
                                                            "psr[p].obsn[i].bat",
                                                        )?)
                                                        .clone()
                                                            - (context.pulsars[(p) as usize]
                                                                .param
                                                                .slots
                                                                [(param_pepoch as i32) as usize]
                                                                .cells
                                                                [(0i32) as usize]
                                                                .val)
                                                                .clone())
                                                        .to_f64(),
                                                    ),
                                                    Argument::from(
                                                        ((read(
                                                            &output_rows(context, (p) as usize)?
                                                                [(i) as usize]
                                                                .residual,
                                                            "psr[p].obsn[i].residual",
                                                        )?)
                                                        .clone())
                                                        .to_f64(),
                                                    ),
                                                    Argument::from(
                                                        (*read(
                                                            &output_rows(context, (p) as usize)?
                                                                [(i) as usize]
                                                                .toaErr,
                                                            "psr[p].obsn[i].toaErr",
                                                        )? * 1.0e-6f64),
                                                    ),
                                                ],
                                            )?,
                                        )?;
                                        i = (i + 1);
                                    }
                                }
                                fout.take();
                            }
                        }
                    }
                }
                calc_rms_inner(context, p as usize, d)?;
                io.drain(d)?;
                if (*read(&context.pulsars[(p) as usize].nobs, "psr[p].nobs")? > 0i32) {
                    {
                        let mut centrePos: Float;
                        let mut closestV: Option<Float> = None;
                        let mut check: Float;
                        let mut closestI: i32 = (-1i32);
                        centrePos = (((context.pulsars[(p) as usize].param.slots
                            [(param_start as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone()
                            + (context.pulsars[(p) as usize].param.slots
                                [(param_finish as i32) as usize]
                                .cells[(0i32) as usize]
                                .val)
                                .clone())
                            / Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap()));
                        {
                            i = 0i32;
                            while (i < *read(&context.pulsars[(p) as usize].nobs, "psr[p].nobs")?) {
                                {
                                    if (*read(
                                        &output_rows(context, (p) as usize)?[(i) as usize].deleted,
                                        "psr[p].obsn[i].deleted",
                                    )? == 0i32)
                                    {
                                        {
                                            check = ((read(
                                                &output_rows(context, (p) as usize)?[(i) as usize]
                                                    .sat,
                                                "psr[p].obsn[i].sat",
                                            )?)
                                            .clone()
                                                - (centrePos).clone())
                                            .abs();
                                            if ((closestI == (-1i32))
                                                || ((check).clone()
                                                    < (read(&closestV, "closestV")?).clone()))
                                            {
                                                {
                                                    closestV = Some((check).clone());
                                                    closestI = i;
                                                }
                                            }
                                        }
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                        if (closestI == (-1i32)) {
                            {
                                d.log(
                                    b'e',
                                    "textOutput.C",
                                    122,
                                    &crate::source_format::format("Cannot calculate TZRMJD", &[])?,
                                );
                                io.drain(d)?;
                                context.pulsars[(p) as usize].param.slots
                                    [(param_tzrmjd as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set = 0i32;
                            }
                        } else {
                            {
                                // fda6ad4 textOutput.C:127-133,1926 activates a report field without prefit.
                                // Snapshot its defined previous value/error before activation (0053 poison witness).
                                if !(context.pulsars[(p) as usize].param.slots
                                    [(param_tzrmjd as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    != 0)
                                {
                                    {
                                        context.pulsars[(p) as usize].param.slots
                                            [(param_tzrmjd as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .prefit = Some(
                                            (context.pulsars[(p) as usize].param.slots
                                                [(param_tzrmjd as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone(),
                                        );
                                        context.pulsars[(p) as usize].param.slots
                                            [(param_tzrmjd as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .prefit_err = Some(
                                            (context.pulsars[(p) as usize].param.slots
                                                [(param_tzrmjd as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone(),
                                        );
                                    }
                                }
                                context.pulsars[(p) as usize].param.slots
                                    [(param_tzrmjd as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val = ((read(
                                    &output_rows(context, (p) as usize)?[(closestI) as usize].sat,
                                    "psr[p].obsn[closestI].sat",
                                )?)
                                .clone()
                                    - ((read(
                                        &output_rows(context, (p) as usize)?[(closestI) as usize]
                                            .residual,
                                        "psr[p].obsn[closestI].residual",
                                    )?)
                                    .clone()
                                        / Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("86400.0").unwrap(),
                                        )));
                                context.pulsars[(p) as usize].param.slots
                                    [(param_tzrmjd as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set = 1i32;
                                // fda6ad4 textOutput.C:127-133,1926 activates a report field without prefit.
                                // Snapshot its defined previous value/error before activation (0053 poison witness).
                                if !(context.pulsars[(p) as usize].param.slots
                                    [(param_tzrfrq as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    != 0)
                                {
                                    {
                                        context.pulsars[(p) as usize].param.slots
                                            [(param_tzrfrq as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .prefit = Some(
                                            (context.pulsars[(p) as usize].param.slots
                                                [(param_tzrfrq as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone(),
                                        );
                                        context.pulsars[(p) as usize].param.slots
                                            [(param_tzrfrq as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .prefit_err = Some(
                                            (context.pulsars[(p) as usize].param.slots
                                                [(param_tzrfrq as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone(),
                                        );
                                    }
                                }
                                context.pulsars[(p) as usize].param.slots
                                    [(param_tzrfrq as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val = Float::with_val(
                                    PRECISION_BITS,
                                    *read(
                                        &output_rows(context, (p) as usize)?[(closestI) as usize]
                                            .freq,
                                        "psr[p].obsn[closestI].freq",
                                    )?,
                                );
                                context.pulsars[(p) as usize].param.slots
                                    [(param_tzrfrq as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set = 1i32;
                                {
                                    let bytes = read_string(
                                        &output_rows(context, (p) as usize)?[(closestI) as usize]
                                            .telID[..],
                                        "psr[p].obsn[closestI].telID",
                                    )?;
                                    crate::observation::write_string(
                                        &mut context.pulsars[(p) as usize].tzrsite[..],
                                        &bytes,
                                        "output_string",
                                    )?;
                                }
                            }
                        }
                    }
                }
                {
                    let bytes = crate::source_format::format("\n\n", &[])?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "Results for PSR %s\n",
                        &[Argument::from(read_string(
                            &context.pulsars[(p) as usize].name[..],
                            "psr[p].name",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format("\n\n", &[])?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "RMS pre-fit residual = %.3f (us), RMS post-fit residual = %.3f (us)\n",
                        &[
                            Argument::from(*read(
                                &context.pulsars[(p) as usize].rmsPre,
                                "psr[p].rmsPre",
                            )?),
                            Argument::from(*read(
                                &context.pulsars[(p) as usize].rmsPost,
                                "psr[p].rmsPost",
                            )?),
                        ],
                    )?;
                    io.write(&bytes)?;
                }
                if (*read(&context.pulsars[(p) as usize].fitMode, "psr[p].fitMode")? == 1i32) {
                    {
                        {
                            let bytes = crate::source_format::format(
                                "Fit Chisq = %.4g\t",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].fitChisq,
                                    "psr[p].fitChisq",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        chisqr = *read(&context.pulsars[(p) as usize].fitChisq, "psr[p].fitChisq")?;
                        {
                            let bytes = crate::source_format::format(
                                "Chisqr/nfree = %.2f/%d = %g\t",
                                &[
                                    Argument::from(chisqr),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].fitNfree,
                                        "psr[p].fitNfree",
                                    )?),
                                    Argument::from(
                                        (chisqr
                                            / (*read(
                                                &context.pulsars[(p) as usize].fitNfree,
                                                "psr[p].fitNfree",
                                            )?)
                                                as f64),
                                    ),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "pre/post = %g\n",
                                &[Argument::from(
                                    (*read(
                                        &context.pulsars[(p) as usize].rmsPre,
                                        "psr[p].rmsPre",
                                    )? / *read(
                                        &context.pulsars[(p) as usize].rmsPost,
                                        "psr[p].rmsPost",
                                    )?),
                                )],
                            )?;
                            io.write(&bytes)?;
                        }
                        if ((*read(&context.pulsars[(p) as usize].robust, "psr[p].robust")?) as i32
                            > 0i32)
                        {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        " >>> Robust fitting enabled. Robust Mode = %c\n",
                                        &[Argument::from(
                                            (*read(
                                                &context.pulsars[(p) as usize].robust,
                                                "psr[p].robust",
                                            )?) as i32,
                                        )],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                    }
                }
                {
                    let bytes = crate::source_format::format(
                        "Number of fit parameters: %d\n",
                        &[Argument::from(*read(
                            &context.pulsars[(p) as usize].nParam,
                            "psr[p].nParam",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                if (*read(&context.pulsars[(0i32) as usize].nGlobal, "psr[0].nGlobal")? > 0i32) {
                    {
                        {
                            let bytes = crate::source_format::format("\nGlobal fit:\n\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Number of fit parameters in total, nt: %d\n",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].globalNfit,
                                    "psr[p].globalNfit",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (*read(
                    &context.pulsars[(p) as usize].nconstraints,
                    "psr[p].nconstraints",
                )? > 0i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format(
                                "Number of points in fit (including constraint points) = %d\n",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].nFit,
                                    "psr[p].nFit",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Number of constraints in fit = %d\n",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].nconstraints,
                                    "psr[p].nconstraints",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Number of observations in fit = %d\n",
                                &[Argument::from(
                                    (*read(&context.pulsars[(p) as usize].nFit, "psr[p].nFit")?
                                        - *read(
                                            &context.pulsars[(p) as usize].nconstraints,
                                            "psr[p].nconstraints",
                                        )?),
                                )],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Constraint EFACTOR = %g\n",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].constraint_efactor,
                                    "psr[p].constraint_efactor",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                } else {
                    {
                        let bytes = crate::source_format::format(
                            "Number of points in fit = %d\n",
                            &[Argument::from(*read(
                                &context.pulsars[(p) as usize].nFit,
                                "psr[p].nFit",
                            )?)],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                {
                    let bytes = crate::source_format::format(
                        "Offset: %g %g offset_e*sqrt(n) = %g n = %d\n",
                        &[
                            Argument::from(*read(
                                &context.pulsars[(p) as usize].offset,
                                "psr[p].offset",
                            )?),
                            Argument::from(*read(
                                &context.pulsars[(p) as usize].offset_e,
                                "psr[p].offset_e",
                            )?),
                            Argument::from(
                                (*read(
                                    &context.pulsars[(p) as usize].offset_e,
                                    "psr[p].offset_e",
                                )? * ((*read(&context.pulsars[(p) as usize].nFit, "psr[p].nFit")?
                                    - *read(
                                        &context.pulsars[(p) as usize].nconstraints,
                                        "psr[p].nconstraints",
                                    )?) as f64)
                                    .sqrt()),
                            ),
                            Argument::from(
                                (*read(&context.pulsars[(p) as usize].nFit, "psr[p].nFit")?
                                    - *read(
                                        &context.pulsars[(p) as usize].nconstraints,
                                        "psr[p].nconstraints",
                                    )?),
                            ),
                        ],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format("\n\n", &[])?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "PARAMETER       Pre-fit                   Post-fit                  Uncertainty   Difference   Fit\n",
                        &[],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "---------------------------------------------------------------------------------------------------\n",
                        &[],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    i = 0i32;
                    while (i < crate::tempo2::MAX_PARAMS as i32) {
                        {
                            {
                                k = 0i32;
                                while (k < *read(
                                    &context.pulsars[(p) as usize].param.slots[(i) as usize].a_size,
                                    "psr[p].param[i].aSize",
                                )?) {
                                    {
                                        if (context.pulsars[(p) as usize].param.slots[(i) as usize]
                                            .cells[(k) as usize]
                                            .param_set
                                            == 1i32)
                                        {
                                            {
                                                if ((((i == (param_chromxr1 as i32))
                                                    || (i == (param_chromxr2 as i32)))
                                                    || (i == (param_dmxr1 as i32)))
                                                    || (i == (param_dmxr2 as i32)))
                                                {
                                                    k = (k + 1);
                                                    continue;
                                                }
                                                if ((i == (param_raj as i32))
                                                    && (*read(
                                                        &context.pulsars[(p) as usize].eclCoord,
                                                        "psr[p].eclCoord",
                                                    )? == 1i32))
                                                {
                                                    {
                                                        let bytes = crate::source_format::format(
                                                            "%-15.15s ",
                                                            &[Argument::from("ELONG")],
                                                        )?;
                                                        io.write(&bytes)?;
                                                    }
                                                } else {
                                                    if ((i == (param_decj as i32))
                                                        && (*read(
                                                            &context.pulsars[(p) as usize].eclCoord,
                                                            "psr[p].eclCoord",
                                                        )? == 1i32))
                                                    {
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "%-15.15s ",
                                                                    &[Argument::from("ELAT")],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                    } else {
                                                        if ((i == (param_pmra as i32))
                                                            && (*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .eclCoord,
                                                                "psr[p].eclCoord",
                                                            )? == 1i32))
                                                        {
                                                            {
                                                                let bytes =
                                                                    crate::source_format::format(
                                                                        "%-15.15s ",
                                                                        &[Argument::from(
                                                                            "PMELONG",
                                                                        )],
                                                                    )?;
                                                                io.write(&bytes)?;
                                                            }
                                                        } else {
                                                            if ((i == (param_pmdec as i32))
                                                                && (*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .eclCoord,
                                                                    "psr[p].eclCoord",
                                                                )? == 1i32))
                                                            {
                                                                {
                                                                    let bytes = crate::source_format::format("%-15.15s ", &[Argument::from("PMELAT")])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            } else {
                                                                if ((i == (param_pmra2 as i32))
                                                                    && (*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .eclCoord,
                                                                        "psr[p].eclCoord",
                                                                    )? == 1i32))
                                                                {
                                                                    {
                                                                        let bytes = crate::source_format::format("%-15.15s ", &[Argument::from("PMELONG2")])?;
                                                                        io.write(&bytes)?;
                                                                    }
                                                                } else {
                                                                    if ((i
                                                                        == (param_pmdec2 as i32))
                                                                        && (*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .eclCoord,
                                                                            "psr[p].eclCoord",
                                                                        )? == 1i32))
                                                                    {
                                                                        {
                                                                            let bytes = crate::source_format::format("%-15.15s ", &[Argument::from("PMELAT2")])?;
                                                                            io.write(&bytes)?;
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let bytes = crate::source_format::format("%-15.15s ", &[Argument::from(crate::clock_text::c_bytes(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].label).to_vec())])?;
                                                                            io.write(&bytes)?;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if (((i == (param_raj as i32))
                                                    || (i == (param_decj as i32)))
                                                    && (*read(
                                                        &context.pulsars[(p) as usize].eclCoord,
                                                        "psr[p].eclCoord",
                                                    )? == 1i32))
                                                {
                                                    {
                                                        let bytes = crate::source_format::format(
                                                            "%-25.15g ",
                                                            &[Argument::from(
                                                                ((((read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .param
                                                                        .slots
                                                                        [(i) as usize]
                                                                        .cells
                                                                        [(k) as usize]
                                                                        .prefit,
                                                                    "psr[p].param[i].prefit[k]",
                                                                )?)
                                                                .clone())
                                                                .to_f64()
                                                                    * 180.0f64)
                                                                    / std::f64::consts::PI),
                                                            )],
                                                        )?;
                                                        io.write(&bytes)?;
                                                    }
                                                } else {
                                                    if ((i == (param_sini as i32))
                                                        && (*read(
                                                            &context.pulsars[(p) as usize]
                                                                .param
                                                                .slots
                                                                [(param_sini as i32) as usize]
                                                                .n_link_to,
                                                            "psr[p].param[param_sini].nLinkTo",
                                                        )? != 0i32))
                                                    {
                                                        {
                                                            let bytes = crate::source_format::format("%-25.15g ", &[Argument::from((((((read(&context.pulsars[(p) as usize].param.slots[((param_kin as i32)) as usize].cells[(0i32) as usize].prefit, "psr[p].param[param_kin].prefit[0]")?).clone() / Float::with_val(PRECISION_BITS, 180.0f64)) * Float::with_val(PRECISION_BITS, std::f64::consts::PI))).sin()).to_f64())])?;
                                                            io.write(&bytes)?;
                                                        }
                                                    } else {
                                                        if (i == (param_ephver as i32)) {
                                                            {
                                                                if ((read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .param
                                                                        .slots
                                                                        [(i) as usize]
                                                                        .cells
                                                                        [(k) as usize]
                                                                        .prefit,
                                                                    "psr[p].param[i].prefit[k]",
                                                                )?)
                                                                .clone()
                                                                    == Float::with_val(
                                                                        PRECISION_BITS,
                                                                        2i32,
                                                                    ))
                                                                {
                                                                    {
                                                                        let bytes = crate::source_format::format("%-25.25s ", &[Argument::from("TEMPO1")])?;
                                                                        io.write(&bytes)?;
                                                                    }
                                                                } else {
                                                                    if ((read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .param
                                                                            .slots
                                                                            [(i) as usize]
                                                                            .cells
                                                                            [(k) as usize]
                                                                            .prefit,
                                                                        "psr[p].param[i].prefit[k]",
                                                                    )?)
                                                                    .clone()
                                                                        == Float::with_val(
                                                                            PRECISION_BITS,
                                                                            5i32,
                                                                        ))
                                                                    {
                                                                        {
                                                                            let bytes = crate::source_format::format("%-25.25s ", &[Argument::from("TEMPO2")])?;
                                                                            io.write(&bytes)?;
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let bytes = crate::source_format::format("%-25.25g ", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64())])?;
                                                                            io.write(&bytes)?;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let bytes = crate::source_format::format("%-25.15g ", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64())])?;
                                                                io.write(&bytes)?;
                                                            }
                                                        }
                                                    }
                                                }
                                                if (i == (param_pmra as i32)) {
                                                    {
                                                        let bytes = crate::source_format::format(
                                                            "%-25.15g ",
                                                            &[Argument::from(
                                                                ((context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(i) as usize]
                                                                    .cells
                                                                    [(k) as usize]
                                                                    .val)
                                                                    .clone())
                                                                .to_f64(),
                                                            )],
                                                        )?;
                                                        io.write(&bytes)?;
                                                    }
                                                } else {
                                                    if (i == (param_pmdec as i32)) {
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "%-25.15g ",
                                                                    &[Argument::from(
                                                                        ((context.pulsars
                                                                            [(p) as usize]
                                                                            .param
                                                                            .slots
                                                                            [(i) as usize]
                                                                            .cells
                                                                            [(k) as usize]
                                                                            .val)
                                                                            .clone())
                                                                        .to_f64(),
                                                                    )],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                    } else {
                                                        if (i == (param_pmra2 as i32)) {
                                                            {
                                                                let bytes =
                                                                    crate::source_format::format(
                                                                        "%-25.15g ",
                                                                        &[Argument::from(
                                                                            ((context.pulsars
                                                                                [(p) as usize]
                                                                                .param
                                                                                .slots
                                                                                [(i) as usize]
                                                                                .cells
                                                                                [(k) as usize]
                                                                                .val)
                                                                                .clone())
                                                                            .to_f64(),
                                                                        )],
                                                                    )?;
                                                                io.write(&bytes)?;
                                                            }
                                                        } else {
                                                            if (i == (param_pmdec2 as i32)) {
                                                                {
                                                                    let bytes = crate::source_format::format("%-25.15g ", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64())])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            } else {
                                                                if (((i == (param_raj as i32))
                                                                    || (i == (param_decj as i32)))
                                                                    && (*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .eclCoord,
                                                                        "psr[p].eclCoord",
                                                                    )? == 1i32))
                                                                {
                                                                    {
                                                                        let bytes = crate::source_format::format("%-25.15g ", &[Argument::from(((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64() * 180.0f64) / std::f64::consts::PI))])?;
                                                                        io.write(&bytes)?;
                                                                    }
                                                                } else {
                                                                    if (i == (param_ephver as i32))
                                                                    {
                                                                        {
                                                                            if ((context.pulsars
                                                                                [(p) as usize]
                                                                                .param
                                                                                .slots
                                                                                [(i) as usize]
                                                                                .cells
                                                                                [(k) as usize]
                                                                                .val)
                                                                                .clone()
                                                                                == Float::with_val(
                                                                                    PRECISION_BITS,
                                                                                    2i32,
                                                                                ))
                                                                            {
                                                                                {
                                                                                    let bytes = crate::source_format::format("%-25.25s ", &[Argument::from("TEMPO1")])?;
                                                                                    io.write(
                                                                                        &bytes,
                                                                                    )?;
                                                                                }
                                                                            } else {
                                                                                if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone() == Float::with_val(PRECISION_BITS, 5i32)) {
{ let bytes = crate::source_format::format("%-25.25s ", &[Argument::from("TEMPO2")])?; io.write(&bytes)?; }
} else {
{ let bytes = crate::source_format::format("%-25.25g ", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64())])?; io.write(&bytes)?; }
}
                                                                            }
                                                                            {
                                                                                let bytes = crate::source_format::format("\n", &[])?;
                                                                                io.write(&bytes)?;
                                                                            }
                                                                            k = (k + 1);
                                                                            continue;
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let bytes = crate::source_format::format("%-25.15g ", &[Argument::from((crate::tempo2_util::get_parameter_value_inner(&context.pulsars[(p) as usize], (i) as usize, (k) as usize, d)?).to_f64())])?;
                                                                            io.write(&bytes)?;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if (((i == (param_raj as i32))
                                                    || (i == (param_decj as i32)))
                                                    && (*read(
                                                        &context.pulsars[(p) as usize].eclCoord,
                                                        "psr[p].eclCoord",
                                                    )? == 1i32))
                                                {
                                                    {
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "%-13.5g ",
                                                                    &[Argument::from(
                                                                        ((((context.pulsars
                                                                            [(p) as usize]
                                                                            .param
                                                                            .slots
                                                                            [(i) as usize]
                                                                            .cells
                                                                            [(k) as usize]
                                                                            .err)
                                                                            .clone())
                                                                        .to_f64()
                                                                            * 180.0f64)
                                                                            / std::f64::consts::PI),
                                                                    )],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                        {
                                                            let bytes = crate::source_format::format("%-13.5g ", &[Argument::from(((180.0f64 / std::f64::consts::PI) * (((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64() - ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64())))])?;
                                                            io.write(&bytes)?;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "%-13.5g ",
                                                                    &[Argument::from(
                                                                        ((context.pulsars
                                                                            [(p) as usize]
                                                                            .param
                                                                            .slots
                                                                            [(i) as usize]
                                                                            .cells
                                                                            [(k) as usize]
                                                                            .err)
                                                                            .clone())
                                                                        .to_f64(),
                                                                    )],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                        {
                                                            let bytes = crate::source_format::format("%-13.5g ", &[Argument::from(((crate::tempo2_util::get_parameter_value_inner(&context.pulsars[(p) as usize], (i) as usize, (k) as usize, d)?).to_f64() - ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64()))])?;
                                                            io.write(&bytes)?;
                                                        }
                                                    }
                                                }
                                                if ((i == (param_start as i32))
                                                    || (i == (param_finish as i32)))
                                                {
                                                    {
                                                        if (context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(i) as usize]
                                                            .cells
                                                            [(k) as usize]
                                                            .fit_flag
                                                            == 1i32)
                                                        {
                                                            {
                                                                let bytes =
                                                                    crate::source_format::format(
                                                                        "S",
                                                                        &[],
                                                                    )?;
                                                                io.write(&bytes)?;
                                                            }
                                                        } else {
                                                            if (context.pulsars[(p) as usize]
                                                                .param
                                                                .slots
                                                                [(i) as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .fit_flag
                                                                == 2i32)
                                                            {
                                                                {
                                                                    let bytes = crate::source_format::format("B", &[])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            } else {
                                                                {
                                                                    let bytes = crate::source_format::format("N", &[])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        if (context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(i) as usize]
                                                            .cells
                                                            [(k) as usize]
                                                            .fit_flag
                                                            == 1i32)
                                                        {
                                                            {
                                                                let bytes =
                                                                    crate::source_format::format(
                                                                        "Y",
                                                                        &[],
                                                                    )?;
                                                                io.write(&bytes)?;
                                                            }
                                                        } else {
                                                            if (context.pulsars[(p) as usize]
                                                                .param
                                                                .slots
                                                                [(i) as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .fit_flag
                                                                == 2i32)
                                                            {
                                                                {
                                                                    let bytes = crate::source_format::format("G", &[])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            } else {
                                                                {
                                                                    let bytes = crate::source_format::format("N", &[])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                {
                                                    let bytes =
                                                        crate::source_format::format("\n", &[])?;
                                                    io.write(&bytes)?;
                                                }
                                                if (i == (param_tzrfrq as i32)) {
                                                    {
                                                        if (i32::from(
                                                            read_string(
                                                                &context.pulsars[(p) as usize]
                                                                    .tzrsite[..],
                                                                "psr[p].tzrsite",
                                                            )? != "NULL".as_bytes(),
                                                        ) != 0i32)
                                                        {
                                                            {
                                                                let bytes = crate::source_format::format("%-15.15s %-25.25s\n", &[Argument::from("TZRSITE"), Argument::from(read_string(&context.pulsars[(p) as usize].tzrsite[..], "psr[p].tzrsite")?)])?;
                                                                io.write(&bytes)?;
                                                            }
                                                        }
                                                    }
                                                }
                                                if ((i == (param_raj as i32))
                                                    && (*read(
                                                        &context.pulsars[(p) as usize].eclCoord,
                                                        "psr[p].eclCoord",
                                                    )? == 0i32))
                                                {
                                                    {
                                                        {
                                                            let bytes = crate::source_format::format("%-15.15s %-25.25s %-25.25s %-13.5g %-13.5g\n", &[Argument::from("RAJ (hms)"), Argument::from(read_string(&context.pulsars[(p) as usize].rajStrPre[..], "psr[p].rajStrPre")?), Argument::from(read_string(&context.pulsars[(p) as usize].rajStrPost[..], "psr[p].rajStrPost")?), Argument::from(((((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone()).to_f64() * 12.0f64) * 60.0f64) * 60.0f64) / std::f64::consts::PI)), Argument::from((((((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64() - ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64()) * 12.0f64) * 60.0f64) * 60.0f64) / std::f64::consts::PI))])?;
                                                            io.write(&bytes)?;
                                                        }
                                                    }
                                                } else {
                                                    if ((i == (param_decj as i32))
                                                        && (*read(
                                                            &context.pulsars[(p) as usize].eclCoord,
                                                            "psr[p].eclCoord",
                                                        )? == 0i32))
                                                    {
                                                        {
                                                            {
                                                                let bytes = crate::source_format::format("%-15.15s %-25.25s %-25.25s %-13.5g %-13.5g\n", &[Argument::from("DECJ (dms)"), Argument::from(read_string(&context.pulsars[(p) as usize].decjStrPre[..], "psr[p].decjStrPre")?), Argument::from(read_string(&context.pulsars[(p) as usize].decjStrPost[..], "psr[p].decjStrPost")?), Argument::from(((((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone()).to_f64() * 180.0f64) * 60.0f64) * 60.0f64) / std::f64::consts::PI)), Argument::from((((((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone()).to_f64() - ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit, "psr[p].param[i].prefit[k]")?).clone()).to_f64()) * 180.0f64) * 60.0f64) * 60.0f64) / std::f64::consts::PI))])?;
                                                                io.write(&bytes)?;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    k = (k + 1);
                                }
                            }
                        }
                        i = (i + 1);
                    }
                }
                if (*read(
                    &context.pulsars[(p) as usize].param.slots[(param_dm as i32) as usize].a_size,
                    "psr[p].param[param_dm].aSize",
                )? > 2i32)
                {
                    {
                        if (*read(
                            &context.pulsars[(p) as usize].dm_series_type,
                            "psr[p].dm_series_type",
                        )?) as i32
                            == 1
                        {
                            {
                                let bytes = crate::source_format::format(
                                    "%-15.15s %-25.25s\n",
                                    &[Argument::from("DM_SERIES"), Argument::from("POLY")],
                                )?;
                                io.write(&bytes)?;
                            }
                        } else {
                            {
                                let bytes = crate::source_format::format(
                                    "%-15.15s %-25.25s\n",
                                    &[Argument::from("DM_SERIES"), Argument::from("TAYLOR")],
                                )?;
                                io.write(&bytes)?;
                            }
                        }
                    }
                }
                {
                    let bytes = crate::source_format::format(
                        "---------------------------------------------------------------------------------------------------\n",
                        &[],
                    )?;
                    io.write(&bytes)?;
                }
                if ((*read(
                    &context.pulsars[(p) as usize].rescaleErrChisq,
                    "psr[p].rescaleErrChisq",
                )? == 1i32)
                    && (*read(&context.pulsars[(p) as usize].fitMode, "psr[p].fitMode")? == 1i32))
                {
                    d.log(
                        b'm',
                        "textOutput.C",
                        306,
                        &crate::source_format::format(
                            "Notice: Parameter uncertainties multiplied by sqrt(red. chisq)\n",
                            &[],
                        )?,
                    );
                    io.drain(d)?;
                }
                if ((*read(
                    &context.pulsars[(p) as usize].rescaleErrChisq,
                    "psr[p].rescaleErrChisq",
                )? == 0i32)
                    && (*read(&context.pulsars[(p) as usize].fitMode, "psr[p].fitMode")? == 1i32))
                {
                    d.log(
                        b'm',
                        "textOutput.C",
                        309,
                        &crate::source_format::format(
                            "Notice: Parameter uncertainties NOT multiplied by sqrt(red. chisq)\n",
                            &[],
                        )?,
                    );
                    io.drain(d)?;
                }
                if (*read(
                    &context.pulsars[(p) as usize].nconstraints,
                    "psr[p].nconstraints",
                )? > 0i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format("\nCONSTRAINTS:\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].nconstraints,
                                "psr[p].nconstraints",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "%s\n",
                                            &[Argument::from(
                                                crate::constraints::name(
                                                    crate::pulsar::Constraint::from_index(
                                                        ((*read(
                                                            &context.pulsars[(p) as usize]
                                                                .constraints
                                                                [(i) as usize],
                                                            "psr[p].constraints[i]",
                                                        )?)
                                                            as i32)
                                                            as i32,
                                                    )
                                                    .ok_or_else(|| {
                                                        boundary("output_constraint_index")
                                                    })?,
                                                )
                                                .as_bytes()
                                                .to_vec(),
                                            )],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                    }
                }
                {
                    i = 1i32;
                    while (i <= *read(&context.pulsars[(p) as usize].nJumps, "psr[p].nJumps")?) {
                        {
                            {
                                if ((*read(
                                    &context.pulsars[(p) as usize].jumpSAT[(i) as usize],
                                    "psr[p].jumpSAT[i]",
                                )?) as i32
                                    == 1i32)
                                {
                                    {
                                        {
                                            let bytes = crate::source_format::format(
                                                "SAT Jump %d (%s): %.14g %.14g ",
                                                &[
                                                    Argument::from(i),
                                                    Argument::from(read_string(
                                                        &context.pulsars[(p) as usize].jumpStr
                                                            [(i) as usize][..],
                                                        "psr[p].jumpStr[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].jumpVal
                                                            [(i) as usize],
                                                        "psr[p].jumpVal[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].jumpValErr
                                                            [(i) as usize],
                                                        "psr[p].jumpValErr[i]",
                                                    )?),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                } else {
                                    {
                                        {
                                            let bytes = crate::source_format::format(
                                                "Jump %d (%s): %.14g %.14g ",
                                                &[
                                                    Argument::from(i),
                                                    Argument::from(read_string(
                                                        &context.pulsars[(p) as usize].jumpStr
                                                            [(i) as usize][..],
                                                        "psr[p].jumpStr[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].jumpVal
                                                            [(i) as usize],
                                                        "psr[p].jumpVal[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].jumpValErr
                                                            [(i) as usize],
                                                        "psr[p].jumpValErr[i]",
                                                    )?),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].fitJump[(i) as usize],
                                    "psr[p].fitJump[i]",
                                )? == 1i32)
                                {
                                    {
                                        let bytes = crate::source_format::format("Y\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                } else {
                                    {
                                        let bytes = crate::source_format::format("N\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                }
                            }
                        }
                        i = (i + 1);
                    }
                }
                {
                    i = 1i32;
                    while (i <= *read(&context.pulsars[(p) as usize].nfdJumps, "psr[p].nfdJumps")?)
                    {
                        {
                            {
                                if (*read(
                                    &context.pulsars[(p) as usize].fdjumpIdx[(i) as usize],
                                    "psr[p].fdjumpIdx[i]",
                                )? == (-2i32))
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "FDJUMPDM (%s): %.14g %.14g ",
                                            &[
                                                Argument::from(read_string(
                                                    &context.pulsars[(p) as usize].fdjumpStr
                                                        [(i) as usize][..],
                                                    "psr[p].fdjumpStr[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].fdjumpVal
                                                        [(i) as usize],
                                                    "psr[p].fdjumpVal[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].fdjumpValErr
                                                        [(i) as usize],
                                                    "psr[p].fdjumpValErr[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                } else {
                                    {
                                        let bytes = crate::source_format::format(
                                            "FDJUMP%d (%s): %.14g %.14g ",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].fdjumpIdx
                                                        [(i) as usize],
                                                    "psr[p].fdjumpIdx[i]",
                                                )?),
                                                Argument::from(read_string(
                                                    &context.pulsars[(p) as usize].fdjumpStr
                                                        [(i) as usize][..],
                                                    "psr[p].fdjumpStr[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].fdjumpVal
                                                        [(i) as usize],
                                                    "psr[p].fdjumpVal[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].fdjumpValErr
                                                        [(i) as usize],
                                                    "psr[p].fdjumpValErr[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].fitfdJump[(i) as usize],
                                    "psr[p].fitfdJump[i]",
                                )? == 1i32)
                                {
                                    {
                                        let bytes = crate::source_format::format("Y\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                } else {
                                    {
                                        let bytes = crate::source_format::format("N\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                }
                            }
                        }
                        i = (i + 1);
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_wave_om as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        if (*read(&context.pulsars[(p) as usize].waveScale, "psr[p].waveScale")?
                            == (2i32) as f64)
                        {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                        &[
                                            Argument::from("Freq"),
                                            Argument::from("Period"),
                                            Argument::from("Cosine amp"),
                                            Argument::from("Sine amp"),
                                            Argument::from("Power"),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                        &[
                                            Argument::from("(yr^-1)"),
                                            Argument::from("(yr)"),
                                            Argument::from("(s)"),
                                            Argument::from("(s)"),
                                            Argument::from("(s^2)"),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------------------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nWhite,
                                        "psr[p].nWhite",
                                    )?) {
                                        {
                                            {
                                                let bytes = crate::source_format::format(
                                                    "WAVE%d\t%-15.5Lg %-15.5Lg %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g  %-+10.5g %-+10.5g  \n",
                                                    &[
                                                        Argument::from((i + 1i32)),
                                                        Argument::from(
                                                            ((((Float::with_val(
                                                                PRECISION_BITS,
                                                                (i + 1i32),
                                                            ) * (context.pulsars
                                                                [(p) as usize]
                                                                .param
                                                                .slots
                                                                [(param_wave_om as i32)
                                                                    as usize]
                                                                .cells
                                                                [(0i32) as usize]
                                                                .val)
                                                                .clone())
                                                                / Float::with_val(
                                                                    PRECISION_BITS,
                                                                    2.0f64,
                                                                ))
                                                                / Float::with_val(
                                                                    PRECISION_BITS,
                                                                    std::f64::consts::PI,
                                                                ))
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    365.25f64,
                                                                )),
                                                        ),
                                                        Argument::from(
                                                            (Float::with_val(
                                                                PRECISION_BITS,
                                                                1.0f64,
                                                            ) / ((((Float::with_val(
                                                                PRECISION_BITS,
                                                                (i + 1i32),
                                                            ) * (context.pulsars
                                                                [(p) as usize]
                                                                .param
                                                                .slots
                                                                [(param_wave_om as i32)
                                                                    as usize]
                                                                .cells
                                                                [(0i32) as usize]
                                                                .val)
                                                                .clone())
                                                                / Float::with_val(
                                                                    PRECISION_BITS,
                                                                    2.0f64,
                                                                ))
                                                                / Float::with_val(
                                                                    PRECISION_BITS,
                                                                    std::f64::consts::PI,
                                                                ))
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    365.25f64,
                                                                ))),
                                                        ),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize].wave_cos
                                                                [(i) as usize],
                                                            "psr[p].wave_cos[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_cos_err
                                                                [(i) as usize],
                                                            "psr[p].wave_cos_err[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine
                                                                [(i) as usize],
                                                            "psr[p].wave_sine[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine_err
                                                                [(i) as usize],
                                                            "psr[p].wave_sine_err[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_cos_dm
                                                                [(i) as usize],
                                                            "psr[p].wave_cos_dm[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_cos_dm_err
                                                                [(i) as usize],
                                                            "psr[p].wave_cos_dm_err[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine_dm
                                                                [(i) as usize],
                                                            "psr[p].wave_sine_dm[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine_dm_err
                                                                [(i) as usize],
                                                            "psr[p].wave_sine_dm_err[i]",
                                                        )?),
                                                    ],
                                                )?;
                                                io.write(&bytes)?;
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------------------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                let mut om: f64;
                                let mut perr: f64;
                                let mut pwr: f64;
                                {
                                    let bytes = crate::source_format::format(
                                        "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                        &[
                                            Argument::from("Freq"),
                                            Argument::from("Period"),
                                            Argument::from("Cosine amp"),
                                            Argument::from("Sine amp"),
                                            Argument::from("Power"),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                        &[
                                            Argument::from("(yr^-1)"),
                                            Argument::from("(yr)"),
                                            Argument::from("(s)"),
                                            Argument::from("(s)"),
                                            Argument::from("(s^2)"),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------------------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nWhite,
                                        "psr[p].nWhite",
                                    )?) {
                                        {
                                            pwr = ((*read(
                                                &context.pulsars[(p) as usize].wave_cos
                                                    [(i) as usize],
                                                "psr[p].wave_cos[i]",
                                            )?)
                                            .powf((2i32) as f64)
                                                + (*read(
                                                    &context.pulsars[(p) as usize].wave_sine
                                                        [(i) as usize],
                                                    "psr[p].wave_sine[i]",
                                                )?)
                                                .powf((2i32) as f64));
                                            perr = ((((2i32) as f64
                                                * *read(
                                                    &context.pulsars[(p) as usize].wave_cos
                                                        [(i) as usize],
                                                    "psr[p].wave_cos[i]",
                                                )?)
                                                * *read(
                                                    &context.pulsars[(p) as usize].wave_cos_err
                                                        [(i) as usize],
                                                    "psr[p].wave_cos_err[i]",
                                                )?)
                                            .powf((2i32) as f64)
                                                + (((2i32) as f64
                                                    * *read(
                                                        &context.pulsars[(p) as usize]
                                                            .wave_sine_err
                                                            [(i) as usize],
                                                        "psr[p].wave_sine_err[i]",
                                                    )?)
                                                    * *read(
                                                        &context.pulsars[(p) as usize]
                                                            .wave_sine_err
                                                            [(i) as usize],
                                                        "psr[p].wave_sine_err[i]",
                                                    )?)
                                                .powf((2i32) as f64))
                                            .sqrt();
                                            if (i == 0i32) {
                                                {
                                                    om = ((((Float::with_val(
                                                        PRECISION_BITS,
                                                        ((i) as f64 + 1.0f64),
                                                    ) * (context.pulsars
                                                        [(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_wave_om as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            2.0f64,
                                                        ))
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            365.25f64,
                                                        ))
                                                    .to_f64();
                                                }
                                            } else {
                                                {
                                                    om = ((((Float::with_val(
                                                        PRECISION_BITS,
                                                        ((i) as f64 + 1.0f64),
                                                    ) * (context.pulsars
                                                        [(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_wave_om as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            2.0f64,
                                                        ))
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            365.25f64,
                                                        ))
                                                    .to_f64();
                                                }
                                            }
                                            {
                                                let bytes = crate::source_format::format(
                                                    "WAVE%d\t%-15.5Lg %-15.5Lg %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g\n",
                                                    &[
                                                        Argument::from((i + 1i32)),
                                                        Argument::from(Float::with_val(
                                                            PRECISION_BITS,
                                                            om,
                                                        )),
                                                        Argument::from(
                                                            (Float::with_val(
                                                                PRECISION_BITS,
                                                                1.0f64,
                                                            ) / Float::with_val(
                                                                PRECISION_BITS,
                                                                om,
                                                            )),
                                                        ),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize].wave_cos
                                                                [(i) as usize],
                                                            "psr[p].wave_cos[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_cos_err
                                                                [(i) as usize],
                                                            "psr[p].wave_cos_err[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine
                                                                [(i) as usize],
                                                            "psr[p].wave_sine[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .wave_sine_err
                                                                [(i) as usize],
                                                            "psr[p].wave_sine_err[i]",
                                                        )?),
                                                        Argument::from(pwr),
                                                        Argument::from(perr),
                                                    ],
                                                )?;
                                                io.write(&bytes)?;
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------------------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_wave_dm as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut perr: f64;
                        let mut pwr: f64;
                        {
                            let bytes = crate::source_format::format(
                                "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                &[
                                    Argument::from("Freq"),
                                    Argument::from("Period"),
                                    Argument::from("Cosine amp"),
                                    Argument::from("Sine amp"),
                                    Argument::from("Power"),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "      \t%-15.15s %-15.15s %-15.15s %-15.15s %-15.15s\n",
                                &[
                                    Argument::from("(yr^-1)"),
                                    Argument::from("(yr)"),
                                    Argument::from("(s)"),
                                    Argument::from("(s)"),
                                    Argument::from("(s^2)"),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "------------------------------------------------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].nWhite_dm,
                                "psr[p].nWhite_dm",
                            )?) {
                                {
                                    pwr = ((*read(
                                        &context.pulsars[(p) as usize].wave_cos_dm[(i) as usize],
                                        "psr[p].wave_cos_dm[i]",
                                    )?)
                                    .powf((2i32) as f64)
                                        + (*read(
                                            &context.pulsars[(p) as usize].wave_sine_dm
                                                [(i) as usize],
                                            "psr[p].wave_sine_dm[i]",
                                        )?)
                                        .powf((2i32) as f64));
                                    perr = ((((2i32) as f64
                                        * *read(
                                            &context.pulsars[(p) as usize].wave_cos_dm
                                                [(i) as usize],
                                            "psr[p].wave_cos_dm[i]",
                                        )?)
                                        * *read(
                                            &context.pulsars[(p) as usize].wave_cos_dm_err
                                                [(i) as usize],
                                            "psr[p].wave_cos_dm_err[i]",
                                        )?)
                                    .powf((2i32) as f64)
                                        + (((2i32) as f64
                                            * *read(
                                                &context.pulsars[(p) as usize].wave_sine_dm
                                                    [(i) as usize],
                                                "psr[p].wave_sine_dm[i]",
                                            )?)
                                            * *read(
                                                &context.pulsars[(p) as usize].wave_sine_dm_err
                                                    [(i) as usize],
                                                "psr[p].wave_sine_dm_err[i]",
                                            )?)
                                        .powf((2i32) as f64))
                                    .sqrt();
                                    {
                                        let bytes = crate::source_format::format(
                                            "WAVE%d\t%-15.5Lg %-15.5Lg %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g %-+10.5g\n",
                                            &[
                                                Argument::from((i + 1i32)),
                                                Argument::from(
                                                    ((((Float::with_val(
                                                        PRECISION_BITS,
                                                        (i + 1i32),
                                                    ) * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_wave_dm as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            2.0f64,
                                                        ))
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            365.25f64,
                                                        )),
                                                ),
                                                Argument::from(
                                                    (Float::with_val(PRECISION_BITS, 1.0f64)
                                                        / ((((Float::with_val(
                                                            PRECISION_BITS,
                                                            (i + 1i32),
                                                        ) * (context.pulsars
                                                            [(p) as usize]
                                                            .param
                                                            .slots
                                                            [(param_wave_dm as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone())
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                2.0f64,
                                                            ))
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                std::f64::consts::PI,
                                                            ))
                                                            * Float::with_val(
                                                                PRECISION_BITS,
                                                                365.25f64,
                                                            ))),
                                                ),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].wave_cos_dm
                                                        [(i) as usize],
                                                    "psr[p].wave_cos_dm[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].wave_cos_dm_err
                                                        [(i) as usize],
                                                    "psr[p].wave_cos_dm_err[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].wave_sine_dm
                                                        [(i) as usize],
                                                    "psr[p].wave_sine_dm[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].wave_sine_dm_err
                                                        [(i) as usize],
                                                    "psr[p].wave_sine_dm_err[i]",
                                                )?),
                                                Argument::from(pwr),
                                                Argument::from(perr),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                        {
                            let bytes = crate::source_format::format(
                                "------------------------------------------------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                {
                    let mut basisPolySubtracted: i32 = ((*read(
                        &context.pulsars[(p) as usize].TNsubtractPoly,
                        "psr[p].TNsubtractPoly",
                    )? & 2u64)
                        != (0i32) as u64)
                        as i32;
                    let mut anyNoise: i32 =
                        (((*read(&context.pulsars[(p) as usize].TNRedAmp, "psr[p].TNRedAmp")?
                            != (0i32) as f64)
                            || (*read(&context.pulsars[(p) as usize].TNDMAmp, "psr[p].TNDMAmp")?
                                != (0i32) as f64))
                            || (*read(
                                &context.pulsars[(p) as usize].TNChromAmp,
                                "psr[p].TNChromAmp",
                            )? != (0i32) as f64)) as i32;
                    if (anyNoise != 0) {
                        {
                            {
                                let bytes = crate::source_format::format(
                                    "\nFourier GP Noise Model:\n",
                                    &[],
                                )?;
                                io.write(&bytes)?;
                            }
                            if (*read(&context.pulsars[(p) as usize].TNRedAmp, "psr[p].TNRedAmp")?
                                != (0i32) as f64)
                            {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "  TNRed   : log10A=%g gamma=%g nFreq=%d nLogFreq=%d polySub=%s (ord=%d)\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedAmp,
                                                    "psr[p].TNRedAmp",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedGam,
                                                    "psr[p].TNRedGam",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedC,
                                                    "psr[p].TNRedC",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRed_log_freqs,
                                                    "psr[p].TNRed_log_freqs",
                                                )?),
                                                Argument::from(if (basisPolySubtracted != 0) {
                                                    "Y"
                                                } else {
                                                    "N"
                                                }),
                                                Argument::from(
                                                    crate::t2fit_nestlike::t2DecodeTNRedSubPolyOrd(
                                                        *read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNsubtractPoly,
                                                            "psr[p].TNsubtractPoly",
                                                        )?,
                                                    ),
                                                ),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                    if (*read(
                                        &context.pulsars[(p) as usize].TNRedFLow,
                                        "psr[p].TNRedFLow",
                                    )? != (0i32) as f64)
                                    {
                                        {
                                            {
                                                let bytes = crate::source_format::format(
                                                    "            Tspan scale factor = %lg\n",
                                                    &[Argument::from(
                                                        (10.0f64).powf(
                                                            (-*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .TNRedFLow,
                                                                "psr[p].TNRedFLow",
                                                            )?),
                                                        ),
                                                    )],
                                                )?;
                                                io.write(&bytes)?;
                                            }
                                        }
                                    }
                                }
                            }
                            if (*read(&context.pulsars[(p) as usize].TNDMAmp, "psr[p].TNDMAmp")?
                                != (0i32) as f64)
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "  TNDM    : log10A=%g gamma=%g nFreq=%d nLogFreq=%d polySub=%s (ord=%d)\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNDMAmp,
                                                "psr[p].TNDMAmp",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNDMGam,
                                                "psr[p].TNDMGam",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNDMC,
                                                "psr[p].TNDMC",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNDM_log_freqs,
                                                "psr[p].TNDM_log_freqs",
                                            )?),
                                            Argument::from(if (basisPolySubtracted != 0) {
                                                "Y"
                                            } else {
                                                "N"
                                            }),
                                            Argument::from(
                                                crate::t2fit_nestlike::t2DecodeTNDMSubPolyOrd(
                                                    *read(
                                                        &context.pulsars[(p) as usize]
                                                            .TNsubtractPoly,
                                                        "psr[p].TNsubtractPoly",
                                                    )?,
                                                ),
                                            ),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                            if (*read(
                                &context.pulsars[(p) as usize].TNChromAmp,
                                "psr[p].TNChromAmp",
                            )? != (0i32) as f64)
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "  TNChrom : log10A=%g gamma=%g nFreq=%d nLogFreq=%d polySub=%s (ord=%d)\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNChromAmp,
                                                "psr[p].TNChromAmp",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNChromGam,
                                                "psr[p].TNChromGam",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNChromC,
                                                "psr[p].TNChromC",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].TNChrom_log_freqs,
                                                "psr[p].TNChrom_log_freqs",
                                            )?),
                                            Argument::from(if (basisPolySubtracted != 0) {
                                                "Y"
                                            } else {
                                                "N"
                                            }),
                                            Argument::from(
                                                crate::t2fit_nestlike::t2DecodeTNChromSubPolyOrd(
                                                    *read(
                                                        &context.pulsars[(p) as usize]
                                                            .TNsubtractPoly,
                                                        "psr[p].TNsubtractPoly",
                                                    )?,
                                                ),
                                            ),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_gwsingle as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format("GW single source:\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Omega: %g\n",
                                &[Argument::from(
                                    ((context.pulsars[(p) as usize].param.slots
                                        [(param_gwsingle as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .val)
                                        .clone())
                                    .to_f64(),
                                )],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Aplus = %lg (%lg) %lg (%lg)\n",
                                &[
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_aplus_r,
                                        "psr[p].gwsrc_aplus_r",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_aplus_r_e,
                                        "psr[p].gwsrc_aplus_r_e",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_aplus_i,
                                        "psr[p].gwsrc_aplus_i",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_aplus_i_e,
                                        "psr[p].gwsrc_aplus_i_e",
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Across = %lg (%lg) %lg (%lg)\n",
                                &[
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_across_r,
                                        "psr[p].gwsrc_across_r",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_across_r_e,
                                        "psr[p].gwsrc_across_r_e",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_across_i,
                                        "psr[p].gwsrc_across_i",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].gwsrc_across_i_e,
                                        "psr[p].gwsrc_across_i_e",
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_quad_om as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut j: i32;
                        {
                            j = 0i32;
                            while (j < *read(&context.pulsars[(p) as usize].nQuad, "psr[p].nQuad")?)
                            {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "QUAD%d %g %g %g %g %g %g %g %g %g %g\n",
                                            &[
                                                Argument::from((j + 1i32)),
                                                Argument::from(
                                                    (((context.pulsars[(p) as usize].param.slots
                                                        [(param_quad_om as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    .to_f64()
                                                        * (j + 1i32) as f64),
                                                ),
                                                Argument::from(
                                                    ((((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_quad_om as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            (j + 1i32),
                                                        ))
                                                        / Float::with_val(PRECISION_BITS, 2.0f64))
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                    .to_f64(),
                                                ),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_aplus_r
                                                        [(j) as usize],
                                                    "psr[p].quad_aplus_r[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_aplus_i
                                                        [(j) as usize],
                                                    "psr[p].quad_aplus_i[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_across_r
                                                        [(j) as usize],
                                                    "psr[p].quad_across_r[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_across_i
                                                        [(j) as usize],
                                                    "psr[p].quad_across_i[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_aplus_r_e
                                                        [(j) as usize],
                                                    "psr[p].quad_aplus_r_e[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_aplus_i_e
                                                        [(j) as usize],
                                                    "psr[p].quad_aplus_i_e[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_across_r_e
                                                        [(j) as usize],
                                                    "psr[p].quad_across_r_e[j]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_across_i_e
                                                        [(j) as usize],
                                                    "psr[p].quad_across_i_e[j]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                                j = (j + 1);
                            }
                        }
                    }
                }
                if (((context.pulsars[(p) as usize].param.slots[(param_gwm_amp as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots
                        [(param_gwm_amp as i32) as usize]
                        .cells[(1i32) as usize]
                        .param_set
                        == 1i32))
                    && ((context.pulsars[(p) as usize].param.slots
                        [(param_gwm_amp as i32) as usize]
                        .cells[(0i32) as usize]
                        .fit_flag
                        > 0i32)
                        || (context.pulsars[(p) as usize].param.slots
                            [(param_gwm_amp as i32) as usize]
                            .cells[(1i32) as usize]
                            .fit_flag
                            > 0i32)))
                {
                    {
                        let mut i0: i32;
                        let mut i1: i32;
                        i0 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwm_amp as i32) as usize],
                            0i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        i1 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwm_amp as i32) as usize],
                            1i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "GWM covariances: A1_A1 = %g A2_A2 = %g A1_A2 = %g\n",
                                &[
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i0) as usize,
                                        (i0) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i1) as usize,
                                        (i1) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i0) as usize,
                                        (i1) as usize,
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (((context.pulsars[(p) as usize].param.slots[(param_gwb_amp as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots
                        [(param_gwb_amp as i32) as usize]
                        .cells[(1i32) as usize]
                        .param_set
                        == 1i32))
                    && ((context.pulsars[(p) as usize].param.slots
                        [(param_gwb_amp as i32) as usize]
                        .cells[(0i32) as usize]
                        .fit_flag
                        > 0i32)
                        || (context.pulsars[(p) as usize].param.slots
                            [(param_gwb_amp as i32) as usize]
                            .cells[(1i32) as usize]
                            .fit_flag
                            > 0i32)))
                {
                    {
                        let mut i0: i32;
                        let mut i1: i32;
                        i0 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwb_amp as i32) as usize],
                            0i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        i1 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwb_amp as i32) as usize],
                            1i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "GW BURST A1: %g A2: %g A1_A1 = %g A2_A2 = %g A1_A2 = %g \n",
                                &[
                                    Argument::from(
                                        ((context.pulsars[(0i32) as usize].param.slots
                                            [(param_gwb_amp as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone())
                                        .to_f64(),
                                    ),
                                    Argument::from(
                                        ((context.pulsars[(0i32) as usize].param.slots
                                            [(param_gwb_amp as i32) as usize]
                                            .cells[(1i32) as usize]
                                            .val)
                                            .clone())
                                        .to_f64(),
                                    ),
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i0) as usize,
                                        (i0) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i1) as usize,
                                        (i1) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (0i32) as usize,
                                        (i0) as usize,
                                        (i1) as usize,
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (((context.pulsars[(p) as usize].param.slots[(param_gwcs_amp as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots
                        [(param_gwcs_amp as i32) as usize]
                        .cells[(1i32) as usize]
                        .param_set
                        == 1i32))
                    && ((context.pulsars[(p) as usize].param.slots
                        [(param_gwcs_amp as i32) as usize]
                        .cells[(0i32) as usize]
                        .fit_flag
                        > 0i32)
                        || (context.pulsars[(p) as usize].param.slots
                            [(param_gwcs_amp as i32) as usize]
                            .cells[(1i32) as usize]
                            .fit_flag
                            > 0i32)))
                {
                    {
                        let mut i0: i32;
                        let mut i1: i32;
                        i0 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwcs_amp as i32) as usize],
                            0i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        i1 = crate::general_fitting::get_param_matrix_row(
                            &context.pulsars[p as usize].fitinfo,
                            p,
                            crate::tempo2::PARAMETER_LABELS[(param_gwcs_amp as i32) as usize],
                            1i32,
                        )
                        .map(|i| i as i32)
                        .unwrap_or(-1);
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "GWCS covariances: A1_A1 = %g A2_A2 = %g A1_A2 = %g\n",
                                &[
                                    Argument::from(output_covar(
                                        context,
                                        (p) as usize,
                                        (i0) as usize,
                                        (i0) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (p) as usize,
                                        (i1) as usize,
                                        (i1) as usize,
                                    )?),
                                    Argument::from(output_covar(
                                        context,
                                        (p) as usize,
                                        (i0) as usize,
                                        (i1) as usize,
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_dmmodel as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes =
                                crate::source_format::format("Dispersion measure values:\n", &[])?;
                            io.write(&bytes)?;
                        }
                        let mut dmfname: Vec<u8> = Vec::new();
                        dmfname = crate::source_format::format(
                            "%s.dm",
                            &[Argument::from(read_string(
                                &context.pulsars[(p) as usize].name[..],
                                "psr[p].name",
                            )?)],
                        )?;
                        let mut dmfile = io.create(&dmfname.clone());
                        let mut sum: f64 = (0i32) as f64;
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].dmoffsDMnum,
                                "psr[p].dmoffsDMnum",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "_DM\t % 7.1f % 10.3g % 10.3g % 5.2f\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsDM_mjd
                                                        [(i) as usize],
                                                    "psr[p].dmoffsDM_mjd[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsDM
                                                        [(i) as usize],
                                                    "psr[p].dmoffsDM[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsDM_error
                                                        [(i) as usize],
                                                    "psr[p].dmoffsDM_error[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsDM_weight
                                                        [(i) as usize],
                                                    "psr[p].dmoffsDM_weight[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                    if dmfile.is_some() {
                                        write_file(
                                            &mut dmfile,
                                            &crate::source_format::format(
                                                "% 7.1f % 15.7g % 15.7g % 7.4f\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].dmoffsDM_mjd
                                                            [(i) as usize],
                                                        "psr[p].dmoffsDM_mjd[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].dmoffsDM
                                                            [(i) as usize],
                                                        "psr[p].dmoffsDM[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .dmoffsDM_error
                                                            [(i) as usize],
                                                        "psr[p].dmoffsDM_error[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .dmoffsDM_weight
                                                            [(i) as usize],
                                                        "psr[p].dmoffsDM_weight[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                    sum = (sum
                                        + *read(
                                            &context.pulsars[(p) as usize].dmoffsDM[(i) as usize],
                                            "psr[p].dmoffsDM[i]",
                                        )?);
                                }
                                i = (i + 1);
                            }
                        }
                        if dmfile.is_some() {
                            dmfile.take();
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Mean DMOFF = %lg\n",
                                &[Argument::from(
                                    (sum / (*read(
                                        &context.pulsars[(p) as usize].dmoffsDMnum,
                                        "psr[p].dmoffsDMnum",
                                    )?) as f64),
                                )],
                            )?;
                            io.write(&bytes)?;
                        }
                        dmfname = crate::source_format::format(
                            "%s.cm",
                            &[Argument::from(read_string(
                                &context.pulsars[(p) as usize].name[..],
                                "psr[p].name",
                            )?)],
                        )?;
                        dmfile = io.create(&dmfname.clone());
                        sum = (0i32) as f64;
                        let mut eee: f64;
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].dmoffsCMnum,
                                "psr[p].dmoffsCMnum",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "_CM\t % 7.1f % 10.3g % 10.3g % 5.2f\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsCM_mjd
                                                        [(i) as usize],
                                                    "psr[p].dmoffsCM_mjd[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsCM
                                                        [(i) as usize],
                                                    "psr[p].dmoffsCM[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsCM_error
                                                        [(i) as usize],
                                                    "psr[p].dmoffsCM_error[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].dmoffsCM_weight
                                                        [(i) as usize],
                                                    "psr[p].dmoffsCM_weight[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                    if dmfile.is_some() {
                                        write_file(
                                            &mut dmfile,
                                            &crate::source_format::format(
                                                "% 7.1f % 15.7g % 15.7g % 7.4f\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].dmoffsCM_mjd
                                                            [(i) as usize],
                                                        "psr[p].dmoffsCM_mjd[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].dmoffsCM
                                                            [(i) as usize],
                                                        "psr[p].dmoffsCM[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .dmoffsCM_error
                                                            [(i) as usize],
                                                        "psr[p].dmoffsCM_error[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .dmoffsCM_weight
                                                            [(i) as usize],
                                                        "psr[p].dmoffsCM_weight[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                    eee = ((*read(
                                        &context.pulsars[(p) as usize].dmoffsCM_error[(i) as usize],
                                        "psr[p].dmoffsCM_error[i]",
                                    )? / 86400.0f64)
                                        / 365.25f64);
                                    sum = (sum + (1.0f64 / (eee * eee)));
                                }
                                i = (i + 1);
                            }
                        }
                        let mut tobs: f64 = ((*read(
                            &context.pulsars[(p) as usize].dmoffsCM_mjd[(*read(
                                &context.pulsars[(p) as usize].dmoffsCMnum,
                                "psr[p].dmoffsCMnum",
                            )? - 1i32)
                                as usize],
                            "psr[p].dmoffsCM_mjd[psr[p].dmoffsCMnum - 1]",
                        )? - *read(
                            &context.pulsars[(p) as usize].dmoffsCM_mjd[(0i32) as usize],
                            "psr[p].dmoffsCM_mjd[0]",
                        )?) / 365.25f64);
                        if (sum > (0i32) as f64) {
                            {
                                let bytes = crate::source_format::format(
                                    "CM white PSD estimate: %lg\n",
                                    &[Argument::from((((2i32) as f64 * tobs) / sum))],
                                )?;
                                io.write(&bytes)?;
                            }
                        }
                        if dmfile.is_some() {
                            dmfile.take();
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_ne_sw_ifunc as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format("\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format("NE_SW values:\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].ne_sw_ifuncN,
                                "psr[p].ne_sw_ifuncN",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "_NE_SW \t % 7.1f % 8.6g % 8.6g\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].ne_sw_ifuncT
                                                        [(i) as usize],
                                                    "psr[p].ne_sw_ifuncT[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].ne_sw_ifuncV
                                                        [(i) as usize],
                                                    "psr[p].ne_sw_ifuncV[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].ne_sw_ifuncE
                                                        [(i) as usize],
                                                    "psr[p].ne_sw_ifuncE[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_clk_offs as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut fout = None::<std::fs::File>;
                        {
                            let bytes = crate::source_format::format("Clock offsets\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].clkOffsN,
                                "psr[p].clkOffsN",
                            )?) {
                                {
                                    let bytes = crate::source_format::format(
                                        "%.2f %.10g %.10g\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].clk_offsT
                                                    [(i) as usize],
                                                "psr[p].clk_offsT[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].clk_offsV
                                                    [(i) as usize],
                                                "psr[p].clk_offsV[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].clk_offsE
                                                    [(i) as usize],
                                                "psr[p].clk_offsE[i]",
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                i = (i + 1);
                            }
                        }
                        fout = io.create(&"clockOffset.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Unable to open file clockOffset.dat for writing\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].clkOffsN,
                                        "psr[p].clkOffsN",
                                    )?) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.2f %.10g %.10g\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].clk_offsT
                                                            [(i) as usize],
                                                        "psr[p].clk_offsT[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].clk_offsV
                                                            [(i) as usize],
                                                        "psr[p].clk_offsV[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].clk_offsE
                                                            [(i) as usize],
                                                        "psr[p].clk_offsE[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                        i = (i + 1);
                                    }
                                }
                                fout.take();
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_ifunc as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes =
                                crate::source_format::format("Interpolated function\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].ifuncN,
                                "psr[p].ifuncN",
                            )?) {
                                {
                                    let bytes = crate::source_format::format(
                                        "%.2f %.10g %.10g %.3f\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].ifuncT[(i) as usize],
                                                "psr[p].ifuncT[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].ifuncV[(i) as usize],
                                                "psr[p].ifuncV[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].ifuncE[(i) as usize],
                                                "psr[p].ifuncE[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].ifunc_weights
                                                    [(i) as usize],
                                                "psr[p].ifunc_weights[i]",
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                i = (i + 1);
                            }
                        }
                        fout = io.create(&"ifunc.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Unable to open file ifunc.dat for writing\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].ifuncN,
                                        "psr[p].ifuncN",
                                    )?) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.2f %.10g %.10g %.3f\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].ifuncT
                                                            [(i) as usize],
                                                        "psr[p].ifuncT[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].ifuncV
                                                            [(i) as usize],
                                                        "psr[p].ifuncV[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].ifuncE
                                                            [(i) as usize],
                                                        "psr[p].ifuncE[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .ifunc_weights
                                                            [(i) as usize],
                                                        "psr[p].ifunc_weights[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                        i = (i + 1);
                                    }
                                }
                                fout.take();
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_tel_dx as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut fout = None::<std::fs::File>;
                        {
                            let bytes =
                                crate::source_format::format("Telescope x function\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].nTelDX,
                                "psr[p].nTelDX",
                            )?) {
                                {
                                    let bytes = crate::source_format::format(
                                        "%.2f %.10g %.10g\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDX_t
                                                    [(i) as usize],
                                                "psr[p].telDX_t[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDX_v
                                                    [(i) as usize],
                                                "psr[p].telDX_v[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDX_e
                                                    [(i) as usize],
                                                "psr[p].telDX_e[i]",
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                i = (i + 1);
                            }
                        }
                        fout = io.create(&"telescopeXYZ.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Unable to open file telescopeXYZ.dat for writing\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTelDX,
                                        "psr[p].nTelDX",
                                    )?) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.2f %.10g %.10g %.10g %.10g %.10g %.10g\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDX_t
                                                            [(i) as usize],
                                                        "psr[p].telDX_t[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDX_v
                                                            [(i) as usize],
                                                        "psr[p].telDX_v[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDX_e
                                                            [(i) as usize],
                                                        "psr[p].telDX_e[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDY_v
                                                            [(i) as usize],
                                                        "psr[p].telDY_v[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDY_e
                                                            [(i) as usize],
                                                        "psr[p].telDY_e[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDZ_v
                                                            [(i) as usize],
                                                        "psr[p].telDZ_v[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].telDZ_e
                                                            [(i) as usize],
                                                        "psr[p].telDZ_e[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                        i = (i + 1);
                                    }
                                }
                                fout.take();
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_tel_dy as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes =
                                crate::source_format::format("Telescope y function\n", &[])?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].nTelDY,
                                "psr[p].nTelDY",
                            )?) {
                                {
                                    let bytes = crate::source_format::format(
                                        "%.2f %.10g %.10g\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDX_t
                                                    [(i) as usize],
                                                "psr[p].telDX_t[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDY_v
                                                    [(i) as usize],
                                                "psr[p].telDY_v[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDY_e
                                                    [(i) as usize],
                                                "psr[p].telDY_e[i]",
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                i = (i + 1);
                            }
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_tel_dz as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        {
                            let bytes = crate::source_format::format(
                                "Telescope z function (%d)\n",
                                &[Argument::from(*read(
                                    &context.pulsars[(p) as usize].nTelDZ,
                                    "psr[p].nTelDZ",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].nTelDZ,
                                "psr[p].nTelDZ",
                            )?) {
                                {
                                    let bytes = crate::source_format::format(
                                        "%.2f %.10g %.10g\n",
                                        &[
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDZ_t
                                                    [(i) as usize],
                                                "psr[p].telDZ_t[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDZ_v
                                                    [(i) as usize],
                                                "psr[p].telDZ_v[i]",
                                            )?),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].telDZ_e
                                                    [(i) as usize],
                                                "psr[p].telDZ_e[i]",
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                i = (i + 1);
                            }
                        }
                    }
                }
                {
                    let mut ishape: i32 = 0i32;
                    while (ishape
                        < *read(
                            &context.pulsars[(p) as usize].nTNShapeletEvents,
                            "psr[p].nTNShapeletEvents",
                        )?)
                    {
                        {
                            {
                                let bytes = crate::source_format::format(
                                    "TNShapeletEvent %d %lg %lg %lg ",
                                    &[
                                        Argument::from(*read(
                                            &context.pulsars[(p) as usize].TNShapeletEvN
                                                [(ishape) as usize],
                                            "psr[p].TNShapeletEvN[ishape]",
                                        )?),
                                        Argument::from(*read(
                                            &context.pulsars[(p) as usize].TNShapeletEvPos
                                                [(ishape) as usize],
                                            "psr[p].TNShapeletEvPos[ishape]",
                                        )?),
                                        Argument::from(*read(
                                            &context.pulsars[(p) as usize].TNShapeletEvWidth
                                                [(ishape) as usize],
                                            "psr[p].TNShapeletEvWidth[ishape]",
                                        )?),
                                        Argument::from(*read(
                                            &context.pulsars[(p) as usize].TNShapeletEvFScale
                                                [(ishape) as usize],
                                            "psr[p].TNShapeletEvFScale[ishape]",
                                        )?),
                                    ],
                                )?;
                                io.write(&bytes)?;
                            }
                            if (*read(
                                &context.pulsars[(p) as usize].TNShapeletEvFitFlag
                                    [(ishape) as usize],
                                "psr[p].TNShapeletEvFitFlag[ishape]",
                            )? != 0)
                            {
                                {
                                    {
                                        let bytes = crate::source_format::format("\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                    {
                                        let mut icoef: i32 = 0i32;
                                        while (icoef
                                            < *read(
                                                &context.pulsars[(p) as usize].TNShapeletEvN
                                                    [(ishape) as usize],
                                                "psr[p].TNShapeletEvN[ishape]",
                                            )?)
                                        {
                                            {
                                                {
                                                    let bytes = crate::source_format::format(
                                                        "     %lg %lg Y\n",
                                                        &[
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .TNShapeletEvCoef
                                                                    [(ishape) as usize]
                                                                    [(icoef) as usize],
                                                                "psr[p].TNShapeletEvCoef[ishape][icoef]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .TNShapeletEvCoefErr
                                                                    [(ishape) as usize]
                                                                    [(icoef) as usize],
                                                                "psr[p].TNShapeletEvCoefErr[ishape][icoef]",
                                                            )?),
                                                        ],
                                                    )?;
                                                    io.write(&bytes)?;
                                                }
                                            }
                                            icoef = (icoef + 1);
                                        }
                                    }
                                }
                            } else {
                                {
                                    {
                                        let mut icoef: i32 = 0i32;
                                        while (icoef
                                            < *read(
                                                &context.pulsars[(p) as usize].TNShapeletEvN
                                                    [(ishape) as usize],
                                                "psr[p].TNShapeletEvN[ishape]",
                                            )?)
                                        {
                                            {
                                                {
                                                    let bytes = crate::source_format::format(
                                                        "%lg ",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNShapeletEvCoef
                                                                [(ishape) as usize]
                                                                [(icoef) as usize],
                                                            "psr[p].TNShapeletEvCoef[ishape][icoef]",
                                                        )?)],
                                                    )?;
                                                    io.write(&bytes)?;
                                                }
                                            }
                                            icoef = (icoef + 1);
                                        }
                                    }
                                    {
                                        let bytes = crate::source_format::format("\n", &[])?;
                                        io.write(&bytes)?;
                                    }
                                }
                            }
                            {
                                let bytes = crate::source_format::format("\n", &[])?;
                                io.write(&bytes)?;
                            }
                        }
                        ishape = (ishape + 1);
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_quad_ifunc_p as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut fout = None::<std::fs::File>;
                        let mut fileout: i32 = 1i32;
                        fout = io.create(&"aplus_t2.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            fileout = 0i32;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "---------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Interpolated quadrupolar plus function\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "---------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].quad_ifuncN_p,
                                "psr[p].quad_ifuncN_p",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "%.2f %.10g %.10g\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncT_p
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncT_p[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncV_p
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncV_p[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncE_p
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncE_p[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                    if (fileout == 1i32) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.2f %.10g %.10g\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncT_p
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncT_p[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncV_p
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncV_p[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncE_p
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncE_p[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                        if (fileout == 1i32) {
                            fout.take();
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_quad_ifunc_c as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut fout = None::<std::fs::File>;
                        let mut fileout: i32 = 1i32;
                        fout = io.create(&"across_t2.dat".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            fileout = 0i32;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "---------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "Interpolated quadrupolar cross function\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "---------------------------------------\n",
                                &[],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            i = 0i32;
                            while (i < *read(
                                &context.pulsars[(p) as usize].quad_ifuncN_c,
                                "psr[p].quad_ifuncN_c",
                            )?) {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "%.2f %.10g %.10g\n",
                                            &[
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncT_c
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncT_c[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncV_c
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncV_c[i]",
                                                )?),
                                                Argument::from(*read(
                                                    &context.pulsars[(p) as usize].quad_ifuncE_c
                                                        [(i) as usize],
                                                    "psr[p].quad_ifuncE_c[i]",
                                                )?),
                                            ],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                    if (fileout == 1i32) {
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "%.2f %.10g %.10g\n",
                                                &[
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncT_c
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncT_c[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncV_c
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncV_c[i]",
                                                    )?),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .quad_ifuncE_c
                                                            [(i) as usize],
                                                        "psr[p].quad_ifuncE_c[i]",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                }
                                i = (i + 1);
                            }
                        }
                        if (fileout == 1i32) {
                            fout.take();
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize].cells
                        [(1i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        let mut p0: Float;
                        let mut p1: Float;
                        let mut age: Float;
                        let mut bs: Float;
                        let mut p0e: Float;
                        let mut p1e: Float;
                        let mut f0: Float;
                        let mut f1: Float;
                        let mut f0e: Float;
                        let mut f1e: Float;
                        {
                            let bytes =
                                crate::source_format::format("\nDerived parameters:\n\n", &[])?;
                            io.write(&bytes)?;
                        }
                        f0 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone();
                        f1 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(1i32) as usize]
                            .val)
                            .clone();
                        f0e = (context.pulsars[(p) as usize].param.slots
                            [(param_f as i32) as usize]
                            .cells[(0i32) as usize]
                            .err)
                            .clone();
                        f1e = (context.pulsars[(p) as usize].param.slots
                            [(param_f as i32) as usize]
                            .cells[(1i32) as usize]
                            .err)
                            .clone();
                        p0 = (Float::with_val(PRECISION_BITS, 1.0f64) / (f0).clone());
                        p0e = (((Float::with_val(PRECISION_BITS, 1.0f64) / (f0).clone())
                            / (f0).clone())
                            * (f0e).clone());
                        p1 = ((Float::with_val(PRECISION_BITS, (-1.0f64))
                            / ((f0).clone() * (f0).clone()))
                            * (f1).clone());
                        p1e = ((((p0).clone() * (p0).clone()) * (f1e).clone())
                            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                            + (((((Float::with_val(PRECISION_BITS, 2i32) * (p0).clone())
                                * (p0).clone())
                                * (p0).clone())
                                * (f1).clone())
                                * (f0e).clone())
                            .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                        .sqrt();
                        age = ((((((((p0).clone() / Float::with_val(PRECISION_BITS, 2.0f64))
                            / (p1).clone())
                            / Float::with_val(PRECISION_BITS, 60.0f64))
                            / Float::with_val(PRECISION_BITS, 60.0f64))
                            / Float::with_val(PRECISION_BITS, 24.0f64))
                            / Float::with_val(PRECISION_BITS, 365.0f64))
                            / Float::with_val(PRECISION_BITS, 1.0e6f64));
                        bs = (((-(f1).clone())
                            / ((f0).clone()).pow(&(Float::with_val(PRECISION_BITS, 3i32))))
                        .sqrt()
                            * Float::with_val(PRECISION_BITS, 3.2e19f64));
                        {
                            let bytes = crate::source_format::format(
                                "P0 (s)      = %-25.15g %-13.5g\n",
                                &[
                                    Argument::from(((p0).clone()).to_f64()),
                                    Argument::from(((p0e).clone()).to_f64()),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "P1          = %-25.15g %-13.5g\n",
                                &[
                                    Argument::from(((p1).clone()).to_f64()),
                                    Argument::from(((p1e).clone()).to_f64()),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "tau_c (Myr) = %.5g\n",
                                &[Argument::from(((age).clone()).to_f64())],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "bs (G)      = %.5g\n\n",
                                &[Argument::from(((bs).clone()).to_f64())],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots[(param_brake as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize].cells
                        [(1i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        let mut F2brake: Float;
                        let mut F1: Float;
                        let mut F0: Float;
                        let mut F3brake: Float;
                        let mut bindex: Float;
                        F1 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(1i32) as usize]
                            .val)
                            .clone();
                        F0 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone();
                        bindex = (context.pulsars[(p) as usize].param.slots
                            [(param_brake as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone();
                        F2brake =
                            ((((bindex).clone() * (F1).clone()) * (F1).clone()) / (F0).clone());
                        F3brake = (((((((bindex).clone()
                            * ((Float::with_val(PRECISION_BITS, 2i32) * (bindex).clone())
                                - Float::with_val(PRECISION_BITS, 1i32)))
                            * (F1).clone())
                            * (F1).clone())
                            * (F1).clone())
                            / (F0).clone())
                            / (F0).clone());
                        {
                            let bytes = crate::source_format::format(
                                "F2 derived from braking index %.5Le\n",
                                &[Argument::from((F2brake).clone())],
                            )?;
                            io.write(&bytes)?;
                        }
                        {
                            let bytes = crate::source_format::format(
                                "F3 derived from braking index %.5Le\n",
                                &[Argument::from((F3brake).clone())],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize].cells
                    [(2i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots[(param_brake as i32) as usize]
                        .cells[(0i32) as usize]
                        .param_set
                        == 0i32))
                {
                    {
                        let mut brake: Float;
                        let mut F1: Float;
                        let mut F0: Float;
                        let mut F2: Float;
                        F1 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(1i32) as usize]
                            .val)
                            .clone();
                        F0 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone();
                        F2 = (context.pulsars[(p) as usize].param.slots[(param_f as i32) as usize]
                            .cells[(2i32) as usize]
                            .val)
                            .clone();
                        brake = (((F0).clone() * (F2).clone()) / ((F1).clone() * (F1).clone()));
                        {
                            let bytes = crate::source_format::format(
                                "Braking index derived from F0,F1,F2 %.3Lf\n",
                                &[Argument::from((brake).clone())],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots[(param_pb as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                    || (context.pulsars[(p) as usize].param.slots[(param_fb as i32) as usize]
                        .cells[(0i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        let mut err: f64;
                        let mut pb: Float;
                        let mut a1: Float;
                        let mut si: Float = Float::with_val(PRECISION_BITS, (-2i32));
                        let mut si_lo: Float = Float::with_val(PRECISION_BITS, (-2i32));
                        let mut si_hi: Float = Float::with_val(PRECISION_BITS, (-2i32));
                        if (context.pulsars[(p) as usize].param.slots[(param_pb as i32) as usize]
                            .cells[(0i32) as usize]
                            .param_set
                            == 1i32)
                        {
                            {
                                pb = ((context.pulsars[(p) as usize].param.slots
                                    [(param_pb as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone()
                                    * Float::with_val(PRECISION_BITS, 86400.0f64));
                            }
                        } else {
                            {
                                pb = (Float::with_val(PRECISION_BITS, 1i32)
                                    / (context.pulsars[(p) as usize].param.slots
                                        [(param_fb as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .val)
                                        .clone());
                            }
                        }
                        a1 = ((context.pulsars[(p) as usize].param.slots
                            [(param_a1 as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone()
                            * Float::with_val(PRECISION_BITS, 299792458.0f64));
                        {
                            let bytes = crate::source_format::format(
                                "Binary model: %s\n",
                                &[Argument::from(read_string(
                                    &context.pulsars[(p) as usize].binaryModel[..],
                                    "psr[p].binaryModel",
                                )?)],
                            )?;
                            io.write(&bytes)?;
                        }
                        if (context.pulsars[(p) as usize].param.slots[(param_a1 as i32) as usize]
                            .cells[(0i32) as usize]
                            .param_set
                            == 1i32)
                        {
                            {
                                let mut c1: i32 = 0i32;
                                let mut c2: i32 = 0i32;
                                let mut mass_function: Float;
                                mass_function = ((Float::with_val(
                                    PRECISION_BITS,
                                    ((((4i32) as f64 * std::f64::consts::PI)
                                        * std::f64::consts::PI)
                                        / 1.3271243999e20f64),
                                ) * ((a1).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, 3i32))))
                                    / ((pb).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32))));
                                {
                                    let bytes = crate::source_format::format(
                                        "Mass function                  = %.12f ",
                                        &[Argument::from(((mass_function).clone()).to_f64())],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                c1 = crate::general_fitting::get_param_matrix_row(
                                    &context.pulsars[p as usize].fitinfo,
                                    p,
                                    crate::tempo2::PARAMETER_LABELS[(param_a1 as i32) as usize],
                                    0i32,
                                )
                                .map(|i| i as i32)
                                .unwrap_or(-1);
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_pb as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        c2 = crate::general_fitting::get_param_matrix_row(
                                            &context.pulsars[p as usize].fitinfo,
                                            p,
                                            crate::tempo2::PARAMETER_LABELS
                                                [(param_pb as i32) as usize],
                                            0i32,
                                        )
                                        .map(|i| i as i32)
                                        .unwrap_or(-1);
                                    }
                                } else {
                                    {
                                        c2 = crate::general_fitting::get_param_matrix_row(
                                            &context.pulsars[p as usize].fitinfo,
                                            p,
                                            crate::tempo2::PARAMETER_LABELS
                                                [(param_fb as i32) as usize],
                                            0i32,
                                        )
                                        .map(|i| i as i32)
                                        .unwrap_or(-1);
                                    }
                                }
                                if ((c1 >= 0i32) && (c2 >= 0i32)) {
                                    {
                                        err = ((mass_function).clone()
                                            * (((Float::with_val(PRECISION_BITS, 9.0f64)
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    ((output_covar(
                                                        context,
                                                        (p) as usize,
                                                        (c1) as usize,
                                                        (c1) as usize,
                                                    )?)
                                                    .sqrt()
                                                        * 299792458.0f64),
                                                ) / (a1).clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                                + (Float::with_val(PRECISION_BITS, 4.0f64)
                                                    * (Float::with_val(
                                                        PRECISION_BITS,
                                                        (output_covar(
                                                            context,
                                                            (p) as usize,
                                                            (c2) as usize,
                                                            (c2) as usize,
                                                        )?)
                                                        .sqrt(),
                                                    ) / (pb).clone())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 2i32)),
                                                    )))
                                                - (Float::with_val(
                                                    PRECISION_BITS,
                                                    ((12.0f64
                                                        * output_covar(
                                                            context,
                                                            (p) as usize,
                                                            (c1) as usize,
                                                            (c2) as usize,
                                                        )?)
                                                        * 299792458.0f64),
                                                ) / ((a1).clone() * (pb).clone())))
                                            .sqrt())
                                        .to_f64();
                                        {
                                            let bytes = crate::source_format::format(
                                                " +- %.12f solar masses \n",
                                                &[Argument::from(err)],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                } else {
                                    {
                                        {
                                            let bytes = crate::source_format::format("\n", &[])?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "Minimum, median and maximum companion mass: %.4g < %.4f < %.4f solar masses\n",
                                        &[
                                            Argument::from(m2_inner(
                                                (mass_function).clone(),
                                                Float::with_val(PRECISION_BITS, 1.0f64),
                                                Float::with_val(PRECISION_BITS, 1.35f64),
                                                io,
                                            )?),
                                            Argument::from(m2_inner(
                                                (mass_function).clone(),
                                                Float::with_val(PRECISION_BITS, 0.866025403f64),
                                                Float::with_val(PRECISION_BITS, 1.35f64),
                                                io,
                                            )?),
                                            Argument::from(m2_inner(
                                                (mass_function).clone(),
                                                Float::with_val(PRECISION_BITS, 0.4358898944f64),
                                                Float::with_val(PRECISION_BITS, 1.35f64),
                                                io,
                                            )?),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_h3 as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        let mut h3: Float = (context.pulsars[(p) as usize]
                                            .param
                                            .slots
                                            [(param_h3 as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone();
                                        let mut m2: Option<Float> = None;
                                        let mut sini: Float;
                                        if (context.pulsars[(p) as usize].param.slots
                                            [(param_stig as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .param_set
                                            == 1i32)
                                        {
                                            {
                                                let mut stig: Float =
                                                    (context.pulsars[(p) as usize].param.slots
                                                        [(param_stig as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone();
                                                m2 = Some(
                                                    (((h3).clone()
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            4.925490947e-6f64,
                                                        ))
                                                        * ((stig).clone()).pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                (-3.0f64),
                                                            )),
                                                        )),
                                                );
                                                sini = ((Float::with_val(PRECISION_BITS, 2.0f64)
                                                    * (stig).clone())
                                                    / (Float::with_val(PRECISION_BITS, 1.0f64)
                                                        + ((stig).clone()).pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2.0f64,
                                                            )),
                                                        )));
                                                {
                                                    let bytes = crate::source_format::format(
                                                        " DDH-model Derived parameters:  M2 = %lg\n",
                                                        &[Argument::from(
                                                            ((read(&m2, "m2")?).clone()).to_f64(),
                                                        )],
                                                    )?;
                                                    io.write(&bytes)?;
                                                }
                                                {
                                                    let bytes = crate::source_format::format(
                                                        "                              SINI = %lg\n",
                                                        &[Argument::from(
                                                            ((sini).clone()).to_f64(),
                                                        )],
                                                    )?;
                                                    io.write(&bytes)?;
                                                }
                                            }
                                        } else {
                                            if (context.pulsars[(p) as usize].param.slots
                                                [(param_h4 as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .param_set
                                                == 1i32)
                                            {
                                                {
                                                    let mut h4: Float =
                                                        (context.pulsars[(p) as usize].param.slots
                                                            [(param_h4 as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone();
                                                    if ((h4).clone()
                                                        != Float::with_val(PRECISION_BITS, 0.0f64))
                                                    {
                                                        m2 = Some(
                                                            ((((h3).clone()).pow(
                                                                &(Float::with_val(
                                                                    PRECISION_BITS,
                                                                    4.0f64,
                                                                )),
                                                            ) / ((h4).clone()).pow(
                                                                &(Float::with_val(
                                                                    PRECISION_BITS,
                                                                    3.0f64,
                                                                )),
                                                            )) / Float::with_val(
                                                                PRECISION_BITS,
                                                                4.925490947e-6f64,
                                                            )),
                                                        );
                                                    }
                                                    sini = (((Float::with_val(
                                                        PRECISION_BITS,
                                                        2.0f64,
                                                    ) * (h3).clone())
                                                        * (h4).clone())
                                                        / (((h3).clone() * (h3).clone())
                                                            + ((h4).clone() * (h4).clone())));
                                                    {
                                                        let bytes = crate::source_format::format(
                                                            " DDH-model Derived parameters:  M2 = %lg\n",
                                                            &[Argument::from(
                                                                ((read(&m2, "m2")?).clone())
                                                                    .to_f64(),
                                                            )],
                                                        )?;
                                                        io.write(&bytes)?;
                                                    }
                                                    {
                                                        let bytes = crate::source_format::format(
                                                            "                              SINI = %lg\n",
                                                            &[Argument::from(
                                                                ((sini).clone()).to_f64(),
                                                            )],
                                                        )?;
                                                        io.write(&bytes)?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if (((((context.pulsars[(p) as usize].param.slots
                                    [(param_sini as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_m2 as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_a1 as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_pb as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    == 1i32)
                                    && (*read(
                                        &context.pulsars[(p) as usize].param.slots
                                            [(param_sini as i32) as usize]
                                            .n_link_to,
                                        "psr[p].param[param_sini].nLinkTo",
                                    )? == 0i32))
                                {
                                    {
                                        let mut mp: [Float; 2] = std::array::from_fn(|_| {
                                            Float::with_val(PRECISION_BITS, 0)
                                        });
                                        let mut DAY2S: f64 = (Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("24.0").unwrap(),
                                        ) * Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("3600.0").unwrap(),
                                        ))
                                        .to_f64();
                                        mp[(0i32) as usize] = ((-(context.pulsars[(p) as usize]
                                            .param
                                            .slots
                                            [(param_m2 as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone())
                                            + ((Float::with_val(
                                                PRECISION_BITS,
                                                4.925490947e-6f64,
                                            ) * ((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_pb as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone()
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                / Float::with_val(PRECISION_BITS, 2.0f64))
                                                / Float::with_val(
                                                    PRECISION_BITS,
                                                    std::f64::consts::PI,
                                                ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                                * (((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_sini as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 3.0f64))))
                                            .sqrt());
                                        let mut Cte: Float =
                                            (Float::with_val(PRECISION_BITS, 4.925490947e-6f64)
                                                * ((Float::with_val(PRECISION_BITS, 1i32)
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ))
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        std::f64::consts::PI,
                                                    ))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            .sqrt();
                                        mp[(1i32) as usize] = (((((context.pulsars
                                            [(p) as usize]
                                            .param
                                            .slots
                                            [(param_m2 as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .err)
                                            .clone()
                                            * (Float::with_val(PRECISION_BITS, (-1.0f64))
                                                + (((((Float::with_val(
                                                    PRECISION_BITS,
                                                    1.5f64,
                                                ) * (Cte).clone())
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        DAY2S,
                                                    ))
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_pb as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    * ((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_sini as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        / (context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(param_a1 as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 1.5f64)),
                                                    ))
                                                    * ((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_m2 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    .sqrt())))
                                        .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                            + ((((context.pulsars[(p) as usize].param.slots
                                                [(param_pb as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                * (Cte).clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * (((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_sini as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(
                                                    &(Float::with_val(PRECISION_BITS, 1.5f64)),
                                                ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            + (((((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_sini as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    Float::parse("1.5").unwrap(),
                                                ))
                                                * ((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_sini as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                .sqrt())
                                                * (Cte).clone())
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * ((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 1.5f64))))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            + (((((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_a1 as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                * Float::with_val(PRECISION_BITS, 1.5f64))
                                                / ((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_a1 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                .pow(
                                                    &(Float::with_val(PRECISION_BITS, 2.5f64)),
                                                ))
                                                * (Cte).clone())
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * ((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_sini as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 1.5f64))))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                        .sqrt();
                                        {
                                            let bytes = crate::source_format::format(
                                                "Pulsar Mass (Shapiro Delay): %lg (+/- %lg) Msun.\n",
                                                &[
                                                    Argument::from(
                                                        ((mp[(0i32) as usize]).clone()).to_f64(),
                                                    ),
                                                    Argument::from(
                                                        ((mp[(1i32) as usize]).clone()).to_f64(),
                                                    ),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                if ((((context.pulsars[(p) as usize].param.slots
                                    [(param_kin as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_m2 as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_a1 as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    * context.pulsars[(p) as usize].param.slots
                                        [(param_pb as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .param_set)
                                    == 1i32)
                                {
                                    {
                                        let mut mp: [Float; 2] = std::array::from_fn(|_| {
                                            Float::with_val(PRECISION_BITS, 0)
                                        });
                                        let mut DAY2S: f64 = (Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("24.0").unwrap(),
                                        ) * Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("3600.0").unwrap(),
                                        ))
                                        .to_f64();
                                        mp[(0i32) as usize] = ((-(context.pulsars[(p) as usize]
                                            .param
                                            .slots
                                            [(param_m2 as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone())
                                            + ((Float::with_val(
                                                PRECISION_BITS,
                                                4.925490947e-6f64,
                                            ) * ((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_pb as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone()
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                / Float::with_val(PRECISION_BITS, 2.0f64))
                                                / Float::with_val(
                                                    PRECISION_BITS,
                                                    std::f64::consts::PI,
                                                ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                                * (((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_kin as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            180.0f64,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                    .sin())
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 3.0f64))))
                                            .sqrt());
                                        let mut Cte: Float =
                                            (Float::with_val(PRECISION_BITS, 4.925490947e-6f64)
                                                * ((Float::with_val(PRECISION_BITS, 1i32)
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ))
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        std::f64::consts::PI,
                                                    ))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            .sqrt();
                                        mp[(1i32) as usize] = (((((context.pulsars
                                            [(p) as usize]
                                            .param
                                            .slots
                                            [(param_m2 as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .err)
                                            .clone()
                                            * (Float::with_val(PRECISION_BITS, (-1.0f64))
                                                + (((((Float::with_val(
                                                    PRECISION_BITS,
                                                    1.5f64,
                                                ) * (Cte).clone())
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        DAY2S,
                                                    ))
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_pb as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    * ((((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_kin as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            180.0f64,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                    .sin()
                                                        / (context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(param_a1 as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 1.5f64)),
                                                    ))
                                                    * ((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_m2 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    .sqrt())))
                                        .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                            + ((((context.pulsars[(p) as usize].param.slots
                                                [(param_pb as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                * (Cte).clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * (((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_kin as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            180.0f64,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                    .sin())
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(
                                                    &(Float::with_val(PRECISION_BITS, 1.5f64)),
                                                ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            + ((((((((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_kin as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                / Float::with_val(
                                                    PRECISION_BITS,
                                                    180.0f64,
                                                ))
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    std::f64::consts::PI,
                                                ))
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    Float::parse("1.5").unwrap(),
                                                ))
                                                * (((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_kin as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        180.0f64,
                                                    ))
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        std::f64::consts::PI,
                                                    ))
                                                .cos())
                                                * ((((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_kin as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        180.0f64,
                                                    ))
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        std::f64::consts::PI,
                                                    ))
                                                .sin())
                                                .sqrt())
                                                * (Cte).clone())
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * ((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    / (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_a1 as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 1.5f64))))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            + (((((((context.pulsars[(p) as usize]
                                                .param
                                                .slots
                                                [(param_a1 as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone()
                                                * Float::with_val(PRECISION_BITS, 1.5f64))
                                                / ((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_a1 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                .pow(
                                                    &(Float::with_val(PRECISION_BITS, 2.5f64)),
                                                ))
                                                * (Cte).clone())
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, DAY2S))
                                                * ((context.pulsars[(p) as usize].param.slots
                                                    [(param_m2 as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * (((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_kin as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        / Float::with_val(
                                                            PRECISION_BITS,
                                                            180.0f64,
                                                        ))
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            std::f64::consts::PI,
                                                        ))
                                                    .sin())
                                                .pow(&(Float::with_val(PRECISION_BITS, 1.5f64))))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                        .sqrt();
                                        {
                                            let bytes = crate::source_format::format(
                                                "Pulsar Mass (annual orbital parallax): %lg (+/- %lg) Msun.\n",
                                                &[
                                                    Argument::from(
                                                        ((mp[(0i32) as usize]).clone()).to_f64(),
                                                    ),
                                                    Argument::from(
                                                        ((mp[(1i32) as usize]).clone()).to_f64(),
                                                    ),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_px as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        let mut pxdist: [Float; 2] = std::array::from_fn(|_| {
                                            Float::with_val(PRECISION_BITS, 0)
                                        });
                                        pxdist[(0i32) as usize] =
                                            ((Float::with_val(PRECISION_BITS, 1.0f64)
                                                / (context.pulsars[(p) as usize].param.slots
                                                    [(param_px as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 1000.0f64));
                                        pxdist[(1i32) as usize] =
                                            (((Float::with_val(PRECISION_BITS, 1i32)
                                                / ((context.pulsars[(p) as usize].param.slots
                                                    [(param_px as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_px as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .err)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 1000.0f64));
                                        {
                                            let bytes = crate::source_format::format(
                                                "\nParallax distance is %lg (+/- %lg) pc.\n",
                                                &[
                                                    Argument::from(
                                                        ((pxdist[(0i32) as usize]).clone())
                                                            .to_f64(),
                                                    ),
                                                    Argument::from(
                                                        ((pxdist[(1i32) as usize]).clone())
                                                            .to_f64(),
                                                    ),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_pbdot as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        let mut pbdotdist: [Float; 2] = std::array::from_fn(|_| {
                                            Float::with_val(PRECISION_BITS, 0)
                                        });
                                        let mut pmsqrd: [Float; 2] = std::array::from_fn(|_| {
                                            Float::with_val(PRECISION_BITS, 0)
                                        });
                                        pmsqrd[(0i32) as usize] =
                                            (((context.pulsars[(p) as usize].param.slots
                                                [(param_pmra as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone()
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    1.53628185004416045e-16f64,
                                                ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                                + ((context.pulsars[(p) as usize].param.slots
                                                    [(param_pmdec as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        1.53628185004416045e-16f64,
                                                    ))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))));
                                        pmsqrd[(1i32) as usize] = ((((Float::with_val(
                                            PRECISION_BITS,
                                            2i32,
                                        ) * (context.pulsars
                                            [(p) as usize]
                                            .param
                                            .slots
                                            [(param_pmra as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone())
                                            * (context.pulsars[(p) as usize].param.slots
                                                [(param_pmra as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .err)
                                                .clone())
                                            * (Float::with_val(
                                                PRECISION_BITS,
                                                1.53628185004416045e-16f64,
                                            ))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                        .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                            + (((Float::with_val(PRECISION_BITS, 2i32)
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pmdec as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone())
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pmdec as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .err)
                                                    .clone())
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    1.53628185004416045e-16f64,
                                                ))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))));
                                        pbdotdist[(0i32) as usize] = (((((context.pulsars
                                            [(p) as usize]
                                            .param
                                            .slots
                                            [(param_pbdot as i32) as usize]
                                            .cells[(0i32) as usize]
                                            .val)
                                            .clone()
                                            * Float::with_val(PRECISION_BITS, 299792458.0f64))
                                            / ((context.pulsars[(p) as usize].param.slots
                                                [(param_pb as i32) as usize]
                                                .cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone()
                                                * Float::with_val(PRECISION_BITS, 86400.0f64)))
                                            / (pmsqrd[(0i32) as usize]).clone())
                                            / Float::with_val(PRECISION_BITS, 3.08568025e16f64));
                                        pbdotdist[(1i32) as usize] = Float::with_val(
                                            PRECISION_BITS,
                                            ((((((Float::with_val(PRECISION_BITS, 299792458.0f64)
                                                / (((context.pulsars[(p) as usize].param.slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .val)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        86400.0f64,
                                                    ))
                                                    * (pmsqrd[(0i32) as usize]).clone()))
                                                * (context.pulsars[(p) as usize].param.slots
                                                    [(param_pbdot as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .err)
                                                    .clone())
                                            .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                                + (((((context.pulsars[(p) as usize]
                                                    .param
                                                    .slots
                                                    [(param_pb as i32) as usize]
                                                    .cells
                                                    [(0i32) as usize]
                                                    .err)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        86400.0f64,
                                                    ))
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        299792458.0f64,
                                                    ))
                                                    * (context.pulsars[(p) as usize].param.slots
                                                        [(param_pbdot as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    / ((pmsqrd[(0i32) as usize]).clone()
                                                        * ((context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(param_pb as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone()
                                                            * Float::with_val(
                                                                PRECISION_BITS,
                                                                86400.0f64,
                                                            ))
                                                        .pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2.0f64,
                                                            )),
                                                        )))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                                + ((((pmsqrd[(1i32) as usize]).clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        299792458.0f64,
                                                    ))
                                                    * (context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_pbdot as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone())
                                                    / (((context.pulsars[(p) as usize]
                                                        .param
                                                        .slots
                                                        [(param_pb as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone()
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            86400.0f64,
                                                        ))
                                                        * ((pmsqrd[(0i32) as usize]).clone())
                                                            .pow(
                                                                &(Float::with_val(
                                                                    PRECISION_BITS,
                                                                    2.0f64,
                                                                )),
                                                            )))
                                                .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                            .to_f64())
                                            .sqrt()
                                                / 3.08568025e16f64),
                                        );
                                        {
                                            let bytes = crate::source_format::format(
                                                "Pbdot distance is %lg (+/- %lg) pc.\n",
                                                &[
                                                    Argument::from(
                                                        ((pbdotdist[(0i32) as usize]).clone())
                                                            .to_f64(),
                                                    ),
                                                    Argument::from(
                                                        ((pbdotdist[(1i32) as usize]).clone())
                                                            .to_f64(),
                                                    ),
                                                ],
                                            )?;
                                            io.write(&bytes)?;
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_daop as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "Used daop distance of %g for Annual-Orbital parallax delays.\n",
                                            &[Argument::from(
                                                (crate::tempo2_util::get_parameter_value_inner(
                                                    &context.pulsars[(p) as usize],
                                                    (param_daop as i32) as usize,
                                                    (0i32) as usize,
                                                    d,
                                                )?)
                                                .to_f64(),
                                            )],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                            }
                        }
                        if (context.pulsars[(p) as usize].param.slots
                            [(param_shapmax as i32) as usize]
                            .cells[(0i32) as usize]
                            .param_set
                            == 1i32)
                        {
                            {
                                let mut smax: Float;
                                let mut err: f64;
                                smax = (context.pulsars[(p) as usize].param.slots
                                    [(param_shapmax as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone();
                                err = ((context.pulsars[(p) as usize].param.slots
                                    [(param_shapmax as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .err)
                                    .clone())
                                .to_f64();
                                si = (Float::with_val(PRECISION_BITS, 1.0f64)
                                    - (Float::with_val(PRECISION_BITS, (-1.0f64))
                                        * (smax).clone())
                                    .exp());
                                {
                                    let bytes = crate::source_format::format(
                                        "sini derived from SHAPMAX      = %.14Lf ",
                                        &[Argument::from((si).clone())],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                si_lo = (Float::with_val(PRECISION_BITS, 1.0f64)
                                    - (Float::with_val(PRECISION_BITS, (-1.0f64))
                                        * ((smax).clone() - Float::with_val(PRECISION_BITS, err)))
                                    .exp());
                                si_hi = (Float::with_val(PRECISION_BITS, 1.0f64)
                                    - (Float::with_val(PRECISION_BITS, (-1.0f64))
                                        * ((smax).clone() + Float::with_val(PRECISION_BITS, err)))
                                    .exp());
                                {
                                    let bytes = crate::source_format::format(
                                        " (+ %.8f  - %.8f)\n",
                                        &[
                                            Argument::from(
                                                ((si_hi).clone() - (si).clone()).to_f64(),
                                            ),
                                            Argument::from(
                                                ((si).clone() - (si_lo).clone()).to_f64(),
                                            ),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                        if ((context.pulsars[(p) as usize].param.slots[(param_m2 as i32) as usize]
                            .cells[(0i32) as usize]
                            .param_set
                            == 1i32)
                            && (((si).clone() != Float::with_val(PRECISION_BITS, (-2i32)))
                                || (context.pulsars[(p) as usize].param.slots
                                    [(param_sini as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)))
                        {
                            {
                                let mut m2: Option<Float> = None;
                                let mut amtot: Float;
                                if ((si).clone() == Float::with_val(PRECISION_BITS, (-2i32))) {
                                    si = crate::tempo2_util::get_parameter_value_inner(
                                        &context.pulsars[(p) as usize],
                                        (param_sini as i32) as usize,
                                        (0i32) as usize,
                                        d,
                                    )?;
                                }
                                m2 = Some(
                                    ((context.pulsars[(p) as usize].param.slots
                                        [(param_m2 as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .val)
                                        .clone()
                                        * Float::with_val(PRECISION_BITS, 1.98892e30f64)),
                                );
                                amtot = (((((pb).clone()
                                    / Float::with_val(PRECISION_BITS, 2.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI))
                                    * ((((read(&m2, "m2")?).clone() * (si).clone())
                                        .pow(&(Float::with_val(PRECISION_BITS, 3i32)))
                                        * Float::with_val(PRECISION_BITS, 6.673e-11f64))
                                        / ((a1).clone())
                                            .pow(&(Float::with_val(PRECISION_BITS, 3i32))))
                                    .sqrt())
                                    / Float::with_val(PRECISION_BITS, 1.98892e30f64));
                                {
                                    let bytes = crate::source_format::format(
                                        "MTOT derived from sin i and m2 = %.14g\n",
                                        &[Argument::from(((amtot).clone()).to_f64())],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                        if ((((si).clone() != Float::with_val(PRECISION_BITS, (-2i32)))
                            || (context.pulsars[(p) as usize].param.slots
                                [(param_sini as i32) as usize]
                                .cells[(0i32) as usize]
                                .param_set
                                == 1i32))
                            && (*read(
                                &context.pulsars[(p) as usize].param.slots
                                    [(param_sini as i32) as usize]
                                    .n_link_to,
                                "psr[p].param[param_sini].nLinkTo",
                            )? == 0i32))
                        {
                            {
                                let mut inc: Float;
                                let mut inc_lo: Float;
                                let mut inc_hi: Float;
                                if ((si).clone() == Float::with_val(PRECISION_BITS, (-2i32))) {
                                    si = crate::tempo2_util::get_parameter_value_inner(
                                        &context.pulsars[(p) as usize],
                                        (param_sini as i32) as usize,
                                        (0i32) as usize,
                                        d,
                                    )?;
                                }
                                if ((si_lo).clone() == Float::with_val(PRECISION_BITS, (-2i32))) {
                                    si_lo = (crate::tempo2_util::get_parameter_value_inner(
                                        &context.pulsars[(p) as usize],
                                        (param_sini as i32) as usize,
                                        (0i32) as usize,
                                        d,
                                    )? - (context.pulsars[(p) as usize].param.slots
                                        [(param_sini as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .err)
                                        .clone());
                                }
                                if ((si_hi).clone() == Float::with_val(PRECISION_BITS, (-2i32))) {
                                    si_hi = (crate::tempo2_util::get_parameter_value_inner(
                                        &context.pulsars[(p) as usize],
                                        (param_sini as i32) as usize,
                                        (0i32) as usize,
                                        d,
                                    )? + (context.pulsars[(p) as usize].param.slots
                                        [(param_sini as i32) as usize]
                                        .cells[(0i32) as usize]
                                        .err)
                                        .clone());
                                }
                                inc = ((((si).clone()).asin()
                                    * Float::with_val(PRECISION_BITS, 180.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                                inc_lo = ((((si_lo).clone()).asin()
                                    * Float::with_val(PRECISION_BITS, 180.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                                inc_hi = ((((si_hi).clone()).asin()
                                    * Float::with_val(PRECISION_BITS, 180.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                                {
                                    let bytes = crate::source_format::format(
                                        "Inclination angle (deg)        = %.14g (+ %.7g - %.7g)\n",
                                        &[
                                            Argument::from(((inc).clone()).to_f64()),
                                            Argument::from(
                                                ((inc_hi).clone() - (inc).clone()).to_f64(),
                                            ),
                                            Argument::from(
                                                ((inc).clone() - (inc_lo).clone()).to_f64(),
                                            ),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                        if ((context.pulsars[(p) as usize].param.slots
                            [(param_eps1 as i32) as usize]
                            .cells[(0i32) as usize]
                            .param_set
                            == 1i32)
                            && (context.pulsars[(p) as usize].param.slots
                                [(param_eps2 as i32) as usize]
                                .cells[(0i32) as usize]
                                .param_set
                                == 1i32))
                        {
                            {
                                let mut om: Float;
                                let mut ecc: Float;
                                let mut t0: Float;
                                let mut pb: Float;
                                let mut ecc_err: Float;
                                let mut om_err: Float;
                                let mut t0_err: Float;
                                let mut eps1: Float;
                                let mut eps2: Float;
                                let mut tasc: Float;
                                let mut err1: Float;
                                let mut err2: Float;
                                let mut err3: Float;
                                eps1 = (context.pulsars[(p) as usize].param.slots
                                    [(param_eps1 as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone();
                                eps2 = (context.pulsars[(p) as usize].param.slots
                                    [(param_eps2 as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone();
                                tasc = (context.pulsars[(p) as usize].param.slots
                                    [(param_tasc as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone();
                                pb = (context.pulsars[(p) as usize].param.slots
                                    [(param_pb as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .val)
                                    .clone();
                                err1 = (context.pulsars[(p) as usize].param.slots
                                    [(param_eps1 as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .err)
                                    .clone();
                                err2 = (context.pulsars[(p) as usize].param.slots
                                    [(param_eps2 as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .err)
                                    .clone();
                                err3 = (context.pulsars[(p) as usize].param.slots
                                    [(param_tasc as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .err)
                                    .clone();
                                ecc = (((eps1).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))
                                    + ((eps2).clone())
                                        .pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
                                .sqrt();
                                om = ((((eps1).clone()).atan2(&((eps2).clone()))
                                    * Float::with_val(PRECISION_BITS, 180.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                                if ((om).clone() < Float::with_val(PRECISION_BITS, 0i32)) {
                                    om = ((om).clone() + Float::with_val(PRECISION_BITS, 360.0f64));
                                }
                                t0 = ((tasc).clone()
                                    + (((pb).clone() / Float::with_val(PRECISION_BITS, 360i32))
                                        * (om).clone()));
                                om_err = (((((((eps2).clone() * (err1).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                    + ((eps1).clone() * (err2).clone())
                                        .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                .pow(&(Float::with_val(PRECISION_BITS, 0.5f64)))
                                    / (ecc).clone())
                                    / (ecc).clone())
                                    * Float::with_val(PRECISION_BITS, 180.0f64))
                                    / Float::with_val(PRECISION_BITS, std::f64::consts::PI));
                                if (((eps1).clone() == Float::with_val(PRECISION_BITS, 0.0f64))
                                    && ((eps2).clone() == Float::with_val(PRECISION_BITS, 0.0f64)))
                                {
                                    ecc_err = (((err1).clone())
                                        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                        + ((err2).clone())
                                            .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                    .sqrt();
                                } else {
                                    ecc_err = ((((eps1).clone() * (err1).clone())
                                        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                        + ((eps2).clone() * (err2).clone())
                                            .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                    .pow(&(Float::with_val(PRECISION_BITS, 0.5f64)))
                                        / (ecc).clone());
                                }
                                t0_err = (((err3).clone() * (err3).clone())
                                    + (((pb).clone() / Float::with_val(PRECISION_BITS, 360.0f64))
                                        * (om_err).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                .sqrt();
                                {
                                    let bytes = crate::source_format::format(
                                        "\nConversion from ELL1 parameters: \n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "ECC = %.15lg +/- %.15g\n",
                                        &[
                                            Argument::from(((ecc).clone()).to_f64()),
                                            Argument::from(((ecc_err).clone()).to_f64()),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "OM  = %.15g +/- %.15g degrees\n",
                                        &[
                                            Argument::from(((om).clone()).to_f64()),
                                            Argument::from(((om_err).clone()).to_f64()),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "T0  = %.15Lg +/- %.15g\n",
                                        &[
                                            Argument::from((t0).clone()),
                                            Argument::from(((t0_err).clone()).to_f64()),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "------------------------------------\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots[(param_pmra as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                    && (context.pulsars[(p) as usize].param.slots[(param_pmdec as i32) as usize]
                        .cells[(0i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        let mut pmtot: f64;
                        let mut epmtot: f64;
                        let mut val1: f64;
                        let mut val2: f64;
                        let mut err1: f64;
                        let mut err2: f64;
                        val1 = ((context.pulsars[(p) as usize].param.slots
                            [(param_pmra as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone())
                        .to_f64();
                        val2 = ((context.pulsars[(p) as usize].param.slots
                            [(param_pmdec as i32) as usize]
                            .cells[(0i32) as usize]
                            .val)
                            .clone())
                        .to_f64();
                        err1 = ((context.pulsars[(p) as usize].param.slots
                            [(param_pmra as i32) as usize]
                            .cells[(0i32) as usize]
                            .err)
                            .clone())
                        .to_f64();
                        err2 = ((context.pulsars[(p) as usize].param.slots
                            [(param_pmdec as i32) as usize]
                            .cells[(0i32) as usize]
                            .err)
                            .clone())
                        .to_f64();
                        pmtot = ((val1).powf((2i32) as f64) + (val2).powf((2i32) as f64)).sqrt();
                        epmtot = (((val1 * err1).powf((2i32) as f64)
                            + (val2 * err2).powf((2i32) as f64))
                            / ((val1 * val1) + (val2 * val2)))
                            .sqrt();
                        {
                            let bytes = crate::source_format::format(
                                "Total proper motion = %.5g +/- %.5g mas/yr\n",
                                &[Argument::from(pmtot), Argument::from(epmtot)],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                if (context.pulsars[(p) as usize].param.slots[(param_glep as i32) as usize].cells
                    [(0i32) as usize]
                    .param_set
                    == 1i32)
                {
                    print_glitch_inner(&context.pulsars[p as usize], io, d)?;
                }
                if (context.pulsars[(p) as usize].param.slots[(param_dmassplanet as i32) as usize]
                    .cells[(4i32) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        let mut diff: Float;
                        let mut err: Float;
                        if (read_string(
                            &context.pulsars[(p) as usize].JPL_EPHEMERIS[..],
                            "psr[p].JPL_EPHEMERIS",
                        )?)
                        .windows(("DE405").len())
                        .any(|w| w == ("DE405").as_bytes())
                        {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "M_Jupiter fit   = %.15Lg +/- %.15Lg (Solar masses)\n",
                                        &[
                                            Argument::from(
                                                ((context.pulsars[(p) as usize].param.slots
                                                    [(param_dmassplanet as i32) as usize]
                                                    .cells
                                                    [(4i32) as usize]
                                                    .val)
                                                    .clone()
                                                    + Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("0.00095479193842432214")
                                                            .unwrap(),
                                                    )),
                                            ),
                                            Argument::from(
                                                (context.pulsars[(p) as usize].param.slots
                                                    [(param_dmassplanet as i32) as usize]
                                                    .cells
                                                    [(4i32) as usize]
                                                    .err)
                                                    .clone(),
                                            ),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "M_Jupiter DE405 = %.15Lg (Solar masses)\n",
                                        &[Argument::from(Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("0.00095479193842432214").unwrap(),
                                        ))],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                {
                                    let bytes = crate::source_format::format(
                                        "M_Jupiter best  = 0.000954791915(11) (Solar masses)\n",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                diff = (((context.pulsars[(p) as usize].param.slots
                                    [(param_dmassplanet as i32) as usize]
                                    .cells[(4i32) as usize]
                                    .val)
                                    .clone()
                                    + Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("0.00095479193842432214").unwrap(),
                                    ))
                                    - Float::with_val(PRECISION_BITS, 0.000954791915f64));
                                err = Float::with_val(
                                    PRECISION_BITS,
                                    ((((context.pulsars[(p) as usize].param.slots
                                        [(param_dmassplanet as i32) as usize]
                                        .cells[(4i32) as usize]
                                        .err)
                                        .clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                        + (Float::with_val(PRECISION_BITS, 0.000000000011f64))
                                            .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                    .to_f64())
                                    .sqrt(),
                                );
                                {
                                    let bytes = crate::source_format::format(
                                        "diff            = %.15Lg +/- %.15Lg (Solar masses)\n",
                                        &[
                                            Argument::from((diff).clone()),
                                            Argument::from((err).clone()),
                                        ],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            if (read_string(
                                &context.pulsars[(p) as usize].JPL_EPHEMERIS[..],
                                "psr[p].JPL_EPHEMERIS",
                            )?)
                            .windows(("DE200").len())
                            .any(|w| w == ("DE200").as_bytes())
                            {
                                {
                                    {
                                        let bytes = crate::source_format::format(
                                            "DE200 Jupiter mass not set\n",
                                            &[],
                                        )?;
                                        io.write(&bytes)?;
                                    }
                                }
                            }
                        }
                    }
                }
                if (((*read(
                    &context.pulsars[(p) as usize].quad_ifuncN_p,
                    "psr[p].quad_ifuncN_p",
                )? > 0i32)
                    || (context.pulsars[(p) as usize].param.slots
                        [(param_gwm_amp as i32) as usize]
                        .cells[(1i32) as usize]
                        .param_set
                        == 1i32))
                    || (context.pulsars[(p) as usize].param.slots
                        [(param_gwcs_amp as i32) as usize]
                        .cells[(1i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        {
                            let bytes = crate::source_format::format(
                                "Geometrical factor for pulsar %d (%s) = %g %g\n",
                                &[
                                    Argument::from(p),
                                    Argument::from(read_string(
                                        &context.pulsars[(p) as usize].name[..],
                                        "psr[p].name",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].quad_ifunc_geom_p,
                                        "psr[p].quad_ifunc_geom_p",
                                    )?),
                                    Argument::from(*read(
                                        &context.pulsars[(p) as usize].quad_ifunc_geom_c,
                                        "psr[p].quad_ifunc_geom_c",
                                    )?),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
                {
                    let mut start: Option<f64> = None;
                    let mut end: Option<f64> = None;
                    let mut go: i32 = 0i32;
                    {
                        i = 0i32;
                        while (i < *read(&context.pulsars[(p) as usize].nobs, "psr[p].nobs")?) {
                            {
                                if (*read(
                                    &output_rows(context, (p) as usize)?[(i) as usize].deleted,
                                    "psr[p].obsn[i].deleted",
                                )? == 0i32)
                                {
                                    {
                                        if (go == 0i32) {
                                            {
                                                go = 1i32;
                                                start = Some(
                                                    ((read(
                                                        &output_rows(context, (p) as usize)?
                                                            [(i) as usize]
                                                            .sat,
                                                        "psr[p].obsn[i].sat",
                                                    )?)
                                                    .clone())
                                                    .to_f64(),
                                                );
                                                end = Some(*read(&start, "start")?);
                                            }
                                        } else {
                                            {
                                                if (*read(&start, "start")?
                                                    > ((read(
                                                        &output_rows(context, (p) as usize)?
                                                            [(i) as usize]
                                                            .sat,
                                                        "psr[p].obsn[i].sat",
                                                    )?)
                                                    .clone())
                                                    .to_f64())
                                                {
                                                    start = Some(
                                                        ((read(
                                                            &output_rows(context, (p) as usize)?
                                                                [(i) as usize]
                                                                .sat,
                                                            "psr[p].obsn[i].sat",
                                                        )?)
                                                        .clone())
                                                        .to_f64(),
                                                    );
                                                }
                                                if (*read(&end, "end")?
                                                    < ((read(
                                                        &output_rows(context, (p) as usize)?
                                                            [(i) as usize]
                                                            .sat,
                                                        "psr[p].obsn[i].sat",
                                                    )?)
                                                    .clone())
                                                    .to_f64())
                                                {
                                                    end = Some(
                                                        ((read(
                                                            &output_rows(context, (p) as usize)?
                                                                [(i) as usize]
                                                                .sat,
                                                            "psr[p].obsn[i].sat",
                                                        )?)
                                                        .clone())
                                                        .to_f64(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            i = (i + 1);
                        }
                    }
                    {
                        let bytes = crate::source_format::format(
                            "Total time span = %.3f days = %.3f years\n",
                            &[
                                Argument::from((*read(&end, "end")? - *read(&start, "start")?)),
                                Argument::from(
                                    ((*read(&end, "end")? - *read(&start, "start")?) / 365.25f64),
                                ),
                            ],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                {
                    let bytes = crate::source_format::format("\nTempo2 usage\n", &[])?;
                    io.write(&bytes)?;
                }
                if (*read(&context.pulsars[(p) as usize].units, "psr[p].units")? == 2i32) {
                    {
                        let bytes = crate::source_format::format(
                            "Units:                 TDB (tempo1)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                } else {
                    {
                        let bytes = crate::source_format::format(
                            "Units:                 TCB (tempo2)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                if (*read(
                    &context.pulsars[(p) as usize].timeEphemeris,
                    "psr[p].timeEphemeris",
                )? == 2i32)
                {
                    {
                        let bytes = crate::source_format::format(
                            "Time ephemeris:        FB90 (tempo1)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                } else {
                    {
                        let bytes = crate::source_format::format(
                            "Time ephemeris:        IF99 (tempo2)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                if (*read(
                    &context.pulsars[(p) as usize].correctTroposphere,
                    "psr[p].correctTroposphere",
                )? == 0i32)
                {
                    {
                        let bytes = crate::source_format::format(
                            "Troposphere corr.?     No (tempo1)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                } else {
                    {
                        let bytes = crate::source_format::format(
                            "Troposphere corr.?     Yes (tempo2)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                if (*read(
                    &context.pulsars[(p) as usize].dilateFreq,
                    "psr[p].dilateFreq",
                )? == 0i32)
                {
                    {
                        let bytes = crate::source_format::format(
                            "Dilate freq?           No (tempo1)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                } else {
                    {
                        let bytes = crate::source_format::format(
                            "Dilate freq?           Yes (tempo2)\n",
                            &[],
                        )?;
                        io.write(&bytes)?;
                    }
                }
                {
                    let bytes = crate::source_format::format(
                        "Electron density (1AU) %g\n",
                        &[Argument::from(*read(
                            &context.pulsars[(p) as usize].ne_sw,
                            "psr[p].ne_sw",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "Solar system ephem     %s\n",
                        &[Argument::from(read_string(
                            &context.pulsars[(p) as usize].ephemeris[..],
                            "psr[p].ephemeris",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "Time scale             %s\n",
                        &[Argument::from(read_string(
                            &context.pulsars[(p) as usize].clock[..],
                            "psr[p].clock",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                {
                    let bytes = crate::source_format::format(
                        "Binary model           %s\n",
                        &[Argument::from(read_string(
                            &context.pulsars[(p) as usize].binaryModel[..],
                            "psr[p].binaryModel",
                        )?)],
                    )?;
                    io.write(&bytes)?;
                }
                if (*read(&context.pulsars[(p) as usize].setUnits, "psr[p].setUnits")? == 0i32) {
                    {
                        if (*read(&context.pulsars[(p) as usize].units, "psr[p].units")? != 1i32) {
                            d.log(
                                b'w',
                                "textOutput.C",
                                1155,
                                &crate::source_format::format(
                                    "UNITS was not set in the parameter file: using TDB (tempo1)",
                                    &[],
                                )?,
                            );
                            io.drain(d)?;
                        } else {
                            d.log(
                                b'w',
                                "textOutput.C",
                                1157,
                                &crate::source_format::format(
                                    "UNITS was not set in the parameter file: using TCB (tempo2)",
                                    &[],
                                )?,
                            );
                            io.drain(d)?;
                        }
                    }
                }
                if (((p == 0i32)
                    && (*read(&context.pulsars[(0i32) as usize].nGlobal, "psr[0].nGlobal")?
                        > 0i32))
                    && (*read(
                        &context.pulsars[(0i32) as usize].globalNfit,
                        "psr[0].globalNfit",
                    )? > 0i32))
                {
                    {
                        let mut fout = io.create(&"global_covar.cvm".as_bytes().to_vec());
                        if !(fout.is_some()) {
                            {
                                d.log(
                                    b'w',
                                    "textOutput.C",
                                    1165,
                                    &crate::source_format::format(
                                        "Unable to open global_covar.cvm for writing",
                                        &[],
                                    )?,
                                );
                                io.drain(d)?;
                            }
                        } else {
                            {
                                let mut ii: i32;
                                let mut jj: i32;
                                {
                                    ii = 0i32;
                                    while (ii
                                        < *read(
                                            &context.pulsars[(0i32) as usize].globalNfit,
                                            "psr[0].globalNfit",
                                        )?)
                                    {
                                        {
                                            {
                                                jj = 0i32;
                                                while (jj
                                                    < *read(
                                                        &context.pulsars[(0i32) as usize]
                                                            .globalNfit,
                                                        "psr[0].globalNfit",
                                                    )?)
                                                {
                                                    {
                                                        if ((output_covar(
                                                            context,
                                                            (0i32) as usize,
                                                            (ii) as usize,
                                                            (ii) as usize,
                                                        )? == (0i32) as f64)
                                                            || (output_covar(
                                                                context,
                                                                (0i32) as usize,
                                                                (jj) as usize,
                                                                (jj) as usize,
                                                            )? == (0i32) as f64))
                                                        {
                                                            {
                                                                {
                                                                    let bytes = crate::source_format::format("Diagonal element of covariance matrix = 0\n", &[])?;
                                                                    io.write(&bytes)?;
                                                                }
                                                                write_file(
                                                                    &mut fout,
                                                                    &crate::source_format::format(
                                                                        "%d %d nan\n",
                                                                        &[
                                                                            Argument::from(ii),
                                                                            Argument::from(jj),
                                                                        ],
                                                                    )?,
                                                                )?;
                                                            }
                                                        } else {
                                                            write_file(
                                                                &mut fout,
                                                                &crate::source_format::format(
                                                                    "%d %d %g\n",
                                                                    &[
                                                                        Argument::from(ii),
                                                                        Argument::from(jj),
                                                                        Argument::from(
                                                                            (output_covar(
                                                                                context,
                                                                                (0i32) as usize,
                                                                                (ii) as usize,
                                                                                (jj) as usize,
                                                                            )? / (output_covar(
                                                                                context,
                                                                                (0i32) as usize,
                                                                                (ii) as usize,
                                                                                (ii) as usize,
                                                                            )?
                                                                                * output_covar(
                                                                                    context,
                                                                                    (0i32) as usize,
                                                                                    (jj) as usize,
                                                                                    (jj) as usize,
                                                                                )?)
                                                                            .sqrt()),
                                                                        ),
                                                                    ],
                                                                )?,
                                                            )?;
                                                        }
                                                    }
                                                    jj = (jj + 1);
                                                }
                                            }
                                            write_file(
                                                &mut fout,
                                                &crate::source_format::format("\n", &[])?,
                                            )?;
                                        }
                                        ii = (ii + 1);
                                    }
                                }
                                {
                                    jj = 0i32;
                                    while (jj
                                        < *read(
                                            &context.pulsars[(0i32) as usize].globalNfit,
                                            "psr[0].globalNfit",
                                        )?)
                                    {
                                        {
                                            write_file(&mut fout, &crate::source_format::format("# %d %d %d %s", &[Argument::from(jj), Argument::from(*read(&context.pulsars[(0i32) as usize].fitinfo.output.indexParam[(jj) as usize], "psr[0].fitinfo.output.indexParam[jj]")?), Argument::from(*read(&context.pulsars[(0i32) as usize].fitinfo.output.indexCounter[(jj) as usize], "psr[0].fitinfo.output.indexCounter[jj]")?), Argument::from(crate::core_output::LABEL_STR[(*read(&context.pulsars[(0i32) as usize].fitinfo.output.indexParam[(jj) as usize], "psr[0].fitinfo.output.indexParam[jj]")?) as usize].as_bytes().to_vec())])?)?;
                                            if (*read(
                                                &context.pulsars[(0i32) as usize]
                                                    .fitinfo
                                                    .output
                                                    .indexPsr
                                                    [(jj) as usize],
                                                "psr[0].fitinfo.output.indexPsr[jj]",
                                            )? == (-1i32))
                                            {
                                                {
                                                    write_file(
                                                        &mut fout,
                                                        &crate::source_format::format(
                                                            " GLOBAL",
                                                            &[],
                                                        )?,
                                                    )?;
                                                }
                                            } else {
                                                {
                                                    write_file(&mut fout, &crate::source_format::format(" %s", &[Argument::from(read_string(&context.pulsars[(*read(&context.pulsars[(0i32) as usize].fitinfo.output.indexPsr[(jj) as usize], "psr[0].fitinfo.output.indexPsr[jj]")?) as usize].name[..], "psr[psr[0].fitinfo.output.indexPsr[jj]].name")?)])?)?;
                                                }
                                            }
                                            write_file(
                                                &mut fout,
                                                &crate::source_format::format("\n", &[])?,
                                            )?;
                                        }
                                        jj = (jj + 1);
                                    }
                                }
                                fout.take();
                            }
                        }
                    }
                }
                if ((context.pulsars[(p) as usize].param.slots
                    [(param_quad_ifunc_p as i32) as usize]
                    .cells[(0i32) as usize]
                    .param_set
                    == 1i32)
                    || (context.pulsars[(p) as usize].param.slots
                        [(param_quad_ifunc_c as i32) as usize]
                        .cells[(0i32) as usize]
                        .param_set
                        == 1i32))
                {
                    {
                        if (p == 0i32) {
                            {
                                let mut fout = io.create(&"aplus_across.cvm".as_bytes().to_vec());
                                if !(fout.is_some()) {
                                    {
                                        d.log(
                                            b'w',
                                            "textOutput.C",
                                            1208,
                                            &crate::source_format::format(
                                                "Unable to open aplus_across.cvm for writing",
                                                &[],
                                            )?,
                                        );
                                        io.drain(d)?;
                                    }
                                } else {
                                    {
                                        let mut ii: i32;
                                        let mut jj: i32;
                                        {
                                            ii = 0i32;
                                            while (ii
                                                < *read(
                                                    &context.pulsars[(0i32) as usize].globalNfit,
                                                    "psr[0].globalNfit",
                                                )?)
                                            {
                                                {
                                                    {
                                                        jj = 0i32;
                                                        while (jj
                                                            < *read(
                                                                &context.pulsars[(0i32) as usize]
                                                                    .globalNfit,
                                                                "psr[0].globalNfit",
                                                            )?)
                                                        {
                                                            {
                                                                if (((*read(
                                                                    &context.pulsars
                                                                        [(0i32) as usize]
                                                                        .fitinfo
                                                                        .output
                                                                        .indexParam
                                                                        [(ii) as usize],
                                                                    "psr[0].fitinfo.output.indexParam[ii]",
                                                                )? == (param_quad_ifunc_p
                                                                    as i32))
                                                                    || (*read(
                                                                        &context.pulsars
                                                                            [(0i32) as usize]
                                                                            .fitinfo
                                                                            .output
                                                                            .indexParam
                                                                            [(ii) as usize],
                                                                        "psr[0].fitinfo.output.indexParam[ii]",
                                                                    )? == (param_quad_ifunc_c
                                                                        as i32)))
                                                                    && ((*read(
                                                                        &context.pulsars
                                                                            [(0i32) as usize]
                                                                            .fitinfo
                                                                            .output
                                                                            .indexParam
                                                                            [(jj) as usize],
                                                                        "psr[0].fitinfo.output.indexParam[jj]",
                                                                    )? == (param_quad_ifunc_p
                                                                        as i32))
                                                                        || (*read(
                                                                            &context.pulsars
                                                                                [(0i32) as usize]
                                                                                .fitinfo
                                                                                .output
                                                                                .indexParam
                                                                                [(jj) as usize],
                                                                            "psr[0].fitinfo.output.indexParam[jj]",
                                                                        )?
                                                                            == (param_quad_ifunc_c
                                                                                as i32))))
                                                                {
                                                                    {
                                                                        write_file(&mut fout, &crate::source_format::format("%d %d %.15g\n", &[Argument::from(ii), Argument::from(jj), Argument::from(output_covar(context, (0i32) as usize, (ii) as usize, (jj) as usize)?)])?)?;
                                                                    }
                                                                }
                                                            }
                                                            jj = (jj + 1);
                                                        }
                                                    }
                                                }
                                                ii = (ii + 1);
                                            }
                                        }
                                        write_file(
                                            &mut fout,
                                            &crate::source_format::format(
                                                "# globalNfit = %d\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(0i32) as usize].globalNfit,
                                                    "psr[0].globalNfit",
                                                )?)],
                                            )?,
                                        )?;
                                        fout.take();
                                    }
                                }
                            }
                        }
                    }
                }
                if (newpar == 1i32) {
                    {
                        let mut fout2 = None::<std::fs::File>;
                        let mut fname2: Vec<u8> = Vec::new();
                        let mut str1: Vec<u8> = Vec::new();
                        let mut str2: Vec<u8> = Vec::new();
                        let mut str3: Vec<u8> = Vec::new();
                        let mut str4: Vec<u8> = Vec::new();
                        let mut str5: Vec<u8> = Vec::new();
                        {
                            let bytes = crate::source_format::format(
                                "In here writing a new parameter file: %s\n",
                                &[Argument::from(fname.to_vec())],
                            )?;
                            io.write(&bytes)?;
                        }
                        if ((fname.to_vec()).len() as i32 == 0i32) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Enter filename for new parameter file ",
                                        &[],
                                    )?;
                                    io.write(&bytes)?;
                                }
                                fname2 = io.input_token()?;
                            }
                        } else {
                            {
                                if (context.pulsars.len() as i32 > 1i32) {
                                    fname2 = crate::source_format::format(
                                        "%s_%d",
                                        &[
                                            Argument::from(fname.to_vec()),
                                            Argument::from((p + 1i32)),
                                        ],
                                    )?;
                                } else {
                                    {
                                        fname2 = fname.to_vec();
                                    }
                                }
                            }
                        }
                        fout2 = io.create(&fname2.clone());
                        if !(fout2.is_some()) {
                            {
                                {
                                    let bytes = crate::source_format::format(
                                        "Sorry, unable to write to file %s\n",
                                        &[Argument::from(fname2.clone())],
                                    )?;
                                    io.write(&bytes)?;
                                }
                            }
                        } else {
                            {
                                write_file(
                                    &mut fout2,
                                    &crate::source_format::format(
                                        "%-15.15s%s\n",
                                        &[
                                            Argument::from("PSRJ"),
                                            Argument::from(read_string(
                                                &context.pulsars[(p) as usize].name[..],
                                                "psr[p].name",
                                            )?),
                                        ],
                                    )?,
                                )?;
                                {
                                    i = 0i32;
                                    while (i < crate::tempo2::MAX_PARAMS as i32) {
                                        {
                                            {
                                                k = 0i32;
                                                while (k < *read(
                                                    &context.pulsars[(p) as usize].param.slots
                                                        [(i) as usize]
                                                        .a_size,
                                                    "psr[p].param[i].aSize",
                                                )?) {
                                                    {
                                                        if ((((((context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(i) as usize]
                                                            .cells
                                                            [(k) as usize]
                                                            .param_set
                                                            == 1i32)
                                                            && (i != (param_wave_om as i32)))
                                                            && (i != (param_wave_dm as i32)))
                                                            && (i != (param_waveepoch as i32)))
                                                            && (i != (param_ifunc as i32)))
                                                            && (i != (param_dmmodel as i32)))
                                                        {
                                                            {
                                                                if ((i32::from(
                                                                    crate::clock_text::c_bytes(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .param
                                                                            .slots
                                                                            [(i) as usize]
                                                                            .cells
                                                                            [(k) as usize]
                                                                            .shortlabel,
                                                                    )
                                                                    .to_vec()
                                                                        != "PB".as_bytes(),
                                                                ) == 0i32)
                                                                    || (i32::from(
                                                                        crate::clock_text::c_bytes(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .param
                                                                                .slots
                                                                                [(i) as usize]
                                                                                .cells
                                                                                [(k) as usize]
                                                                                .shortlabel,
                                                                        )
                                                                        .to_vec()
                                                                            != "FB0".as_bytes(),
                                                                    ) == 0i32))
                                                                {
                                                                    write_file(&mut fout2, &crate::source_format::format("%-15.15s%s\n", &[Argument::from("BINARY"), Argument::from(read_string(&context.pulsars[(p) as usize].binaryModel[..], "psr[p].binaryModel")?)])?)?;
                                                                }
                                                                if ((i == (param_raj as i32))
                                                                    && (*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .eclCoord,
                                                                        "psr[p].eclCoord",
                                                                    )? == 1i32))
                                                                {
                                                                    write_file(&mut fout2, &crate::source_format::format("%-15.15s", &[Argument::from("ELONG")])?)?;
                                                                } else {
                                                                    if ((i == (param_decj as i32))
                                                                        && (*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .eclCoord,
                                                                            "psr[p].eclCoord",
                                                                        )? == 1i32))
                                                                    {
                                                                        write_file(&mut fout2, &crate::source_format::format("%-15.15s", &[Argument::from("ELAT")])?)?;
                                                                    } else {
                                                                        if ((i
                                                                            == (param_pmra as i32))
                                                                            && (*read(
                                                                                &context.pulsars
                                                                                    [(p) as usize]
                                                                                    .eclCoord,
                                                                                "psr[p].eclCoord",
                                                                            )? == 1i32))
                                                                        {
                                                                            write_file(&mut fout2, &crate::source_format::format("%-15.15s", &[Argument::from("PMELONG")])?)?;
                                                                        } else {
                                                                            if ((i == (param_pmdec as i32)) && (*read(&context.pulsars[(p) as usize].eclCoord, "psr[p].eclCoord")? == 1i32)) {
write_file(&mut fout2, &crate::source_format::format("%-15.15s", &[Argument::from("PMELAT")])?)?;
} else {
{
if ((i == (param_track as i32)) && ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone() == Float::with_val(PRECISION_BITS, 0i32))) {
{
}
} else {
write_file(&mut fout2, &crate::source_format::format("%-15.15s", &[Argument::from(crate::clock_text::c_bytes(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].shortlabel).to_vec())])?)?;
}
}
}
                                                                        }
                                                                    }
                                                                }
                                                                if ((*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .eclCoord,
                                                                    "psr[p].eclCoord",
                                                                )? == 0i32)
                                                                    && (i == (param_raj as i32)))
                                                                {
                                                                    write_file(&mut fout2, &crate::source_format::format("%-25.25s", &[Argument::from(read_string(&context.pulsars[(p) as usize].rajStrPost[..], "psr[p].rajStrPost")?)])?)?;
                                                                } else {
                                                                    if ((*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .eclCoord,
                                                                        "psr[p].eclCoord",
                                                                    )? == 1i32)
                                                                        && (i
                                                                            == (param_raj as i32)))
                                                                    {
                                                                        write_file(&mut fout2, &crate::source_format::format("%-25.25Lf", &[Argument::from((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone() * Float::with_val(PRECISION_BITS, 180.0f64)) / Float::with_val(PRECISION_BITS, rug::float::Constant::Pi)))])?)?;
                                                                    } else {
                                                                        if ((*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .eclCoord,
                                                                            "psr[p].eclCoord",
                                                                        )? == 0i32)
                                                                            && (i
                                                                                == (param_decj
                                                                                    as i32)))
                                                                        {
                                                                            write_file(&mut fout2, &crate::source_format::format("%-25.25s", &[Argument::from(read_string(&context.pulsars[(p) as usize].decjStrPost[..], "psr[p].decjStrPost")?)])?)?;
                                                                        } else {
                                                                            if ((*read(
                                                                                &context.pulsars
                                                                                    [(p) as usize]
                                                                                    .eclCoord,
                                                                                "psr[p].eclCoord",
                                                                            )? == 1i32)
                                                                                && (i
                                                                                    == (param_decj
                                                                                        as i32)))
                                                                            {
                                                                                write_file(&mut fout2, &crate::source_format::format("%-25.25Lf", &[Argument::from((((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone() * Float::with_val(PRECISION_BITS, 180.0f64)) / Float::with_val(PRECISION_BITS, rug::float::Constant::Pi)))])?)?;
                                                                            } else {
                                                                                if ((i == (param_sini as i32)) && (*read(&context.pulsars[(p) as usize].param.slots[(i) as usize].n_link_to, "psr[p].param[i].nLinkTo")? > 0i32)) {
{
write_file(&mut fout2, &crate::source_format::format(" KIN\n", &[])?)?;
write_file(&mut fout2, &crate::source_format::format("#SINI\t\t%-25.20Lg", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone())])?)?;
}
} else {
if (i == (param_tres as i32)) {
write_file(&mut fout2, &crate::source_format::format("%-10.3Lf", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone())])?)?;
} else {
if (i == (param_tzrfrq as i32)) {
{
write_file(&mut fout2, &crate::source_format::format("%-25.20Lg\n", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone())])?)?;
if (i32::from(read_string(&context.pulsars[(p) as usize].tzrsite[..], "psr[p].tzrsite")? != "NULL".as_bytes()) != 0i32) {
write_file(&mut fout2, &crate::source_format::format("%-15.15s%s", &[Argument::from("TZRSITE"), Argument::from(read_string(&context.pulsars[(p) as usize].tzrsite[..], "psr[p].tzrsite")?)])?)?;
}
}
} else {
{
if ((i == (param_track as i32)) && ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone() == Float::with_val(PRECISION_BITS, 0i32))) {
{
}
} else {
write_file(&mut fout2, &crate::source_format::format("%-25.20Lg", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].val).clone())])?)?;
}
}
}
}
}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if (context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(i) as usize]
                                                                    .cells
                                                                    [(k) as usize]
                                                                    .fit_flag
                                                                    == 1i32)
                                                                {
                                                                    {
                                                                        write_file(&mut fout2, &crate::source_format::format(" 1 ", &[])?)?;
                                                                        if (i == (param_raj as i32))
                                                                        {
                                                                            {
                                                                                let mut fac: f64 = (((12.0f64 * 60.0f64) * 60.0f64) / std::f64::consts::PI);
                                                                                if (*read(&context.pulsars[(p) as usize].eclCoord, "psr[p].eclCoord")? == 1i32) {
fac = (180.0f64 / std::f64::consts::PI);
}
                                                                                if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
} else {
if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
}
}
                                                                            }
                                                                        } else {
                                                                            if (i
                                                                                == (param_decj
                                                                                    as i32))
                                                                            {
                                                                                {
                                                                                    let mut fac: f64 = (((180.0f64 * 60.0f64) * 60.0f64) / std::f64::consts::PI);
                                                                                    if (*read(&context.pulsars[(p) as usize].eclCoord, "psr[p].eclCoord")? == 1i32) {
fac = (180.0f64 / std::f64::consts::PI);
}
                                                                                    if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
} else {
if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from(((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
}
}
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone())])?)?;
} else {
if ((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from((context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].err).clone())])?)?;
}
}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        if (((i
                                                                            == (param_start
                                                                                as i32))
                                                                            || (i
                                                                                == (param_finish
                                                                                    as i32)))
                                                                            && (context.pulsars
                                                                                [(p) as usize]
                                                                                .param
                                                                                .slots
                                                                                [(i) as usize]
                                                                                .cells
                                                                                [(k) as usize]
                                                                                .fit_flag
                                                                                == 2i32))
                                                                        {
                                                                            write_file(&mut fout2, &crate::source_format::format(" 2 ", &[])?)?;
                                                                        }
                                                                        write_file(&mut fout2, &crate::source_format::format("   ", &[])?)?;
                                                                        if (i == (param_raj as i32))
                                                                        {
                                                                            {
                                                                                let mut fac: f64 = (((12.0f64 * 60.0f64) * 60.0f64) / std::f64::consts::PI);
                                                                                if (*read(&context.pulsars[(p) as usize].eclCoord, "psr[p].eclCoord")? == 1i32) {
fac = (180.0f64 / std::f64::consts::PI);
}
                                                                                if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
} else {
if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
}
}
                                                                            }
                                                                        } else {
                                                                            if (i
                                                                                == (param_decj
                                                                                    as i32))
                                                                            {
                                                                                {
                                                                                    let mut fac: f64 = (((180.0f64 * 60.0f64) * 60.0f64) / std::f64::consts::PI);
                                                                                    if (*read(&context.pulsars[(p) as usize].eclCoord, "psr[p].eclCoord")? == 1i32) {
fac = (180.0f64 / std::f64::consts::PI);
}
                                                                                    if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
} else {
if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from(((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() * Float::with_val(PRECISION_BITS, fac)))])?)?;
}
}
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 1e-12f64)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lf", &[Argument::from((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone())])?)?;
} else {
if ((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
write_file(&mut fout2, &crate::source_format::format(" %-25.20Lg", &[Argument::from((read(&context.pulsars[(p) as usize].param.slots[(i) as usize].cells[(k) as usize].prefit_err, "psr[p].param[i].prefitErr[k]")?).clone())])?)?;
}
}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                write_file(
                                                                    &mut fout2,
                                                                    &crate::source_format::format(
                                                                        "\n",
                                                                        &[],
                                                                    )?,
                                                                )?;
                                                            }
                                                        }
                                                    }
                                                    k = (k + 1);
                                                }
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                if (*read(&context.pulsars[(p) as usize].tempo1, "psr[p].tempo1")?
                                    == 1i32)
                                {
                                    {
                                        if !(i32::from(
                                            read_string(
                                                &context.pulsars[(p) as usize].clock[..],
                                                "psr[p].clock",
                                            )? != "TT(UTC(NIST))".as_bytes(),
                                        ) != 0)
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "%-15.15s%s\n",
                                                    &[
                                                        Argument::from("CLK"),
                                                        Argument::from("UTC(NIST)"),
                                                    ],
                                                )?,
                                            )?;
                                        } else {
                                            if !(i32::from(
                                                read_string(
                                                    &context.pulsars[(p) as usize].clock[..],
                                                    "psr[p].clock",
                                                )? != "TT(TAI))".as_bytes(),
                                            ) != 0)
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "%-15.15s%s\n",
                                                        &[
                                                            Argument::from("CLK"),
                                                            Argument::from("UTC(BIPM)"),
                                                        ],
                                                    )?,
                                                )?;
                                            } else {
                                                if !(i32::from(
                                                    read_string(
                                                        &context.pulsars[(p) as usize].clock[..],
                                                        "psr[p].clock",
                                                    )? != "TT(UTC(PTB))".as_bytes(),
                                                ) != 0)
                                                {
                                                    write_file(
                                                        &mut fout2,
                                                        &crate::source_format::format(
                                                            "%-15.15s%s\n",
                                                            &[
                                                                Argument::from("CLK"),
                                                                Argument::from("PTB"),
                                                            ],
                                                        )?,
                                                    )?;
                                                } else {
                                                    if !(i32::from(
                                                        read_string(
                                                            &context.pulsars[(p) as usize].clock[..],
                                                            "psr[p].clock",
                                                        )? != "TT(TA(NIST))".as_bytes(),
                                                    ) != 0)
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "%-15.15s%s\n",
                                                                &[
                                                                    Argument::from("CLK"),
                                                                    Argument::from("AT1"),
                                                                ],
                                                            )?,
                                                        )?;
                                                    } else {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "%-15.15s%s\n",
                                                                &[
                                                                    Argument::from("CLK"),
                                                                    Argument::from(read_string(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .clock[..],
                                                                        "psr[p].clock",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[
                                                Argument::from("CLK"),
                                                Argument::from(read_string(
                                                    &context.pulsars[(p) as usize].clock[..],
                                                    "psr[p].clock",
                                                )?),
                                            ],
                                        )?,
                                    )?;
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].fitMode,
                                    "psr[p].fitMode",
                                )? == 1i32)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format("MODE 1\n", &[])?,
                                    )?;
                                }
                                if (*read(&context.pulsars[(p) as usize].units, "psr[p].units")?
                                    != 1i32)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("UNITS"), Argument::from("TDB")],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("UNITS"), Argument::from("TCB")],
                                        )?,
                                    )?;
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].timeEphemeris,
                                    "psr[p].timeEphemeris",
                                )? != 1i32)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("TIMEEPH"), Argument::from("FB90")],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("TIMEEPH"), Argument::from("IF99")],
                                        )?,
                                    )?;
                                }
                                if !(*read(
                                    &context.pulsars[(p) as usize].dilateFreq,
                                    "psr[p].dilateFreq",
                                )? != 0)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("DILATEFREQ"), Argument::from("N")],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("DILATEFREQ"), Argument::from("Y")],
                                        )?,
                                    )?;
                                }
                                if !(*read(
                                    &context.pulsars[(p) as usize].planetShapiro,
                                    "psr[p].planetShapiro",
                                )? != 0)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[
                                                Argument::from("PLANET_SHAPIRO"),
                                                Argument::from("N"),
                                            ],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[
                                                Argument::from("PLANET_SHAPIRO"),
                                                Argument::from("Y"),
                                            ],
                                        )?,
                                    )?;
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].t2cMethod,
                                    "psr[p].t2cMethod",
                                )? != 1i32)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[Argument::from("T2CMETHOD"), Argument::from("TEMPO")],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-15.15s%s\n",
                                            &[
                                                Argument::from("T2CMETHOD"),
                                                Argument::from("IAU2000B"),
                                            ],
                                        )?,
                                    )?;
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_ne_sw as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 0i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "%-15.15s%.3f\n",
                                                &[
                                                    Argument::from("NE_SW"),
                                                    Argument::from(*read(
                                                        &context.pulsars[(p) as usize].ne_sw,
                                                        "psr[p].ne_sw",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                }
                                if !(*read(
                                    &context.pulsars[(p) as usize].correctTroposphere,
                                    "psr[p].correctTroposphere",
                                )? != 0)
                                {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-21.21s%s\n",
                                            &[
                                                Argument::from("CORRECT_TROPOSPHERE"),
                                                Argument::from("N"),
                                            ],
                                        )?,
                                    )?;
                                } else {
                                    write_file(
                                        &mut fout2,
                                        &crate::source_format::format(
                                            "%-21.21s%s\n",
                                            &[
                                                Argument::from("CORRECT_TROPOSPHERE"),
                                                Argument::from("Y"),
                                            ],
                                        )?,
                                    )?;
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].useCalceph,
                                    "psr[p].useCalceph",
                                )? == 0i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "%-15.15s%s\n",
                                                &[
                                                    Argument::from("EPHEM"),
                                                    Argument::from(read_string(
                                                        &context.pulsars[(p) as usize].ephemeris[..],
                                                        "psr[p].ephemeris",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                } else {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "%-15.15s%s\n",
                                                &[
                                                    Argument::from("EPH_FILE"),
                                                    Argument::from(read_string(
                                                        &context.pulsars[(p) as usize].ephemeris[..],
                                                        "psr[p].ephemeris",
                                                    )?),
                                                ],
                                            )?,
                                        )?;
                                    }
                                }
                                if ((*read(&context.pulsars[(p) as usize].refphs, "psr[p].refphs")?)
                                    as i32
                                    == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "%-15.15s%s\n",
                                                &[Argument::from("REFPHS"), Argument::from("TZR")],
                                            )?,
                                        )?;
                                    }
                                }
                                write_file(
                                    &mut fout2,
                                    &crate::source_format::format(
                                        "%-15.15s%s\n",
                                        &[Argument::from("NITS"), Argument::from("1")],
                                    )?,
                                )?;
                                write_file(
                                    &mut fout2,
                                    &crate::source_format::format(
                                        "%-15.15s%d\n",
                                        &[
                                            Argument::from("NTOA"),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].nFit,
                                                "psr[p].nFit",
                                            )?),
                                        ],
                                    )?,
                                )?;
                                write_file(
                                    &mut fout2,
                                    &crate::source_format::format(
                                        "%-15.15s%.4f %d\n",
                                        &[
                                            Argument::from("CHI2R"),
                                            Argument::from(
                                                (chisqr
                                                    / (*read(
                                                        &context.pulsars[(p) as usize].fitNfree,
                                                        "psr[p].fitNfree",
                                                    )?)
                                                        as f64),
                                            ),
                                            Argument::from(*read(
                                                &context.pulsars[(p) as usize].fitNfree,
                                                "psr[p].fitNfree",
                                            )?),
                                        ],
                                    )?,
                                )?;
                                {
                                    i = 1i32;
                                    while (i
                                        <= *read(
                                            &context.pulsars[(p) as usize].nJumps,
                                            "psr[p].nJumps",
                                        )?)
                                    {
                                        {
                                            let tokens = crate::clock_text::tokens(&read_string(
                                                &context.pulsars[(p) as usize].jumpStr
                                                    [(i) as usize][..],
                                                "psr[p].jumpStr[i]",
                                            )?)
                                            .map(|s| s.to_vec())
                                            .collect::<Vec<_>>();
                                            if let Some(v) = tokens.get(0) {
                                                str1 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(1) {
                                                str2 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(2) {
                                                str3 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(3) {
                                                str4 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(4) {
                                                str5 = v.clone();
                                            }
                                            if ((i32::from(
                                                !(str1.clone())
                                                    .eq_ignore_ascii_case(&"FREQ".as_bytes()),
                                            ) == 0i32)
                                                || (i32::from(
                                                    !(str1.clone())
                                                        .eq_ignore_ascii_case(&"MJD".as_bytes()),
                                                ) == 0i32))
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "JUMP %s %s %s %.14g %d\n",
                                                        &[
                                                            Argument::from(str1.clone()),
                                                            Argument::from(str2.clone()),
                                                            Argument::from(str3.clone()),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .jumpVal
                                                                    [(i) as usize],
                                                                "psr[p].jumpVal[i]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .fitJump
                                                                    [(i) as usize],
                                                                "psr[p].fitJump[i]",
                                                            )?),
                                                        ],
                                                    )?,
                                                )?;
                                            } else {
                                                if ((((i32::from(
                                                    !(str1.clone())
                                                        .eq_ignore_ascii_case(&"NAME".as_bytes()),
                                                ) == 0i32)
                                                    || (i32::from(
                                                        !(str1.clone()).eq_ignore_ascii_case(
                                                            &"TEL".as_bytes(),
                                                        ),
                                                    ) == 0i32))
                                                    || (i32::from(
                                                        !(str1.clone()).eq_ignore_ascii_case(
                                                            &"SCALED".as_bytes(),
                                                        ),
                                                    ) == 0i32))
                                                    || (str1[(0i32) as usize] as i32 == 45i32))
                                                {
                                                    write_file(
                                                        &mut fout2,
                                                        &crate::source_format::format(
                                                            "JUMP %s %s %.14g %d\n",
                                                            &[
                                                                Argument::from(str1.clone()),
                                                                Argument::from(str2.clone()),
                                                                Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .jumpVal
                                                                        [(i) as usize],
                                                                    "psr[p].jumpVal[i]",
                                                                )?),
                                                                Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .fitJump
                                                                        [(i) as usize],
                                                                    "psr[p].fitJump[i]",
                                                                )?),
                                                            ],
                                                        )?,
                                                    )?;
                                                }
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                let mut fdjump_format_pint: i32 = (*read(
                                    &context.pulsars[(p) as usize].fdjump_pint_format,
                                    "psr[p].fdjump_pint_format",
                                )?)
                                    as i32;
                                {
                                    i = 1i32;
                                    while (i
                                        <= *read(
                                            &context.pulsars[(p) as usize].nfdJumps,
                                            "psr[p].nfdJumps",
                                        )?)
                                    {
                                        {
                                            let tokens = crate::clock_text::tokens(&read_string(
                                                &context.pulsars[(p) as usize].fdjumpStr
                                                    [(i) as usize][..],
                                                "psr[p].fdjumpStr[i]",
                                            )?)
                                            .map(|s| s.to_vec())
                                            .collect::<Vec<_>>();
                                            if let Some(v) = tokens.get(0) {
                                                str1 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(1) {
                                                str2 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(2) {
                                                str3 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(3) {
                                                str4 = v.clone();
                                            }
                                            if let Some(v) = tokens.get(4) {
                                                str5 = v.clone();
                                            }
                                            if (*read(
                                                &context.pulsars[(p) as usize].fdjumpIdx
                                                    [(i) as usize],
                                                "psr[p].fdjumpIdx[i]",
                                            )? == (-2i32))
                                            {
                                                {
                                                    if ((i32::from(
                                                        !(str1.clone()).eq_ignore_ascii_case(
                                                            &"FREQ".as_bytes(),
                                                        ),
                                                    ) == 0i32)
                                                        || (i32::from(
                                                            !(str1.clone()).eq_ignore_ascii_case(
                                                                &"MJD".as_bytes(),
                                                            ),
                                                        ) == 0i32))
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "FDJUMPDM %s %s %s %.14g %d\n",
                                                                &[
                                                                    Argument::from(str1.clone()),
                                                                    Argument::from(str2.clone()),
                                                                    Argument::from(str3.clone()),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .fdjumpVal
                                                                            [(i) as usize],
                                                                        "psr[p].fdjumpVal[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .fitfdJump
                                                                            [(i) as usize],
                                                                        "psr[p].fitfdJump[i]",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                    } else {
                                                        if (((i32::from(
                                                            !(str1.clone()).eq_ignore_ascii_case(
                                                                &"NAME".as_bytes(),
                                                            ),
                                                        ) == 0i32)
                                                            || (i32::from(
                                                                !(str1.clone())
                                                                    .eq_ignore_ascii_case(
                                                                        &"TEL".as_bytes(),
                                                                    ),
                                                            ) == 0i32))
                                                            || (str1[(0i32) as usize] as i32
                                                                == 45i32))
                                                        {
                                                            write_file(
                                                                &mut fout2,
                                                                &crate::source_format::format(
                                                                    "FDJUMPDM %s %s %.14g %d\n",
                                                                    &[
                                                                        Argument::from(
                                                                            str1.clone(),
                                                                        ),
                                                                        Argument::from(
                                                                            str2.clone(),
                                                                        ),
                                                                        Argument::from(*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .fdjumpVal
                                                                                [(i) as usize],
                                                                            "psr[p].fdjumpVal[i]",
                                                                        )?),
                                                                        Argument::from(*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .fitfdJump
                                                                                [(i) as usize],
                                                                            "psr[p].fitfdJump[i]",
                                                                        )?),
                                                                    ],
                                                                )?,
                                                            )?;
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let mut fdjump_kw: Vec<u8> = Vec::new();
                                                    if (fdjump_format_pint != 0) {
                                                        fdjump_kw = crate::source_format::format(
                                                            "FD%dJUMP",
                                                            &[Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .fdjumpIdx
                                                                    [(i) as usize],
                                                                "psr[p].fdjumpIdx[i]",
                                                            )?)],
                                                        )?;
                                                    } else {
                                                        fdjump_kw = crate::source_format::format(
                                                            "FDJUMP%d",
                                                            &[Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .fdjumpIdx
                                                                    [(i) as usize],
                                                                "psr[p].fdjumpIdx[i]",
                                                            )?)],
                                                        )?;
                                                    }
                                                    if ((i32::from(
                                                        !(str1.clone()).eq_ignore_ascii_case(
                                                            &"FREQ".as_bytes(),
                                                        ),
                                                    ) == 0i32)
                                                        || (i32::from(
                                                            !(str1.clone()).eq_ignore_ascii_case(
                                                                &"MJD".as_bytes(),
                                                            ),
                                                        ) == 0i32))
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "%s %s %s %s %.14g %d\n",
                                                                &[
                                                                    Argument::from(
                                                                        fdjump_kw.clone(),
                                                                    ),
                                                                    Argument::from(str1.clone()),
                                                                    Argument::from(str2.clone()),
                                                                    Argument::from(str3.clone()),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .fdjumpVal
                                                                            [(i) as usize],
                                                                        "psr[p].fdjumpVal[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .fitfdJump
                                                                            [(i) as usize],
                                                                        "psr[p].fitfdJump[i]",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                    } else {
                                                        if (((i32::from(
                                                            !(str1.clone()).eq_ignore_ascii_case(
                                                                &"NAME".as_bytes(),
                                                            ),
                                                        ) == 0i32)
                                                            || (i32::from(
                                                                !(str1.clone())
                                                                    .eq_ignore_ascii_case(
                                                                        &"TEL".as_bytes(),
                                                                    ),
                                                            ) == 0i32))
                                                            || (str1[(0i32) as usize] as i32
                                                                == 45i32))
                                                        {
                                                            write_file(
                                                                &mut fout2,
                                                                &crate::source_format::format(
                                                                    "%s %s %s %.14g %d\n",
                                                                    &[
                                                                        Argument::from(
                                                                            fdjump_kw.clone(),
                                                                        ),
                                                                        Argument::from(
                                                                            str1.clone(),
                                                                        ),
                                                                        Argument::from(
                                                                            str2.clone(),
                                                                        ),
                                                                        Argument::from(*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .fdjumpVal
                                                                                [(i) as usize],
                                                                            "psr[p].fdjumpVal[i]",
                                                                        )?),
                                                                        Argument::from(*read(
                                                                            &context.pulsars
                                                                                [(p) as usize]
                                                                                .fitfdJump
                                                                                [(i) as usize],
                                                                            "psr[p].fitfdJump[i]",
                                                                        )?),
                                                                    ],
                                                                )?,
                                                            )?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].nfdJumps,
                                    "psr[p].nfdJumps",
                                )? > 0i32)
                                {
                                    {
                                        if ((*read(
                                            &context.pulsars[(p) as usize].fdjump_pint_format,
                                            "psr[p].fdjump_pint_format",
                                        )?) as i32
                                            != 0)
                                        {
                                            {
                                                if ((*read(
                                                    &context.pulsars[(p) as usize].fdjump_log,
                                                    "psr[p].fdjump_log",
                                                )?)
                                                    as i32
                                                    != 0)
                                                {
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "FDJUMPLOG Y\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    }
                                                } else {
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "FDJUMPLOG N\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                if ((*read(
                                                    &context.pulsars[(p) as usize].fdjump_log,
                                                    "psr[p].fdjump_log",
                                                )?)
                                                    as i32
                                                    != 0)
                                                {
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "FDJUMP_SCALE LOG\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    }
                                                } else {
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "FDJUMP_SCALE LINEAR\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nT2efac,
                                        "psr[p].nT2efac",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "T2EFAC %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .T2efacFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].T2efacFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .T2efacFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].T2efacFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .T2efacVal
                                                                [(i) as usize],
                                                            "psr[p].T2efacVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nT2equad,
                                        "psr[p].nT2equad",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "T2EQUAD %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .T2equadFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].T2equadFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .T2equadFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].T2equadFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .T2equadVal
                                                                [(i) as usize],
                                                            "psr[p].T2equadVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNGlobalEF,
                                    "psr[p].TNGlobalEF",
                                )? != (0i32) as f64)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNGlobalEF %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNGlobalEF,
                                                    "psr[p].TNGlobalEF",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNGlobalEQ,
                                    "psr[p].TNGlobalEQ",
                                )? != (0i32) as f64)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNGlobalEQ %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNGlobalEQ,
                                                    "psr[p].TNGlobalEQ",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNEF,
                                        "psr[p].nTNEF",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNEF %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNEFFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].TNEFFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNEFFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].TNEFFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize].TNEFVal
                                                                [(i) as usize],
                                                            "psr[p].TNEFVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNEQ,
                                        "psr[p].nTNEQ",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNEQ %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNEQFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].TNEQFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNEQFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].TNEQFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize].TNEQVal
                                                                [(i) as usize],
                                                            "psr[p].TNEQVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNECORR,
                                        "psr[p].nTNECORR",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNECORR %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNECORRFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].TNECORRFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNECORRFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].TNECORRFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNECORRVal
                                                                [(i) as usize],
                                                            "psr[p].TNECORRVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNSECORR,
                                        "psr[p].nTNSECORR",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNSECORR %s %s %g\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNSECORRFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].TNSECORRFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNSECORRFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].TNSECORRFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNSECORRVal
                                                                [(i) as usize],
                                                            "psr[p].TNSECORRVal[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                if ((*read(
                                    &context.pulsars[(p) as usize].TNDMAmp,
                                    "psr[p].TNDMAmp",
                                )? != (0i32) as f64)
                                    && (*read(
                                        &context.pulsars[(p) as usize].TNDMGam,
                                        "psr[p].TNDMGam",
                                    )? != (0i32) as f64))
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNDMAmp %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNDMAmp,
                                                    "psr[p].TNDMAmp",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNDMGam %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNDMGam,
                                                    "psr[p].TNDMGam",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNDMC %i\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNDMC,
                                                    "psr[p].TNDMC",
                                                )?)],
                                            )?,
                                        )?;
                                        if (*read(
                                            &context.pulsars[(p) as usize].TNDM_log_freqs,
                                            "psr[p].TNDM_log_freqs",
                                        )? > 0i32)
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNDMFLog %i\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNDM_log_freqs,
                                                            "psr[p].TNDM_log_freqs",
                                                        )?)],
                                                    )?,
                                                )?;
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNDMFLog_factor %lf\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNDM_log_factor,
                                                            "psr[p].TNDM_log_factor",
                                                        )?)],
                                                    )?,
                                                )?;
                                            }
                                        }
                                    }
                                }
                                if ((*read(
                                    &context.pulsars[(p) as usize].TNChromAmp,
                                    "psr[p].TNChromAmp",
                                )? != (0i32) as f64)
                                    && (*read(
                                        &context.pulsars[(p) as usize].TNChromGam,
                                        "psr[p].TNChromGam",
                                    )? != (0i32) as f64))
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNChromAmp %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNChromAmp,
                                                    "psr[p].TNChromAmp",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNChromGam %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNChromGam,
                                                    "psr[p].TNChromGam",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNChromIdx %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNChromIdx,
                                                    "psr[p].TNChromIdx",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNChromC %i\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNChromC,
                                                    "psr[p].TNChromC",
                                                )?)],
                                            )?,
                                        )?;
                                        if (*read(
                                            &context.pulsars[(p) as usize].TNChrom_log_freqs,
                                            "psr[p].TNChrom_log_freqs",
                                        )? > 0i32)
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNChromFLog %i\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNChrom_log_freqs,
                                                            "psr[p].TNChrom_log_freqs",
                                                        )?)],
                                                    )?,
                                                )?;
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNChromFLog_factor %lf\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNChrom_log_factor,
                                                            "psr[p].TNChrom_log_factor",
                                                        )?)],
                                                    )?,
                                                )?;
                                            }
                                        }
                                    }
                                } else {
                                    if (*read(
                                        &context.pulsars[(p) as usize].TNChromIdx,
                                        "psr[p].TNChromIdx",
                                    )? != (0i32) as f64)
                                    {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "CHROM_INDEX %g\n",
                                                    &[Argument::from(*read(
                                                        &context.pulsars[(p) as usize].TNChromIdx,
                                                        "psr[p].TNChromIdx",
                                                    )?)],
                                                )?,
                                            )?;
                                        }
                                    }
                                }
                                if ((*read(
                                    &context.pulsars[(p) as usize].TNRedAmp,
                                    "psr[p].TNRedAmp",
                                )? != (0i32) as f64)
                                    && (*read(
                                        &context.pulsars[(p) as usize].TNRedGam,
                                        "psr[p].TNRedGam",
                                    )? != (0i32) as f64))
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNRedAmp %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedAmp,
                                                    "psr[p].TNRedAmp",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNRedGam %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedGam,
                                                    "psr[p].TNRedGam",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNRedC %i\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedC,
                                                    "psr[p].TNRedC",
                                                )?)],
                                            )?,
                                        )?;
                                        if (*read(
                                            &context.pulsars[(p) as usize].TNRed_log_freqs,
                                            "psr[p].TNRed_log_freqs",
                                        )? > 0i32)
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNRedFLog %i\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNRed_log_freqs,
                                                            "psr[p].TNRed_log_freqs",
                                                        )?)],
                                                    )?,
                                                )?;
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNRedFLog_factor %lf\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNRed_log_factor,
                                                            "psr[p].TNRed_log_factor",
                                                        )?)],
                                                    )?,
                                                )?;
                                            }
                                        }
                                        if ((*read(
                                            &context.pulsars[(p) as usize].TNRedAmp2,
                                            "psr[p].TNRedAmp2",
                                        )? != (0i32) as f64)
                                            && (*read(
                                                &context.pulsars[(p) as usize].TNRedGam2,
                                                "psr[p].TNRedGam2",
                                            )? != (0i32) as f64))
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNRedAmp2 %g\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNRedAmp2,
                                                            "psr[p].TNRedAmp2",
                                                        )?)],
                                                    )?,
                                                )?;
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "TNRedGam2 %g\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNRedGam2,
                                                            "psr[p].TNRedGam2",
                                                        )?)],
                                                    )?,
                                                )?;
                                            }
                                        }
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TN_QpPeriod,
                                    "psr[p].TN_QpPeriod",
                                )? > (0i32) as f64)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TN_QpPeriod %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TN_QpPeriod,
                                                    "psr[p].TN_QpPeriod",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TN_QpRatio %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TN_QpRatio,
                                                    "psr[p].TN_QpRatio",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TN_QpLam %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TN_QpLam,
                                                    "psr[p].TN_QpLam",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TN_QpSig %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TN_QpSig,
                                                    "psr[p].TN_QpSig",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNRedFLow,
                                    "psr[p].TNRedFLow",
                                )? != (0i32) as f64)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNRedFLow %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].TNRedFLow,
                                                    "psr[p].TNRedFLow",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNGroupNoise,
                                        "psr[p].nTNGroupNoise",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNGroupNoise %s %s %g %g %i\n",
                                                    &[
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNGroupNoiseFlagID
                                                                [(i) as usize][..],
                                                            "psr[p].TNGroupNoiseFlagID[i]",
                                                        )?),
                                                        Argument::from(read_string(
                                                            &context.pulsars[(p) as usize]
                                                                .TNGroupNoiseFlagVal
                                                                [(i) as usize][..],
                                                            "psr[p].TNGroupNoiseFlagVal[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNGroupNoiseAmp
                                                                [(i) as usize],
                                                            "psr[p].TNGroupNoiseAmp[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNGroupNoiseGam
                                                                [(i) as usize],
                                                            "psr[p].TNGroupNoiseGam[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNGroupNoiseC
                                                                [(i) as usize],
                                                            "psr[p].TNGroupNoiseC[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nTNBandNoise,
                                        "psr[p].nTNBandNoise",
                                    )?) {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNBandNoise %g %g %g %g %i\n",
                                                    &[
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNBandNoiseLF
                                                                [(i) as usize],
                                                            "psr[p].TNBandNoiseLF[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNBandNoiseHF
                                                                [(i) as usize],
                                                            "psr[p].TNBandNoiseHF[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNBandNoiseAmp
                                                                [(i) as usize],
                                                            "psr[p].TNBandNoiseAmp[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNBandNoiseGam
                                                                [(i) as usize],
                                                            "psr[p].TNBandNoiseGam[i]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNBandNoiseC
                                                                [(i) as usize],
                                                            "psr[p].TNBandNoiseC[i]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                        }
                                        i = (i + 1);
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNsubtractRed,
                                    "psr[p].TNsubtractRed",
                                )? == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNsubtractRed 1\n",
                                                &[],
                                            )?,
                                        )?;
                                    }
                                }
                                if ((((*read(
                                    &context.pulsars[(p) as usize].TNRedAmp,
                                    "psr[p].TNRedAmp",
                                )? != (0i32) as f64)
                                    && (*read(
                                        &context.pulsars[(p) as usize].TNRedGam,
                                        "psr[p].TNRedGam",
                                    )? != (0i32) as f64))
                                    || ((*read(
                                        &context.pulsars[(p) as usize].TNDMAmp,
                                        "psr[p].TNDMAmp",
                                    )? != (0i32) as f64)
                                        && (*read(
                                            &context.pulsars[(p) as usize].TNDMGam,
                                            "psr[p].TNDMGam",
                                        )? != (0i32) as f64)))
                                    || ((*read(
                                        &context.pulsars[(p) as usize].TNChromAmp,
                                        "psr[p].TNChromAmp",
                                    )? != (0i32) as f64)
                                        && (*read(
                                            &context.pulsars[(p) as usize].TNChromGam,
                                            "psr[p].TNChromGam",
                                        )? != (0i32) as f64)))
                                {
                                    {
                                        let mut tnsubFlags: u64 =
                                            crate::t2fit_nestlike::t2DecodeTNsubtractPolyFlags(
                                                *read(
                                                    &context.pulsars[(p) as usize].TNsubtractPoly,
                                                    "psr[p].TNsubtractPoly",
                                                )?,
                                            );
                                        let mut redOrdStored: i32 =
                                            crate::t2fit_nestlike::t2DecodeTNRedSubPolyOrd(*read(
                                                &context.pulsars[(p) as usize].TNsubtractPoly,
                                                "psr[p].TNsubtractPoly",
                                            )?);
                                        let mut dmOrdStored: i32 =
                                            crate::t2fit_nestlike::t2DecodeTNDMSubPolyOrd(*read(
                                                &context.pulsars[(p) as usize].TNsubtractPoly,
                                                "psr[p].TNsubtractPoly",
                                            )?);
                                        let mut chromOrdStored: i32 =
                                            crate::t2fit_nestlike::t2DecodeTNChromSubPolyOrd(
                                                *read(
                                                    &context.pulsars[(p) as usize].TNsubtractPoly,
                                                    "psr[p].TNsubtractPoly",
                                                )?,
                                            );
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNsubtractPoly %llu\n",
                                                &[Argument::from(tnsubFlags)],
                                            )?,
                                        )?;
                                        if ((tnsubFlags & 2u64) != 0) {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNRedSubPolyOrd %d\n",
                                                    &[Argument::from(redOrdStored)],
                                                )?,
                                            )?;
                                        }
                                        if ((tnsubFlags & 2u64) != 0) {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNDMSubPolyOrd %d\n",
                                                    &[Argument::from(dmOrdStored)],
                                                )?,
                                            )?;
                                        }
                                        if ((tnsubFlags & 2u64) != 0) {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNChromSubPolyOrd %d\n",
                                                    &[Argument::from(chromOrdStored)],
                                                )?,
                                            )?;
                                        }
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNsubtractDM,
                                    "psr[p].TNsubtractDM",
                                )? == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format("TNsubtractDM 1\n", &[])?,
                                        )?;
                                    }
                                }
                                if ((*read(
                                    &context.pulsars[(p) as usize].SWGPAmp,
                                    "psr[p].SWGPAmp",
                                )? != (0i32) as f64)
                                    && (*read(
                                        &context.pulsars[(p) as usize].SWGPGam,
                                        "psr[p].SWGPGam",
                                    )? != (0i32) as f64))
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "SWGPAmp %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].SWGPAmp,
                                                    "psr[p].SWGPAmp",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "SWGPGam %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].SWGPGam,
                                                    "psr[p].SWGPGam",
                                                )?)],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "SWGPC %i\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize].SWGPC,
                                                    "psr[p].SWGPC",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNsubtractSWGP,
                                    "psr[p].TNsubtractSWGP",
                                )? == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNsubtractSWGP 1\n",
                                                &[],
                                            )?,
                                        )?;
                                    }
                                }
                                if (*read(
                                    &context.pulsars[(p) as usize].TNsubtractChrom,
                                    "psr[p].TNsubtractChrom",
                                )? == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "TNsubtractChrom 1\n",
                                                &[],
                                            )?,
                                        )?;
                                    }
                                }
                                {
                                    let mut ishape: i32 = 0i32;
                                    while (ishape
                                        < *read(
                                            &context.pulsars[(p) as usize].nTNShapeletEvents,
                                            "psr[p].nTNShapeletEvents",
                                        )?)
                                    {
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "TNShapeletEvent %d %lg %lg %lg ",
                                                    &[
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNShapeletEvN
                                                                [(ishape) as usize],
                                                            "psr[p].TNShapeletEvN[ishape]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNShapeletEvPos
                                                                [(ishape) as usize],
                                                            "psr[p].TNShapeletEvPos[ishape]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNShapeletEvWidth
                                                                [(ishape) as usize],
                                                            "psr[p].TNShapeletEvWidth[ishape]",
                                                        )?),
                                                        Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .TNShapeletEvFScale
                                                                [(ishape) as usize],
                                                            "psr[p].TNShapeletEvFScale[ishape]",
                                                        )?),
                                                    ],
                                                )?,
                                            )?;
                                            {
                                                let mut icoef: i32 = 0i32;
                                                while (icoef
                                                    < *read(
                                                        &context.pulsars[(p) as usize]
                                                            .TNShapeletEvN
                                                            [(ishape) as usize],
                                                        "psr[p].TNShapeletEvN[ishape]",
                                                    )?)
                                                {
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "%lg ",
                                                                &[Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .TNShapeletEvCoef
                                                                        [(ishape) as usize]
                                                                        [(icoef) as usize],
                                                                    "psr[p].TNShapeletEvCoef[ishape][icoef]",
                                                                )?)],
                                                            )?,
                                                        )?;
                                                    }
                                                    icoef = (icoef + 1);
                                                }
                                            }
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "%d\n",
                                                    &[Argument::from(*read(
                                                        &context.pulsars[(p) as usize]
                                                            .TNShapeletEvFitFlag
                                                            [(ishape) as usize],
                                                        "psr[p].TNShapeletEvFitFlag[ishape]",
                                                    )?)],
                                                )?,
                                            )?;
                                        }
                                        ishape = (ishape + 1);
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_wave_om as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "WAVEEPOCH %.14Lg\n",
                                                &[Argument::from(
                                                    (context.pulsars[(p) as usize].param.slots
                                                        [(param_waveepoch as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone(),
                                                )],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "WAVE_OM %.14Lg 0\n",
                                                &[Argument::from(
                                                    (context.pulsars[(p) as usize].param.slots
                                                        [(param_wave_om as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone(),
                                                )],
                                            )?,
                                        )?;
                                        if (*read(
                                            &context.pulsars[(p) as usize].waveScale,
                                            "psr[p].waveScale",
                                        )? != (0i32) as f64)
                                        {
                                            write_file(
                                                &mut fout2,
                                                &crate::source_format::format(
                                                    "WAVE_SCALE %g\n",
                                                    &[Argument::from(*read(
                                                        &context.pulsars[(p) as usize].waveScale,
                                                        "psr[p].waveScale",
                                                    )?)],
                                                )?,
                                            )?;
                                        }
                                        {
                                            i = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nWhite,
                                                "psr[p].nWhite",
                                            )?) {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "WAVE%d %.14g %.14g\n",
                                                        &[
                                                            Argument::from((i + 1i32)),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .wave_sine
                                                                    [(i) as usize],
                                                                "psr[p].wave_sine[i]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .wave_cos
                                                                    [(i) as usize],
                                                                "psr[p].wave_cos[i]",
                                                            )?),
                                                        ],
                                                    )?,
                                                )?;
                                                i = (i + 1);
                                            }
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_wave_dm as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "WAVEEPOCH_DM %.14Lg\n",
                                                &[Argument::from(
                                                    (context.pulsars[(p) as usize].param.slots
                                                        [(param_waveepoch_dm as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone(),
                                                )],
                                            )?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "WAVDM_DM %.14Lg 0\n",
                                                &[Argument::from(
                                                    (context.pulsars[(p) as usize].param.slots
                                                        [(param_wave_dm as i32) as usize]
                                                        .cells
                                                        [(0i32) as usize]
                                                        .val)
                                                        .clone(),
                                                )],
                                            )?,
                                        )?;
                                        {
                                            i = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nWhite_dm,
                                                "psr[p].nWhite_dm",
                                            )?) {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "WAVDM%d %.14g %.14g\n",
                                                        &[
                                                            Argument::from((i + 1i32)),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .wave_sine_dm
                                                                    [(i) as usize],
                                                                "psr[p].wave_sine_dm[i]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .wave_cos_dm
                                                                    [(i) as usize],
                                                                "psr[p].wave_cos_dm[i]",
                                                            )?),
                                                        ],
                                                    )?,
                                                )?;
                                                i = (i + 1);
                                            }
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_ifunc as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "SIFUNC %d %d\n",
                                                &[
                                                    Argument::from(crate::ifteph::extended_i32(
                                                        &((context.pulsars[(p) as usize]
                                                            .param
                                                            .slots
                                                            [(param_ifunc as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .val)
                                                            .clone()),
                                                        "binary_integer",
                                                    )?),
                                                    Argument::from(
                                                        context.pulsars[(p) as usize].param.slots
                                                            [(param_ifunc as i32) as usize]
                                                            .cells
                                                            [(0i32) as usize]
                                                            .fit_flag,
                                                    ),
                                                ],
                                            )?,
                                        )?;
                                        {
                                            i = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].ifuncN,
                                                "psr[p].ifuncN",
                                            )?) {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "IFUNC%d %.15f %.10f %.10f\n",
                                                        &[
                                                            Argument::from((i + 1i32)),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .ifuncT
                                                                    [(i) as usize],
                                                                "psr[p].ifuncT[i]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .ifuncV
                                                                    [(i) as usize],
                                                                "psr[p].ifuncV[i]",
                                                            )?),
                                                            Argument::from(*read(
                                                                &context.pulsars[(p) as usize]
                                                                    .ifuncE
                                                                    [(i) as usize],
                                                                "psr[p].ifuncE[i]",
                                                            )?),
                                                        ],
                                                    )?,
                                                )?;
                                                i = (i + 1);
                                            }
                                        }
                                    }
                                }
                                {
                                    i = 0i32;
                                    while (i < *read(
                                        &context.pulsars[(p) as usize].nPhaseJump,
                                        "psr[p].nPhaseJump",
                                    )?) {
                                        {
                                            if (*read(
                                                &context.pulsars[(p) as usize].phaseJumpDir
                                                    [(i) as usize],
                                                "psr[p].phaseJumpDir[i]",
                                            )? != 0i32)
                                            {
                                                write_file(&mut fout2, &crate::source_format::format("PHASE %+d %.14g\n", &[Argument::from(*read(&context.pulsars[(p) as usize].phaseJumpDir[(i) as usize], "psr[p].phaseJumpDir[i]")?), Argument::from((((read(&output_rows(context, (p) as usize)?[(*read(&context.pulsars[(p) as usize].phaseJumpID[(i) as usize], "psr[p].phaseJumpID[i]")?) as usize].sat, "psr[p].obsn[psr[p].phaseJumpID[i]].sat")?).clone() + Float::with_val(PRECISION_BITS, (1.0f64 / 86400.0f64)))).to_f64())])?)?;
                                            }
                                        }
                                        i = (i + 1);
                                    }
                                }
                                {
                                    let mut k: i32 = 2i32;
                                    while (k < *read(
                                        &context.pulsars[(p) as usize].param.slots
                                            [(param_dm as i32) as usize]
                                            .a_size,
                                        "psr[p].param[param_dm].aSize",
                                    )?) {
                                        {
                                            if (context.pulsars[(p) as usize].param.slots
                                                [(param_dm as i32) as usize]
                                                .cells
                                                [(k) as usize]
                                                .param_set
                                                != 0)
                                            {
                                                {
                                                    if (*read(
                                                        &context.pulsars[(p) as usize]
                                                            .dm_series_type,
                                                        "psr[p].dm_series_type",
                                                    )?)
                                                        as i32
                                                        == 1
                                                    {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "DM_SERIES POLY\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    } else {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "DM_SERIES TAYLOR\n",
                                                                &[],
                                                            )?,
                                                        )?;
                                                    }
                                                    break;
                                                }
                                            }
                                        }
                                        k = (k + 1);
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_dmmodel as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        if (*read(
                                            &context.pulsars[(p) as usize].param.slots
                                                [(param_dmmodel as i32) as usize]
                                                .link_to
                                                [(0i32) as usize],
                                            "psr[p].param[param_dmmodel].linkTo[0]",
                                        )? == (param_dm as i32))
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "DMMODEL DM %d\n",
                                                        &[Argument::from(
                                                            context.pulsars[(p) as usize]
                                                                .param
                                                                .slots
                                                                [(param_dmmodel as i32) as usize]
                                                                .cells
                                                                [(0i32) as usize]
                                                                .fit_flag,
                                                        )],
                                                    )?,
                                                )?;
                                            }
                                        } else {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "DMMODEL %.14Lg %d\n",
                                                        &[
                                                            Argument::from(
                                                                (context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(param_dmmodel as i32)
                                                                        as usize]
                                                                    .cells
                                                                    [(0i32) as usize]
                                                                    .val)
                                                                    .clone(),
                                                            ),
                                                            Argument::from(
                                                                context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(param_dmmodel as i32)
                                                                        as usize]
                                                                    .cells
                                                                    [(0i32) as usize]
                                                                    .fit_flag,
                                                            ),
                                                        ],
                                                    )?,
                                                )?;
                                            }
                                        }
                                        let mut useDMOFF: bool = (*read(
                                            &context.pulsars[(p) as usize].dmoffsDMnum,
                                            "psr[p].dmoffsDMnum",
                                        )? == *read(
                                            &context.pulsars[(p) as usize].dmoffsCMnum,
                                            "psr[p].dmoffsCMnum",
                                        )?);
                                        if useDMOFF {
                                            {
                                                i = 0i32;
                                                while (i < *read(
                                                    &context.pulsars[(p) as usize].dmoffsDMnum,
                                                    "psr[p].dmoffsDMnum",
                                                )?) {
                                                    {
                                                        if (*read(
                                                            &context.pulsars[(p) as usize]
                                                                .dmoffsDM_mjd
                                                                [(i) as usize],
                                                            "psr[p].dmoffsDM_mjd[i]",
                                                        )? != *read(
                                                            &context.pulsars[(p) as usize]
                                                                .dmoffsCM_mjd
                                                                [(i) as usize],
                                                            "psr[p].dmoffsCM_mjd[i]",
                                                        )?) {
                                                            useDMOFF = false;
                                                        }
                                                        break;
                                                    }
                                                    i = (i + 1);
                                                }
                                            }
                                        }
                                        if useDMOFF {
                                            {
                                                {
                                                    i = 0i32;
                                                    while (i < *read(
                                                        &context.pulsars[(p) as usize].dmoffsDMnum,
                                                        "psr[p].dmoffsDMnum",
                                                    )?) {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "DMOFF\t %.15g %.15g %.15g\n",
                                                                &[
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM_mjd
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM_mjd[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM_error
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM_error[i]",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                        i = (i + 1);
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                {
                                                    i = 0i32;
                                                    while (i < *read(
                                                        &context.pulsars[(p) as usize].dmoffsDMnum,
                                                        "psr[p].dmoffsDMnum",
                                                    )?) {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "_DM\t %.15g %.15g %.15g\n",
                                                                &[
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM_mjd
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM_mjd[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsDM_error
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsDM_error[i]",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                        i = (i + 1);
                                                    }
                                                }
                                                {
                                                    i = 0i32;
                                                    while (i < *read(
                                                        &context.pulsars[(p) as usize].dmoffsCMnum,
                                                        "psr[p].dmoffsCMnum",
                                                    )?) {
                                                        write_file(
                                                            &mut fout2,
                                                            &crate::source_format::format(
                                                                "_CM\t %.15g %.15g %.15g\n",
                                                                &[
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsCM_mjd
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsCM_mjd[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsCM
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsCM[i]",
                                                                    )?),
                                                                    Argument::from(*read(
                                                                        &context.pulsars
                                                                            [(p) as usize]
                                                                            .dmoffsCM_error
                                                                            [(i) as usize],
                                                                        "psr[p].dmoffsCM_error[i]",
                                                                    )?),
                                                                ],
                                                            )?,
                                                        )?;
                                                        i = (i + 1);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if (context.pulsars[(p) as usize].param.slots
                                    [(param_ne_sw_ifunc as i32) as usize]
                                    .cells[(0i32) as usize]
                                    .param_set
                                    == 1i32)
                                {
                                    {
                                        {
                                            i = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].ne_sw_ifuncN,
                                                "psr[p].ne_sw_ifuncN",
                                            )?) {
                                                {
                                                    write_file(
                                                        &mut fout2,
                                                        &crate::source_format::format(
                                                            "_NE_SW \t %.15g %.15g %.15g\n",
                                                            &[
                                                                Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .ne_sw_ifuncT
                                                                        [(i) as usize],
                                                                    "psr[p].ne_sw_ifuncT[i]",
                                                                )?),
                                                                Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .ne_sw_ifuncV
                                                                        [(i) as usize],
                                                                    "psr[p].ne_sw_ifuncV[i]",
                                                                )?),
                                                                Argument::from(*read(
                                                                    &context.pulsars[(p) as usize]
                                                                        .ne_sw_ifuncE
                                                                        [(i) as usize],
                                                                    "psr[p].ne_sw_ifuncE[i]",
                                                                )?),
                                                            ],
                                                        )?,
                                                    )?;
                                                }
                                                i = (i + 1);
                                            }
                                        }
                                    }
                                }
                                if ((*read(
                                    &context.pulsars[(p) as usize].auto_constraints,
                                    "psr[p].auto_constraints",
                                )?) as i32
                                    != 0)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format("CONSTRAIN AUTO\n", &[])?,
                                        )?;
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "CONSTRAINT_EFACTOR %g\n",
                                                &[Argument::from(*read(
                                                    &context.pulsars[(p) as usize]
                                                        .constraint_efactor,
                                                    "psr[p].constraint_efactor",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                } else {
                                    {
                                        if (*read(
                                            &context.pulsars[(p) as usize].nconstraints,
                                            "psr[p].nconstraints",
                                        )? > 0i32)
                                        {
                                            {
                                                write_file(
                                                    &mut fout2,
                                                    &crate::source_format::format(
                                                        "CONSTRAINT_EFACTOR %g\n",
                                                        &[Argument::from(*read(
                                                            &context.pulsars[(p) as usize]
                                                                .constraint_efactor,
                                                            "psr[p].constraint_efactor",
                                                        )?)],
                                                    )?,
                                                )?;
                                            }
                                        }
                                        {
                                            let mut i: i32 = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nconstraints,
                                                "psr[p].nconstraints",
                                            )?) {
                                                {
                                                    if ((*read(&context.pulsars[(p) as usize].constraints[(i) as usize], "psr[p].constraints[i]")?) as i32 == (crate::pulsar::Constraint::constraint_dmmodel_mean as i32)) {
{
write_file(&mut fout2, &crate::source_format::format("CONSTRAIN DMMODEL\n", &[])?)?;
}
}
                                                }
                                                i = (i + 1);
                                            }
                                        }
                                        {
                                            let mut i: i32 = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nconstraints,
                                                "psr[p].nconstraints",
                                            )?) {
                                                {
                                                    if ((*read(&context.pulsars[(p) as usize].constraints[(i) as usize], "psr[p].constraints[i]")?) as i32 == (crate::pulsar::Constraint::constraint_param as i32)) {
{
write_file(&mut fout2, &crate::source_format::format("CONSTRAIN PARAM %s\n", &[Argument::from(output_constraint(context, (p) as usize, (i) as usize)?)])?)?;
}
}
                                                }
                                                i = (i + 1);
                                            }
                                        }
                                        {
                                            let mut i: i32 = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nconstraints,
                                                "psr[p].nconstraints",
                                            )?) {
                                                {
                                                    if ((*read(&context.pulsars[(p) as usize].constraints[(i) as usize], "psr[p].constraints[i]")?) as i32 == (crate::pulsar::Constraint::constraint_ne_sw_ifunc_sin as i32)) {
{
write_file(&mut fout2, &crate::source_format::format("CONSTRAIN NE_SW_IFUNC_SIN %s\n", &[Argument::from(output_constraint(context, (p) as usize, (i) as usize)?)])?)?;
}
}
                                                }
                                                i = (i + 1);
                                            }
                                        }
                                        {
                                            let mut i: i32 = 0i32;
                                            while (i < *read(
                                                &context.pulsars[(p) as usize].nconstraints,
                                                "psr[p].nconstraints",
                                            )?) {
                                                {
                                                    if ((*read(&context.pulsars[(p) as usize].constraints[(i) as usize], "psr[p].constraints[i]")?) as i32 == (crate::pulsar::Constraint::constraint_ne_sw_ifunc_sigma as i32)) {
{
write_file(&mut fout2, &crate::source_format::format("CONSTRAIN NE_SW_IFUNC_SIGMA %s\n", &[Argument::from(output_constraint(context, (p) as usize, (i) as usize)?)])?)?;
}
}
                                                }
                                                i = (i + 1);
                                            }
                                        }
                                    }
                                }
                                if (i32::from(
                                    read_string(
                                        &context.pulsars[(p) as usize].whiteNoiseModelFile[..],
                                        "psr[p].whiteNoiseModelFile",
                                    )? != "NULL".as_bytes(),
                                ) != 0i32)
                                {
                                    {
                                        write_file(
                                            &mut fout2,
                                            &crate::source_format::format(
                                                "WHITE_NOISE_MODEL %s",
                                                &[Argument::from(read_string(
                                                    &context.pulsars[(p) as usize]
                                                        .whiteNoiseModelFile[..],
                                                    "psr[p].whiteNoiseModelFile",
                                                )?)],
                                            )?,
                                        )?;
                                    }
                                }
                                fout2.take();
                            }
                        }
                    }
                }
                let mut notwarned: bool = true;
                let mut count_bad_covariances: i32 = 0i32;
                {
                    let mut i: u32 = (0i32) as u32;
                    while (i < *read(
                        &context.pulsars[(p) as usize].fitinfo.nParams,
                        "psr[p].fitinfo.nParams",
                    )?) {
                        {
                            {
                                let mut j: u32 = (0i32) as u32;
                                while (j < i) {
                                    {
                                        let mut corr: f64 = (output_covar(
                                            context,
                                            (p) as usize,
                                            (i) as usize,
                                            (j) as usize,
                                        )? / (output_covar(
                                            context,
                                            (p) as usize,
                                            (i) as usize,
                                            (i) as usize,
                                        )? * output_covar(
                                            context,
                                            (p) as usize,
                                            (j) as usize,
                                            (j) as usize,
                                        )?)
                                        .sqrt());
                                        if ((corr).abs() > 0.99f64) {
                                            {
                                                if notwarned {
                                                    {
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "\n",
                                                                    &[],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                        d.log(b'w',"textOutput.C",1806,&crate::source_format::format("There are output parameters with very high covariance", &[])?);
                                                        io.drain(d)?;
                                                        notwarned = false;
                                                    }
                                                }
                                                if (count_bad_covariances < 20i32) {
                                                    {
                                                        let mut p1: i32 = *read(
                                                            &context.pulsars[(p) as usize]
                                                                .fitinfo
                                                                .paramIndex
                                                                [(i) as usize],
                                                            "psr[p].fitinfo.paramIndex[i]",
                                                        )?;
                                                        let mut p2: i32 = *read(
                                                            &context.pulsars[(p) as usize]
                                                                .fitinfo
                                                                .paramIndex
                                                                [(j) as usize],
                                                            "psr[p].fitinfo.paramIndex[j]",
                                                        )?;
                                                        let mut c1: i32 = *read(
                                                            &context.pulsars[(p) as usize]
                                                                .fitinfo
                                                                .paramCounters
                                                                [(i) as usize],
                                                            "psr[p].fitinfo.paramCounters[i]",
                                                        )?;
                                                        let mut c2: i32 = *read(
                                                            &context.pulsars[(p) as usize]
                                                                .fitinfo
                                                                .paramCounters
                                                                [(j) as usize],
                                                            "psr[p].fitinfo.paramCounters[j]",
                                                        )?;
                                                        let mut n1: Vec<u8> = Vec::new();
                                                        let mut n2: Vec<u8> = Vec::new();
                                                        if ((c1
                                                            < *read(
                                                                &context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(p1) as usize]
                                                                    .a_size,
                                                                "psr[p].param[p1].aSize",
                                                            )?)
                                                            && ((crate::clock_text::c_bytes(
                                                                &context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(p1) as usize]
                                                                    .cells
                                                                    [(c1) as usize]
                                                                    .shortlabel,
                                                            )
                                                            .to_vec())
                                                            .len()
                                                                as i32
                                                                > 0i32))
                                                        {
                                                            {
                                                                n1 = crate::clock_text::c_bytes(
                                                                    &context.pulsars[(p) as usize]
                                                                        .param
                                                                        .slots
                                                                        [(p1) as usize]
                                                                        .cells
                                                                        [(c1) as usize]
                                                                        .shortlabel,
                                                                )
                                                                .to_vec();
                                                            }
                                                        } else {
                                                            {
                                                                n1 = crate::source_format::format("%s(%d)", &[Argument::from(crate::core_output::LABEL_STR[(p1) as usize].as_bytes().to_vec()), Argument::from(c1)])?;
                                                            }
                                                        }
                                                        if ((c2
                                                            < *read(
                                                                &context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(p2) as usize]
                                                                    .a_size,
                                                                "psr[p].param[p2].aSize",
                                                            )?)
                                                            && ((crate::clock_text::c_bytes(
                                                                &context.pulsars[(p) as usize]
                                                                    .param
                                                                    .slots
                                                                    [(p2) as usize]
                                                                    .cells
                                                                    [(c2) as usize]
                                                                    .shortlabel,
                                                            )
                                                            .to_vec())
                                                            .len()
                                                                as i32
                                                                > 0i32))
                                                        {
                                                            {
                                                                n2 = crate::clock_text::c_bytes(
                                                                    &context.pulsars[(p) as usize]
                                                                        .param
                                                                        .slots
                                                                        [(p2) as usize]
                                                                        .cells
                                                                        [(c2) as usize]
                                                                        .shortlabel,
                                                                )
                                                                .to_vec();
                                                            }
                                                        } else {
                                                            {
                                                                n2 = crate::source_format::format("%s(%d)", &[Argument::from(crate::core_output::LABEL_STR[(p2) as usize].as_bytes().to_vec()), Argument::from(c2)])?;
                                                            }
                                                        }
                                                        {
                                                            let bytes =
                                                                crate::source_format::format(
                                                                    "% 20s % 20s %+.5f\n",
                                                                    &[
                                                                        Argument::from(n1.clone()),
                                                                        Argument::from(n2.clone()),
                                                                        Argument::from(corr),
                                                                    ],
                                                                )?;
                                                            io.write(&bytes)?;
                                                        }
                                                    }
                                                }
                                                count_bad_covariances = (count_bad_covariances + 1);
                                            }
                                        }
                                    }
                                    j = (j + 1);
                                }
                            }
                        }
                        i = (i + 1);
                    }
                }
                if (count_bad_covariances > 20i32) {
                    {
                        {
                            let bytes = crate::source_format::format(
                                " ... There were %d more covariances not shown\n",
                                &[Argument::from((count_bad_covariances - 20i32))],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
            }
            p = (p + 1);
        }
    }
    if (nGlobal > 0i32) {
        {
            {
                let bytes = crate::source_format::format("Global Parameters:\n\n", &[])?;
                io.write(&bytes)?;
            }
            {
                let bytes = crate::source_format::format(
                    "Global term = %g\n",
                    &[Argument::from(globalParameter)],
                )?;
                io.write(&bytes)?;
            }
            {
                let bytes = crate::source_format::format(
                    "------------------------------------------------------------------------------------\n",
                    &[],
                )?;
                io.write(&bytes)?;
            }
        }
    }
    let mut errorCount: i32 = d.check();
    if (errorCount != 0) {
        {
            d.log(b'm',"textOutput.C",1847,&crate::source_format::format("NOTICE! There were %d errors thrown!! Check the logs as the results may not be valid.", &[Argument::from(errorCount)])?);
            io.drain(d)?;
        }
    }
    Ok(())
}

/// textOutput.C:1851::calcRMS.
pub(crate) fn calc_rms_values(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut sumwt: f64 = 0.0f64;
    let mut rms_pre: f64 = 0.0f64;
    let mut rms_post: f64 = 0.0f64;
    let mut wgt: f64 = 0.0f64;
    let mut sumsq_post: f64 = 0.0f64;
    let mut sumsq_pre: f64 = 0.0f64;
    let mut sum_pre: f64 = 0.0f64;
    let mut sum_post: f64 = 0.0f64;
    let mut mean_post: f64 = 0.0f64;
    let mut mean_pre: f64 = 0.0f64;
    let mut i: i32;
    let mut count: i32 = 0i32;
    let mut sum_tn: f64 = 0.0f64;
    let mut sumsq_tn: f64 = 0.0f64;
    let mut rms_tn: f64 = 0.0f64;
    let mut startSet: bool =
        ((psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].param_set == 1i32)
            && (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].fit_flag
                == 1i32));
    let mut finishSet: bool =
        ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].param_set
            == 1i32)
            && (psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].fit_flag
                == 1i32));
    let mut bat_startSet: bool =
        ((psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].param_set == 1i32)
            && (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].fit_flag
                == 2i32));
    let mut bat_finishSet: bool =
        ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].param_set
            == 1i32)
            && (psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].fit_flag
                == 2i32));
    let mut start: Float = Float::with_val(PRECISION_BITS, 1e10f64);
    let mut finish: Float = Float::with_val(PRECISION_BITS, 0i32);
    if (startSet || bat_startSet) {
        start = (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val).clone();
    }
    if (finishSet || bat_finishSet) {
        finish =
            (psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val).clone();
    }
    {
        i = 0i32;
        while (i < *read(&psr.nobs, "psr[p].nobs")?) {
            {
                if (*read(&rows[(i) as usize].deleted, "psr[p].obsn[i].deleted")? != 0) {
                    i = (i + 1);
                    continue;
                }
                if (startSet
                    && ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?).clone()
                        < ((start).clone()
                            - Float::with_val(PRECISION_BITS, Float::parse("1e-11").unwrap()))))
                {
                    i = (i + 1);
                    continue;
                }
                if (finishSet
                    && ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?).clone()
                        > ((finish).clone()
                            + Float::with_val(PRECISION_BITS, Float::parse("1e-11").unwrap()))))
                {
                    i = (i + 1);
                    continue;
                }
                if (bat_startSet
                    && ((read(&rows[(i) as usize].bat, "psr[p].obsn[i].bat")?).clone()
                        < ((start).clone()
                            - Float::with_val(PRECISION_BITS, Float::parse("1e-11").unwrap()))))
                {
                    i = (i + 1);
                    continue;
                }
                if (bat_finishSet
                    && ((read(&rows[(i) as usize].bat, "psr[p].obsn[i].bat")?).clone()
                        > ((finish).clone()
                            + Float::with_val(PRECISION_BITS, Float::parse("1e-11").unwrap()))))
                {
                    i = (i + 1);
                    continue;
                }
                mean_pre = (mean_pre
                    + ((read(
                        &rows[(i) as usize].prefitResidual,
                        "psr[p].obsn[i].prefitResidual",
                    )?)
                    .clone())
                    .to_f64());
                if (*read(&psr.fitMode, "psr[p].fitMode")? == 1i32) {
                    wgt = (Float::with_val(PRECISION_BITS, 1.0f64)
                        / ((((Float::with_val(
                            PRECISION_BITS,
                            (1.0e-6f64
                                * *read(&rows[(i) as usize].toaErr, "psr[p].obsn[i].toaErr")?),
                        ) * (psr.param.slots[(param_f as i32) as usize].cells
                            [(0i32) as usize]
                            .val)
                            .clone())
                            * Float::with_val(PRECISION_BITS, 1.0e-6f64))
                            * Float::with_val(
                                PRECISION_BITS,
                                *read(&rows[(i) as usize].toaErr, "psr[p].obsn[i].toaErr")?,
                            ))
                            * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                                .val)
                                .clone()))
                    .to_f64();
                } else {
                    wgt = (Float::with_val(PRECISION_BITS, 1.0f64)
                        / (((Float::with_val(PRECISION_BITS, 1.0e-6f64)
                            * (psr.param.slots[(param_f as i32) as usize].cells
                                [(0i32) as usize]
                                .val)
                                .clone())
                            * Float::with_val(PRECISION_BITS, 1.0e-6f64))
                            * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                                .val)
                                .clone()))
                    .to_f64();
                }
                sumsq_pre = (sumsq_pre
                    + ((((Float::with_val(PRECISION_BITS, wgt)
                        * (read(
                            &rows[(i) as usize].prefitResidual,
                            "psr[p].obsn[i].prefitResidual",
                        )?)
                        .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                            .val)
                            .clone())
                        * (read(
                            &rows[(i) as usize].prefitResidual,
                            "psr[p].obsn[i].prefitResidual",
                        )?)
                        .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sum_pre = (sum_pre
                    + ((Float::with_val(PRECISION_BITS, wgt)
                        * (read(
                            &rows[(i) as usize].prefitResidual,
                            "psr[p].obsn[i].prefitResidual",
                        )?)
                        .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sumsq_post = (sumsq_post
                    + ((((Float::with_val(PRECISION_BITS, wgt)
                        * (read(&rows[(i) as usize].residual, "psr[p].obsn[i].residual")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                            .val)
                            .clone())
                        * (read(&rows[(i) as usize].residual, "psr[p].obsn[i].residual")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sum_post = (sum_post
                    + ((Float::with_val(PRECISION_BITS, wgt)
                        * (read(&rows[(i) as usize].residual, "psr[p].obsn[i].residual")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sumsq_tn = (sumsq_tn
                    + ((((Float::with_val(PRECISION_BITS, wgt)
                        * (read(&rows[(i) as usize].residualtn, "psr[p].obsn[i].residualtn")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                            .val)
                            .clone())
                        * (read(&rows[(i) as usize].residualtn, "psr[p].obsn[i].residualtn")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sum_tn = (sum_tn
                    + ((Float::with_val(PRECISION_BITS, wgt)
                        * (read(&rows[(i) as usize].residualtn, "psr[p].obsn[i].residualtn")?)
                            .clone())
                        * (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                            .clone())
                    .to_f64());
                sumwt = (sumwt + wgt);
                mean_post = (mean_post
                    + ((read(&rows[(i) as usize].residual, "psr[p].obsn[i].residual")?).clone())
                        .to_f64());
                count = (count + 1);
            }
            i = (i + 1);
        }
    }
    d.log(
        b'd',
        "textOutput.C",
        1910,
        &crate::source_format::format(
            "textOutput %g %g %d",
            &[
                Argument::from(mean_pre),
                Argument::from(mean_post),
                Argument::from(count),
            ],
        )?,
    );

    mean_pre = (mean_pre / (count) as f64);
    mean_post = (mean_post / (count) as f64);
    rms_pre = ((Float::with_val(
        PRECISION_BITS,
        (((sumsq_pre - ((sum_pre * sum_pre) / sumwt)) / sumwt).sqrt() * 1e3f64),
    ) / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
        .clone())
        * Float::with_val(PRECISION_BITS, 1e3f64))
    .to_f64();
    rms_post = ((Float::with_val(
        PRECISION_BITS,
        (((sumsq_post - ((sum_post * sum_post) / sumwt)) / sumwt).sqrt() * 1e3f64),
    ) / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
        .clone())
        * Float::with_val(PRECISION_BITS, 1e3f64))
    .to_f64();
    rms_tn = ((Float::with_val(
        PRECISION_BITS,
        (((sumsq_tn - ((sum_tn * sum_tn) / sumwt)) / sumwt).sqrt() * 1e3f64),
    ) / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
        .clone())
        * Float::with_val(PRECISION_BITS, 1e3f64))
    .to_f64();
    d.log(
        b'd',
        "textOutput.C",
        1919,
        &crate::source_format::format(
            "textOutput %g %g %g %G %d",
            &[
                Argument::from(rms_pre),
                Argument::from(rms_post),
                Argument::from(sumsq_pre),
                Argument::from(sum_pre),
                Argument::from(count),
            ],
        )?,
    );

    psr.rmsPre = Some(rms_pre);
    psr.rmsPost = Some(rms_post);
    psr.rmstn = Some(rms_tn);
    // fda6ad4 textOutput.C:127-133,1926 activates a report field without prefit.
    // Snapshot its defined previous value/error before activation (0053 poison witness).
    if !(psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].param_set != 0) {
        {
            psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].prefit = Some(
                (psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].val).clone(),
            );
            psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].prefit_err = Some(
                (psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].err).clone(),
            );
        }
    }
    psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].val =
        Float::with_val(PRECISION_BITS, rms_post);
    psr.param.slots[(param_trestn as i32) as usize].cells[(0i32) as usize].val =
        Float::with_val(PRECISION_BITS, rms_tn);
    psr.param.slots[(param_tres as i32) as usize].cells[(0i32) as usize].param_set = 1i32;
    return Ok(sumwt);
}

/// textOutput.C:1945::m2.
pub(crate) fn m2_inner(
    mf: Float,
    sini: Float,
    m1: Float,
    io: &mut OutputIo<'_>,
) -> Result<f64, TimingError> {
    let mut guess: f64 = ((m1).clone()).to_f64();
    let mut dx: f64 = 0.0f64;
    let mut eq: f64 = 0.0f64;
    let mut deq_dm2: f64 = 0.0f64;
    let mut gi: i32 = 0i32;
    {
        gi = 0i32;
        while (gi < 10000i32) {
            {
                eq = (((m1).clone() + Float::with_val(PRECISION_BITS, guess))
                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                    - ((Float::with_val(PRECISION_BITS, guess) * (sini).clone())
                        .pow(&(Float::with_val(PRECISION_BITS, 3i32)))
                        / (mf).clone()))
                .to_f64();
                deq_dm2 = ((Float::with_val(PRECISION_BITS, 2.0f64)
                    * ((m1).clone() + Float::with_val(PRECISION_BITS, guess)))
                    - ((Float::with_val(PRECISION_BITS, 3.0f64)
                        * (Float::with_val(PRECISION_BITS, guess) * (sini).clone())
                            .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                        / (mf).clone()))
                .to_f64();
                dx = (eq / deq_dm2);
                guess = (guess - dx);
                if ((dx).abs() <= ((guess).abs() * 1e-10f64)) {
                    return Ok(guess);
                }
            }
            gi = (gi + 1);
        }
    }
    io.write(&crate::source_format::format(
        "m2: maximum iterations exceeded - %lf\n",
        &[Argument::from((dx / guess).abs())],
    )?)?;
    return Ok((-1.0f64));
}

/// textOutput.C:1967::printGlitch.
pub(crate) fn print_glitch_inner(
    psr: &SourcePulsar,
    io: &mut OutputIo<'_>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut glep1z: f64;
    let mut glep2z: f64;
    let mut glepe: f64;
    let mut iph: i32;
    let mut fph: f64;
    let mut dfof: f64;
    let mut edfof: f64;
    let mut df1of1: f64;
    let mut edf1of1: f64;
    let mut iglitch: i32;
    {
        iglitch = 0i32;
        while (iglitch
            < *read(
                &psr.param.slots[(param_glep as i32) as usize].a_size,
                "psr.param[param_glep].aSize",
            )?)
        {
            {
                if (psr.param.slots[(param_glep as i32) as usize].cells[(iglitch) as usize]
                    .param_set
                    == 1i32)
                {
                    {
                        iph = source_nint(
                            ((psr.param.slots[(param_glph as i32) as usize].cells
                                [(iglitch) as usize]
                                .val)
                                .clone())
                            .to_f64(),
                        )?;
                        fph = (((psr.param.slots[(param_glph as i32) as usize].cells
                            [(iglitch) as usize]
                            .val)
                            .clone())
                        .to_f64()
                            - (iph) as f64);
                        glep1z = (((psr.param.slots[(param_glep as i32) as usize].cells
                            [(iglitch) as usize]
                            .val)
                            .clone())
                        .to_f64()
                            + {
                                let value = dglep_inner(psr, iglitch, fph, d)?;
                                io.drain(d)?;
                                value
                            });
                        if (fph >= (0i32) as f64) {
                            glep2z = (((psr.param.slots[(param_glep as i32) as usize].cells
                                [(iglitch) as usize]
                                .val)
                                .clone())
                            .to_f64()
                                + {
                                    let value =
                                        dglep_inner(psr, iglitch, (fph - (1i32) as f64), d)?;
                                    io.drain(d)?;
                                    value
                                });
                        } else {
                            glep2z = (((psr.param.slots[(param_glep as i32) as usize].cells
                                [(iglitch) as usize]
                                .val)
                                .clone())
                            .to_f64()
                                + {
                                    let value =
                                        dglep_inner(psr, iglitch, (fph + (1i32) as f64), d)?;
                                    io.drain(d)?;
                                    value
                                });
                        }
                        glepe = (((psr.param.slots[(param_glph as i32) as usize].cells
                            [(iglitch) as usize]
                            .err)
                            .clone())
                        .to_f64()
                            / (((psr.param.slots[(param_glf0 as i32) as usize].cells
                                [(iglitch) as usize]
                                .val)
                                .clone()
                                + (psr.param.slots[(param_glf0d as i32) as usize].cells
                                    [(iglitch) as usize]
                                    .val)
                                    .clone())
                            .abs()
                                * Float::with_val(PRECISION_BITS, 86400.0f64))
                            .to_f64());
                        {
                            let bytes = crate::source_format::format(
                                "MJD for zero glitch  %d phase = %.6f or %.6f, error = %g\n",
                                &[
                                    Argument::from(iglitch),
                                    Argument::from(glep1z),
                                    Argument::from(glep2z),
                                    Argument::from(glepe),
                                ],
                            )?;
                            io.write(&bytes)?;
                        }
                        dfof = ((psr.param.slots[(param_glf0 as i32) as usize].cells
                            [(iglitch) as usize]
                            .val)
                            .clone()
                            / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                                .val)
                                .clone())
                        .to_f64();
                        edfof = ((psr.param.slots[(param_glf0 as i32) as usize].cells
                            [(iglitch) as usize]
                            .err)
                            .clone()
                            / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize]
                                .val)
                                .clone())
                        .to_f64();
                        {
                            let bytes = crate::source_format::format(
                                "Delta f/f = %g +/- %g\n",
                                &[Argument::from(dfof), Argument::from(edfof)],
                            )?;
                            io.write(&bytes)?;
                        }
                        df1of1 = ((psr.param.slots[(param_glf1 as i32) as usize].cells
                            [(iglitch) as usize]
                            .val)
                            .clone()
                            / (psr.param.slots[(param_f as i32) as usize].cells[(1i32) as usize]
                                .val)
                                .clone())
                        .to_f64();
                        edf1of1 = ((psr.param.slots[(param_glf1 as i32) as usize].cells
                            [(iglitch) as usize]
                            .err)
                            .clone()
                            / ((psr.param.slots[(param_f as i32) as usize].cells
                                [(1i32) as usize]
                                .val)
                                .clone())
                            .abs())
                        .to_f64();
                        {
                            let bytes = crate::source_format::format(
                                "Delta f1/f1 = %g +/- %g\n",
                                &[Argument::from(df1of1), Argument::from(edf1of1)],
                            )?;
                            io.write(&bytes)?;
                        }
                    }
                }
            }
            iglitch = (iglitch + 1);
        }
    }
    Ok(())
}

/// textOutput.C:2011::dglep_eval_phase.
fn dglep_eval_phase(
    t: f64,
    fph: f64,
    glf0: f64,
    glf0d: f64,
    glf1: f64,
    tds: f64,
    dph: &mut Option<f64>,
    ddph_dt: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<i32, TimingError> {
    let mut phase: f64 = ((fph + (glf0 * t)) + (((0.5f64 * glf1) * t) * t));
    let mut deriv: f64 = (glf0 + (glf1 * t));
    if (tds > 0.0f64) {
        {
            let mut x: f64 = ((-t) / tds);
            if (!((x).is_finite()) || (x > 700.0f64)) {
                {
                    return Ok(0i32);
                }
            }
            let mut em1: f64 = if (x < (-700.0f64)) {
                (-1.0f64)
            } else {
                (x).exp_m1()
            };
            let mut expx: f64 = if (x < (-700.0f64)) {
                0.0f64
            } else {
                (em1 + 1.0f64)
            };
            phase = (phase + ((glf0d * tds) * (-em1)));
            deriv = (deriv + (glf0d * expx));
        }
    }
    if (!((phase).is_finite()) || !((deriv).is_finite())) {
        {
            return Ok(0i32);
        }
    }
    *dph = Some(phase);
    *ddph_dt = Some(deriv);
    return Ok(1i32);
}

/// textOutput.C:2036::dglep.
pub(crate) fn dglep_inner(
    psr: &SourcePulsar,
    gn: i32,
    fph: f64,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut glf0: f64 =
        ((psr.param.slots[(param_glf0 as i32) as usize].cells[(gn) as usize].val).clone()).to_f64();
    let mut glf0d: f64 =
        ((psr.param.slots[(param_glf0d as i32) as usize].cells[(gn) as usize].val).clone())
            .to_f64();
    let mut glf1: f64 =
        ((psr.param.slots[(param_glf1 as i32) as usize].cells[(gn) as usize].val).clone()).to_f64();
    let mut tds: f64 = ((psr.param.slots[(param_gltd as i32) as usize].cells[(gn) as usize].val)
        .clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    let mut plim: f64 = 1.0e-6f64;
    let mut teps: f64 = 1.0e-12f64;
    let mut deriv_eps: f64 = 1.0e-30f64;
    let mut max_iter: i32 = 200i32;
    let mut max_expand: i32 = 60i32;
    let mut lin: f64 = (glf0 + glf0d);
    let mut t0: f64 = 0.0f64;
    if ((lin).abs() > deriv_eps) {
        {
            t0 = ((-fph) / lin);
        }
    }
    let mut f0: Option<f64> = None;
    let mut df0: Option<f64> = None;
    if !(dglep_eval_phase(t0, fph, glf0, glf0d, glf1, tds, &mut f0, &mut df0, d)? != 0) {
        {
            d.log(b'w',"textOutput.C",2057,&crate::source_format::format("Glitch epoch convergence failed (non-finite initial state for zero GLPH solve)", &[])?);

            return Ok((0i32) as f64);
        }
    }
    if ((*read(&f0, "f0")?).abs() <= plim) {
        {
            return Ok((t0 / 86400.0f64));
        }
    }
    let mut halfw: f64 = (1.0f64).max((0.25f64 * (t0).abs()));
    if (tds > 0.0f64) {
        {
            halfw = (halfw).max((0.25f64 * tds));
        }
    }
    let mut left: f64 = (t0 - halfw);
    let mut right: f64 = (t0 + halfw);
    let mut fleft: Option<f64> = None;
    let mut dfleft: Option<f64> = None;
    let mut fright: Option<f64> = None;
    let mut dfright: Option<f64> = None;
    let mut left_ok: i32 = dglep_eval_phase(
        left,
        fph,
        glf0,
        glf0d,
        glf1,
        tds,
        &mut fleft,
        &mut dfleft,
        d,
    )?;
    let mut right_ok: i32 = dglep_eval_phase(
        right,
        fph,
        glf0,
        glf0d,
        glf1,
        tds,
        &mut fright,
        &mut dfright,
        d,
    )?;
    let mut bracketed: i32 = (((left_ok != 0) && (right_ok != 0))
        && ((*read(&fleft, "fleft")? * *read(&fright, "fright")?) <= 0.0f64))
        as i32;
    {
        let mut iexp: i32 = 0i32;
        while ((iexp < max_expand) && !(bracketed != 0)) {
            {
                halfw = (halfw * 2.0f64);
                left = (t0 - halfw);
                right = (t0 + halfw);
                left_ok = dglep_eval_phase(
                    left,
                    fph,
                    glf0,
                    glf0d,
                    glf1,
                    tds,
                    &mut fleft,
                    &mut dfleft,
                    d,
                )?;
                right_ok = dglep_eval_phase(
                    right,
                    fph,
                    glf0,
                    glf0d,
                    glf1,
                    tds,
                    &mut fright,
                    &mut dfright,
                    d,
                )?;
                bracketed = (((left_ok != 0) && (right_ok != 0))
                    && ((*read(&fleft, "fleft")? * *read(&fright, "fright")?) <= 0.0f64))
                    as i32;
            }
            iexp = (iexp + 1);
        }
    }
    if (bracketed != 0) {
        {
            let mut t: f64 = t0;
            if ((t <= left) || (t >= right)) {
                {
                    t = (0.5f64 * (left + right));
                }
            }
            {
                let mut niter: i32 = 0i32;
                while (niter < max_iter) {
                    {
                        let mut dph: Option<f64> = None;
                        let mut ddph_dt: Option<f64> = None;
                        if !(dglep_eval_phase(
                            t,
                            fph,
                            glf0,
                            glf0d,
                            glf1,
                            tds,
                            &mut dph,
                            &mut ddph_dt,
                            d,
                        )? != 0)
                        {
                            {
                                break;
                            }
                        }
                        if ((*read(&dph, "dph")?).abs() <= plim) {
                            {
                                return Ok((t / 86400.0f64));
                            }
                        }
                        let mut tnext: f64;
                        if ((*read(&ddph_dt, "ddph_dt")?).abs() > deriv_eps) {
                            {
                                tnext = (t - (*read(&dph, "dph")? / *read(&ddph_dt, "ddph_dt")?));
                                if (!((tnext > left) && (tnext < right)) || !((tnext).is_finite()))
                                {
                                    {
                                        tnext = (0.5f64 * (left + right));
                                    }
                                }
                            }
                        } else {
                            {
                                tnext = (0.5f64 * (left + right));
                            }
                        }
                        let mut fnext: Option<f64> = None;
                        let mut dfnext: Option<f64> = None;
                        if !(dglep_eval_phase(
                            tnext,
                            fph,
                            glf0,
                            glf0d,
                            glf1,
                            tds,
                            &mut fnext,
                            &mut dfnext,
                            d,
                        )? != 0)
                        {
                            {
                                break;
                            }
                        }
                        if (((*read(&fnext, "fnext")?).abs() <= plim)
                            || ((tnext - t).abs() <= (teps * (1.0f64 + (tnext).abs()))))
                        {
                            {
                                return Ok((tnext / 86400.0f64));
                            }
                        }
                        if ((*read(&fleft, "fleft")? * *read(&fnext, "fnext")?) <= 0.0f64) {
                            {
                                right = tnext;
                                fright = Some(*read(&fnext, "fnext")?);
                            }
                        } else {
                            {
                                left = tnext;
                                fleft = Some(*read(&fnext, "fnext")?);
                            }
                        }
                        if ((right - left).abs()
                            <= (teps * (1.0f64 + ((left).abs()).max((right).abs()))))
                        {
                            {
                                return Ok(((0.5f64 * (left + right)) / 86400.0f64));
                            }
                        }
                        t = tnext;
                    }
                    niter = (niter + 1);
                }
            }
        }
    }
    {
        let mut t: f64 = t0;
        {
            let mut niter: i32 = 0i32;
            while (niter < max_iter) {
                {
                    let mut dph: Option<f64> = None;
                    let mut ddph_dt: Option<f64> = None;
                    if !(dglep_eval_phase(
                        t,
                        fph,
                        glf0,
                        glf0d,
                        glf1,
                        tds,
                        &mut dph,
                        &mut ddph_dt,
                        d,
                    )? != 0)
                    {
                        {
                            break;
                        }
                    }
                    if ((*read(&dph, "dph")?).abs() <= plim) {
                        {
                            return Ok((t / 86400.0f64));
                        }
                    }
                    let mut max_step: f64 = (10.0f64).max(((0.5f64 * (t).abs()) + 1.0f64));
                    let mut step: f64;
                    if ((*read(&ddph_dt, "ddph_dt")?).abs() > deriv_eps) {
                        {
                            step = (*read(&dph, "dph")? / *read(&ddph_dt, "ddph_dt")?);
                            if (step > max_step) {
                                step = max_step;
                            }
                            if (step < (-max_step)) {
                                step = (-max_step);
                            }
                        }
                    } else {
                        {
                            step = if (*read(&dph, "dph")? > 0.0f64) {
                                max_step
                            } else {
                                (-max_step)
                            };
                        }
                    }
                    let mut tnext: f64 = (t - step);
                    if !((tnext).is_finite()) {
                        {
                            break;
                        }
                    }
                    if ((tnext - t).abs() <= (teps * (1.0f64 + (tnext).abs()))) {
                        {
                            return Ok((tnext / 86400.0f64));
                        }
                    }
                    t = tnext;
                }
                niter = (niter + 1);
            }
        }
    }
    d.log(
        b'w',
        "textOutput.C",
        2166,
        &crate::source_format::format(
            "Glitch epoch convergence failed (Couldn't guess a new PEPOCH for zero GLPH)",
            &[],
        )?,
    );

    return Ok((0i32) as f64);
}
