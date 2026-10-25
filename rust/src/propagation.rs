//! calculate_bclt, shapiro_delay, dm_delays, formBats and secularMotion source chain.
use crate::{
    PRECISION_BITS, TimingError,
    clkcorr::ClockSession,
    observation::{SourceObservation, read, read_string},
    preprocess::{flags, is_set, value},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
    tempo2_util::dot_product,
};
use rug::Float;
use std::{f64::consts::PI, path::Path};
const AULTSC: f64 = 499.00478364;
fn vector(v: &[Option<f64>]) -> Result<[f64; 3], TimingError> {
    Ok([
        *read(&v[0], "vector[0]")?,
        *read(&v[1], "vector[1]")?,
        *read(&v[2], "vector[2]")?,
    ])
}
fn direction(p: &SourcePulsar, delt: f64) -> Result<[f64; 3], TimingError> {
    let mut pos = [0.0; 3];
    for (k, v) in pos.iter_mut().enumerate() {
        *v =
            read(&p.velPulsar[k], "velPulsar")?.mul_add(delt, *read(&p.posPulsar[k], "posPulsar")?);
    }
    // AB's GCC16/O2 objects contract the direction and its local norm in
    // this order; source dotproduct has its own independently measured order.
    let norm = pos[2]
        .mul_add(pos[2], pos[0].mul_add(pos[0], pos[1] * pos[1]))
        .sqrt();
    for x in &mut pos {
        *x /= norm;
    }
    Ok(pos)
}
fn sun_to_observer(o: &SourceObservation) -> Result<[f64; 3], TimingError> {
    let satellite = read_string(&o.telID, "telID")? == b"STL_FBAT";
    let mut r = [0.0; 3];
    for (j, x) in r.iter_mut().enumerate() {
        *x = -*read(&o.sun_ssb[j], "sun_ssb")?;
        if !satellite {
            *x += read(&o.earth_ssb[j], "earth_ssb")?;
        }
        *x += read(&o.observatory_earth[j], "observatory_earth")?;
    }
    Ok(r)
}
pub fn shapiro_delay(
    p: &SourcePulsar,
    o: &mut SourceObservation,
    delt: f64,
    dt_ssb: f64,
) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| shapiro(p, o, delt, dt_ssb, &mut s.diagnostics))
}
fn shapiro(
    p: &SourcePulsar,
    o: &mut SourceObservation,
    delt: f64,
    _dt_ssb: f64,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.version("shapiro_delay.C", "shapiro_delay()");
    let pos = direction(p, delt)?;
    o.psrPos = pos.map(Some);
    let site = read_string(&o.telID, "telID")?;
    if site == b"STL_BAT" || o.delayCorr == Some(0) {
        o.shapiroDelaySun = Some(0.0);
        o.shapiroDelayJupiter = Some(0.0);
    } else {
        let r = sun_to_observer(o)?;
        let norm = dot_product(r, r).sqrt();
        let cosine = dot_product(pos, r) / norm;
        o.shapiroDelaySun = Some(-2.0 * 4.925490947e-6 * (norm / AULTSC * (1.0 + cosine)).ln());
        let site_vector = vector(&o.observatory_earth)?;
        if site != b"STL_FBAT" {
            for (body, mass, out) in [
                (&o.jupiter_earth, 4.70255e-9, &mut o.shapiroDelayJupiter),
                (&o.saturn_earth, 1.40797e-9, &mut o.shapiroDelaySaturn),
                (&o.venus_earth, 1.2061e-11, &mut o.shapiroDelayVenus),
                (&o.uranus_earth, 2.14539e-10, &mut o.shapiroDelayUranus),
                (&o.neptune_earth, 2.54488e-10, &mut o.shapiroDelayNeptune),
            ] {
                let body = vector(body)?;
                let r = std::array::from_fn(|j| -body[j] + site_vector[j]);
                let norm = dot_product(r, r).sqrt();
                let cosine = dot_product(pos, r) / norm;
                let delay = -2.0 * mass * (norm / AULTSC * (1.0 + cosine)).ln();
                *out = Some(*read(&p.planetShapiro, "planetShapiro")? as f64 * delay);
            }
        } else {
            for out in [
                &mut o.shapiroDelayNeptune,
                &mut o.shapiroDelayJupiter,
                &mut o.shapiroDelaySaturn,
                &mut o.shapiroDelayUranus,
                &mut o.shapiroDelayVenus,
            ] {
                *out = Some(0.0);
            }
        }
    }
    Ok(())
}
/// storePrecision.C is intentionally a source no-op.
pub fn record_precision(_p: &mut SourcePulsar, _precision: f64, _name: &[u8], _comment: &[u8]) {}

