//! polyco.C: polyco, atimfake, tzFit, chebpc and pcshft at fda6ad4.
use crate::{
    PRECISION_BITS, RuntimeContext, TimingError,
    observation::{read, read_string, write_string},
    predictor::{self, error},
    predictor_number::N,
    pulsar_lifecycle::PulsarContext,
    t1polyco::{T1Polyco, T1PolycoSet, scientific},
    tempo2::Label::*,
    tksvd::ld,
};
use rug::Float;
use serde::Serialize;
use std::path::{Path, PathBuf};
#[derive(Clone, Debug)]
pub struct PolycoRequest {
    pub start_mjd: Float,
    pub end_mjd: Float,
    pub span_minutes: i32,
    pub coefficients: usize,
    pub max_hour_angle: Float,
    pub site: String,
    pub frequency_mhz: Float,
    pub true_dm: bool,
    pub file_prefix: PathBuf,
}
#[derive(Debug)]
pub struct ArtificialToas {
    pub tmin: Vec<Float>,
    pub val: Vec<Float>,
    pub tmid_mjd: Float,
    pub nsets: usize,
}
#[derive(Debug, Serialize)]
pub struct PolycoResult {
    pub models: T1PolycoSet,
    pub coefficients: Vec<crate::FitNumber>,
    pub files: Vec<PathBuf>,
    pub warnings: Vec<String>,
}
fn file(prefix: &Path, suffix: &str) -> PathBuf {
    let mut p = prefix.as_os_str().to_os_string();
    p.push(suffix);
    p.into()
}
pub fn atimfake(
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    r: &PolycoRequest,
    afmjd: &Float,
) -> Result<ArtificialToas, TimingError> {
    if r.span_minutes <= 0 || r.max_hour_angle <= 0 || !(2..=31).contains(&r.coefficients) {
        return Err(error(
            "polyco requires span > 0, maxha > 0, 2..=31 fitted coefficients",
        ));
    }
    let solsid = ld(1.002737909f64);
    let alng = if r.site == "@" || r.site.eq_ignore_ascii_case("bat") {
        0.0f64
    } else {
        crate::clkcorr::with_resources(&runtime.root, |s| {
            let obs = s.observatories.get(
                r.site.as_bytes(),
                s.root.as_deref(),
                s.alias.as_deref(),
                &mut s.diagnostics,
            )?;
            s.export_observatory_resources(&runtime.root, &mut runtime.resources);
            Ok((-obs.xyz[1]).atan2(obs.xyz[0]))
        })?
    };
    let hlst = crate::tempo2_util::fortran_mod(
        solsid.clone() * afmjd + 0.154374f64 - alng / (2.0f64 * std::f64::consts::PI),
        &ld(1.0f64),
    )? * 24.0f64;
    let raj = context.pulsars[0].param.slots[param_raj as usize].cells[0]
        .val
        .clone()
        / std::f64::consts::PI
        * 12.0f64;
    let integer = crate::ifteph::extended_i32(&raj, "polyco RA hour")?;
    let minutes = crate::ifteph::extended_i32(&((raj - integer) * 60.0f64), "polyco RA minute")?;
    let rax = integer as f64 + (minutes + 2) as f64 / 60.0f64;
    let mut wait = (ld(rax) - hlst) / &solsid;
    if wait < -r.max_hour_angle.clone() {
        wait += ld(24.0f64) / &solsid;
    }
    let mjd = afmjd.clone()
        + (wait - &r.max_hour_angle) / 24.0f64
        + r.span_minutes as f64 / (2.0f64 * 1440.0f64);
    let nmjd = crate::ifteph::extended_i32(&mjd, "polyco day")?;
    let half = ((mjd - nmjd) * 48f64).to_f64();
    let rounded = crate::ifteph::native_i32(
        half + if half > 0.0f64 { 0.5f64 } else { -0.5f64 },
        "polyco half hour",
    )?;
    let first = cut_fraction(ld(rounded as f64 / 48.0f64))?;
    let sets = crate::ifteph::extended_i32(
        &((ld(120.0f64) * &r.max_hour_angle + r.span_minutes - 1) / r.span_minutes),
        "polyco nsets",
    )?;
    // fda6ad4 polyco.C:262 checks i>800, but val[i], tmin[i+1] and
    // tzFit phase[i+1] each have 800 cells. Check the one-based extent before
    // any write. ncoeff=32 also reads unwritten c[31] (31 construction samples).
    if sets <= 0 || sets > 25 {
        return Err(error(
            "polyco artificial sample count exceeds one-based 800-cell storage",
        ));
    }
    let nsets = sets as usize;
    let count = 31 * nsets + 1;
    context.ensure_observation_capacity(0, count)?;
    context.pulsars[0].nobs = Some(count as i32);
    let b = r.span_minutes as f64 / 2.0f64 + 5.0f64;
    let x: Vec<_> = (1..=31)
        .map(|k| (ld(std::f64::consts::PI) * (ld(k as f64) - 0.5f64) / 31.0f64).cos() * b)
        .collect();
    let mut tmin = vec![ld(0.0f64); count];
    let mut val = vec![];
    let mut mjd2 = first.clone();
    {
        let (_, rows) = context.pulsar_observations_mut(0)?;
        for j in 0..nsets {
            mjd2 = first.clone() + ld(j as f64) * r.span_minutes / 1440.0f64;
            for x in &x {
                let v = cut_fraction(mjd2.clone() + x.clone() / 1440.0f64)?;
                let val0 = val.first().unwrap_or(&v);
                let i = val.len();
                tmin[i + 1] = (v.clone() - val0) * 1440.0f64;
                predictor::synthetic(
                    &mut rows[i + 1],
                    v.clone() + nmjd,
                    r.frequency_mhz.to_f64(),
                    r.site.as_bytes(),
                    b"POLYCO",
                )?;
                // fda6ad4 polyco.C::tzFit:398 reads tdis1/2 for trueDM, but
                // calculate_bclt.C:80 bypasses dm_delays for barycentric rows.
                // Apply dm_delays.C:52-56's explicit no-correction zero state
                // here; do not preserve undefined/stale delays across site reuse.
                if rows[i + 1].delayCorr == Some(0) {
                    rows[i + 1].tdis1 = Some(0.0);
                    rows[i + 1].tdis2 = Some(0.0);
                }
                val.push(v);
            }
        }
    }
    predictor::reference(context)?;
    {
        let (p, rows) = context.pulsar_observations_mut(0)?;
        write_string(&mut rows[0].fname, b"POLYCO_REF", "fname")?;
        let bary = read_string(&p.tzrsite[..], "tzrsite")? == b"@";
        rows[0].clockCorr = Some(i32::from(!bary));
        rows[0].delayCorr = Some(i32::from(!bary));
    }
    Ok(ArtificialToas {
        tmin,
        val,
        tmid_mjd: mjd2,
        nsets,
    })
}
fn cut_fraction(v: Float) -> Result<Float, TimingError> {
    // The L literals here are ordinary long double, even in polyco's timing
    // expressions. Preserve target rounding before promoting them to Rug128.
    let e8 = N::parse_prefix(b"1e8").map_err(error)?;
    let a = v * Float::with_val(PRECISION_BITS, e8.as_float());
    let i = crate::ifteph::extended_i32(&a, "polyco fractional truncation")?;
    let j = crate::ifteph::extended_i32(&((a - i) * 100.0f64), "polyco fractional remainder")?;
    // fda6ad4 polyco.C:230/279: measured GCC16 arm64 contracts the
    // native-long-double reconstruction into FMADD after the second product.
    let result = N::parse_prefix(b"1e-10").map_err(error)?.source_mul_add(
        &N::from(j as f64),
        &(N::parse_prefix(b"1e-8").map_err(error)? * i as f64),
    );
    Ok(Float::with_val(PRECISION_BITS, result.as_float()))
}
pub fn chebpc(c: &[Float]) -> Result<Vec<Float>, TimingError> {
    let count = c.len();
    if count == 0 || count > 50 {
        return Err(error("chebpc requires 1..=50 coefficients"));
    }
    let mut d = vec![ld(0.0f64); count];
    let mut dd = d.clone();
    d[0] = c[count - 1].clone();
    for j in (1..count - 1).rev() {
        for k in (1..=count - 1 - j).rev() {
            let sv = d[k].clone();
            d[k] = d[k - 1].clone() * 2.0f64 - &dd[k];
            dd[k] = sv;
        }
        let sv = d[0].clone();
        d[0] = -dd[0].clone() + &c[j];
        dd[0] = sv;
    }
    for j in (1..count).rev() {
        d[j] = d[j - 1].clone() - &dd[j];
    }
    d[0] = -dd[0].clone() + c[0].clone() * 0.5f64;
    Ok(d)
}
pub fn pcshft(a: &Float, b: &Float, d: &mut [Float]) -> Result<(), TimingError> {
    if d.is_empty() || a == b {
        return Err(error("pcshft requires coefficients and distinct endpoints"));
    }
    let constant = ld(2.0f64) / (b.clone() - a);
    let mut fac = constant.clone();
    for v in d.iter_mut().skip(1) {
        *v *= &fac;
        fac *= &constant;
    }
    let constant: Float = (a.clone() + b) * 0.5f64;
    for j in 1..d.len() {
        for k in (j..d.len()).rev() {
            d[k - 1] = d[k - 1].clone() - constant.clone() * &d[k];
        }
    }
    Ok(())
}
#[derive(Debug)]
pub struct PolycoFit {
    pub model: T1Polyco,
    pub timing_coefficients: Vec<Float>,
    pub phase: Float,
    pub dm: Float,
    pub mid: Float,
    pub binary_text: String,
}
pub fn tz_fit(
    context: &PulsarContext,
    samples: &ArtificialToas,
    r: &PolycoRequest,
) -> Result<Vec<PolycoFit>, TimingError> {
    let p = &context.pulsars()[0];
    let rows = predictor::observations(context)?;
    let f0 = predictor::spin_frequency(context)?;
    let mut out = vec![];
    for nb in 0..samples.nsets {
        let iref = (nb + 1) * 31 - 15;
        let rtime = &samples.tmin[iref];
        let sat = read(&rows[iref].sat, "sat")?;
        let day = crate::ifteph::extended_i32(sat, "polyco day")?;
        let mid = sat.clone() - day;
        let phase = read(&rows[iref].phase, "phase")?.clone();
        let mut t = vec![];
        let mut ph: Vec<Float> = vec![];
        let range = nb * 31 + 1..=(nb + 1) * 31;
        for (row, time) in rows[range.clone()].iter().zip(&samples.tmin[range]) {
            let dt = time.clone() - rtime;
            ph.push(read(&row.phase, "phase")?.clone() - &phase - dt.clone() * &f0 * 60.0f64);
            t.push(dt);
        }
        let mut c = vec![];
        for j in 0..31 {
            let mut sum = ld(0.0f64);
            for k in 1..=31 {
                let angle: Float =
                    ld(std::f64::consts::PI * j as f64) * (ld(k as f64 - 0.5f64) / 31.0f64);
                sum += ph[k - 1].clone() * angle.cos();
            }
            c.push(sum * (2.0f64 / 31.0f64));
        }
        let mut coeff = chebpc(&c[..r.coefficients])?;
        let b = ld(r.span_minutes as f64 / 2.0f64 + 5.0f64);
        pcshft(&(-b.clone()), &b, &mut coeff)?;
        let mut sq = ld(0.0f64);
        for j in 0..31 {
            let mut pred = coeff[0].clone();
            let mut arg = t[j].clone();
            for d in coeff.iter().skip(1) {
                pred += d.clone() * &arg;
                arg *= &t[j];
            }
            let diff = pred - &ph[j];
            sq += diff.clone() * diff;
        }
        let rms_us = ((sq / 31.0f64).sqrt() * 1e6f64 / &f0).to_f64();
        let log10rms = (ld(1e-6 * rms_us) * &f0).log10().to_f64();
        let sec = crate::ifteph::extended_i32(
            &crate::tempo2_util::fortran_mod(mid.clone() * 86400.0f64 + 0.005f64, &ld(86400.0f64))?,
            "polyco UTC",
        )?;
        let hour = (sec as f64 / 3600.0f64) as i32;
        let minute = ((sec - 3600 * hour) as f64 / 60.0f64) as i32;
        let utc = 10000.0f64 * hour as f64
            + 100.0f64 * minute as f64
            + (sec - 3600 * hour - 60 * minute) as f64;
        let mut date = crate::solar_wind::SolarDate::default();
        crate::solar_wind::mjd2date(day, &mut date)?;
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let date_string = format!(
            "{:2}-{}-{:02}",
            date.day,
            months
                .get((date.month - 1) as usize)
                .ok_or_else(|| error("polyco calendar month"))?,
            date.year.rem_euclid(100)
        );
        let (mut binary_text, mut binary_phase, mut binary_frequency) =
            (" ".to_owned(), 0.0f64, 0.0f64);
        if p.param.slots[param_pb as usize].cells[0].param_set == 1 {
            let ct =
                read(&rows[1].bat, "bat")?.clone() + (sat.clone() - read(&rows[1].sat, "sat")?);
            let factor =
                ld(8.64e4) / (p.param.slots[param_pb as usize].cells[0].val.clone() * 86400.0f64);
            let phi = crate::tempo2_util::fortran_mod(
                (ct - &p.param.slots[param_t0 as usize].cells[0].val) * &factor + 90000.0f64,
                &ld(1.0f64),
            )?;
            binary_text = format!(
                "{:>7}{:>9}",
                String::from_utf8(crate::source_text::fixed(&phi, 4)).unwrap(),
                String::from_utf8(crate::source_text::fixed(&factor, 4)).unwrap()
            );
            binary_phase = phi.to_f64();
            binary_frequency = factor.to_f64();
        }
        let obs = &rows[iref];
        let freq = *read(&obs.freq, "freq")?;
        let ssb = *read(&obs.freqSSB, "freqSSB")?;
        let doppler = (ssb - freq * 1e6) / (freq * 1e6) * (1e6 / 100.0f64);
        let dm = if r.true_dm {
            ld((*read(&obs.tdis1, "tdis1")? + *read(&obs.tdis2, "tdis2")?)
                * 2.41e-4
                * 1e-12
                * ssb
                * ssb)
        } else {
            p.param.slots[param_dm as usize].cells[0].val.clone()
        };
        let mut model = T1Polyco::new(coeff.iter().map(N::from_float).collect()).map_err(error)?;
        model.psrname = String::from_utf8(read_string(&p.name[..], "name")?)
            .map_err(|e| error(e.to_string()))?;
        model.date_string = date_string;
        model.utc_string = format!("{utc:.2}");
        model.mjd_mid = N::from_float(sat);
        model.dm = dm.to_f64();
        model.doppler = doppler;
        model.log10rms = log10rms;
        model.reference_phase = N::from_float(&phase);
        model.frequency_psr_0 = N::from_float(&f0);
        model.sitename = r.site.clone();
        model.span = r.span_minutes;
        model.frequency_obs = r.frequency_mhz.to_f64();
        model.binary_phase = binary_phase;
        model.binary_frequency = binary_frequency;
        out.push(PolycoFit {
            model,
            timing_coefficients: coeff,
            phase,
            dm,
            mid: sat.clone(),
            binary_text,
        });
    }
    Ok(out)
}
pub fn polyco(
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    r: &PolycoRequest,
) -> Result<PolycoResult, TimingError> {
    if !r.start_mjd.is_finite() || !r.end_mjd.is_finite() || r.end_mjd < r.start_mjd {
        return Err(error("invalid polyco MJD range"));
    }
    context.pulsars[0].param.slots[param_track as usize].cells[0].param_set = 0;
    let paths = vec![
        file(&r.file_prefix, "polyco_new.dat"),
        file(&r.file_prefix, "newpolyco.dat"),
        file(&r.file_prefix, "polyco.tim"),
    ];
    let (mut legacy, mut modern) = (String::new(), String::new());
    let mut models = T1PolycoSet::default();
    let mut coefficients = vec![];
    let tsid = ld(1.0)
        / "1.002737909"
            .parse::<crate::Scalar>()
            .map_err(error)?
            .as_float();
    let mut af = r.start_mjd.clone();
    while af <= r.end_mjd {
        let samples = atimfake(context, runtime, r, &af)?;
        predictor::timing(context, runtime, &paths[2])?;
        let f0 = predictor::spin_frequency(context)?;
        for fit in tz_fit(context, &samples, r)? {
            let m = &fit.model;
            let name = m.psrname.strip_prefix('J').unwrap_or(&m.psrname);
            legacy.push_str(&format!("{:<10.10} {:>9.9}{:>11}{:>20}{:>21.6} {:>6.3}{:>7.3}\n{:>20}{:>18.12}{:>5}{:>5}{:>5}{:>10.3}{:>16}\n",name,m.date_string,m.utc_string,N::from(fit.mid.to_f64()).fixed(11),m.dm,m.doppler,m.log10rms,String::from_utf8(crate::source_text::fixed(&fit.phase,6)).unwrap(),f0.to_f64(),m.sitename,m.span,r.coefficients,m.frequency_obs,fit.binary_text));
            for (i, c) in fit.timing_coefficients.iter().enumerate() {
                legacy.push_str(&format!("{:>25}", scientific(&ld(c.to_f64()), 17)));
                if (i + 1) % 3 == 0 {
                    legacy.push('\n');
                }
            }
            if !r.coefficients.is_multiple_of(3) {
                legacy.push('\n');
            }
            modern.push_str(&format!("TEMPO2: POLYCO TEMPO1 emulation\n{:<10.10}\n{:<9.9}\n{:<11}\n{}\n{:<25}\n{:<10.7}\n{:<7.3}\n{}\n{}\n{:<5}\n{:<5}\n{:<5}\n{:<10.3}\n{}\n",name,m.date_string,m.utc_string,String::from_utf8(crate::source_text::fixed(&fit.mid,20)).unwrap(),String::from_utf8(crate::source_text::fixed(&fit.dm,10)).unwrap(),m.doppler,m.log10rms,String::from_utf8(crate::source_text::fixed(&fit.phase,15)).unwrap(),String::from_utf8(crate::source_text::fixed(&f0,20)).unwrap(),m.sitename,m.span,r.coefficients,m.frequency_obs,if fit.binary_text.len()<2{"0.000000 0.00000"}else{&fit.binary_text}));
            for c in &fit.timing_coefficients {
                modern.push_str(&scientific(c, 30));
                modern.push('\n');
            }
            coefficients = fit
                .timing_coefficients
                .into_iter()
                .map(crate::FitNumber)
                .collect();
            models.segments.push(fit.model);
        }
        af += &tsid;
    }
    // polyco.C:128/322 ignores failed fopen and may dereference NULL. Safe
    // publication reports I/O errors and emits no success; partial files remain
    // visible, not an atomic-publication claim about the original routine.
    std::fs::write(&paths[0], legacy).map_err(|e| error(e.to_string()))?;
    std::fs::write(&paths[1], modern).map_err(|e| error(e.to_string()))?;
    Ok(PolycoResult {
        models,
        coefficients,
        files: paths,
        warnings: if context.pulsars[0].tempo1 == Some(0) {
            vec!["Should probably use -tempo1 option".into()]
        } else {
            vec![]
        },
    })
}
