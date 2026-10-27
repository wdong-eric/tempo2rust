//! clkcorr.C constructors, helpers and four public clock operations, at fda6ad4.
//! Source bytes, directed binary32 routing, lazy tables and append-only sequences.
//! The outer synchronized session is a race-safety/lifetime adapter (0023), not
//! a per-context mode. Deterministic defects and fallback zeros are retained.
use crate::{
    PRECISION_BITS, Scalar, TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{self, NumberFormat, c_bytes, format_number},
    observation::{format_string, read, read_string},
    observatory::{ObservatoryDatabase, resource, source_files},
    runtime::Resource,
    tabulated_function::{TabulatedFunction, chunks, os_error},
    tempo2::{MAX_CLK_CORR, SourceObservation},
};
use rug::Float;
use serde::Serialize;
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

#[derive(Debug, Serialize, Clone)]
pub struct ClockStep {
    pub corrects_to: String,
    pub correction_s: Scalar,
    pub resource: String,
}
pub struct ClockCorrectionFunction {
    pub table: TabulatedFunction,
    pub path: PathBuf,
    pub name: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub badness: f32,
    pub loaded: bool,
    pub start: f64,
    pub end: f64,
    pub indexed: Resource,
}
#[derive(Default, Clone)]
pub struct ClockSequence {
    pub indices: Vec<usize>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub from_key: Vec<u8>,
    pub to_key: Vec<u8>,
    pub start: f64,
    pub end: f64,
}
#[derive(Default)]
pub struct PathBuildResult {
    pub indices: Vec<usize>,
    pub source_present: bool,
    pub target_present: bool,
    pub reverse_hint: bool,
}
pub struct TableQuery {
    #[cfg(test)]
    pub name: Vec<u8>,
    #[cfg(test)]
    pub epoch: f64,
    pub value: f64,
    pub function: usize,
}
#[derive(Default)]
pub struct ClockSession {
    pub(crate) source: crate::source_state::SourceState,
    pub initialized: bool,
    pub functions: Vec<ClockCorrectionFunction>,
    pub sequences: Vec<ClockSequence>,
    pub root: Option<PathBuf>,
    pub extra: Vec<u8>,
    pub alias: Option<Vec<u8>>,
    pub diagnostics: Diagnostics,
    pub observatories: ObservatoryDatabase,
    pub eop: crate::eop::Eop,
    pub meteorology: crate::tropo::Meteorology,
    pub ifte: crate::ifteph::IfteEphemeris,
    pub fortran: crate::read_fortran::FortranReader,
    pub fortran2: crate::read_fortran::FortranReader,
    pub fb: crate::tt2tdb::FbCache,
    pub ut1: crate::tai2ut1::Ut1,
    pub queries: Vec<TableQuery>,
    pub fault: Option<TimingError>,
}
static SESSION: LazyLock<Mutex<ClockSession>> =
    LazyLock::new(|| Mutex::new(ClockSession::default()));
