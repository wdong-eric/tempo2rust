//! Rust-owned resource/provenance context; no standalone source counterpart.
//! PKS geometry follows observatory.C::getObservatory/ITRF_to_GRS80; weather loading
//! follows tropo.C::initialize_meteorology_tables. Orchestrates Rust counterparts
//! of clkcorr.C::getClockCorrections, eop.C::load_EOP, ifteph.C::IFTE_init and
//! jpleph.c::jpl_init_ephemeris; resource hashing/ownership are Rust-only glue.
use crate::{TimingError, ifteph::IfteEphemeris, read_ephemeris::JplEphemeris};
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

pub(crate) use crate::observatory::Observatory;

/// Owned resources for the declared PKS / DE405 / IF99 / TT(TAI) path.
/// Construct separately from `load`; barycentric evaluation never needs a runtime.
pub struct RuntimeContext {
    pub(crate) root: PathBuf,
    pub(crate) resources: Vec<Resource>,
    pub(crate) observatory: Observatory,
    pub(crate) ifte: IfteEphemeris,
    pub(crate) jpl: JplEphemeris,
}
impl RuntimeContext {
    pub fn new(root: &Path) -> Result<Self, TimingError> {
        let root = fs::canonicalize(root)
            .map_err(|e| TimingError::new(root, None, "runtime root", e.to_string()))?;
        let mut resources = Vec::new();
        let observatory = crate::clkcorr::with_resources(&root, |session| {
            let observatory = session.observatories.get(
                b"PKS",
                session.root.as_deref(),
                session.alias.as_deref(),
                &mut session.diagnostics,
            )?;
            session.export_observatory_resources(&root, &mut resources);
            Ok(observatory)
        })?;
        // Existing timing geometry guard is outside L's selected lookup transfer.
        if !observatory.height.is_finite()
            || !observatory.latitude.is_finite()
            || !observatory.longitude.is_finite()
            || observatory.height == 0.0
        {
            return Err(TimingError::new(
                &root,
                None,
                "PKS",
                "invalid GRS80 geometry",
            ));
        }
        let path = root.join("ephemeris/TIMEEPH_short.te405");
        let ifte = IfteEphemeris::init(
            &path,
            &read_resource(&root, &path, "IF99 time ephemeris", &mut resources)?,
        )?;
        resources.last_mut().unwrap().coverage_mjd =
            Some([ifte.start_jd - 2400000.5, ifte.end_jd - 2400000.5]);
        let path = root.join("ephemeris/DE405.1950.2050");
        // Provenance and parsing consume one opened object. The digest records
        // bytes observed by this pass, not a snapshot guarantee against writers.
        use std::io::{Read, Seek, SeekFrom};
        let mut file = fs::File::open(&path)
            .map_err(|e| TimingError::new(&path, None, "JPL", e.to_string()))?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|e| TimingError::new(&path, None, "JPL", e.to_string()))?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| TimingError::new(&path, None, "JPL", e.to_string()))?;
        resources.push(Resource {
            path: path
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            sha256: format!("{:x}", Sha256::digest(&data)),
            size_bytes: data.len(),
            role: "DE405 planetary ephemeris".into(),
            coverage_mjd: None,
        });
        let jpl = JplEphemeris::open(&path, file)?;
        resources.last_mut().unwrap().coverage_mjd =
            Some([jpl.start_jd - 2400000.5, jpl.end_jd - 2400000.5]);
        crate::clkcorr::with_resources(&root, |session| {
            session.eop.initialize(
                Some(&root),
                b"earth/eopc04_IAU2000.62-now",
                &mut session.diagnostics,
            )?;
            if !session.meteorology.initialized {
                session
                    .meteorology
                    .initialize(Some(&root), 0, &mut session.diagnostics)?;
            }
            session.export_terrestrial_resources(&root, &mut resources);
            Ok(())
        })?;
        Ok(Self {
            root,
            resources,
            observatory,
            ifte,
            jpl,
        })
    }
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }
}
