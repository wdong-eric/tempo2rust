use crate::{
    TimingError, clkcorr::ClockCorrections, ifteph::IfteEphemeris, read_ephemeris::JplEphemeris,
    tabulated_function::TabulatedFunction,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize)]
pub struct Resource {
    pub path: String,
    pub sha256: String,
    pub size_bytes: usize,
    pub role: String,
    pub coverage_mjd: Option<[f64; 2]>,
}

pub(crate) fn read_resource(
    root: &Path,
    path: &Path,
    role: &str,
    resources: &mut Vec<Resource>,
) -> Result<Vec<u8>, TimingError> {
    // The source resource loaders exit() on I/O errors; returning a contextual
    // Result keeps the host alive and prevents publication of a partial result.
    let data =
        fs::read(path).map_err(|e| TimingError::new(path, None, "resource", e.to_string()))?;
    let relative = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();
    let resource = Resource {
        path: relative,
        sha256: format!("{:x}", Sha256::digest(&data)),
        size_bytes: data.len(),
        role: role.into(),
        coverage_mjd: None,
    };
    if let Some(old) = resources.iter_mut().find(|r| r.path == resource.path) {
        *old = resource;
    } else {
        resources.push(resource);
    }
    Ok(data)
}

pub(crate) fn paths_with_extension(
    dir: &Path,
    extension: &str,
    optional: bool,
) -> Result<Vec<PathBuf>, TimingError> {
    let entries = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(e) if optional && e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => {
            return Err(TimingError::new(
                dir,
                None,
                "resource directory",
                e.to_string(),
            ));
        }
    };
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| TimingError::new(dir, None, "directory entry", e.to_string()))?
            .path();
        if path.extension().is_some_and(|s| s == extension) {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

pub(crate) struct Observatory {
    pub xyz: [f64; 3],
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub code: String,
    pub clock: String,
}

fn get_observatory(root: &Path, resources: &mut Vec<Resource>) -> Result<Observatory, TimingError> {
    // The declared profile accepts literal PKS, so the source's additional alias
    // lookup is outside this subset; no alias file is reported as read or applied.
    let main = root.join("observatory/observatories.dat");
    let mut paths = vec![main.clone()];
    paths.extend(
        paths_with_extension(&root.join("observatory"), "dat", false)?
            .into_iter()
            .filter(|p| p != &main),
    );
    let mut selected = None;
    for path in paths {
        let data = read_resource(root, &path, "observatory definitions", resources)?;
        let text = std::str::from_utf8(&data)
            .map_err(|e| TimingError::new(&path, None, "observatory", e.to_string()))?;
        for (line, s) in text.lines().enumerate() {
            if s.starts_with('#') || s.trim().is_empty() {
                continue;
            }
            let t: Vec<_> = s.split_whitespace().collect();
            let err = |m| TimingError::new(&path, Some(line + 1), "observatory", m);
            if t.len() < 5 {
                return Err(err("expected X Y Z NAME CODE [CLOCK]"));
            }
            let mut xyz = [0.0; 3];
            for i in 0..3 {
                xyz[i] = crate::tempo2_util::parse_binary64(t[i])
                    .map_err(|_| err("invalid coordinate"))?;
            }
            if xyz[2].abs() > 1.0 && xyz[2].abs() < 10000.0 {
                return Err(err(
                    "geodetic input is unsupported; require ITRF cartesian coordinates",
                ));
            }
            if selected.is_none()
                && (t[3].eq_ignore_ascii_case("pks") || t[4].eq_ignore_ascii_case("pks"))
            {
                selected = Some((
                    xyz,
                    t[4].to_string(),
                    t.get(5)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("UTC({})", t[4])),
                ));
            }
        }
    }
    let (xyz, code, clock) = selected
        .ok_or_else(|| TimingError::new(main, None, "PKS", "observatory definition missing"))?;
    // ITRF_to_GRS80, including its operation order and GRS80 constants.
    let [x, y, z] = xyz;
    let a = 6378137.0;
    let flattening = 1.0 / 298.257222101;
    let p = (x * x + y * y) / (a * a);
    let esq = flattening * (2.0 - flattening);
    let q = (1.0 - esq) / (a * a) * z * z;
    let r = (p + q - esq * esq) / 6.0;
    let s = esq * esq * p * q / (4.0 * r * r * r);
    let t = (1.0 + s + (s * (2.0 + s)).sqrt()).powf(1.0 / 3.0);
    let u = r * (1.0 + t + 1.0 / t);
    let v = (u * u + esq * esq * q).sqrt();
    let w = esq * (u + v - q) / (2.0 * v);
    let k = (u + v + w * w).sqrt() - w;
    let d = k * (x * x + y * y).sqrt() / (k + esq);
    let height = (k + esq - 1.0) / k * (d * d + z * z).sqrt();
    let latitude = 2.0 * z.atan2(d + (d * d + z * z).sqrt());
    let longitude = if y >= 0.0 {
        0.5 * std::f64::consts::PI - 2.0 * x.atan2((x * x + y * y).sqrt() + y)
    } else {
        -0.5 * std::f64::consts::PI + 2.0 * x.atan2((x * x + y * y).sqrt() - y)
    };
    // ITRF_to_GRS80 can propagate invalid coordinates as NaN; zero height also
    // gives tropo.C a zero zenith norm. Return a resource error before either
    // reaches a timing result (public resource-recovery example).
    if !height.is_finite() || !latitude.is_finite() || !longitude.is_finite() || height == 0.0 {
        return Err(TimingError::new(
            root,
            None,
            "PKS",
            "invalid GRS80 geometry",
        ));
    }
    Ok(Observatory {
        xyz,
        latitude,
        longitude,
        height,
        code,
        clock,
    })
}