/// Source operations that do not choose a runtime root share the process owner.
/// The callback must use borrowed-session operations, never reacquire this lock.
pub fn with_process<T>(
    operation: impl FnOnce(&mut ClockSession) -> Result<T, TimingError>,
) -> Result<T, TimingError> {
    let mut session = SESSION
        .lock()
        .map_err(|_| boundary("clock_session_poisoned"))?;
    operation(&mut session)
}
/// Test entry for the process clock terminal-state contract.
#[cfg(test)]
pub(crate) fn with_session<T>(
    root: &Path,
    operation: impl FnOnce(&mut ClockSession) -> Result<T, TimingError>,
) -> Result<T, TimingError> {
    let mut session = SESSION
        .lock()
        .map_err(|_| boundary("clock_session_poisoned"))?;
    session.root = Some(root.into());
    session.alias = std::env::var_os("TEMPO2_ALIAS").map(|v| v.as_bytes().to_vec());
    session.run(operation)
}
/// Resource operations share ownership/diagnostics but not a clock terminal fault.
pub fn with_resources<T>(
    root: &Path,
    operation: impl FnOnce(&mut ClockSession) -> Result<T, TimingError>,
) -> Result<T, TimingError> {
    let mut session = SESSION
        .lock()
        .map_err(|_| boundary("clock_session_poisoned"))?;
    session.root = Some(root.into());
    session.alias = std::env::var_os("TEMPO2_ALIAS").map(|v| v.as_bytes().to_vec());
    operation(&mut session)
}
pub(crate) fn with_diagnostics<T>(
    operation: impl FnOnce(&mut Diagnostics) -> Result<T, TimingError>,
) -> Result<T, TimingError> {
    let mut session = SESSION
        .lock()
        .map_err(|_| boundary("clock_session_poisoned"))?;
    operation(&mut session.diagnostics)
}
/// Lifecycle logging uses the same sink, but is independent of a terminal clock
/// calculation. Native allocation/initialization logs do not reenter that work.
pub(crate) fn lifecycle_diagnostics(
    operation: impl FnOnce(&mut Diagnostics),
) -> Result<(), TimingError> {
    let mut session = SESSION
        .lock()
        .map_err(|_| boundary("clock_session_poisoned"))?;
    operation(&mut session.diagnostics);
    Ok(())
}
pub(crate) fn trim(name: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = name.len();
    while start < end && matches!(name[start], b' ' | b'\t' | b'\r' | b'\n') {
        start += 1;
    }
    while end > start && matches!(name[end - 1], b' ' | b'\t' | b'\r' | b'\n') {
        end -= 1;
    }
    &name[start..end]
}
pub(crate) fn normalize_clock_name(name: &[u8]) -> Vec<u8> {
    trim(name).iter().map(u8::to_ascii_uppercase).collect()
}
pub(crate) fn clock_name_equals(a: &[u8], b: &[u8]) -> bool {
    normalize_clock_name(a) == normalize_clock_name(b)
}
pub(crate) fn basename_from_path(path: &[u8]) -> &[u8] {
    path.rsplit(|&b| b == b'/').next().unwrap_or_default()
}
impl ClockSession {
    pub fn run<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, TimingError>,
    ) -> Result<T, TimingError> {
        if let Some(error) = &self.fault {
            return Err(error.clone());
        }
        let result = f(self);
        if let Err(error) = &result {
            self.fault = Some(error.clone());
        }
        result
    }
    pub fn add_indexed_clock_file(&mut self, path: &Path, offset: i32) -> Result<(), TimingError> {
        let data = fs::read(path).map_err(|e| {
            self.diagnostics.stderr(
                &[
                    b"Fatal Error: Unable to open file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b" for reading: ",
                    os_error(&e).as_bytes(),
                    b"\n",
                ]
                .concat(),
            );
            terminal(path, "clock_index_open")
        })?;
        let Some(header) = chunks(&data).next() else {
            self.diagnostics.stderr(
                &[
                    b"Error parsing clock file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b": file appears empty or corrupted.\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "clock_index_empty"));
        };
        let Some((from, to, badness)) = clock_text::header(header) else {
            self.diagnostics.stderr(
                &[
                    b"Error parsing clock file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b": first line must be of form # clock_from clock_to [badness]\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "clock_index_header"));
        };
        let name = basename_from_path(path.as_os_str().as_bytes()).to_vec();
        let badness = badness + offset as f32;
        self.diagnostics.log(
            b'd',
            "clkcorr.C",
            207,
            &[
                b"Indexed clock correction ".as_slice(),
                &name,
                b" : ",
                &from,
                b" -> ",
                &to,
                b" badness ",
                &format_number(badness as f64, NumberFormat::GeneralThree),
            ]
            .concat(),
        );
        self.functions.push(ClockCorrectionFunction {
            table: TabulatedFunction::default(),
            path: path.into(),
            name,
            from,
            to,
            badness,
            loaded: false,
            start: 0.0,
            end: 0.0,
            indexed: resource(path, &data, "indexed clock", None),
        });
        Ok(())
    }
    pub fn ensure_initialized(&mut self, warnings: i32) -> Result<(), TimingError> {
        if self.initialized {
            return Ok(());
        }
        self.diagnostics
            .log(b'd', "clkcorr.C", 244, b"Initialise clock file index");
        let mut offset = 0;
        if !c_bytes(&self.extra).is_empty() {
            self.diagnostics.log(
                b'm',
                "clkcorr.C",
                249,
                &[
                    b"Note, using '".as_slice(),
                    &self.extra,
                    b"' to look for extra clock files",
                ]
                .concat(),
            );
            let files = source_files(&[c_bytes(&self.extra), b"/*.clk"].concat());
            if files.is_empty() {
                self.diagnostics.log(
                    b'e',
                    "clkcorr.C",
                    254,
                    &[
                        b"No clock correction files in '".as_slice(),
                        &self.extra,
                        b"'",
                    ]
                    .concat(),
                );
            } else {
                for path in files {
                    self.add_indexed_clock_file(&path, 0)?;
                    self.log_loaded(b'm', 261, b"Loaded extra clock correction ");
                }
                offset = 1;
            }
        }
        let Some(root) = self.root.as_ref().filter(|r| !r.as_os_str().is_empty()) else {
            self.diagnostics.display(
                2,
                b"CLK11",
                b"TEMPO2 environment path is not set",
                b"",
                warnings,
            )?;
            return Err(terminal(Path::new(""), "clock_environment"));
        };
        let files = source_files(&[c_bytes(root.as_os_str().as_bytes()), b"/clock/*.clk"].concat());
        if files.is_empty() {
            self.diagnostics
                .stderr(b"No clock correction files in $TEMPO2/clock\n");
            return Err(terminal(root, "clock_index_absent"));
        }
        for path in files {
            self.add_indexed_clock_file(&path, offset)?;
            self.log_loaded(b'd', 287, b"Loaded clock correction ");
        }
        self.initialized = true;
        self.diagnostics
            .log(b'd', "clkcorr.C", 292, b"Done with clock file index");
        Ok(())
    }
    fn log_loaded(&mut self, kind: u8, line: usize, prefix: &[u8]) {
        let f = self.functions.last().unwrap();
        self.diagnostics.log(
            kind,
            "clkcorr.C",
            line,
            &[
                prefix,
                &f.name,
                b" : ",
                &f.from,
                b" -> ",
                &f.to,
                b" badness ",
                &format_number(f.badness as f64, NumberFormat::GeneralThree),
            ]
            .concat(),
        );
    }
    pub fn ensure_function_loaded(&mut self, index: usize) -> Result<(), TimingError> {
        let f = self
            .functions
            .get_mut(index)
            .ok_or_else(|| boundary("clock_function_storage"))?;
        if f.loaded {
            return Ok(());
        }
        self.diagnostics.log(
            b'd',
            "clkcorr.C",
            300,
            &[
                b"Loading clock correction function from ".as_slice(),
                &f.name,
                b" (",
                f.path.as_os_str().as_bytes(),
                b")",
            ]
            .concat(),
        );
        f.table.load(&f.path, &mut self.diagnostics)?;
        f.start = f.table.start()?;
        f.end = f.table.end()?;
        f.loaded = true;
        Ok(())
    }
    fn function_valid_at_mjd(&mut self, index: usize, mjd: f64) -> Result<bool, TimingError> {
        self.ensure_function_loaded(index)?;
        let f = &self.functions[index];
        Ok(f.start <= mjd && f.end >= mjd)
    }
    fn sequence_mjd_range(&mut self, indices: &[usize]) -> Result<(f64, f64), TimingError> {
        let (mut start, mut end) = (0.0, 1e6);
        for &i in indices {
            self.ensure_function_loaded(i)?;
            let f = &self.functions[i];
            if f.start > start {
                start = f.start;
            }
            if f.end < end {
                end = f.end;
            }
        }
        Ok((start, end))
    }
    pub fn build_directed_path(
        &mut self,
        from: &[u8],
        to: &[u8],
        mjd: f64,
    ) -> Result<PathBuildResult, TimingError> {
        let mut result = PathBuildResult::default();
        let mut nodes: Vec<Vec<u8>> = Vec::new();
        let mut edges = Vec::new();
        for f in &self.functions {
            let mut pair = [0; 2];
            for (i, name) in [&f.from, &f.to].iter().enumerate() {
                let key = normalize_clock_name(name);
                pair[i] = if let Some(n) = nodes.iter().position(|k| k == &key) {
                    n
                } else {
                    if nodes.len() > i32::MAX as usize {
                        return Err(boundary("clock_node_integer"));
                    }
                    nodes.push(key);
                    nodes.len() - 1
                };
            }
            edges.push(pair);
        }
        let source = nodes.iter().position(|n| n == &normalize_clock_name(from));
        let target = nodes.iter().position(|n| n == &normalize_clock_name(to));
        result.source_present = source.is_some();
        result.target_present = target.is_some();
        let (Some(source), Some(target)) = (source, target) else {
            return Ok(result);
        };
        let mut adjacency = vec![Vec::new(); nodes.len()];
        let mut reverse = vec![Vec::new(); nodes.len()];
        for (i, &[a, b]) in edges.iter().enumerate() {
            adjacency[a].push(i);
            reverse[b].push(a);
        }
        let mut blocked = vec![false; edges.len()];
        loop {
            let mut dist = vec![f32::MAX; nodes.len()];
            let mut visited = vec![false; nodes.len()];
            let mut previous = vec![None; nodes.len()];
            dist[source] = 0.0;
            loop {
                let mut best = f32::MAX;
                let mut node = None;
                for i in 0..dist.len() {
                    if !visited[i] && dist[i] < best {
                        best = dist[i];
                        node = Some(i);
                    }
                }
                let Some(node) = node else {
                    break;
                };
                visited[node] = true;
                if node == target {
                    break;
                }
                for &edge in &adjacency[node] {
                    if blocked[edge] {
                        continue;
                    }
                    let dest = edges[edge][1];
                    let new = dist[node] + self.functions[edge].badness;
                    if new < dist[dest] {
                        dist[dest] = new;
                        previous[dest] = Some(edge);
                    }
                }
            }
            if !visited[target] {
                let mut stack = vec![source];
                let mut seen = vec![false; nodes.len()];
                seen[source] = true;
                while let Some(node) = stack.pop() {
                    if node == target {
                        result.reverse_hint = true;
                        break;
                    }
                    for &next in &reverse[node] {
                        if !seen[next] {
                            seen[next] = true;
                            stack.push(next);
                        }
                    }
                }
                return Ok(result);
            }
            let mut candidate = Vec::new();
            let mut node = target;
            // The source has no predecessor-cycle guard. Do not execute the
            // withheld cycle fixture or silently substitute a repaired algorithm.
            while node != source {
                let Some(edge) = previous[node] else {
                    candidate.clear();
                    break;
                };
                candidate.push(edge);
                node = edges[edge][0];
            }
            candidate.reverse();
            if candidate.is_empty() {
                return Ok(result);
            }
            let mut valid = true;
            for &edge in &candidate {
                if !self.function_valid_at_mjd(edge, mjd)? {
                    blocked[edge] = true;
                    valid = false;
                }
            }
            if valid {
                result.indices = candidate;
                return Ok(result);
            }
        }
    }
    pub fn cache_sequence(
        &mut self,
        indices: Vec<usize>,
        from: &[u8],
        to: &[u8],
    ) -> Result<usize, TimingError> {
        let (start, end) = self.sequence_mjd_range(&indices)?;
        self.sequences.push(ClockSequence {
            indices,
            from: from.to_vec(),
            to: to.to_vec(),
            from_key: normalize_clock_name(from),
            to_key: normalize_clock_name(to),
            start,
            end,
        });
        Ok(self.sequences.len() - 1)
    }
    pub fn get_sequence(
        &mut self,
        from: &[u8],
        to: &[u8],
        mjd: f64,
        warnings: i32,
    ) -> Result<Option<usize>, TimingError> {
        self.ensure_initialized(warnings)?;
        if let Some(i) = self.sequences.iter().position(|s| {
            s.from_key == normalize_clock_name(from)
                && s.to_key == normalize_clock_name(to)
                && s.start <= mjd
                && s.end >= mjd
        }) {
            return Ok(Some(i));
        }
        self.diagnostics.log(
            b'd',
            "clkcorr.C",
            559,
            &[b"Making clock sequence from ".as_slice(), from, b" to ", to].concat(),
        );
        let result = self.build_directed_path(from, to, mjd)?;
        if !result.source_present || !result.target_present {
            let name = if !result.source_present { from } else { to };
            let body = [
                b"no clock corrections available for clock ".as_slice(),
                name,
                b" for MJD",
            ]
            .concat();
            if body.len() >= 1000 {
                return Err(boundary("clock_message_storage"));
            }
            self.diagnostics.display(
                2,
                b"CLK3",
                &body,
                &format_number(mjd, NumberFormat::One),
                warnings,
            )?;
            return Ok(None);
        }
        if result.indices.is_empty() {
            let body = [
                b"no directed clock corrections available from ".as_slice(),
                from,
                b" to ",
                to,
                b" for MJD",
            ]
            .concat();
            if body.len() >= 1000 {
                return Err(boundary("clock_message_storage"));
            }
            let mut more = format_number(mjd, NumberFormat::One);
            if result.reverse_hint {
                more.extend(b" (reverse path exists but reverse traversal is disabled)");
            }
            self.diagnostics
                .display(2, b"CLK7", &body, &more, warnings)?;
            return Ok(None);
        }
        if warnings == 0 {
            self.diagnostics.stdout(
                &[
                    b"Using the following chain of clock corrections for ".as_slice(),
                    from,
                    b" -> ",
                    to,
                    b"\n",
                ]
                .concat(),
            );
            for &i in &result.indices {
                let f = &self.functions[i];
                self.diagnostics
                    .stdout(&[f.name.as_slice(), b" : ", &f.from, b" -> ", &f.to, b"\n"].concat());
            }
        }
        self.cache_sequence(result.indices, from, to).map(Some)
    }
    fn find_function_index_from_token(&self, token: &[u8]) -> Option<usize> {
        self.functions
            .iter()
            .position(|f| f.name == token)
            .or_else(|| {
                self.functions
                    .iter()
                    .position(|f| f.name == basename_from_path(token))
            })
    }
    pub fn define_sequence(
        &mut self,
        files: Option<&[u8]>,
        warnings: i32,
    ) -> Result<(), TimingError> {
        self.ensure_initialized(warnings)?;
        let mut indices = Vec::new();
        for token in clock_text::tokens(files.unwrap_or_default()) {
            if let Some(i) = self.find_function_index_from_token(token) {
                indices.push(i);
            } else {
                self.diagnostics.stdout(
                    &[
                        b"Requested clock correction file ".as_slice(),
                        token,
                        b" not found!\n",
                    ]
                    .concat(),
                );
                return Err(terminal(Path::new(""), "clock_chain_missing"));
            }
        }
        if indices.is_empty() {
            self.diagnostics.display(
                2,
                b"CLK13",
                b"Empty CLK_CORR_CHAIN definition",
                b"",
                warnings,
            )?;
            return Err(terminal(Path::new(""), "clock_chain_empty"));
        }
        for pair in indices.windows(2) {
            let prev = &self.functions[pair[0]];
            let next = &self.functions[pair[1]];
            if !clock_name_equals(&prev.to, &next.from) {
                self.diagnostics.stderr(
                    &[
                        b"Invalid directed clock correction chain: ".as_slice(),
                        &prev.name,
                        b" (",
                        &prev.from,
                        b" -> ",
                        &prev.to,
                        b") cannot precede ",
                        &next.name,
                        b" (",
                        &next.from,
                        b" -> ",
                        &next.to,
                        b")\n",
                    ]
                    .concat(),
                );
                return Err(terminal(Path::new(""), "clock_chain_direction"));
            }
        }
        let from = self.functions[indices[0]].from.clone();
        let to = self.functions[*indices.last().unwrap()].to.clone();
        self.cache_sequence(indices, &from, &to)?;
        Ok(())
    }
    fn evaluate_function(
        &mut self,
        index: usize,
        sat: f64,
        total: f64,
    ) -> Result<f64, TimingError> {
        let f = &self.functions[index];
        let epoch = sat + total / 86400.0;
        let value = f.table.get_value(epoch, &mut self.diagnostics)?;
        self.queries.push(TableQuery {
            #[cfg(test)]
            name: f.table.name.clone(),
            #[cfg(test)]
            epoch,
            value,
            function: index,
        });
        Ok(value)
    }
    pub fn fill_corrections(
        &mut self,
        obs: &mut SourceObservation,
        sequence: usize,
        to: &[u8],
        from: &[u8],
        total: &mut f64,
    ) -> Result<bool, TimingError> {
        let indices = self
            .sequences
            .get(sequence)
            .ok_or_else(|| boundary("clock_sequence_storage"))?
            .indices
            .clone();
        let mut current = from.to_vec();
        for i in indices {
            if clock_name_equals(&current, to) {
                break;
            }
            if *read(&obs.nclock_correction, "nclock_correction")? >= MAX_CLK_CORR as i32 {
                self.diagnostics.display(
                    2,
                    b"CLK12",
                    b"clock correction chain exceeded MAX_CLK_CORR",
                    b"",
                    0,
                )?;
                return Ok(false);
            }
            self.ensure_function_loaded(i)?;
            if !clock_name_equals(&current, &self.functions[i].from) {
                self.broken_chain(&current, i);
                return Ok(false);
            }
            let value = self.evaluate_function(i, read(&obs.sat, "sat")?.to_f64(), *total)?;
            let slot = usize::try_from(*read(&obs.nclock_correction, "nclock_correction")?)
                .ok()
                .and_then(|n| obs.correctionsTT.get_mut(n))
                .ok_or_else(|| boundary("clock_correction_storage"))?;
            slot.correction = Some(value);
            format_string(&mut slot.corrects_to, &self.functions[i].to);
            obs.nclock_correction = Some(*read(&obs.nclock_correction, "nclock_correction")? + 1);
            *total += value;
            current = self.functions[i].to.clone();
        }
        Ok(clock_name_equals(&current, to))
    }
    fn broken_chain(&mut self, current: &[u8], index: usize) {
        self.diagnostics.log(
            b'e',
            "clkcorr.C",
            625,
            &[
                b"Broken directed clock correction chain: expected ".as_slice(),
                current,
                b", got ",
                &self.functions[index].from,
            ]
            .concat(),
        );
    }
    pub fn accumulate_correction(
        &mut self,
        obs: &SourceObservation,
        sequence: usize,
        to: &[u8],
        from: &[u8],
        total: &mut f64,
    ) -> Result<bool, TimingError> {
        let indices = self
            .sequences
            .get(sequence)
            .ok_or_else(|| boundary("clock_sequence_storage"))?
            .indices
            .clone();
        let mut current = from.to_vec();
        for i in indices {
            if clock_name_equals(&current, to) {
                break;
            }
            self.ensure_function_loaded(i)?;
            if !clock_name_equals(&current, &self.functions[i].from) {
                self.diagnostics.log(
                    b'e',
                    "clkcorr.C",
                    659,
                    &[
                        b"Broken directed clock correction chain: expected ".as_slice(),
                        &current,
                        b", got ",
                        &self.functions[i].from,
                    ]
                    .concat(),
                );
                return Ok(false);
            }
            *total += self.evaluate_function(i, read(&obs.sat, "sat")?.to_f64(), *total)?;
            current = self.functions[i].to.clone();
        }
        Ok(clock_name_equals(&current, to))
    }
    fn source_clock(
        &mut self,
        obs: &SourceObservation,
        from: Option<&[u8]>,
    ) -> Result<Vec<u8>, TimingError> {
        let name = c_bytes(from.unwrap_or_default());
        if !name.is_empty() {
            return Ok(name.to_vec());
        }
        self.observatories
            .get(
                &read_string(&obs.telID, "telID")?,
                self.root.as_deref(),
                self.alias.as_deref(),
                &mut self.diagnostics,
            )?
            .clock()
    }
    pub fn get_clock_corrections(
        &mut self,
        obs: &mut SourceObservation,
        from: Option<&[u8]>,
        to: Option<&[u8]>,
        warnings: i32,
    ) -> Result<(), TimingError> {
        obs.nclock_correction = Some(0);
        let mut from = self.source_clock(obs, from)?;
        let original = from.clone();
        let mut to = c_bytes(to.unwrap_or_default()).to_vec();
        if clock_name_equals(&from, &to) {
            return Ok(());
        }
        let mut sequence =
            self.get_sequence(&from, &to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
        let mut approximation = false;
        if sequence.is_none() {
            if from.len() >= 1000 {
                return Err(boundary("clock_message_storage"));
            }
            self.diagnostics
                .display(1, b"CLK4", b"Trying assuming UTC =", &from, warnings)?;
            from = b"UTC".to_vec();
            if clock_name_equals(&from, &to) {
                return Ok(());
            }
            sequence = self.get_sequence(&from, &to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
            approximation = true;
            if sequence.is_none() {
                if to.len() >= 1000 {
                    return Err(boundary("clock_message_storage"));
                }
                self.diagnostics.display(
                    2,
                    b"CLK5",
                    b"Trying TT(TAI) instead of ",
                    &to,
                    warnings,
                )?;
                from = original;
                to = b"TT(TAI)".to_vec();
                if clock_name_equals(&from, &to) {
                    return Ok(());
                }
                sequence =
                    self.get_sequence(&from, &to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
                if sequence.is_none() {
                    self.diagnostics
                        .display(2, b"CLK8", b"Trying both", b"", warnings)?;
                    from = b"UTC".to_vec();
                    sequence =
                        self.get_sequence(&from, &to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
                    if sequence.is_none() {
                        if warnings == 0 {
                            self.diagnostics.stdout(&[b"Warning [CLK:7], no directed clock correction available for TOA @ MJD ".as_slice(),&format_number(read(&obs.sat, "sat")?.to_f64(),NumberFormat::Four),b"!\n"].concat());
                        }
                        return Ok(());
                    }
                }
            }
        }
        let mut total = 0.0;
        if !self.fill_corrections(obs, sequence.unwrap(), &to, &from, &mut total)? {
            self.diagnostics.display(
                2,
                b"CLK14",
                b"Broken directed clock correction chain",
                b"",
                warnings,
            )?;
            obs.nclock_correction = Some(0);
            return Ok(());
        }
        if approximation {
            self.diagnostics.display(
                1,
                b"CLK9",
                b"... ok, using stated approximation",
                b"",
                warnings,
            )?;
        }
        Ok(())
    }
    pub fn get_correction(
        &mut self,
        obs: &SourceObservation,
        from: Option<&[u8]>,
        to: Option<&[u8]>,
        warnings: i32,
    ) -> Result<f64, TimingError> {
        let from = self.source_clock(obs, from)?;
        self.diagnostics.version("clkcorr.C", "getCorrection()");
        let to = c_bytes(to.unwrap_or_default());
        if clock_name_equals(&from, to) {
            return Ok(0.0);
        }
        let mut current = from.clone();
        let mut sequence =
            self.get_sequence(&from, to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
        if sequence.is_none() {
            if current.len() >= 1000 {
                return Err(boundary("clock_message_storage"));
            }
            self.diagnostics.display(
                1,
                b"CLK6",
                b"Proceeding assuming UTC = ",
                &current,
                warnings,
            )?;
            if clock_name_equals(to, b"UTC") {
                return Ok(0.0);
            }
            sequence = self.get_sequence(b"UTC", to, read(&obs.sat, "sat")?.to_f64(), warnings)?;
            current = b"UTC".to_vec();
            if sequence.is_none() {
                if warnings == 0 {
                    self.diagnostics.stdout(&[b"!Warning [CLK:9], no directed clock correction available for TOA @ MJD ".as_slice(),&format_number(read(&obs.sat, "sat")?.to_f64(),NumberFormat::Four),b"!\n"].concat());
                }
                return Ok(0.0);
            }
        }
        let mut total = 0.0;
        if !self.accumulate_correction(obs, sequence.unwrap(), to, &current, &mut total)? {
            self.diagnostics.display(
                2,
                b"CLK14",
                b"Broken directed clock correction chain",
                b"",
                warnings,
            )?;
            return Ok(0.0);
        }
        Ok(total)
    }
    pub fn export_observatory_resources(&self, root: &Path, resources: &mut Vec<Resource>) {
        for (path, r) in &self.observatories.resources {
            let mut r = r.clone();
            r.path = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .into_owned();
            if !resources.iter().any(|old| old.path == r.path) {
                resources.push(r);
            }
        }
    }
    pub fn export_terrestrial_resources(&self, root: &Path, resources: &mut Vec<Resource>) {
        self.export_observatory_resources(root, resources);
        let mut records: Vec<Resource> = self.eop.resource.iter().cloned().collect();
        for f in self
            .meteorology
            .wet
            .iter()
            .chain(&self.meteorology.pressure)
        {
            records.push(Resource {
                source_path: f.table.path.clone(),
                path: f.table.path.to_string_lossy().into_owned(),
                sha256: f.table.sha256.clone(),
                size_bytes: f.table.size_bytes,
                role: "meteorology table".into(),
                coverage_mjd: f
                    .table
                    .samples
                    .first()
                    .zip(f.table.samples.last())
                    .map(|(a, b)| [a[0], b[0]]),
            });
        }
        for mut r in records {
            r.path = Path::new(&r.path)
                .strip_prefix(root)
                .unwrap_or(Path::new(&r.path))
                .to_string_lossy()
                .into_owned();
            if let Some(old) = resources.iter_mut().find(|old| old.path == r.path) {
                *old = r;
            } else {
                resources.push(r);
            }
        }
    }
    pub fn export_time_resources(&self, root: &Path, resources: &mut Vec<Resource>) {
        for record in [self.ifte.resource.as_ref(), self.fortran.resource.as_ref()]
            .into_iter()
            .flatten()
        {
            let mut r = record.clone();
            r.path = Path::new(&r.path)
                .strip_prefix(root)
                .unwrap_or(Path::new(&r.path))
                .to_string_lossy()
                .into_owned();
            if let Some(old) = resources.iter_mut().find(|old| old.path == r.path) {
                *old = r;
            } else {
                resources.push(r);
            }
        }
    }
    pub fn export_resources(&self, root: &Path, resources: &mut Vec<Resource>) {
        self.export_terrestrial_resources(root, resources);
        for f in &self.functions {
            let mut r = f.indexed.clone();
            r.path = f
                .path
                .strip_prefix(root)
                .unwrap_or(&f.path)
                .to_string_lossy()
                .into_owned();
            if f.loaded {
                r.sha256 = f.table.sha256.clone();
                r.size_bytes = f.table.size_bytes;
                r.coverage_mjd = Some([f.start, f.end]);
                r.role = "inspected clock".into();
            }
            if let Some(old) = resources.iter_mut().find(|old| old.path == r.path) {
                *old = r;
            } else {
                resources.push(r);
            }
        }
    }
    pub fn steps(&self, root: &Path, count: usize) -> Result<Vec<ClockStep>, TimingError> {
        self.queries
            .iter()
            .take(count)
            .map(|q| {
                let f = &self.functions[q.function];
                let text = std::str::from_utf8(&f.to).map_err(|_| boundary("clock_json_text"))?;
                Ok(ClockStep {
                    corrects_to: text.to_owned(),
                    correction_s: Scalar(Float::with_val(PRECISION_BITS, q.value)),
                    resource: f
                        .path
                        .strip_prefix(root)
                        .unwrap_or(&f.path)
                        .to_string_lossy()
                        .into_owned(),
                })
            })
            .collect()
    }
}
pub fn get_correction_tt(obs: &SourceObservation) -> Result<f64, TimingError> {
    let mut total = 0.0;
    for i in 0..*read(&obs.nclock_correction, "nclock_correction")? {
        total += *read(
            &obs.correctionsTT
                .get(i as usize)
                .ok_or_else(|| boundary("clock_correction_storage"))?
                .correction,
            &format!("correctionsTT[{i}].correction"),
        )?;
    }
    Ok(total)
}

#[cfg(test)]
pub(crate) fn test_session() -> std::sync::MutexGuard<'static, ClockSession> {
    SESSION.lock().unwrap()
}

/// Drain presentation events without resetting counters, key history or terminal state.
pub fn take_clock_diagnostics() -> Vec<crate::clock_diagnostics::ClockDiagnostic> {
    let Ok(mut session) = SESSION.lock() else {
        return Vec::new();
    };
    session
        .diagnostics
        .events
        .drain(..)
        .map(
            |(source_stderr, bytes)| crate::clock_diagnostics::ClockDiagnostic {
                source_stderr,
                bytes,
            },
        )
        .collect()
}
