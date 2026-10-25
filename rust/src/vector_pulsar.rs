//! vectorPulsar.C::vectorPulsar and its tempo2Util.C frame rotations at fda6ad4.
use crate::{
    PRECISION_BITS, TimingError, clkcorr::ClockSession, clock_diagnostics::boundary,
    observation::read, pulsar_lifecycle::PulsarContext, tempo2::Label::*,
};
use rug::Float;

/// Shared fixed-direction calculation; source vectors are not normalized here.
pub(crate) fn position(alpha: f64, delta: f64) -> [f64; 3] {
    [
        alpha.cos() * delta.cos(),
        alpha.sin() * delta.cos(),
        delta.sin(),
    ]
}

/// tempo2Util.C::equ2ecl/ecl2equ: double input, extended rotation, double writes.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "released canonical driver/helper; general input orchestration remains separate"
    )
)]
pub(crate) fn rotate(x: &mut [f64; 3], to_equatorial: bool) {
    // LD_PI is the M_PIq decimal literal in the accepted compiler macro map.
    let pi = Float::with_val(
        PRECISION_BITS,
        Float::parse("3.141592653589793238462643383279502884").unwrap(),
    );
    let angle = (pi / 648000.0f64) * 84381.4059f64;
    let ce = angle.clone().cos();
    let se = angle.sin();
    let y = Float::with_val(PRECISION_BITS, x[1]);
    let z = Float::with_val(PRECISION_BITS, x[2]);
    x[1] = (Float::with_val(PRECISION_BITS, &ce * &y)
        + if to_equatorial {
            -Float::with_val(PRECISION_BITS, &se * &z)
        } else {
            Float::with_val(PRECISION_BITS, &se * &z)
        })
    .to_f64();
    x[2] = (if to_equatorial {
        Float::with_val(PRECISION_BITS, &se * &y)
    } else {
        -Float::with_val(PRECISION_BITS, &se * &y)
    } + Float::with_val(PRECISION_BITS, &ce * &z))
    .to_f64();
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "released canonical driver/helper; general input orchestration remains separate"
    )
)]
pub(crate) fn vector_pulsar(
    session: &mut ClockSession,
    context: &mut PulsarContext,
    npsr: i32,
) -> Result<(), TimingError> {
    session
        .diagnostics
        .version("vectorPulsar.C", "vectorPulsar()");
    for p in 0..npsr {
        let psr = context
            .pulsars
            .get_mut(p as usize)
            .ok_or_else(|| boundary("vector_pulsar_storage"))?;
        let val = |label| {
            psr.param
                .parameter(label)
                .cell(0)
                .map(|c| c.value().clone())
                .map_err(|_| boundary("vector_parameter_storage"))
        };
        let alpha = val(param_raj)?.to_f64();
        let delta = val(param_decj)?.to_f64();
        let ca = alpha.cos();
        let sa = alpha.sin();
        let cd = delta.cos();
        let sd = delta.sin();
        let pos = position(alpha, delta);
        *psr.posPulsar = pos.map(Some);
        *psr.posPulsarEquatorial = pos.map(Some);
        if *read(&psr.eclCoord, "eclCoord")? != 0 {
            let mut equ = pos;
            rotate(&mut equ, true);
            *psr.posPulsarEquatorial = equ.map(Some);
        }
        for (ra, dec, acceleration) in [
            (param_pmra, param_pmdec, false),
            (param_pmra2, param_pmdec2, true),
        ] {
            let a = psr
                .param
                .parameter(ra)
                .cell(0)
                .map_err(|_| boundary("vector_parameter_storage"))?
                .value();
            let d = psr
                .param
                .parameter(dec)
                .cell(0)
                .map_err(|_| boundary("vector_parameter_storage"))?
                .value();
            let mut convert = 1.0 / 1000.0 / 60.0 / 60.0 * std::f64::consts::PI / 180.0 * 100.0;
            if acceleration {
                convert *= 100.0;
            }
            // Keep the uncancelled cos(dec), extended PM coefficients and source grouping.
            let x = ((-a.clone() / delta.cos() * sa * cd) - (d.clone() * ca * sd)) * convert;
            let y = ((a.clone() / delta.cos() * ca * cd) - (d.clone() * sa * sd)) * convert;
            let z = (d.clone() * cd) * convert;
            let out = [Some(x.to_f64()), Some(y.to_f64()), Some(z.to_f64())];
            if acceleration {
                *psr.accPulsar = out;
            } else {
                *psr.velPulsar = out;
            }
        }
    }
    Ok(())
}
