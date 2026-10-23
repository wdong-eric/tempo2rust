//! Selected observatory.C lookup, initialization, file/alias parsing and geometry.
//! The alias vector intentionally accumulates across lines; lookup/warnings have
//! process lifetime independently of the clock index (decision 0023).
use crate::runtime::Resource;
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{self, c_bytes},
    tabulated_function::{chunks, os_error},
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub(crate) struct Observatory {
    pub xyz: [f64; 3],
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub code: Vec<u8>,
    pub name: Vec<u8>,
    pub clock: Vec<u8>,
}
#[derive(Default)]
pub(crate) struct ObservatoryDatabase {
    pub initialized: bool,
    pub observatories: Vec<Observatory>,
    pub aliases: Vec<AliasList>,
    pub resources: Vec<(PathBuf, Resource)>,
}
pub(crate) struct AliasList {
    pub code: Vec<u8>,
    pub aliases: Vec<Vec<u8>>,
    pub warned: bool,
}

pub(crate) use crate::clock_glob::files as source_files;
pub(crate) fn resource(
    path: &Path,
    data: &[u8],
    role: &str,
    coverage: Option<[f64; 2]>,
) -> Resource {
    Resource {
        path: path.to_string_lossy().into_owned(),
        sha256: format!("{:x}", Sha256::digest(data)),
        size_bytes: data.len(),
        role: role.into(),
        coverage_mjd: coverage,
    }
}
impl ObservatoryDatabase {
    fn read_file(&mut self, path: &Path, diagnostics: &mut Diagnostics) -> Result<(), TimingError> {
        diagnostics.version("observatory.C", "observatory()");
        let data = fs::read(path).map_err(|e| {
            diagnostics.stderr(
                &[
                    b"Unable to open observatory data file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b" : ",
                    os_error(&e).as_bytes(),
                    b"\n",
                ]
                .concat(),
            );
            terminal(path, "observatory_open")
        })?;
        self.resources.push((
            path.into(),
            resource(path, &data, "observatory definitions", None),
        ));
        let mut xyz: [Option<f64>; 3] = [None; 3];
        let mut strings: [Option<Vec<u8>>; 3] = [None, None, None];
        for (line_number, line) in chunks(&data).enumerate() {
            if line.first() == Some(&b'#') {
                continue;
            }
            let mut tail = c_bytes(line);
            let mut nread = 0;
            for x in &mut xyz {
                if let Some((value, n)) = clock_text::number_prefix(tail) {
                    *x = Some(value);
                    tail = &tail[n..];
                    nread += 1;
                } else {
                    break;
                }
            }
            if nread == 3 {
                for (i, token) in clock_text::tokens(tail).take(3).enumerate() {
                    if token.len() >= [32, 16, 16][i] {
                        return Err(boundary("observatory_string_storage"));
                    }
                    strings[i] = Some(token.to_vec());
                    nread += 1;
                }
            }
            if nread == 0 {
                continue;
            }
            let z = xyz[2].ok_or_else(|| boundary("observatory_unwritten_coordinate"))?;
            if z.abs() < 10000.0 && z.abs() > 1.0 {
                let [x, y, z] = [
                    xyz[0].unwrap(),
                    xyz[1].ok_or_else(|| boundary("observatory_unwritten_coordinate"))?,
                    z,
                ];
                let latitude = fang(x)?;
                let longitude = -fang(y)?;
                let flattening = 1.0 / 298.257222101;
                let esq = flattening * (2.0 - flattening);
                let n = 6378137.0 / (1.0 - esq * latitude.sin() * latitude.sin()).sqrt();
                let coordinates = [
                    (n + z) * latitude.cos() * longitude.cos(),
                    (n + z) * latitude.cos() * longitude.sin(),
                    (n * (1.0 - esq) + z) * latitude.sin(),
                ];
                diagnostics.stdout(&[format!("\n\nERROR: The observatory coordinates for line {} in ",line_number+1).as_bytes(),path.as_os_str().as_bytes(),b"\n appear to be in TEMPO geodetic format. Tempo2 requires cartesian geocentric\n coordinates w.r.t the ITRF.\n ASSUMING your coordinates are geodetic in the ITRF realisation of the GRS80\n reference ellipsoid (unlikely to be precisely true), then the geocentric\n coordinates should be specified as follows:\n",format!(" {:.2} {:.2} {:.2}\n",coordinates[0],coordinates[1],coordinates[2]).as_bytes()].concat());
                return Err(terminal(path, "observatory_geodetic"));
            }
            if nread != 5 && nread != 6 {
                diagnostics.stderr(
                    &[
                        format!("Error parsing line {} of file ", line_number + 1).as_bytes(),
                        path.as_os_str().as_bytes(),
                        b":\n",
                        c_bytes(line),
                        b"\n",
                    ]
                    .concat(),
                );
                return Err(terminal(path, "observatory_row"));
            }
            let code = strings[1].as_ref().unwrap().clone();
            if nread == 5 {
                let name = [b"UTC(".as_slice(), &code, b")"].concat();
                strings[2] = Some(name[..name.len().min(15)].to_vec());
            }
            self.observatories.push(geometry(
                xyz.map(Option::unwrap),
                code,
                strings[0].as_ref().unwrap().clone(),
                strings[2].as_ref().unwrap().clone(),
            ));
        }
        diagnostics.log(
            b'd',
            "observatory.C",
            144,
            format!(
                "readObservatoryFile: {} entries\n",
                self.observatories.len()
            )
            .as_bytes(),
        );
        Ok(())
    }
    fn read_aliases(
        &mut self,
        path: &Path,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        let data = fs::read(path).map_err(|e| {
            diagnostics.stderr(
                &[
                    b"Unable to open observatory aliases file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b" : ",
                    os_error(&e).as_bytes(),
                    b"\n",
                ]
                .concat(),
            );
            diagnostics.log(
                b'e',
                "observatory.C",
                163,
                &[
                    b"Cannot read '".as_slice(),
                    path.as_os_str().as_bytes(),
                    b"'. Possibly $TEMPO2 is not set, or you have not updated the contents.",
                ]
                .concat(),
            );
            terminal(path, "observatory_alias_open")
        })?;
        self.resources.push((
            path.into(),
            resource(path, &data, "observatory aliases", None),
        ));
        let mut aliases = Vec::new();
        for line in chunks(&data) {
            if line.first() == Some(&b'#') {
                continue;
            }
            let mut words = clock_text::tokens(line);
            if let Some(code) = words.next() {
                if code.len() >= 100 {
                    return Err(boundary("observatory_alias_code_storage"));
                }
                for word in words {
                    if word.len() >= 128 {
                        return Err(boundary("observatory_alias_storage"));
                    }
                    if word.starts_with(b"#") {
                        break;
                    }
                    aliases.push(word.to_vec());
                }
                self.aliases.push(AliasList {
                    code: code.to_vec(),
                    aliases: aliases.clone(),
                    warned: false,
                });
            }
        }
        Ok(())
    }
    pub fn initialize(
        &mut self,
        root: Option<&Path>,
        alias: Option<&[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        if self.initialized {
            return Ok(());
        }
        diagnostics.log(b'd', "observatory.C", 205, b"In initObservatories");
        let root = root.ok_or_else(|| boundary("observatory_environment_storage"))?;
        let path = root.join("observatory/observatories.dat");
        if path.as_os_str().as_bytes().len() >= 1024 {
            return Err(boundary("observatory_path_storage"));
        }
        diagnostics.log(
            b'd',
            "observatory.C",
            211,
            &[
                b"Reading observatories file >".as_slice(),
                path.as_os_str().as_bytes(),
                b"<",
            ]
            .concat(),
        );
        self.read_file(&path, diagnostics)?;
        diagnostics.log(b'd', "observatory.C", 213, b"Reading other .dat files");
        let pattern = root.join("observatory/*.dat");
        if pattern.as_os_str().as_bytes().len() >= 1024 {
            return Err(boundary("observatory_path_storage"));
        }
        diagnostics.log(
            b'd',
            "observatory.C",
            216,
            &[
                b"Reading individual files >".as_slice(),
                pattern.as_os_str().as_bytes(),
                b"<",
            ]
            .concat(),
        );
        diagnostics.log(b'd', "observatory.C", 219, b"Return from glob = 0");
        diagnostics.log(b'd', "observatory.C", 220, b"Glob success");
        for other in source_files(&root.join("observatory"), b"dat") {
            if other != path {
                diagnostics.log(
                    b'd',
                    "observatory.C",
                    225,
                    &[b"Reading >".as_slice(), other.as_os_str().as_bytes(), b"<"].concat(),
                );
                self.read_file(&other, diagnostics)?;
            }
        }
        diagnostics.log(b'd', "observatory.C", 230, b"Reading aliases file");
        let path = if let Some(alias) = alias {
            diagnostics.log(
                b'm',
                "observatory.C",
                233,
                &[b"Note: using alias file ".as_slice(), alias, b".aliases"].concat(),
            );
            root.join("observatory")
                .join(std::ffi::OsStr::from_bytes(&[alias, b".aliases"].concat()))
        } else {
            root.join("observatory/aliases")
        };
        if path.as_os_str().as_bytes().len() >= 1024 {
            return Err(boundary("observatory_path_storage"));
        }
        self.read_aliases(&path, diagnostics)?;
        self.initialized = true;
        diagnostics.log(b'd', "observatory.C", 241, b"Leaving initObservatories");
        Ok(())
    }
    pub fn get(
        &mut self,
        code: &[u8],
        root: Option<&Path>,
        alias: Option<&[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<Observatory, TimingError> {
        let code = c_bytes(code);
        self.initialize(root, alias, diagnostics)?;
        let mut resolved = code;
        if let Some(list) = self
            .aliases
            .iter_mut()
            .find(|l| l.aliases.iter().any(|a| a.eq_ignore_ascii_case(code)))
        {
            if !list.warned {
                diagnostics.log(
                    b'w',
                    "observatory.C",
                    272,
                    &[
                        b"Assuming site '".as_slice(),
                        code,
                        b"' means '",
                        &list.code,
                        b"'",
                    ]
                    .concat(),
                );
                list.warned = true;
            }
            resolved = &list.code;
        }
        if resolved.len() >= 1024 {
            return Err(boundary("observatory_lookup_storage"));
        }
        if let Some(obs) = self.observatories.iter().find(|o| {
            o.code.eq_ignore_ascii_case(resolved) || o.name.eq_ignore_ascii_case(resolved)
        }) {
            return Ok(obs.clone());
        }
        diagnostics.stderr(&[b"Observatory code '".as_slice(), code, b"' not found!\n"].concat());
        Err(terminal(
            root.unwrap_or(Path::new("")),
            "observatory_not_found",
        ))
    }
}
fn fang(f: f64) -> Result<f64, TimingError> {
    let a = f / 1.0e4;
    if !a.is_finite() || a.trunc() < i32::MIN as f64 || a.trunc() > i32::MAX as f64 {
        return Err(boundary("observatory_angle_integer"));
    }
    let ia = a as i32;
    let b = (f - ia as f64 * 1.0e4) / 1.0e2;
    if !b.is_finite() || b.trunc() < i32::MIN as f64 || b.trunc() > i32::MAX as f64 {
        return Err(boundary("observatory_angle_integer"));
    }
    let ib = b as i32;
    let g = f - ia as f64 * 1.0e4 - ib as f64 * 1.0e2;
    Ok((ia as f64 + (ib as f64 + g / 6.0e1) / 6.0e1) / 36.0e1 * std::f64::consts::PI * 2.0)
}
fn geometry(xyz: [f64; 3], code: Vec<u8>, name: Vec<u8>, clock: Vec<u8>) -> Observatory {
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
    Observatory {
        xyz,
        code,
        name,
        clock,
        height,
        latitude,
        longitude,
    }
}
