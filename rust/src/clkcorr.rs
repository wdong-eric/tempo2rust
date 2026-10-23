// Selected TEMPO2 clkcorr.C::getClockCorrections/getCorrection and
// getClockCorrectionSequenceInternal route/cache behavior; explicit chains omitted.
// ClockCorrections owns source process state. GPL-3.0-or-later.
use crate::{
    TimingError,
    runtime::{Resource, paths_with_extension, read_resource},
    tabulated_function::TabulatedFunction,
};
use serde::Serialize;
use std::path::{Path, PathBuf};

struct ClockCorrectionFunction {
    path: PathBuf,
    clock_from: String,
    clock_to: String,
    badness: f32,
    table: Option<TabulatedFunction>,
}
struct ClockSequence {
    clock_from: String,
    clock_to: String,
    start_mjd: f64,
    end_mjd: f64,
    function_indices: Vec<usize>,
}

pub(crate) struct ClockCorrections {
    functions: Vec<ClockCorrectionFunction>,
    sequences: Vec<ClockSequence>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClockStep {
    pub corrects_to: String,
    pub correction_s: crate::Scalar,
    pub resource: String,
}

impl ClockCorrections {
    pub fn load(root: &Path, resources: &mut Vec<Resource>) -> Result<Self, TimingError> {
        let paths = paths_with_extension(&root.join("clock"), "clk", false)?;
        if paths.is_empty() {
            return Err(TimingError::new(
                root.join("clock"),
                None,
                "clock",
                "no clock correction files",
            ));
        }
        let mut functions = Vec::new();
        for path in paths {
            let data = read_resource(root, &path, "indexed clock", resources)?;
            let text = std::str::from_utf8(&data)
                .map_err(|e| TimingError::new(&path, None, "clock", e.to_string()))?;
            let header = text.lines().next().unwrap_or("");
            let tokens: Vec<_> = header
                .strip_prefix('#')
                .unwrap_or("")
                .split_whitespace()
                .collect();
            if tokens.len() < 2 {
                return Err(TimingError::new(
                    &path,
                    Some(1),
                    "clock",
                    "expected # FROM TO [BADNESS]",
                ));
            }
            let badness = tokens
                .get(2)
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(1.0);
            // Dijkstra assumes finite nonnegative weights; the source accepts invalid
            // weights and can build cyclic predecessor chains. Reject such headers.
            if !badness.is_finite() || badness < 0.0 {
                return Err(TimingError::new(
                    &path,
                    Some(1),
                    "clock",
                    "badness must be finite and nonnegative",
                ));
            }
            functions.push(ClockCorrectionFunction {
                path,
                clock_from: tokens[0].to_uppercase(),
                clock_to: tokens[1].to_uppercase(),
                badness,
                table: None,
            });
        }
        // clkcorr.C uses process-global initialized flags/caches. Per-context state
        // prevents a failed load or another runtime root from reusing that database.
        Ok(Self {
            functions,
            sequences: Vec::new(),
        })
    }
    fn get_clock_correction_sequence(
        &mut self,
        root: &Path,
        resources: &mut Vec<Resource>,
        clock_from: &str,
        clock_to: &str,
        mjd: f64,
    ) -> Result<Option<Vec<usize>>, TimingError> {
        if let Some(sequence) = self.sequences.iter().find(|s| {
            s.clock_from == clock_from
                && s.clock_to == clock_to
                && s.start_mjd <= mjd
                && mjd <= s.end_mjd
        }) {
            return Ok(Some(sequence.function_indices.clone()));
        }
        let mut nodes: Vec<String> = Vec::new();
        let mut edges = Vec::new();
        for function in &self.functions {
            let mut pair = [0; 2];
            for (i, name) in [&function.clock_from, &function.clock_to]
                .iter()
                .enumerate()
            {
                pair[i] = if let Some(j) = nodes.iter().position(|n| n == *name) {
                    j
                } else {
                    nodes.push((*name).clone());
                    nodes.len() - 1
                };
            }
            edges.push(pair);
        }
        let (Some(source), Some(target)) = (
            nodes.iter().position(|n| n == clock_from),
            nodes.iter().position(|n| n == clock_to),
        ) else {
            return Ok(None);
        };
        let mut edge_blocked = vec![false; edges.len()];
        loop {
            let mut distance = vec![f32::MAX; nodes.len()];
            let mut visited = vec![false; nodes.len()];
            let mut previous_edge = vec![None; nodes.len()];
            distance[source] = 0.0;
            loop {
                let mut best_distance = f32::MAX;
                let mut node = None;
                for i in 0..nodes.len() {
                    if !visited[i] && distance[i] < best_distance {
                        best_distance = distance[i];
                        node = Some(i);
                    }
                }
                let Some(node) = node else { break };
                visited[node] = true;
                if node == target {
                    break;
                }
                for (i, &[a, b]) in edges.iter().enumerate() {
                    if a == node && !edge_blocked[i] {
                        let new_distance = distance[node] + self.functions[i].badness;
                        if new_distance < distance[b] {
                            distance[b] = new_distance;
                            previous_edge[b] = Some(i);
                        }
                    }
                }
            }
            if !visited[target] {
                return Ok(None);
            }
            let mut function_indices = Vec::new();
            let mut node = target;
            while node != source {
                let i = previous_edge[node].unwrap();
                function_indices.push(i);
                node = edges[i][0];
            }
            function_indices.reverse();
            let mut valid = true;
            let (mut start_mjd, mut end_mjd) = (0.0f64, 1e6f64);
            for &i in &function_indices {
                let function = &mut self.functions[i];
                if function.table.is_none() {
                    let data = read_resource(root, &function.path, "inspected clock", resources)?;
                    let text = std::str::from_utf8(&data).map_err(|e| {
                        TimingError::new(&function.path, None, "clock", e.to_string())
                    })?;
                    function.table = Some(TabulatedFunction::parse(&function.path, text)?);
                    let table = function.table.as_ref().unwrap();
                    let relative = function.path.strip_prefix(root).unwrap().to_string_lossy();
                    resources
                        .iter_mut()
                        .find(|r| r.path == relative)
                        .unwrap()
                        .coverage_mjd = Some([table.start(), table.end()]);
                }
                let table = function.table.as_ref().unwrap();
                if !table.covers(mjd) {
                    edge_blocked[i] = true;
                    valid = false;
                }
                start_mjd = start_mjd.max(table.start());
                end_mjd = end_mjd.min(table.end());
            }
            if valid {
                self.sequences.push(ClockSequence {
                    clock_from: clock_from.into(),
                    clock_to: clock_to.into(),
                    start_mjd,
                    end_mjd,
                    function_indices: function_indices.clone(),
                });
                return Ok(Some(function_indices));
            }
        }
    }
    pub fn get_clock_corrections(
        &mut self,
        root: &Path,
        resources: &mut Vec<Resource>,
        clock_from: &str,
        clock_to: &str,
        sat: f64,
        warnings: &mut Vec<String>,
    ) -> Result<Vec<ClockStep>, TimingError> {
        let clock_from = clock_from.to_uppercase();
        let clock_to = clock_to.to_uppercase();
        if clock_from == clock_to {
            return Ok(Vec::new());
        }
        let sequence =
            self.get_clock_correction_sequence(root, resources, &clock_from, &clock_to, sat)?;
        let sequence = if let Some(sequence) = sequence {
            sequence
        } else {
            // Retain getClockCorrections/getCorrection's supported UTC assumption.
            warnings.push(format!(
                "CLK4/CLK6: no directed {clock_from} -> {clock_to} at {sat}; assuming {clock_from} = UTC"
            ));
            if clock_to == "UTC" {
                return Ok(Vec::new());
            }
            if let Some(sequence) =
                self.get_clock_correction_sequence(root, resources, "UTC", &clock_to, sat)?
            {
                sequence
            } else {
                // Source eventually returns an empty chain when even UTC->TT is
                // unavailable, yielding apparently complete uncorrected timing.
                // TT is indispensable for M1b: fail instead (decision 0004).
                return Err(TimingError::new(
                    root.join("clock"),
                    None,
                    "clock",
                    format!("no indispensable UTC -> {clock_to} chain at MJD {sat}"),
                ));
            }
        };
        let mut total = 0.0;
        let mut steps = Vec::new();
        for i in sequence {
            let function = &self.functions[i];
            let value = function
                .table
                .as_ref()
                .unwrap()
                .get_value(sat + total / 86400.0, warnings)?;
            total += value;
            let relative = function
                .path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            steps.push(ClockStep {
                corrects_to: function.clock_to.clone(),
                correction_s: crate::Scalar(rug::Float::with_val(crate::PRECISION_BITS, value)),
                resource: relative,
            });
        }
        Ok(steps)
    }
}