pub fn dm_delays(
    context: &mut PulsarContext,
    index: usize,
    observation: usize,
    delt: f64,
    dt_ssb: f64,
    root: &Path,
    resources: &mut Vec<crate::runtime::Resource>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |s| {
        let (psrs, rows) = context.pulsars_observations_mut(index)?;
        dm(
            s,
            &psrs[0],
            &psrs[index],
            index,
            rows.get_mut(observation)
                .ok_or_else(|| crate::clock_diagnostics::boundary("DM_observation_index"))?,
            delt,
            dt_ssb,
            resources,
        )
    })
}
#[allow(clippy::too_many_arguments)] // source pulsar/TOA indices plus the shared session
fn dm(
    s: &mut ClockSession,
    first: &SourcePulsar,
    p: &SourcePulsar,
    index: usize,
    o: &mut SourceObservation,
    delt: f64,
    _dt_ssb: f64,
    resources: &mut Vec<crate::runtime::Resource>,
) -> Result<(), TimingError> {
    s.diagnostics.version("dm_delays.C", "dm_delays()");
    s.diagnostics.log(
        b'd',
        "dm_delays.C",
        50,
        format!(
            "dm_delays with pulsar {index}; number of obs = {}",
            *read(&p.nobs, "nobs")?
        )
        .as_bytes(),
    );
    if o.delayCorr == Some(0) {
        o.tdis1 = Some(0.0);
        o.tdis2 = Some(0.0);
        return Ok(());
    }
    let pos = direction(p, delt)?;
    let rsa = sun_to_observer(o)?;
    let satellite = read_string(&o.telID, "telID")? == b"STL_FBAT";
    let mut velocity = [0.0; 3];
    for (j, v) in velocity.iter_mut().enumerate() {
        if satellite {
            if is_set(p, param_telx, 1) && is_set(p, param_tely, 1) && is_set(p, param_telz, 1) {
                *v = value(p, [param_telx, param_tely, param_telz][j], 1).to_f64();
            } else {
                s.diagnostics.log(
                    b'e',
                    "dm_delays.C",
                    94,
                    b"setting vobs = 0, this can give a large error!",
                );
                *v = 0.0;
            }
        } else {
            *v = *read(&o.earth_ssb[j + 3], "earth velocity")? + read(&o.siteVel[j], "siteVel")?;
        }
    }
    let r = dot_product(rsa, rsa).sqrt();
    let cosine = dot_product(pos, rsa) / r;
    let voverc = dot_product(pos, velocity);
    let mut freq = *read(&o.freq, "freq")? * 1.0e6 * (1.0 - voverc);
    if *read(&p.dilateFreq, "dilateFreq")? != 0
        && freq > 0.0
        && *read(&o.einsteinRate, "einsteinRate")? != 0.0
    {
        freq /= o.einsteinRate.unwrap();
    }
    o.freqSSB = Some(freq);
    let dt = read(&o.sat, "sat")?.clone() - value(p, param_dmepoch, 0);
    let yrs = (dt.clone() / 365.25f64).to_f64();
    let taylor = first.dm_series_type == Some(crate::pulsar::SeriesType::series_taylor_pn);
    let (mut arg, mut series, mut dm_dot) = (Float::with_val(PRECISION_BITS, 1), 1.0, 0.0);
    for k in 1..9 {
        arg *= yrs;
        if taylor {
            series *= k as f64;
        }
        if is_set(p, param_dm, k) {
            dm_dot += (value(p, param_dm, k) * &arg / series).to_f64();
        }
    }
    let mut dm_value = value(p, param_dm, 0).to_f64();
    // AB's measured macOS profile has built-in long double == binary64. cmval
    // is that type; TKlongdouble.h nevertheless maps powl to powq below.
    let mut cm = value(p, param_cm, 0).to_f64();
    arg = Float::with_val(PRECISION_BITS, 1);
    series = 1.0;
    for k in 1..9 {
        arg *= yrs;
        if taylor {
            series *= k as f64;
        }
        if is_set(p, param_cm, k) {
            cm += (value(p, param_cm, k) * &arg / series).to_f64();
        }
    }
    if is_set(p, param_dmmodel, 0) {
        let mean = crate::tempo2_util::get_parameter_value_inner(
            p,
            param_dmmodel as usize,
            0,
            &mut s.diagnostics,
        )?
        .to_f64();
        // dm_delays.C:165 reads linkTo[0] even when initialise.C has only
        // set nLinkTo=0. A fresh unwritten slot represents no link; retain every
        // defined slot (including a caller's target with nLinkTo=0).
        let link = &first.param.slots[param_dmmodel as usize];
        let target = match link.link_to[0] {
            Some(target) => target,
            None if link.n_link_to == Some(0) => -1,
            None => *read(&link.link_to[0], "DMMODEL linkTo[0]")?,
        };
        if is_set(p, param_dm, 0)
            && target != param_dm as i32
            && s.source.dmmodel_warning & 0xf != 0
        {
            s.diagnostics.log(
                b'e',
                "dm_delays.C",
                168,
                b"Using DMMODEL value for dispersion measure instead of the DM parameter",
            );
            s.diagnostics.log(
                b'e',
                "dm_delays.C",
                169,
                b"Please set 'DMMODEL DM' in the par file rather than specifying a value",
            );
            s.source.dmmodel_warning &= 0xfff0;
        }
        let n = *read(&p.dmoffsDMnum, "dmoffsDMnum")?;
        let times = (0..n)
            .map(|k| read(&p.dmoffsDM_mjd[k as usize], "dmoffsDM_mjd").copied())
            .collect::<Result<Vec<_>, _>>()?;
        let vals = (0..n)
            .map(|k| read(&p.dmoffsDM[k as usize], "dmoffsDM").copied())
            .collect::<Result<Vec<_>, _>>()?;
        if s.source.dmmodel_warning & 0xf0 != 0 {
            let mut m = 0.0;
            for v in &vals {
                m += v;
            }
            m /= n as f64;
            if m > 1e-12 {
                s.diagnostics.stdout(
                    &[
                        b"WARNING: Mean DMOFF = ".as_slice(),
                        &crate::clock_text::format_number(
                            m,
                            crate::clock_text::NumberFormat::GeneralSix,
                        ),
                        b"\n",
                    ]
                    .concat(),
                );
            }
            s.source.dmmodel_warning &= 0xff0f;
        }
        dm_value = mean
            + crate::ifunc::interpolate(
                &times,
                &vals,
                read(&o.sat, "sat")?.to_f64(),
                n,
                &mut s.diagnostics,
            )?;
    } else {
        for k in 0..*read(&p.ndmx, "ndmx")? as usize {
            if read(&o.sat, "sat")? > &value(p, param_dmxr1, k)
                && read(&o.sat, "sat")? < &value(p, param_dmxr2, k)
            {
                dm_value =
                    (Float::with_val(PRECISION_BITS, dm_value) + value(p, param_dmx, k)).to_f64();
            }
        }
        let mut previous_offset = None;
        for (id, val) in flags(o)? {
            if id == b"-dm"
                && let Some((v, _)) = crate::clock_text::number_prefix(&val)
            {
                dm_value = v;
            }
            if id == b"-dmo" {
                if let Some((v, _)) = crate::clock_text::number_prefix(&val) {
                    previous_offset = Some(v);
                }
                dm_value += previous_offset.ok_or_else(|| {
                    crate::clock_diagnostics::boundary("DM_flag_offset_unwritten")
                })?;
            }
        }
    }
    for k in 0..*read(&p.nTNShapeletEvents, "nTNShapeletEvents")? as usize {
        if *read(&p.TNShapeletEvFScale[k], "TNShapeletEvFScale")? == 2.0 {
            // dm_delays.C:241 tests BAT==0 to select SAT during the first
            // pass, but initialise.C leaves BAT unwritten. Supply that sentinel
            // only before its first write; subsequent BAT values remain intact.
            let mut t = o
                .bat
                .get_or_insert_with(|| Float::with_val(PRECISION_BITS, 0))
                .to_f64();
            if t == 0.0 {
                t = read(&o.sat, "sat")?.to_f64();
            }
            let n = *read(&first.TNShapeletEvN[k], "TNShapeletEvN")?;
            let coef = (0..n)
                .map(|j| read(&first.TNShapeletEvCoef[k][j as usize], "TNShapeletEvCoef").copied())
                .collect::<Result<Vec<_>, _>>()?;
            dm_value += crate::shapelet::evaluate_shapelet(
                n,
                *read(&first.TNShapeletEvPos[k], "TNShapeletEvPos")?,
                *read(&first.TNShapeletEvWidth[k], "TNShapeletEvWidth")?,
                &coef,
                t,
            )?;
        }
    }
    dm_value += dm_dot;
    if is_set(p, param_dm_sin1yr, 0) {
        dm_value = (Float::with_val(PRECISION_BITS, dm_value)
            + value(p, param_dm_sin1yr, 0) * (dt.clone() * (2.0 * PI / 365.25)).sin())
        .to_f64();
    }
    if is_set(p, param_dm_cos1yr, 0) {
        dm_value = (Float::with_val(PRECISION_BITS, dm_value)
            + value(p, param_dm_cos1yr, 0) * (dt * (2.0 * PI / 365.25)).cos())
        .to_f64();
    }
    o.tdis1 = Some(if freq <= 1.0 {
        0.0
    } else {
        dm_value / 2.41e-4 / 1.0e-12 / freq / freq
    });
    if *read(&p.TNChromIdx, "TNChromIdx")? != 0.0 && freq >= 1.0 {
        for k in 0..*read(&p.nchromx, "nchromx")? as usize {
            if read(&o.sat, "sat")? > &value(p, param_chromxr1, k)
                && read(&o.sat, "sat")? < &value(p, param_chromxr2, k)
            {
                cm = (Float::with_val(PRECISION_BITS, cm) + value(p, param_chromx, k)).to_f64();
            }
        }
        // TKlongdouble.float128.h:20 maps powl to powq. The binary64
        // arguments are promoted; multiplication and addition stay extended
        // until assignment to tdis1. A binary64 pow loses the source semantics.
        use rug::ops::Pow;
        let power = Float::with_val(PRECISION_BITS, freq / 1.4e9).pow(-p.TNChromIdx.unwrap());
        o.tdis1 = Some((power * cm + o.tdis1.unwrap()).to_f64());
    }
    if freq <= 1.0 || p.ipm == Some(0) {
        o.tdis2 = Some(0.0);
        o.spherical_solar_wind = Some(0.0);
    } else if p.swm == Some(0) {
        let scale = 1.0e6 * 1.49598e11 * 1.49598e11 / 299792458.0 / 7.436e6 * cosine.acos()
            / r
            / (-cosine).mul_add(cosine, 1.0).sqrt()
            / freq
            / freq;
        o.spherical_solar_wind = Some(scale);
        o.tdis2 = Some(*read(&p.ne_sw, "ne_sw")? * scale);
        if p.param.slots[param_ne_sw_sin as usize].cells[0].param_set != 0 {
            let seasonal =
                (-((read(&o.sat, "sat")?.clone() - 58818.0f64) * (2.0 * PI) / 4018.0f64).cos()
                    * value(p, param_ne_sw_sin, 0))
                .to_f64();
            o.tdis2 = Some(seasonal.mul_add(scale, o.tdis2.unwrap()));
        }
        let n = *read(&p.ne_sw_ifuncN, "ne_sw_ifuncN")?;
        if n > 0 {
            let t = (0..n)
                .map(|i| read(&p.ne_sw_ifuncT[i as usize], "ne_sw_ifuncT").copied())
                .collect::<Result<Vec<_>, _>>()?;
            let v = (0..n)
                .map(|i| read(&p.ne_sw_ifuncV[i as usize], "ne_sw_ifuncV").copied())
                .collect::<Result<Vec<_>, _>>()?;
            let v = crate::ifunc::interpolate(
                &t,
                &v,
                read(&o.sat, "sat")?.to_f64(),
                n,
                &mut s.diagnostics,
            )?;
            o.tdis2 = Some(v.mul_add(scale, o.tdis2.unwrap()));
        }
    } else if p.swm == Some(1) {
        o.tdis2 =
            Some(crate::solar_wind::model(s, p, o, resources)? / 2.41e-4 / 1.0e-12 / freq / freq);
    }
    Ok(())
}

