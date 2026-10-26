//! displayParameters.C::displayParameters, all position-dependent output.
use crate::{
    TimingError,
    core_output::{OutputIo, output_rows},
    observation::{read, read_string},
    pulsar_lifecycle::PulsarContext,
    source_format::{Argument, format, print_longdouble},
};
use std::{os::unix::ffi::OsStrExt, path::PathBuf};
pub fn display_parameters(
    position: i32,
    tim: &[PathBuf],
    par: &[PathBuf],
    context: &PulsarContext,
    io: &mut OutputIo<'_>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_diagnostics(|d| {
        d.version("displayParameters.C", "displayParameters()");
        io.drain(d)
    })?;
    macro_rules! emit{($fmt:expr $(,$v:expr)*$(,)?)=>{io.write(&format($fmt,&[$(Argument::from($v)),*])?)?};}
    emit!("\n\n");
    if let Some(message) = match position {
        1 => Some("Parameter values after getInputs from command line\n"),
        2 => Some("Parameter values after reading of .par file\n"),
        3 => Some("Parameter values after reading of .tim file\n"),
        4 => Some("After obtaining correction between observatory UTC and UTC(NIST)\n"),
        5 => Some("After calculating correction between UTC and TAI\n"),
        6 => Some("After calculating correction between TAI and TB\n"),
        7 => Some("After calculating vector pointing at pulsar\n"),
        _ => None,
    } {
        emit!(message);
    }
    emit!(
        "\n\nlobal definitions:\n\nMAX_FREQ_DERIVATIVES = %d\nMAX_DM_DERIVATIVES   = %d\nMAX_FILELEN          = %d\n\n\n",
        13,
        10,
        500
    );
    for (index, p) in context.pulsars.iter().enumerate() {
        // Source labels explicitly say [0]; preserve the first input pair here.
        let t = tim
            .first()
            .ok_or_else(|| crate::clock_diagnostics::boundary("display_tim_file"))?;
        let f = par
            .first()
            .ok_or_else(|| crate::clock_diagnostics::boundary("display_par_file"))?;
        emit!(
            "Command line inputs:\n\nPulse arrival time file[0] = %s\nPulsar parameter file[0]   = %s\n\n\nPulsar Parameters:\n\nPulsar name          = %s\n",
            t.as_os_str().as_bytes(),
            f.as_os_str().as_bytes(),
            read_string(&p.name[..], "name")?
        );
        let rows = if position > 2 {
            output_rows(context, index)?
        } else {
            &[]
        };
        if position > 2 {
            emit!(
                "\n\nObservations:\n\nNumber of observations = %d\n",
                *read(&p.nobs, "nobs")?
            );
            let rule = "--------------------------------------------------------------------------------\n";
            emit!(rule);
            emit!("   #  | SAT                   | Clock corrections(s)     |\n");
            emit!(rule);
            for (i, o) in rows.iter().enumerate() {
                emit!(
                    "|%4d | %s |",
                    i as i32 + 1,
                    print_longdouble(read(&o.sat, "sat")?)
                );
                for c in o
                    .correctionsTT
                    .iter()
                    .take(*read(&o.nclock_correction, "nclock_correction")? as usize)
                {
                    emit!(
                        " %-14.7g(->%s)",
                        *read(&c.correction, "correction")?,
                        read_string(&c.corrects_to, "corrects_to")?
                    );
                }
                emit!("\n");
            }
            emit!(rule);
            emit!("   #   TAI->TB   | SAT->UT1      |\n");
            emit!(rule);
            for (i, o) in rows.iter().enumerate() {
                emit!(
                    "|%4d | %-10.9g | %-16.9e |\n",
                    i as i32 + 1,
                    read(&o.correctionTT_TB, "correctionTT_TB")?.to_f64(),
                    read(&o.correctionUT1, "correctionUT1")?.to_f64()
                );
            }
            emit!(rule);
        }
        if position > 6 {
            emit!(
                "\n\nEphemeris information:\n\n3-vector pointing at pulsar       = (%-13.13f,%-13.13f,%13.13f)\n",
                *read(&p.posPulsar[0], "posPulsar")?,
                *read(&p.posPulsar[1], "posPulsar")?,
                *read(&p.posPulsar[2], "posPulsar")?
            );
            emit!(
                "3-vector giving pulsar's velocity = (%-13.8e,%13.8e,%13.8e)\n",
                *read(&p.velPulsar[0], "velPulsar")?,
                *read(&p.velPulsar[1], "velPulsar")?,
                *read(&p.velPulsar[2], "velPulsar")?
            );
        }
        let rule = "--------------------------------------------------------------------------------------------------------------\n";
        if position > 7 {
            for part in 0..2 {
                emit!("\n\n");
                emit!(rule);
                emit!(if part == 0 {
                    "   JD            RCS(1)   RCS(2)     RCS(3)      RCB(1)     RCB(2)     RCB(3)    RBE(1)    RBE(2)    RBE(3)\n"
                } else {
                    "   JD            RCS(4)   RCS(5)     RCS(6)      RCB(4)     RCB(5)     RCB(6)    RBE(4)    RBE(5)    RBE(6)\n"
                });
                emit!(rule);
                for o in rows {
                    let jd = (read(&o.sat, "sat")?.clone()
                        + crate::clkcorr::get_correction_tt(o)? / 86400.0f64
                        + read(&o.correctionTT_TB, "correctionTT_TB")?.clone() / 86400.0f64
                        + 2400000.5f64)
                        .to_f64();
                    let mut args = vec![Argument::from(jd)];
                    for v in [&o.sun_ssb, &o.earthMoonBary_ssb, &o.earthMoonBary_earth] {
                        for x in &v[part * 3..part * 3 + 3] {
                            args.push(Argument::from(*read(x, "ephemeris vector")?));
                        }
                    }
                    io.write(&format(if part==0{"%-.5f %10.5f %-10.5f %-10.5f %-10.5f %-10.5f %-10.5f %-10.5f %-10.5f %-10.5f\n"}else{"%-.5lf %10.3e %-10.3e %-10.3e %-10.3e %-10.3e %-10.3e %-10.3e %-10.3e %-10.3e\n"},&args)?)?;
                }
                emit!(rule);
            }
            emit!(
                "MJD         OBS->EARTH1   OBS->EARTH2  OBS->EARTH3  SITEVEL1    SITEVEL2     SITEVEL3\n"
            );
            emit!(rule);
            for o in rows {
                let mut args = vec![Argument::from(
                    read(&o.sat, "sat")?.to_f64() + crate::clkcorr::get_correction_tt(o)? / 86400.0,
                )];
                for x in o.observatory_earth.iter().take(3).chain(o.siteVel.iter()) {
                    args.push(Argument::from(*read(x, "site vector")?));
                }
                io.write(&format(
                    "%-.5lf %12.5e %-12.5e %-12.5e %-12.5e %-12.5e %-12.5e\n",
                    &args,
                )?)?;
            }
            emit!(rule);
        }
        if position > 9 {
            emit!("\n\nExtra Delays:\n\n");
            emit!(rule);
            emit!("MJD          ShapiroSun          TDIS               Roemer           BAT\n");
            emit!(rule);
            for o in rows {
                emit!(
                    "%s %.12e %.12e %.10Lf %.10f\n",
                    print_longdouble(read(&o.sat, "sat")?),
                    *read(&o.shapiroDelaySun, "shapiroDelaySun")?,
                    *read(&o.tdis1, "tdis1")? + *read(&o.tdis2, "tdis2")?,
                    read(&o.roemer, "roemer")?.clone(),
                    read(&o.bat, "bat")?.to_f64()
                );
            }
            emit!(rule);
        }
    }
    Ok(())
}
