//! observatory.C: all eight active routines at fda6ad4. Fixed source records
//! retain initialized prefixes/tails; snapshots and indices never borrow across growth.
//! The alias vector intentionally accumulates across lines; lookup/warnings have
//! process lifetime independently of the clock index (decision 0023).
use crate::runtime::Resource;
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{self, c_bytes},
    observation::{format_string, read_string, write_string},
    tabulated_function::{chunks, os_error},
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default)]
pub(crate) struct Observatory {
    pub xyz: [f64; 3],
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub code: [Option<u8>; 16],
    pub name: [Option<u8>; 32],
    pub clock: [Option<u8>; 16],
}
#[derive(Default)]
pub(crate) struct ObservatoryDatabase {
    pub initialized: bool,
    pub observatories: Vec<Observatory>,
    pub aliases: Vec<AliasList>,
    pub resources: Vec<(PathBuf, Resource)>,
}
pub(crate) struct AliasList {
    pub code: [Option<u8>; 100],
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
        source_path: path.into(),
        path: path.to_string_lossy().into_owned(),
        sha256: format!("{:x}", Sha256::digest(data)),
        size_bytes: data.len(),
        role: role.into(),
        coverage_mjd: coverage,
    }
}
impl ObservatoryDatabase {
    pub fn read_file(
        &mut self,
        path: &Path,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
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
        let mut record = Observatory::default();
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
                    let storage: &mut [Option<u8>] = match i {
                        0 => &mut record.name,
                        1 => &mut record.code,
                        _ => &mut record.clock,
                    };
                    write_string(storage, token, "observatory_string")?;
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
                let latitude = fang(1, x)?;
                let longitude = -fang(1, y)?;
                record.latitude = latitude;
                record.longitude = longitude;
                record.height = z;
                grs80_to_itrf(&mut record);
                let coordinates = record.xyz;
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
            if nread == 5 {
                let name = [b"UTC(".as_slice(), &record.code()?, b")"].concat();
                format_string(&mut record.clock, &name);
            }
            record.xyz = xyz.map(Option::unwrap);
            itrf_to_grs80(&mut record);
            self.observatories.push(record.clone());
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
    pub fn read_aliases(
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
        let mut code_storage = [None; 100];
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
                write_string(&mut code_storage, code, "alias_code")?;
                self.aliases.push(AliasList {
                    code: code_storage,
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
        diagnostics.log(b'd', "observatory.C", 205, b"In initObservatories");
        let root = root.ok_or_else(|| boundary("observatory_environment_storage"))?;
        let directory = [c_bytes(root.as_os_str().as_bytes()), b"/observatory/"].concat();
        let path = PathBuf::from(std::ffi::OsStr::from_bytes(
            &[directory.as_slice(), b"observatories.dat"].concat(),
        ));
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
        let pattern = PathBuf::from(std::ffi::OsStr::from_bytes(
            &[directory.as_slice(), b"*.dat"].concat(),
        ));
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
        for other in source_files(pattern.as_os_str().as_bytes()) {
            if other.as_os_str().as_bytes() != path.as_os_str().as_bytes() {
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
            PathBuf::from(std::ffi::OsStr::from_bytes(
                &[directory.as_slice(), c_bytes(alias), b".aliases"].concat(),
            ))
        } else {
            PathBuf::from(std::ffi::OsStr::from_bytes(
                &[directory.as_slice(), b"aliases"].concat(),
            ))
        };
        if path.as_os_str().as_bytes().len() >= 1024 {
            return Err(boundary("observatory_path_storage"));
        }
        self.read_aliases(&path, diagnostics)?;
        self.initialized = true;
        diagnostics.log(b'd', "observatory.C", 241, b"Leaving initObservatories");
        Ok(())
    }
    pub fn lookup_alias(
        &mut self,
        code: &[u8],
        output: &mut [Option<u8>],
        root: Option<&Path>,
        alias: Option<&[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        let code = c_bytes(code);
        if !self.initialized {
            self.initialize(root, alias, diagnostics)?;
        }
        let resolved = if let Some(list) = self
            .aliases
            .iter_mut()
            .find(|l| l.aliases.iter().any(|a| a.eq_ignore_ascii_case(code)))
        {
            let resolved = read_string(&list.code, "alias_code")?;
            if !list.warned {
                diagnostics.log(
                    b'w',
                    "observatory.C",
                    272,
                    &[
                        b"Assuming site '".as_slice(),
                        code,
                        b"' means '",
                        &resolved,
                        b"'",
                    ]
                    .concat(),
                );
                list.warned = true;
            }
            resolved
        } else {
            code.to_vec()
        };
        // A no-match in-place source call performs no write. Copying its identical
        // known prefix through NUL is observationally identical, including tails.
        write_string(output, &resolved, "observatory_alias_output")
    }
    pub fn index(
        &mut self,
        code: &[u8],
        root: Option<&Path>,
        alias: Option<&[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<usize, TimingError> {
        let mut output = [None; 1024];
        self.lookup_alias(code, &mut output, root, alias, diagnostics)?;
        let resolved = read_string(&output, "observatory_alias_output")?;
        for (i, o) in self.observatories.iter().enumerate() {
            if o.code()?.eq_ignore_ascii_case(&resolved)
                || o.name()?.eq_ignore_ascii_case(&resolved)
            {
                return Ok(i);
            }
        }
        diagnostics.stderr(
            &[
                b"Observatory code '".as_slice(),
                c_bytes(code),
                b"' not found!\n",
            ]
            .concat(),
        );
        Err(terminal(
            root.unwrap_or(Path::new("")),
            "observatory_not_found",
        ))
    }
    pub fn get(
        &mut self,
        code: &[u8],
        root: Option<&Path>,
        alias: Option<&[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<Observatory, TimingError> {
        let index = self.index(code, root, alias, diagnostics)?;
        Ok(self.observatories[index].clone())
    }
}
impl Observatory {
    pub fn code(&self) -> Result<Vec<u8>, TimingError> {
        read_string(&self.code, "observatory_code")
    }
    pub fn name(&self) -> Result<Vec<u8>, TimingError> {
        read_string(&self.name, "observatory_name")
    }
    pub fn clock(&self) -> Result<Vec<u8>, TimingError> {
        read_string(&self.clock, "observatory_clock")
    }
}

pub(crate) fn fang(mode: i32, f: f64) -> Result<f64, TimingError> {
    // observatory.C:373-375: GCC16 -O2 contracts the component subtractions.
    // W arithmetic evidence retains assembly and fused/unfused witnesses.
    let a = f / 1.0e4;
    if !a.is_finite() || a.trunc() < i32::MIN as f64 || a.trunc() > i32::MAX as f64 {
        return Err(boundary("observatory_angle_integer"));
    }
    let ia = a as i32;
    let b = (-(ia as f64)).mul_add(1.0e4, f) / 1.0e2;
    if !b.is_finite() || b.trunc() < i32::MIN as f64 || b.trunc() > i32::MAX as f64 {
        return Err(boundary("observatory_angle_integer"));
    }
    let ib = b as i32;
    let g = (-(ib as f64)).mul_add(1.0e2, (-(ia as f64)).mul_add(1.0e4, f));
    let mut angle = (ia as f64 + (ib as f64 + g / 6.0e1) / 6.0e1) / 36.0e1;
    if mode > 2 {
        angle *= 15.0;
    }
    if mode == 1 || mode == 3 {
        angle = angle * std::f64::consts::PI * 2.0;
    }
    Ok(angle)
}
pub(crate) fn itrf_to_grs80(obs: &mut Observatory) {
    // observatory.C:396,403,405,408-415: explicit mul_add matches only
    // contractions present in U's GCC16 -O2 body (W arithmetic evidence).
    let [x, y, z] = obs.xyz;
    let a = 6378137.0;
    let flattening = 1.0 / 298.257222101;
    let p = (x.mul_add(x, y * y)) / (a * a);
    let esq = flattening * (2.0 - flattening);
    let q = (1.0 - esq) / (a * a) * z * z;
    let r = (p + q - esq * esq) / 6.0;
    let s = esq * esq * p * q / (4.0 * r * r * r);
    let t = (1.0 + s + (s * (2.0 + s)).sqrt()).powf(1.0 / 3.0);
    let u = r * (1.0 + t + 1.0 / t);
    let v = (u.mul_add(u, esq * esq * q)).sqrt();
    let w = esq * (u + v - q) / (2.0 * v);
    let k = (w.mul_add(w, u + v)).sqrt() - w;
    let d = k * (x.mul_add(x, y * y)).sqrt() / (k + esq);
    let height = (k + esq - 1.0) / k * (d.mul_add(d, z * z)).sqrt();
    let latitude = 2.0 * z.atan2(d + (d.mul_add(d, z * z)).sqrt());
    let longitude = if y >= 0.0 {
        (-2.0f64).mul_add(
            x.atan2((x.mul_add(x, y * y)).sqrt() + y),
            0.5 * std::f64::consts::PI,
        )
    } else {
        2.0f64.mul_add(
            x.atan2((x.mul_add(x, y * y)).sqrt() - y),
            -0.5 * std::f64::consts::PI,
        )
    };
    obs.height = height;
    obs.latitude = latitude;
    obs.longitude = longitude;
}
pub(crate) fn grs80_to_itrf(obs: &mut Observatory) {
    let flattening = 1.0 / 298.257222101;
    let esq = flattening * (2.0 - flattening);
    // observatory.C:423,426: source-compiled subtract/add contractions.
    let n = 6378137.0
        / (-(esq * obs.latitude.sin()))
            .mul_add(obs.latitude.sin(), 1.0)
            .sqrt();
    obs.xyz = [
        (n + obs.height) * obs.latitude.cos() * obs.longitude.cos(),
        (n + obs.height) * obs.latitude.cos() * obs.longitude.sin(),
        n.mul_add(1.0 - esq, obs.height) * obs.latitude.sin(),
    ];
}
