//! sw_delay.C source solar-wind model and its eleven independently callable helpers.
//! mul_add preserves measured GCC16 arm64 O2 contractions in AB's native profile.
use crate::{
    TimingError,
    clkcorr::ClockSession,
    clock_diagnostics::boundary,
    observation::{SourceObservation, read},
    preprocess::value,
    pulsar::SourcePulsar,
    runtime::Resource,
    tempo2::Label::*,
};
use std::{f64::consts::PI, io::Read, path::Path};

pub fn find_angle(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64 {
    let dlon = lon2 - lon1;
    let dlat = lat2 - lat1;
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    if a == 1.0 {
        PI
    } else {
        2.0 * a.sqrt().atan2((1.0 - a).sqrt())
    }
}
pub fn read_current_sheet(
    path: &Path,
    lon: &mut [f32],
    lat: &mut [f32],
    resources: &mut Vec<Resource>,
) -> Result<usize, TimingError> {
    crate::clkcorr::with_process(|s| read_sheet(path, lon, lat, resources, &mut s.diagnostics))
}
fn read_sheet(
    path: &Path,
    lon: &mut [f32],
    lat: &mut [f32],
    resources: &mut Vec<Resource>,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<usize, TimingError> {
    let (mut file, resource) = crate::runtime::open_resource(path, "solar current sheet")
        .inspect_err(|_| {
            use std::os::unix::ffi::OsStrExt;
            d.stdout(
                &[
                    b"Unable to open file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b"\n",
                ]
                .concat(),
            );
        })?;
    resources.push(resource);
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)
        .map_err(|e| TimingError::new(path, None, "current sheet", e.to_string()))?;
    let mut offset = 0;
    let mut n = 0;
    while crate::clock_text::tokens(&bytes[offset..]).next().is_some() {
        let Some((a, k)) = crate::clock_text::float_prefix(&bytes[offset..]) else {
            return Err(TimingError::new(
                path,
                None,
                "current sheet",
                "source fscanf would make no progress",
            ));
        };
        offset += k;
        *lon.get_mut(n)
            .ok_or_else(|| boundary("current_sheet_longitude_capacity"))? = a;
        if crate::clock_text::tokens(&bytes[offset..]).next().is_none() {
            break;
        }
        let Some((b, k)) = crate::clock_text::float_prefix(&bytes[offset..]) else {
            return Err(TimingError::new(
                path,
                None,
                "current sheet",
                "source fscanf would make no progress",
            ));
        };
        offset += k;
        *lat.get_mut(n)
            .ok_or_else(|| boundary("current_sheet_latitude_capacity"))? = b;
        n += 1;
    }
    Ok(n)
}
pub fn output_results(lon: &[f64; 36], lat: &[f64; 36]) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        for i in 0..36 {
            let mut x = (lon[i] * 180.0 / PI) as f32;
            if x < 0.0 {
                x = (x as f64 + 360.0) as f32;
            }
            if x > 360.0 {
                x = (x as f64 - 360.0) as f32;
            }
            let y = (lat[i] * 180.0 / PI) as f32;
            s.diagnostics.stdout(
                &[
                    b"output: ".as_slice(),
                    &crate::clock_text::format_number(
                        x as f64,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b" ",
                    &crate::clock_text::format_number(
                        y as f64,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    b"\n",
                ]
                .concat(),
            );
        }
        Ok(())
    })
}
pub fn amod(a: f64, p: f64) -> Result<f64, TimingError> {
    if p == 0.0 {
        return Ok(0.0);
    }
    Ok((-p).mul_add(
        crate::ifteph::native_i32(a / p, "solar_amod_int")? as f64,
        a,
    ))
}
pub fn elsun2(iyr: i32, iday: i32, secs: f64) -> Result<(f64, f64, f64, f64), TimingError> {
    crate::clkcorr::with_process(|s| solar_position(iyr, iday, secs, &mut s.diagnostics))
}
fn solar_position(
    iyr: i32,
    iday: i32,
    secs: f64,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(f64, f64, f64, f64), TimingError> {
    if !(1..=199).contains(&iyr) {
        d.stdout(b"ERROR in elsun2\n");
        return Err(boundary("elsun2_year_range"));
    }
    let fday = secs as f32 as f64 / 86400.0;
    let idd = 365 * iyr + (iyr - 1) / 4 + iday;
    let dj = idd as f64 + fday - 0.5;
    let t = dj / 36525.0;
    let rad = 57.29578;
    let vl = amod(0.9856473354f64.mul_add(dj, 279.696678), 360.0)?;
    let gst = amod(
        360.0f64.mul_add(fday, 0.9856473354f64.mul_add(dj, 279.690983)) + 180.0,
        360.0,
    )?;
    let g = amod(0.985600267f64.mul_add(dj, 358.475845), 360.0)? / rad;
    let sun = 0.020094f64.mul_add(
        (2.0 * g).sin(),
        (-0.004789f64).mul_add(t, 1.91946).mul_add(g.sin(), vl),
    );
    let obliquity = (-0.0130125f64).mul_add(t, 23.45229) / rad;
    let slp = (sun - 0.005686) / rad;
    let sind = obliquity.sin() * slp.sin();
    let cosd = (-sind).mul_add(sind, 1.0).sqrt();
    let dec = rad * (sind / cosd).atan();
    let cot = obliquity.cos() / obliquity.sin();
    let ra = (-rad).mul_add((sind / cosd * cot).atan2(-slp.cos() / cosd), 180.0);
    Ok((sun / rad, gst, ra / rad, dec / rad))
}
pub fn calc_rot_n(lon: f64, rot: f64) -> Result<(i32, i32, f64), TimingError> {
    let mut b =
        crate::ifteph::native_i32(0.5 + lon * 180.0 / PI, "calcRotN_longitude_int")? as f64 + 180.0;
    let n = crate::ifteph::native_i32(rot, "calcRotN_rotation_int")?;
    if lon <= PI {
        Ok((
            n,
            n.checked_add(1)
                .ok_or_else(|| boundary("calcRotN_integer_overflow"))?,
            b,
        ))
    } else {
        if b > 360.0 {
            b -= 360.0;
        }
        Ok((
            n.checked_sub(1)
                .ok_or_else(|| boundary("calcRotN_integer_overflow"))?,
            n,
            b,
        ))
    }
}
#[derive(Default, Debug)]
pub struct SolarDate {
    pub iyr: i32,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub day_of_year: Option<i32>,
}
pub fn mjd2date(mjd: i32, out: &mut SolarDate) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| calendar(mjd, out, &mut s.diagnostics))
}
fn calendar(
    mjd: i32,
    out: &mut SolarDate,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    let y = crate::ifteph::native_i32((mjd as f64 - 15078.2) / 365.25, "mjd2date_year_int")?;
    let ydays = crate::ifteph::native_i32(y as f64 * 365.25, "mjd2date_year_days")?;
    let m = crate::ifteph::native_i32(
        (mjd as f64 - 14956.1 - ydays as f64) / 30.6001,
        "mjd2date_month_int",
    )?;
    let mdays = crate::ifteph::native_i32(m as f64 * 30.6001, "mjd2date_month_days")?;
    out.day = i32::try_from(mjd as i64 - 14956 - ydays as i64 - mdays as i64)
        .map_err(|_| boundary("mjd2date_day_overflow"))?;
    let k = i32::from(m == 14 || m == 15);
    out.month = m - 1 - k * 12;
    out.iyr = y + k;
    out.year = out.iyr + 1900;
    let leap = out.year % 4 == 0 && out.year % 100 != 0 || out.year % 400 == 0;
    let offsets = if leap {
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335]
    } else {
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
    };
    if (1..=12).contains(&out.month) {
        out.day_of_year = Some(offsets[out.month as usize - 1] + out.day);
    } else {
        d.stdout(b"date error\n");
    } // source retains iday for invalid month
    Ok(())
}
pub fn convert_ecliptic(ra: f64, dec: f64) -> (f64, f64) {
    let epsilon = 23.439292 * (PI / 180.0);
    let sinb = dec.sin() * epsilon.cos() - dec.cos() * epsilon.sin() * ra.sin();
    let beta = sinb.asin();
    let y = ra.sin() * epsilon.cos() + dec.tan() * epsilon.sin();
    let x = ra.cos();
    let longitude = y.atan2(x);
    (
        if longitude < 0.0 {
            longitude + 2.0 * PI
        } else {
            longitude
        },
        beta,
    )
}
#[derive(Debug)]
pub struct MclOutput {
    pub helat: [f64; 36],
    pub crlon: [f64; 36],
    pub rots: [f64; 36],
    pub helate: f64,
    pub crlne: f64,
    pub rote: f64,
    pub elong: f64,
    pub beta: f64,
    pub dlon: f64,
    pub delng: f64,
    pub line_of_sight: i32,
}
impl Default for MclOutput {
    fn default() -> Self {
        Self {
            helat: [0.0; 36],
            crlon: [0.0; 36],
            rots: [0.0; 36],
            helate: 0.0,
            crlne: 0.0,
            rote: 0.0,
            elong: 0.0,
            beta: 0.0,
            dlon: 0.0,
            delng: 0.0,
            line_of_sight: 0,
        }
    }
}
#[allow(clippy::too_many_arguments)] // source arguments are independently meaningful quantities
pub fn mcl2(
    eclon: f64,
    eclat: f64,
    iyr: i32,
    iday: i32,
    secs: f64,
    velocity: f64,
    out: &mut MclOutput,
) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        line_of_sight(
            eclon,
            eclat,
            iyr,
            iday,
            secs,
            velocity,
            out,
            &mut s.diagnostics,
        )
    })
}
#[allow(clippy::too_many_arguments)] // source quantities plus the existing diagnostic session
fn line_of_sight(
    eclon: f64,
    eclat: f64,
    iyr: i32,
    iday: i32,
    secs: f64,
    velocity: f64,
    out: &mut MclOutput,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    let rad2deg = 180.0 / PI;
    let zlonan = 1.28573 + 7.89327 * (iyr as f64 + iday as f64 / 365.0 + 50.0) / 32400.0;
    let zkl = eclat.cos();
    let (ya, _, _, _) = solar_position(iyr, iday, secs, d)?;
    let cl0 = (ya - eclon).cos() * zkl;
    let dp = (-cl0).mul_add(cl0, 1.0).sqrt();
    out.elong = dp.atan2(cl0);
    let inc = 5.0 / rad2deg;
    let mut theta = PI / 2.0 - inc;
    // sw_delay.C:313: GCC16 O2 folds tan(PI/2 - 5/(180/PI)) to this
    // compiler-folded binary64 constant (AB profile); host tan is one ULP
    // away. Subsequent variable-angle iterations still call the source tan.
    let mut cl = dp.mul_add(11.430052302761348, cl0);
    let mut lp = 0;
    let mut last = false;
    loop {
        let clan = zlonan.cos();
        let slan = zlonan.sin();
        let x = [
            [clan, slan, 0.0],
            [-0.9920049497 * slan, 0.9920049497 * clan, 0.1262],
            [0.1261989691 * slan, -0.1261989691 * clan, 0.9920],
        ];
        let sl = ((1.0 - cl0 * cl0) + (cl0 - cl).powi(2)).sqrt();
        let snx = -ya.cos();
        let sny = -ya.sin();
        let mut a = [eclon.cos() * zkl, eclon.sin() * zkl, eclat.sin()];
        let mut b = [a[0].mul_add(cl, snx), a[1].mul_add(cl, sny), a[2] * cl];
        if lp > 35 {
            d.stdout(format!("ERROR: lp cannot be {lp}\n").as_bytes());
            return Err(boundary("mcl2_line_of_sight_capacity"));
        }
        let mut c = [0.0; 3];
        for i in 0..3 {
            for (j, aj) in a.iter().enumerate() {
                c[i] = x[i][j].mul_add(*aj, c[i]);
            }
        }
        out.beta = c[2].atan2(c[0].mul_add(c[0], c[1] * c[1]).sqrt());
        for i in 0..3 {
            a[i] = 0.0;
            for (j, bj) in b.iter().enumerate() {
                a[i] = x[i][j].mul_add(*bj, a[i]);
            }
        }
        out.helat[lp] = a[2].atan2(a[0].mul_add(a[0], a[1] * a[1]).sqrt());
        for i in 0..3 {
            b[i] = x[i][0].mul_add(snx, x[i][1] * sny);
        }
        let b1b2 = b[0].mul_add(b[0], b[1] * b[1]).sqrt();
        out.helate = b[2].atan2(b1b2);
        let mut cep =
            ((-b[0]).mul_add(c[0], -b[1] * c[1]) / b1b2) / c[0].mul_add(c[0], c[1] * c[1]).sqrt();
        if cep > 1.0 {
            cep = 1.0;
        }
        out.delng = (-cep).mul_add(cep, 1.0).sqrt().atan2(cep);
        let mut blp = a[1].atan2(a[0]);
        if blp < 0.0 {
            blp += 2.0 * PI;
        }
        let mut ble = b[1].atan2(b[0]);
        if ble < 0.0 {
            ble += 2.0 * PI;
        }
        let lag = 1.496e8 * sl / 3600.0 / velocity;
        let leap = (iyr - 69) / 4;
        let hr = 24.0 * ((iyr - 69) as f64 * 365.0 + leap as f64 + iday as f64 - 341.0)
            + secs / 3600.0
            - 14.77;
        let le1 = 228.42 - hr / 1.81835;
        let le2 = 228.42 - hr / 1.692002 + ble * rad2deg;
        out.crlne = 360.0 + amod(le2, 360.0)?;
        if out.crlne >= 360.0 {
            out.crlne = amod(out.crlne, 360.0)?;
        }
        let r1 = 1556.0 - le1 / 360.0;
        let r2 = 1556.0 - le2 / 360.0;
        out.rote = crate::ifteph::native_i32(r1, "mcl2_rot1_int")? as f64
            + (r2 - crate::ifteph::native_i32(r2, "mcl2_rot2_int")? as f64);
        if r1 - out.rote > 4.0 / 360.0 {
            out.rote += 1.0;
        }
        if r1 - out.rote < -4.0 / 360.0 {
            out.rote -= 1.0;
        }
        out.dlon = blp - ble;
        let delta = lag / 1.692002 + out.dlon * rad2deg;
        let lpp = le2 + delta;
        out.crlon[lp] = 360.0 + amod(lpp, 360.0)?;
        if out.crlon[lp] >= 360.0 {
            out.crlon[lp] = amod(out.crlon[lp], 360.0)?;
        }
        let rot = crate::ifteph::native_i32(out.rote, "mcl2_rote_int")? as f64;
        if out.crlon[lp] >= out.crlne {
            out.rots[lp] = rot + (1.0 - out.crlon[lp] / 360.0);
        } else {
            out.rots[lp] = rot - out.crlon[lp] / 360.0;
            if out.crlon[lp] / 360.0 < 0.01 {
                out.rots[lp] = rot - 0.01;
            }
        }
        out.crlon[lp] /= rad2deg;
        out.crlne /= rad2deg;
        theta -= inc;
        cl = dp.mul_add(theta.tan(), cl0);
        let finished = last;
        if cl < 0.0 {
            cl = 0.0;
            last = true;
        }
        lp += 1;
        if finished {
            break;
        }
    }
    out.line_of_sight = lp as i32;
    for i in lp..36 {
        out.helat[i] = out.helat[lp - 1];
        out.crlon[i] = out.crlon[lp - 1];
        out.rots[i] = out.rots[lp - 1];
    }
    Ok(())
}
pub fn solar_wind_model(
    p: &SourcePulsar,
    o: &SourceObservation,
    root: &Path,
    resources: &mut Vec<Resource>,
) -> Result<f64, TimingError> {
    crate::clkcorr::with_resources(root, |s| model(s, p, o, resources))
}
pub(crate) fn model(
    s: &mut ClockSession,
    p: &SourcePulsar,
    o: &SourceObservation,
    resources: &mut Vec<Resource>,
) -> Result<f64, TimingError> {
    s.diagnostics.version("sw_delay.C", "solarWindModel()");
    let (lon, lat) = match p.eclCoord {
        Some(1) => (
            value(p, param_raj, 0).to_f64(),
            value(p, param_decj, 0).to_f64(),
        ),
        Some(0) => convert_ecliptic(
            value(p, param_raj, 0).to_f64(),
            value(p, param_decj, 0).to_f64(),
        ),
        _ => return Err(boundary("solar_ecliptic_coordinates_unwritten")),
    };
    let mjd = read(&o.sat, "sat")?;
    let day = crate::ifteph::extended_i32(mjd, "solar_mjd_int")?;
    let mut date = SolarDate::default();
    calendar(day, &mut date, &mut s.diagnostics)?;
    let sec = ((mjd.clone() - day) * 86400i32).to_f64();
    let mut output = MclOutput::default();
    line_of_sight(
        lon,
        lat,
        date.iyr,
        date.day_of_year
            .ok_or_else(|| boundary("solar_day_of_year_unwritten"))?,
        sec,
        400.0,
        &mut output,
        &mut s.diagnostics,
    )?;
    let (r1, r2, b) = calc_rot_n(output.crlne, output.rote)?;
    s.diagnostics.stdout(
        &[
            format!("rotN output = {r1} {r2} ").as_bytes(),
            &crate::clock_text::format_number(b, crate::clock_text::NumberFormat::GeneralSix),
            b"\n",
        ]
        .concat(),
    );
    let mut integral = 0.0;
    let (mut fast, mut slow) = (0, 0);
    let mut longitude = [0.0f32; 500];
    let mut latitude = [0.0f32; 500];
    let deg = PI / 180.0;
    for i in 0..output.line_of_sight as usize {
        let number = crate::ifteph::native_i32(output.rots[i], "solar_sheet_rotation_int")?;
        let path = s
            .root
            .as_deref()
            .ok_or_else(|| boundary("solar_runtime_root"))?
            .join(format!("solarWindModel/CS{number}.txt"));
        let n = read_sheet(
            &path,
            &mut longitude,
            &mut latitude,
            resources,
            &mut s.diagnostics,
        )?;
        if n == 0 {
            return Err(boundary("solar_empty_current_sheet"));
        }
        let mut closest = find_angle(
            output.crlon[i],
            output.helat[i],
            longitude[0] as f64 * deg,
            latitude[0] as f64 * deg,
        ) / deg;
        for j in 1..n {
            let angle = find_angle(
                output.crlon[i],
                output.helat[i],
                longitude[j] as f64 * deg,
                latitude[j] as f64 * deg,
            ) / deg;
            if angle < closest {
                closest = angle;
            }
        }
        if closest < 20.0 {
            integral += 10.0;
            slow += 1;
        } else {
            integral += 2.5;
            fast += 1;
        }
    }
    let dm = integral * 5.0 * (PI / 180.0) * 4.85e-6 / output.elong.sin();
    let g = |v| {
        String::from_utf8(crate::clock_text::format_number(
            v,
            crate::clock_text::NumberFormat::GeneralSix,
        ))
        .unwrap()
    };
    s.diagnostics.stdout(
        format!(
            "Integral = {} {} {} {slow} {fast} {} {}\n",
            String::from_utf8(crate::clock_text::format_number(
                mjd.to_f64(),
                crate::clock_text::NumberFormat::GeneralFive
            ))
            .unwrap(),
            g(integral),
            g(dm),
            g(output.elong),
            g(output.elong / deg)
        )
        .as_bytes(),
    );
    Ok(dm)
}