pub(crate) struct MeteorologyFunction {
    pub site: String,
    pub table: TabulatedFunction,
}

/// Owned resources for the declared PKS / DE405 / IF99 / TT(TAI) path.
/// Construct separately from `load`; barycentric evaluation never needs a runtime.
pub struct RuntimeContext {
    pub(crate) root: PathBuf,
    pub(crate) resources: Vec<Resource>,
    pub(crate) observatory: Observatory,
    pub(crate) clocks: ClockCorrections,
    pub(crate) ifte: IfteEphemeris,
    pub(crate) jpl: JplEphemeris,
    pub(crate) eop: crate::eop::Eop,
    pub(crate) surface_pressure_tables: Vec<MeteorologyFunction>,
    pub(crate) zenith_wet_delay_tables: Vec<MeteorologyFunction>,
}
impl RuntimeContext {
    pub fn new(root: &Path) -> Result<Self, TimingError> {
        let root = fs::canonicalize(root)
            .map_err(|e| TimingError::new(root, None, "runtime root", e.to_string()))?;
        let mut resources = Vec::new();
        let observatory = get_observatory(&root, &mut resources)?;
        let clocks = ClockCorrections::load(&root, &mut resources)?;
        let path = root.join("ephemeris/TIMEEPH_short.te405");
        let ifte = IfteEphemeris::init(
            &path,
            &read_resource(&root, &path, "IF99 time ephemeris", &mut resources)?,
        )?;
        resources.last_mut().unwrap().coverage_mjd =
            Some([ifte.start_jd - 2400000.5, ifte.end_jd - 2400000.5]);
        let path = root.join("ephemeris/DE405.1950.2050");
        let jpl = JplEphemeris::open(
            &path,
            &read_resource(&root, &path, "DE405 planetary ephemeris", &mut resources)?,
        )?;
        resources.last_mut().unwrap().coverage_mjd =
            Some([jpl.start_jd - 2400000.5, jpl.end_jd - 2400000.5]);
        let path = root.join("earth/eopc04_IAU2000.62-now");
        let data = read_resource(&root, &path, "Earth orientation", &mut resources)?;
        let text = std::str::from_utf8(&data)
            .map_err(|e| TimingError::new(&path, None, "EOP", e.to_string()))?;
        let eop = crate::eop::Eop::load_eop(&path, text)?;
        resources.last_mut().unwrap().coverage_mjd = Some(eop.coverage());
        let mut surface_pressure_tables = Vec::new();
        let mut zenith_wet_delay_tables = Vec::new();
        for (extension, tables) in [
            ("sap", &mut surface_pressure_tables),
            ("zwd", &mut zenith_wet_delay_tables),
        ] {
            for path in paths_with_extension(&root.join("atmosphere"), extension, true)? {
                let data = read_resource(&root, &path, "meteorology table", &mut resources)?;
                let text = std::str::from_utf8(&data)
                    .map_err(|e| TimingError::new(&path, None, "meteorology", e.to_string()))?;
                let table = TabulatedFunction::parse(&path, text)?;
                resources.last_mut().unwrap().coverage_mjd = Some([table.start(), table.end()]);
                let site = table.header[1..]
                    .split_whitespace()
                    .next()
                    .ok_or_else(|| {
                        TimingError::new(&path, Some(1), "meteorology", "missing site name")
                    })?
                    .to_string();
                tables.push(MeteorologyFunction { site, table });
            }
        }
        Ok(Self {
            root,
            resources,
            observatory,
            clocks,
            ifte,
            jpl,
            eop,
            surface_pressure_tables,
            zenith_wet_delay_tables,
        })
    }
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }
}