pub fn calculate_bclt(
    context: &mut PulsarContext,
    root: &Path,
    resources: &mut Vec<crate::runtime::Resource>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |s| calculate(s, context, resources))
}
pub(crate) fn calculate(
    s: &mut ClockSession,
    context: &mut PulsarContext,
    resources: &mut Vec<crate::runtime::Resource>,
) -> Result<(), TimingError> {
    s.diagnostics
        .version("calculate_bclt.C", "calculate_bclt()");
    if let Some(p) = context.pulsars.first() {
        s.diagnostics.log(
            b'd',
            "calculate_bclt.C",
            49,
            format!(
                "In calculate_bclt with number of psr = {}, nobs (psr[0]) = {}",
                context.pulsars.len(),
                *read(&p.nobs, "nobs")?
            )
            .as_bytes(),
        );
    }
    let g = |v| {
        String::from_utf8(crate::clock_text::format_number(
            v,
            crate::clock_text::NumberFormat::GeneralSix,
        ))
        .unwrap()
    };
    #[allow(clippy::excessive_precision)] // Preserve the pinned source literal.
    let px_conv: f64 = 1.74532925199432958e-2 / 3600.0e3;
    for index in 0..context.pulsars.len() {
        let (psrs, rows) = context.pulsars_observations_mut(index)?;
        let p = &psrs[index];
        let n = (*read(&p.nobs, "nobs")?).max(0) as usize;
        if p.correctTroposphere == Some(1) {
            s.diagnostics
                .log(b'd', "calculate_bclt.C", 58, b"Correcting troposphere");
            s.diagnostics
                .version("tropo.C", "computer_tropospheric_delays()");
            let mut warned = false;
            for (i, o) in rows[..n].iter_mut().enumerate() {
                crate::tropo::compute_observation(
                    s,
                    &crate::tropo::TropospherePulsar {
                        source: p,
                        position: None,
                        warnings: None,
                    },
                    o,
                    i,
                    &mut warned,
                )?;
            }
        }
        if p.correctTroposphere == Some(1) {
            s.diagnostics.log(
                b'd',
                "calculate_bclt.C",
                60,
                b"Complete correcting troposphere",
            );
        }
        for (obs, o) in rows[..n].iter_mut().enumerate() {
            // calculate_bclt.C:77 writes psr[0] in a loop over p. With a second
            // pulsar this leaves its disabled troposphere field unwritten. Write
            // the selected pulsar's observation, as the enabled branch does.
            if p.correctTroposphere == Some(0) {
                o.troposphericDelay = Some(0.0);
            }
            s.diagnostics.log(
                b'd',
                "calculate_bclt.C",
                78,
                format!(
                    "In tdis2 calculate_bclt with observation {obs} {}",
                    *read(&o.delayCorr, "delayCorr")?
                )
                .as_bytes(),
            );
            if o.delayCorr == Some(0) {
                o.freqSSB = o.freq;
                o.roemer = Some(Float::with_val(PRECISION_BITS, 0));
                let delta = ((read(&o.sat, "sat")?.clone() - value(p, param_posepoch, 0))
                    / 36525.0f64)
                    .to_f64();
                o.psrPos = direction(p, delta)?.map(Some);
                continue;
            }
            let sat = read_string(&o.telID, "telID")? == b"STL_FBAT";
            let mut rca = vector(&o.observatory_earth)?;
            if !sat {
                for (j, x) in rca.iter_mut().enumerate() {
                    #[allow(clippy::assign_op_pattern)] // Preserve source operand order.
                    {
                        *x = *read(&o.earth_ssb[j], "earth_ssb")? + *x;
                    }
                }
            }
            let radial = if is_set(p, param_pmrv, 0) {
                (value(p, param_pmrv, 0) * (2.0 * PI / 360.0) / 36000.0f64).to_f64()
            } else {
                0.0
            };
            let pos = vector(&p.posPulsar[..])?;
            let vel = vector(&p.velPulsar[..])?;
            let acc = vector(&p.accPulsar[..])?;
            let (mut dt_ssb, mut loops) = (0.0f64, 0);
            loop {
                let old = dt_ssb;
                let rcos = dot_product(pos, rca);
                let rr = dot_product(rca, rca);
                let delta = ((read(&o.sat, "sat")?.clone() - value(p, param_posepoch, 0)
                    + (Float::with_val(PRECISION_BITS, crate::clkcorr::get_correction_tt(o)?)
                        + read(&o.correctionTT_TB, "correctionTT_TB")?
                        + dt_ssb)
                        / 86400.0f64)
                    / 36525.0f64)
                    .to_f64();
                let vdot = dot_product(vel, rca);
                let speed = dot_product(vel, vel).sqrt();
                let pm = delta * vdot;
                let pmtt = -0.5 * speed * speed * delta * delta * rcos;
                let acct = 0.5 * delta * delta * dot_product(acc, rca);
                let px = (value(p, param_px, 0) * -0.5f64 * px_conv * (rr - rcos * rcos) / AULTSC)
                    .to_f64();
                let pmtr = -delta.powi(2) * radial * vdot;
                s.diagnostics.log(b'd',"calculate_bclt.C",151,format!("Calculating roemer using {} {} {} {} {} (delt = {}; dt_SSB = {}) loop={loops} sat={}",g(rcos),g(pm),g(pmtt),g(px),g(pmtr),g(delta),g(dt_ssb),g(read(&o.sat,"sat")?.to_f64())).as_bytes());
                o.roemer = Some(Float::with_val(
                    PRECISION_BITS,
                    rcos + pm + pmtt + px + pmtr + acct,
                ));
                shapiro(p, o, delta, dt_ssb, &mut s.diagnostics)?;
                s.diagnostics.log(
                    b'd',
                    "calculate_bclt.C",
                    154,
                    format!(
                        "In tdis2 calculate_bclt with observation {obs} {} calling dmdelays",
                        *read(&o.delayCorr, "delayCorr")?
                    )
                    .as_bytes(),
                );
                dm(s, &psrs[0], p, index, o, delta, dt_ssb, resources)?;
                s.diagnostics
                    .log(b'd', "calculate_bclt.C", 157, b"Complete dm_delays");
                dt_ssb = (read(&o.roemer, "roemer")?.clone()
                    - (*read(&o.tdis1, "tdis1")? + read(&o.tdis2, "tdis2")?)
                    - (*read(&o.shapiroDelaySun, "shapiroDelaySun")?
                        + *read(&p.planetShapiro, "planetShapiro")? as f64
                            * read(&o.shapiroDelayJupiter, "shapiroDelayJupiter")?))
                .to_f64();
                s.diagnostics.log(
                    b'd',
                    "calculate_bclt.C",
                    165,
                    format!(
                        "dt_SSB: {} {} {} {} {}",
                        g(read(&o.roemer, "roemer")?.to_f64()),
                        g(*read(&o.tdis1, "tdis1")?),
                        g(*read(&o.tdis2, "tdis2")?),
                        g(*read(&o.shapiroDelaySun, "shapiroDelaySun")?),
                        g(*read(&p.planetShapiro, "planetShapiro")? as f64
                            * read(&o.shapiroDelayJupiter, "shapiroDelayJupiter")?)
                    )
                    .as_bytes(),
                );
                if s.source.very_fast == 1 {
                    break;
                }
                loops += 1;
                if !((dt_ssb - old).abs() > 1e-10 && o.deleted != Some(1) && loops < 100) {
                    break;
                }
            }
            if loops == 100 {
                s.diagnostics.log(
                    b'w',
                    "calculate_bclt.C",
                    174,
                    b"Warning: the loop to obtain dt_SSB reached 100 iterations\n",
                );
            }
        }
    }
    s.diagnostics
        .log(b'd', "calculate_bclt.C", 181, b"Leaving calculate_bclt");
    Ok(())
}
pub fn form_bats(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| form_bats_inner(context, &mut s.diagnostics))
}
pub(crate) fn form_bats_inner(
    context: &mut PulsarContext,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.version("formBats.C", "formBats()");
    d.log(b'd', "formBats.C", 45, b"In formBats");
    for index in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(index)?;
        for o in &mut rows[..(*read(&p.nobs, "nobs")?).max(0) as usize] {
            if o.delayCorr == Some(0) {
                o.bat = o.sat.clone();
            } else {
                let shapiro = if p.calcShapiro == Some(-1) {
                    0.0
                } else {
                    *read(&o.shapiroDelaySun, "shapiroDelaySun")?
                        + *read(&p.planetShapiro, "planetShapiro")? as f64
                            * (*read(&o.shapiroDelayVenus, "shapiroDelayVenus")?
                                + read(&o.shapiroDelayJupiter, "shapiroDelayJupiter")?
                                + read(&o.shapiroDelaySaturn, "shapiroDelaySaturn")?
                                + read(&o.shapiroDelayUranus, "shapiroDelayUranus")?
                                + read(&o.shapiroDelayNeptune, "shapiroDelayNeptune")?)
                };
                let delays = read(&o.correctionTT_TB, "correctionTT_TB")?.clone()
                    - read(&o.troposphericDelay, "troposphericDelay")?
                    + read(&o.roemer, "roemer")?
                    - shapiro
                    - read(&o.tdis1, "tdis1")?
                    - read(&o.tdis2, "tdis2")?;
                o.batCorr = Some(
                    Float::with_val(
                        PRECISION_BITS,
                        crate::clkcorr::get_correction_tt(o)? / 86400.0f64,
                    ) + delays.clone() / 86400.0f64,
                );
                o.bat = Some(
                    read(&o.sat, "sat")?.clone()
                        + crate::clkcorr::get_correction_tt(o)? / 86400.0f64
                        + delays / 86400.0f64,
                );
                // formBats.C:83 reads shklovskii before the first secularMotion call.
                // Initialise only an unwritten correction; the immediately following
                // secularMotion replaces it using the model. Repeated calls retain it.
                if o.shklovskii.is_none() {
                    o.shklovskii = Some(Float::with_val(PRECISION_BITS, 0));
                }
                o.bbat = Some(
                    read(&o.bat, "bat")?.clone()
                        - read(&o.shklovskii, "shklovskii")?.clone() / 86400.0f64,
                );
            }
        }
    }
    d.log(b'd', "formBats.C", 87, b"Leaving formBats");
    Ok(())
}
pub fn secular_motion(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| secular_inner(context, &mut s.diagnostics))
}
pub(crate) fn secular_inner(
    context: &mut PulsarContext,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.version("secularMotion.C", "secularMotion()");
    if context.pulsars.is_empty() {
        return Ok(());
    }
    let first = &context.pulsars[0];
    let enabled = is_set(first, param_dshk, 0)
        && (is_set(first, param_pmra, 0) || is_set(first, param_pmdec, 0));
    // The L-suffixed constants are binary64 in AB's measured native profile.
    let kpc = Float::with_val(PRECISION_BITS, 3.08568025e19f64);
    let mas = Float::with_val(PRECISION_BITS, 1.536281850e-16f64);
    for index in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(index)?;
        for o in &mut rows[..(*read(&p.nobs, "nobs")?).max(0) as usize] {
            let delay = if enabled {
                let t = (read(&o.bat, "bat")?.clone() - value(p, param_posepoch, 0)) * 86400.0f64;
                t.clone() * t / 2.0f64 / 299792458.0f64
                    * (crate::tempo2_util::get_parameter_value_inner(p, param_dshk as usize, 0, d)?
                        * &kpc)
                    * (value(p, param_pmra, 0) * value(p, param_pmra, 0) * &mas * &mas
                        + value(p, param_pmdec, 0) * value(p, param_pmdec, 0) * &mas * &mas)
            } else {
                Float::with_val(PRECISION_BITS, 0)
            };
            o.shklovskii = Some(delay.clone());
            o.bbat = Some(read(&o.bat, "bat")?.clone() - delay / 86400.0f64);
        }
    }
    Ok(())
}
