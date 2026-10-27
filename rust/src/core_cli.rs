//! tempo2.C::main and getInputs.C::{getInputs,setPlugPath,printplugs}.
//! Source command grammar on canonical owners; no plugin ABI or second engine.
use crate::predictor_number::PredictorNumber as N;
#[cfg(test)]
#[path = "core_cli_tests.rs"]
mod tests;
use crate::{
    RuntimeContext, TimingError,
    clock_diagnostics::boundary,
    core_output::{self, OutputIo, OutputOptions},
    observation::{read, read_string, write_string},
    pulsar_lifecycle::PulsarContext,
    source_format::{Argument, format},
    tempo2::Label::*,
};
use std::{
    ffi::{OsStr, OsString},
    io::{IsTerminal, Write},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};
pub const IDENTITY: &str = concat!(
    "tempo2-rust ",
    env!("CARGO_PKG_VERSION"),
    " (TEMPO2 source fda6ad4a9753ad7dc5d291406f2f310a9161423f; Rug128)"
);
pub const HELP: &str = include_str!("../data/help.txt");
pub const HELP_EXTRA: &str = include_str!("../data/help_extra.txt");
/// Rust environment snapshot. Callers supply paths; executing never changes cwd or environment.
#[derive(Default, Clone, Debug)]
pub struct Environment {
    pub runtime: Option<PathBuf>,
    pub test_runtime: Option<PathBuf>,
    pub clock_directory: Option<PathBuf>,
    pub plugin_path: Option<OsString>,
    pub machine: Option<OsString>,
    pub reminder_date: Option<Vec<u8>>,
    pub log_outfile_is_terminal: bool,
}
impl Environment {
    pub fn from_process() -> Self {
        Self {
            runtime: std::env::var_os("TEMPO2").map(Into::into),
            test_runtime: std::env::var_os("TEMPO2_TEST").map(Into::into),
            clock_directory: std::env::var_os("TEMPO2_CLOCK_DIR").map(Into::into),
            plugin_path: std::env::var_os("TEMPO2_PLUG_PATH"),
            machine: std::env::var_os("LOGIN_ARCH"),
            reminder_date: None,
            log_outfile_is_terminal: std::io::stdout().is_terminal(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Inputs {
    pub arguments: Vec<Vec<u8>>,
    pub par: Vec<PathBuf>,
    pub tim: Vec<PathBuf>,
    pub list: bool,
    pub residuals: bool,
    pub model: bool,
    pub new_par: Option<PathBuf>,
    pub pre_only: bool,
    pub predictor: Option<(i32, Vec<u8>)>,
    pub polyco_prefix: PathBuf,
    pub covariance: Option<String>,
    pub dcm: Option<PathBuf>,
    pub clock_directory: Option<PathBuf>,
    pub fit_function: Option<Vec<u8>>,
    pub output_plugin: Option<Vec<u8>>,
    pub graphics: Option<(bool, Vec<u8>)>,
    pub filter: Vec<u8>,
    pub pass: Vec<u8>,
    pub delete: Option<Vec<u8>>,
    pub no_rescale: bool,
}
fn path(b: &[u8]) -> PathBuf {
    OsStr::from_bytes(b).into()
}
fn operand(args: &[Vec<u8>], i: usize) -> Result<&[u8], TimingError> {
    args.get(i + 1).map(Vec::as_slice).ok_or_else(|| {
        TimingError::new(
            "",
            None,
            "command argument",
            format!(
                "missing operand after {}",
                String::from_utf8_lossy(&args[i])
            ),
        )
    })
}
fn integer(b: &[u8]) -> Result<i32, TimingError> {
    crate::clock_text::integer_prefix(b)?
        .map(|x| x.0)
        .ok_or_else(|| boundary("command_integer"))
}
/// getInputs argument grouping. Unrecognized tokens remain available to preProcess.
pub fn get_inputs(
    args: Vec<Vec<u8>>,
    directory: &Path,
    io: &mut OutputIo<'_>,
) -> Result<Inputs, TimingError> {
    let mut c = Inputs {
        arguments: args,
        par: vec![],
        tim: vec![],
        list: false,
        residuals: false,
        model: false,
        new_par: None,
        pre_only: false,
        predictor: None,
        polyco_prefix: PathBuf::new(),
        covariance: None,
        dcm: None,
        clock_directory: None,
        fit_function: None,
        output_plugin: None,
        graphics: None,
        filter: vec![],
        pass: vec![],
        delete: None,
        no_rescale: false,
    };
    if c.arguments.len() == 2 && !c.arguments[1].eq_ignore_ascii_case(b"-allParTim") {
        let t = &c.arguments[1];
        if t.len() < 3 {
            return Err(boundary("tim_filename_suffix"));
        }
        let mut p = t.clone();
        p.truncate(p.len() - 3);
        p.extend(b"par");
        c.par.push(path(&p));
        c.tim.push(path(t));
        return Ok(c);
    }
    let mut i = 1;
    let mut got_tim = false;
    while i < c.arguments.len() {
        let a = c.arguments[i].clone();
        let value = || operand(&c.arguments, i);
        match a.as_slice() {
            b"-f" => {
                c.par.push(path(value()?));
                i += 1;
                if c.arguments.get(i + 1).is_some_and(|s| !s.starts_with(b"-")) {
                    i += 1;
                    c.tim.push(path(&c.arguments[i]));
                    got_tim = true;
                }
            }
            b"-fitfunc" => {
                c.fit_function = Some(value()?.to_vec());
                i += 1;
            }
            b"-gr" | b"-gr2" => {
                c.graphics = Some((a == b"-gr2", value()?.to_vec()));
            }
            b"-dcm" => {
                c.dcm = Some(path(value()?));
                i += 1;
            }
            b"-dcf" | b"-chol" => {
                c.covariance = Some(
                    std::str::from_utf8(value()?)
                        .map_err(|_| boundary("covariance_selector_encoding"))?
                        .into(),
                );
                i += 1;
            }
            b"-filter" => {
                let v = value()?.to_vec();
                c.filter.extend(v);
                c.filter.push(b' ');
                i += 1;
            }
            b"-pass" => {
                let v = value()?.to_vec();
                c.pass.extend(v);
                c.pass.push(b' ');
                i += 1;
            }
            b"-list" => c.list = true,
            b"-output" => {
                c.output_plugin = Some(value()?.to_vec());
                i += 1;
            }
            b"-residuals" => c.residuals = true,
            b"-del" => {
                c.delete = Some(value()?.to_vec());
                i += 1;
            }
            b"-model" => c.model = true,
            b"-newpar" => c.new_par = Some("new.par".into()),
            b"-outpar" => {
                c.new_par = Some(path(value()?));
                i += 1;
            }
            b"-pre" => c.pre_only = true,
            b"-pred" | b"-polyco" => {
                c.predictor = Some((if a == b"-pred" { 2 } else { 1 }, value()?.to_vec()));
                i += 1;
            }
            b"-polyco_file" => {
                c.polyco_prefix = path(value()?);
                i += 1;
            }
            b"-clkdir" => {
                c.clock_directory = Some(path(value()?));
                i += 1;
            }
            _ if a.eq_ignore_ascii_case(b"-norescale") => c.no_rescale = true,
            _ if a.eq_ignore_ascii_case(b"-allParTim") => {
                io.write(b"Using all available .par and .tim files\n")?;
                // fda6ad4 getInputs.C:131-151 pipes unquoted filenames through shell
                // substitutions and fscanf. Native whitespace/suffix witnesses show
                // corrupted names. Enumerate actual matching files, preserving bytes.
                let mut pairs = vec![];
                for e in std::fs::read_dir(directory)
                    .map_err(|e| TimingError::new(directory, None, "allParTim", e.to_string()))?
                {
                    let e = e.map_err(|e| {
                        TimingError::new(directory, None, "allParTim", e.to_string())
                    })?;
                    let name = e.file_name();
                    if name.as_bytes().ends_with(b".par") {
                        let p = PathBuf::from(name);
                        let t = p.with_extension("tim");
                        if directory.join(&t).is_file() {
                            pairs.push((p, t));
                        }
                    }
                }
                pairs.sort_by(|a, b| a.0.as_os_str().as_bytes().cmp(b.0.as_os_str().as_bytes()));
                for (p, t) in pairs {
                    c.par.push(p);
                    c.tim.push(t);
                    got_tim = true;
                }
                io.write(&format(
                    "Obtained files for %d pulsars\n",
                    &[Argument::from(c.par.len() as i32)],
                )?)?;
            }
            _ if i == c.arguments.len() - 1 && !got_tim => c.tim.push(path(&a)),
            _ => (),
        }
        i += 1;
    }
    if c.graphics.is_none() {
        if c.tim.is_empty() && c.predictor.is_none() {
            return Err(TimingError::new(
                "",
                None,
                "FILE1",
                "No .tim file given on command line",
            ));
        }
        if c.par.is_empty() {
            return Err(TimingError::new(
                "",
                None,
                "FILE2",
                "No .par file given on command line",
            ));
        }
    }
    if c.predictor.is_none() && c.graphics.is_none() && c.tim.len() != c.par.len() {
        return Err(boundary("command_PAR_TIM_pair_count"));
    }
    Ok(c)
}
/// setPlugPath: explicit override paths precede the runtime plugins directory.
pub fn set_plug_path(extra: Option<&OsStr>, runtime: &Path) -> Vec<PathBuf> {
    let mut paths = vec![];
    if let Some(extra) = extra {
        let b = extra.as_bytes();
        let mut start = 0;
        while start < b.len() {
            let end = b[start..]
                .iter()
                .position(|&x| x == b':')
                .map_or(b.len(), |i| i + start);
            paths.push(path(&b[start..end]));
            start = end + 1;
        }
    }
    paths.push(runtime.join("plugins/"));
    paths
}
pub fn printplugs(
    paths: &[PathBuf],
    machine: &[u8],
    full: bool,
    out: &mut dyn Write,
) -> Result<(), TimingError> {
    let mut seen = vec![];
    let suffix = [machine, b"_plug.t2"].concat();
    for p in paths {
        let mut bytes = format("in '%s'\n", &[Argument::from(p.as_os_str().as_bytes())])?;
        let entries = std::fs::read_dir(p);
        let mut count = 0;
        if let Ok(entries) = entries {
            for entry in entries {
                let e = entry
                    .map_err(|e| TimingError::new(p, None, "plugin directory", e.to_string()))?;
                let name = e.file_name();
                let name = name.as_bytes();
                if let Some(j) = name.iter().position(|&b| b == b'_')
                    && name[j..].windows(suffix.len()).any(|x| x == suffix)
                    && name.ends_with(&suffix)
                {
                    let stem = name[..j].to_vec();
                    let duplicate = seen.contains(&stem);
                    if full || !duplicate {
                        bytes.extend(format(
                            " - %s%c\n",
                            &[
                                Argument::from(stem.clone()),
                                Argument::from(if duplicate { b'#' } else { b' ' } as i32),
                            ],
                        )?);
                        count += 1;
                    }
                    seen.push(stem);
                }
            }
            if count == 0 {
                bytes.extend(b"(none or all hidden)\n");
            }
        } else {
            bytes.extend(b"(dir not readable)\n");
        }
        out.write_all(&bytes)
            .map_err(|e| TimingError::new(p, None, "plugins", e.to_string()))?;
    }
    Ok(())
}
#[allow(clippy::type_complexity)] // source command tokens plus its four early controls
fn expand(
    mut args: Vec<Vec<u8>>,
    env: &Environment,
    io: &mut OutputIo<'_>,
) -> Result<(Vec<Vec<u8>>, bool, Vec<u8>, i32, bool), TimingError> {
    let mut test = false;
    let mut machine = env
        .machine
        .as_ref()
        .map(|s| s.as_bytes().to_vec())
        .unwrap_or_else(|| std::env::consts::ARCH.as_bytes().to_vec());
    let mut warnings = 1;
    let mut jbo = false;
    let mut i = 1;
    let mut ancestry: Vec<Vec<(PathBuf, Vec<u8>)>> = vec![vec![]; args.len()];
    while i < args.len() {
        if args[i] == b"-machine" {
            machine = operand(&args, i)?.to_vec();
            i += 2;
            continue;
        }
        if args[i].eq_ignore_ascii_case(b"-noWarnings") {
            warnings = 2;
        } else if args[i].eq_ignore_ascii_case(b"-allInfo") {
            warnings = 0;
        } else if args[i] == b"-jbo" {
            main_log(b'm', 154, b"Reading JBO format!!", io)?;
            jbo = true;
        } else if args[i] == b"-test" {
            test = true;
        } else if let Some(root) = if test {
            env.test_runtime.as_ref()
        } else {
            env.runtime.as_ref()
        } && let Ok(bytes) = std::fs::read(root.join("alias.dat"))
        {
            let mut changed = false;
            for line in bytes.split(|&b| b == b'\n') {
                let mut words = line.split(|&b| b == b' ').filter(|s| !s.is_empty());
                let Some(key) = words.next() else {
                    continue;
                };
                if key == args[i] {
                    // fda6ad4 tempo2.C:155-194 has NULL strtok, fixed storage,
                    // and unbounded recursive aliases. Owned tokens plus explicit
                    // repeated expansion detection preserve valid order and terminate cycles.
                    let signature = (root.clone(), key.to_vec());
                    if ancestry[i].contains(&signature) {
                        return Err(boundary("cyclic_command_alias"));
                    }
                    let replacement = words.map(|s| s.to_vec()).collect::<Vec<_>>();
                    if replacement.is_empty() {
                        return Err(boundary("empty_command_alias"));
                    }
                    io.write(&format("Using alias: %s\n", &[Argument::from(key)])?)?;
                    let mut ancestors = ancestry[i].clone();
                    ancestors.push(signature);
                    ancestry.splice(i..=i, vec![ancestors; replacement.len()]);
                    args.splice(i..=i, replacement);
                    changed = true;
                    break;
                }
            }
            if changed {
                continue;
            }
        }
        i += 1;
    }
    Ok((args, test, machine, warnings, jbo))
}
fn drain(io: &mut OutputIo<'_>) -> Result<(), TimingError> {
    crate::clkcorr::with_diagnostics(|d| io.drain(d))
}
fn main_log(kind: u8, line: usize, body: &[u8], io: &mut OutputIo<'_>) -> Result<(), TimingError> {
    crate::clkcorr::with_diagnostics(|d| {
        if kind == b't' {
            d.timing("tempo2.C", line, body)
        } else {
            d.log(kind, "tempo2.C", line, body)
        }
        io.drain(d)
    })
}
/// Execute argv including argv[0]. Returns source status; errors are recoverable.
pub fn run(
    argv: &[OsString],
    env: &Environment,
    io: &mut OutputIo<'_>,
) -> Result<i32, TimingError> {
    let args = argv
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();
    if args.is_empty() {
        return Err(boundary("command_argv0"));
    }
    if args.get(1).is_some_and(|s| s == b"-version" || s == b"-v") {
        io.write(format!("{IDENTITY}\n").as_bytes())?;
        return Ok(1);
    }
    io.write(b"This program comes with ABSOLUTELY NO WARRANTY.\nThis is free software, and you are welcome to redistribute it\nunder conditions of GPL license.\n\n")?;
    let mut max_psr = 40;
    let mut max_obs = 20000;
    let mut debug = false;
    let mut tcheck = false;
    let mut versions = false;
    let mut writeres = 0;
    let mut writetim = false;
    let mut veryfast = 0;
    let mut allglitch = false;
    let mut acceleration = crate::t2accel::NATIVE_DEFAULT;
    for (i, a) in args.iter().enumerate() {
        let lower = a.to_ascii_lowercase();
        match lower.as_slice() {
            b"-npsr" => max_psr = integer(operand(&args, i)?)?,
            b"-nobs" => max_obs = integer(operand(&args, i)?)?,
            b"-debug" => {
                debug = true;
                tcheck = true;
                writeres = 255;
            }
            b"-tcheck" => tcheck = true,
            b"-displayversion" => versions = true,
            b"-writeres" => writeres = 255,
            b"-writetim" => writetim = true,
            b"-veryfast" => veryfast = 1,
            b"-allglitch" => allglitch = true,
            b"-noaccel" => acceleration = 0,
            b"-qrfit" => acceleration = 2,
            b"-svdfit" if acceleration != 0 => acceleration = 1,
            _ => (),
        }
    }
    let _acceleration_scope = crate::t2accel::select(acceleration);
    if max_psr <= 0 || max_obs <= 0 {
        return Err(boundary("command_allocation_count"));
    }
    let start = crate::clock_diagnostics::cpu_clock();
    crate::clkcorr::with_process(|s| {
        s.source.max_observations = max_obs;
        s.source.very_fast = veryfast;
        s.diagnostics.debug = debug;
        s.diagnostics.tcheck = i32::from(tcheck);
        s.diagnostics.display_version = versions;
        s.diagnostics.log_outfile_is_terminal = env.log_outfile_is_terminal;
        s.diagnostics.timer_clock = start;
        s.diagnostics.version("tempo2.C", "main()");
        Ok(())
    })?;
    drain(io)?;
    let (args, test, machine, warnings, jbo) = expand(args, env, io)?;
    let runtime = (if test {
        &env.test_runtime
    } else {
        &env.runtime
    })
    .as_ref()
    .ok_or_else(|| {
        TimingError::new(
            "",
            None,
            "environment",
            format!(
                "Environment variable >{}< not set",
                if test { "TEMPO2_TEST" } else { "TEMPO2" }
            ),
        )
    })?;
    let plugins = set_plug_path(env.plugin_path.as_deref(), runtime);
    if args.iter().any(|a| a.eq_ignore_ascii_case(b"-reminder")) {
        let date = env
            .reminder_date
            .as_deref()
            .ok_or_else(|| boundary("command_history_date"))?;
        let dest = io.directory.join("T2command.input");
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&dest)
            .map_err(|e| TimingError::new(&dest, None, "reminder", e.to_string()))?;
        for _ in args.iter().filter(|a| a.eq_ignore_ascii_case(b"-reminder")) {
            let mut b = [b"[", date, b"]>> "].concat();
            for a in &args {
                b.extend(b" ");
                b.extend(a);
                b.extend(b" ");
            }
            b.push(b'\n');
            f.write_all(&b)
                .map_err(|e| TimingError::new(&dest, None, "reminder", e.to_string()))?;
        }
    }
    if args.len() == 1 {
        io.write(
            format!(
                "{IDENTITY}\n  Usage:     {} -f XXX.par XXX.tim\nPlugin search paths:\n",
                String::from_utf8_lossy(&args[0])
            )
            .as_bytes(),
        )?;
        for p in &plugins {
            io.write(&format(
                " -- %s/*.t2\n",
                &[Argument::from(p.as_os_str().as_bytes())],
            )?)?;
        }
        io.write(b"\n")?;
        io.write(if cfg!(feature = "lapack") {
            b"* Using LAPACK acceleration for Cholesky decomposition\n"
        } else {
            b"* Using slow linear algebra code (compile with LAPACK for speed improvements)\n"
        })?;
        io.write(if cfg!(feature = "blas") {
            b"* Using BLAS acceleration for matrix mulitplication\n"
        } else {
            b"* Using slow matrix code (compile with BLAS for speed improvements)\n"
        })?;
        io.write(&format(
            "\nFor more help, use %s -h\n",
            &[Argument::from(args[0].clone())],
        )?)?;
        return Ok(1);
    }
    if args.len() == 2 && (args[1] == b"-h" || args[1] == b"-H") {
        io.write(format!("\n{IDENTITY}\nexamples: {} mytim.tim\n          {} -f mypar.par mytim.tim\n          {} -gr plk -f mypar.par mytim.tim\n\n{HELP}\nCurrent Environment:\n",String::from_utf8_lossy(&args[0]),String::from_utf8_lossy(&args[0]),String::from_utf8_lossy(&args[0])).as_bytes())?;
        for (name, value) in [
            ("TEMPO2", env.runtime.as_ref().map(|p| p.as_os_str())),
            (
                "TEMPO2_CLOCK_DIR",
                env.clock_directory.as_ref().map(|p| p.as_os_str()),
            ),
            ("TEMPO2_PLUG_PATH", env.plugin_path.as_deref()),
        ] {
            if let Some(v) = value {
                io.write(&format(
                    "   $%s='%s'\n",
                    &[Argument::from(name), Argument::from(v.as_bytes())],
                )?)?;
            } else {
                io.write(
                    format!(
                        "   ${name} is {}\n",
                        if name == "TEMPO2" {
                            "UNSET (this is BAD!)"
                        } else {
                            "unset (this is normal)"
                        }
                    )
                    .as_bytes(),
                )?;
            }
        }
        if args[1] == b"-H" {
            io.write(format!("{HELP_EXTRA}\n\n\nAvailable plugins\n").as_bytes())?;
            printplugs(&plugins, &machine, false, io.stdout)?;
            io.write(b"-----------------\n")?;
        }
        return Ok(1);
    }
    let directory = io.directory.to_path_buf();
    main_log(b'd', 296, b"Running getInputs 1", io)?;
    let c = get_inputs(args, &directory, io)?;
    main_log(b'd', 299, b"Completed getInputs", io)?;
    if let Some((_, p)) = &c.graphics {
        io.write(&format(
            "Plugin integration unavailable in this Rust build: %s\n",
            &[Argument::from(p.clone())],
        )?)?;
        return Ok(-1);
    }
    if c.par.len() > max_psr as usize {
        return Err(boundary("command_MAX_PSR"));
    }
    let mut context = PulsarContext::unwritten(c.par.len());
    context
        .initialise(c.par.len() as i32, 0, warnings)
        .map_err(|e| TimingError::new("", None, "initialise", e.to_string()))?;
    context.pulsars[0].jboFormat = Some(i32::from(jbo));
    if c.no_rescale {
        for p in &mut context.pulsars {
            p.rescaleErrChisq = Some(0);
        }
    }
    if let Some(v) = &c.fit_function {
        write_string(&mut context.pulsars[0].fitFunc[..], v, "fitFunc")?;
    }
    for (field, v) in [
        ("filter", Some(&c.filter)),
        ("pass", Some(&c.pass)),
        ("delete", c.delete.as_ref()),
    ] {
        if let Some(v) = v {
            let p = &mut context.pulsars[0];
            let target = match field {
                "filter" => &mut p.filterStr[..],
                "pass" => &mut p.passStr[..],
                _ => &mut p.deleteFileName[..],
            };
            write_string(target, v, field)?;
        }
    }
    crate::clkcorr::with_process(|s| {
        s.extra = c
            .clock_directory
            .as_ref()
            .or(env.clock_directory.as_ref())
            .map(|p| p.as_os_str().as_bytes().to_vec())
            .unwrap_or_default();
        Ok(())
    })?;
    let pairs = c
        .par
        .iter()
        .enumerate()
        .map(|(i, p)| crate::bats::InputPair {
            par: directory.join(p),
            tim: c.tim.get(i).map(|t| directory.join(t)).unwrap_or_default(),
        })
        .collect::<Vec<_>>();
    let mut consumed = vec![];
    main_log(b'd', 360, b"Reading par file", io)?;
    crate::source_par::read_parfiles(&mut context, &pairs, runtime, &mut consumed)?;
    main_log(
        b'd',
        362,
        format!(
            "Finished reading par file {}",
            read(&context.pulsars[0].nits, "nits")?
        )
        .as_bytes(),
        io,
    )?;
    if c.predictor.is_none() {
        main_log(b'd', 365, b"Running readTimfile", io)?;
        crate::source_tim::read_timfiles(
            &mut context,
            &pairs.iter().map(|p| p.tim.clone()).collect::<Vec<_>>(),
            runtime,
            &mut consumed,
        )?;
        main_log(
            b'd',
            367,
            format!(
                "Completed readTimfile {}",
                context.pulsars[0].param.slots[param_ecc as usize].cells[1].param_set
            )
            .as_bytes(),
            io,
        )?;
    }
    main_log(
        b'd',
        370,
        format!(
            "Running preProcess {}",
            read(&context.pulsars[0].nits, "nits")?
        )
        .as_bytes(),
        io,
    )?;
    crate::preprocess::pre_process(&mut context, &c.arguments, runtime, &mut consumed)?;
    main_log(
        b'd',
        372,
        format!(
            "Completed preProcess {}",
            read(&context.pulsars[0].nits, "nits")?
        )
        .as_bytes(),
        io,
    )?;
    if writetim {
        main_log(b'm', 374, b"write out.tim", io)?;
        crate::source_tim::write_tim(
            &directory.join("out.tim"),
            &mut context,
            0,
            b"tempo2",
            runtime,
        )?;
        drain(io)?;
    }
    let mut r = RuntimeContext::from_root(runtime)?;
    if let Some((kind, spec)) = &c.predictor {
        predict(&mut context, &mut r, *kind, spec, &c.polyco_prefix, io)?;
        main_log(b't', 503, b"Return from main()", io)?;
        return Ok(0);
    }
    let options = crate::general_fitting::FitOptions {
        covariance: c.covariance.clone(),
        rescale_err_chisq: !c.no_rescale,
        force_always_fit_for_glitches: allglitch,
        acceleration,
        diagnostics: (writeres != 0).then(|| crate::tkfit::FitDiagnostics {
            directory: directory.clone(),
            write_residuals: writeres,
            debug,
        }),
    };
    let nits = *read(&context.pulsars[0].nits, "nits")?;
    for it in 0..nits {
        if it > 0 {
            for p in &mut context.pulsars {
                for s in &mut p.param.slots {
                    for cell in &mut s.cells {
                        cell.prefit = Some(cell.val.clone());
                        cell.prefit_err = Some(cell.err.clone());
                    }
                }
            }
        }
        for iteration in 0..2 {
            main_log(b'd', 531, format!("iteration {iteration}").as_bytes(), io)?;
            main_log(b'd', 532, b"calling formBatsAll", io)?;
            main_log(b't', 533, b"call formBatsAll()", io)?;
            crate::global::form_bats_all(&mut context, &mut r)?;
            main_log(b'd', 536, b"calling formResiduals", io)?;
            main_log(b't', 537, b"call formResiduals()", io)?;
            crate::timing::form_residuals(&mut context, true)?;
            drain(io)?;
            if c.list && iteration == 0 {
                core_output::display_parameters(13, &c.tim, &c.par, &context, io)?;
            }
            if iteration == 0 {
                main_log(b'd', 562, b"calling doFit", io)?;
                main_log(b't', 564, b"calling t2Fit", io)?;
                crate::general_fitting::t2_fit(&mut context, &options)?;
                main_log(b'm', 566, b"Complete fit", io)?;
                main_log(b'd', 568, b"completed doFit", io)?;
            }
            if iteration == 1 || c.pre_only {
                if let Some(p) = &c.output_plugin {
                    io.write(&format(
                        "Plugin integration unavailable in this Rust build: %s\n",
                        &[Argument::from(p.clone())],
                    )?)?;
                    return Ok(-1);
                }
                core_output::text_output(
                    &mut context,
                    &OutputOptions {
                        residuals: c.residuals,
                        new_par: c.new_par.clone(),
                        ..Default::default()
                    },
                    io,
                )?;
            }
            context.pulsars[0].noWarnings = Some(2);
            if c.pre_only {
                break;
            }
        }
    }
    let elapsed = crate::clock_diagnostics::cpu_clock().wrapping_sub(start) as f32 / 1_000_000f32;
    io.write(&format(
        "Finishing off: time taken = %.2f (s)\n",
        &[Argument::from(elapsed)],
    )?)?;
    main_log(b't', 624, b"Exit from main()", io)?;
    Ok(0)
}
fn predict(
    context: &mut PulsarContext,
    r: &mut RuntimeContext,
    kind: i32,
    spec: &[u8],
    prefix: &Path,
    io: &mut OutputIo<'_>,
) -> Result<(), TimingError> {
    let tokens = crate::clock_text::tokens(spec).collect::<Vec<_>>();
    let count = if kind == 1 { 7 } else { 8 };
    if tokens.len() < count {
        return Err(boundary(if kind == 1 {
            "Error parsing -polyco arguments! See tempo2 -h."
        } else {
            "Error parsing -pred arguments! See tempo2 -h."
        }));
    }
    let site = std::str::from_utf8(tokens[if kind == 1 { 5 } else { 0 }])
        .map_err(|_| boundary("predictor_site_encoding"))?;
    if kind == 2 {
        io.write(&format(
            "Calculating predictor >%s<\n",
            &[Argument::from(read_string(
                &context.pulsars[0].tzrsite[..],
                "tzrsite",
            )?)],
        )?)?;
    } else if context.pulsars[0].tempo1 == Some(0) {
        io.write(b"WARNING: Should probably use -tempo1 option\n")?;
    }
    let mut dimensions = (0, 0);
    if kind == 2 {
        dimensions = (integer(tokens[5])?, integer(tokens[6])?);
        for (n, label) in [
            (&mut dimensions.0, "time"),
            (&mut dimensions.1, "frequency"),
        ] {
            if *n % 2 != 0 {
                *n = n
                    .checked_add(1)
                    .ok_or_else(|| boundary("predictor_order"))?;
                io.write(&format(
                    "Adjusting number of coefficients (%s axis) to %d (must be even)\n",
                    &[Argument::from(label), Argument::from(*n)],
                )?)?;
            }
        }
    }
    let p = &context.pulsars[0];
    if p.param.slots[param_tzrmjd as usize].cells[0].param_set == 0 {
        io.write(&format(
            "WARNING: tzrmjd not set.  Setting to %g\n",
            &[Argument::from(
                p.param.slots[param_pepoch as usize].cells[0].val.to_f64(),
            )],
        )?)?;
    }
    if p.param.slots[param_tzrfrq as usize].cells[0].param_set == 0 {
        io.write(b"WARNING: tzrfrq not set.  Setting to 1400\n")?;
    }
    if read_string(&p.tzrsite[..], "tzrsite")? == b"NULL" {
        io.write(&format(
            "WARNING: tzrsite not set.  Setting to %s\n",
            &[Argument::from(site)],
        )?)?;
    }
    crate::predictor::cli_defaults(context, site)?;
    if kind == 1 {
        let mut full = io.directory.as_os_str().to_os_string();
        full.push("/");
        full.push(prefix);
        let req = crate::polyco::PolycoRequest {
            start_mjd: crate::source_text::extended(tokens[0])?,
            end_mjd: crate::source_text::extended(tokens[1])?,
            span_minutes: integer(tokens[2])?,
            coefficients: usize::try_from(integer(tokens[3])?)
                .map_err(|_| boundary("polyco_order"))?,
            max_hour_angle: crate::source_text::extended(tokens[4])?,
            site: site.into(),
            frequency_mhz: crate::source_text::extended(tokens[6])?,
            true_dm: true,
            file_prefix: if prefix.is_absolute() {
                prefix.into()
            } else {
                full.into()
            },
        };
        // fda6ad4 polyco.C:160-161 repeats the main's warning on entry.
        // AF's construction API carries state/results; this adapter owns stdout.
        if context.pulsars[0].tempo1 == Some(0) {
            io.write(b"WARNING: Should probably use -tempo1 option\n")?;
        }
        crate::polyco::polyco(context, r, &req)?;
    } else {
        let days = crate::source_text::extended(tokens[7])?
            / rug::Float::with_val(crate::PRECISION_BITS, 86400);
        let req = crate::cheby2d_int::ChebyRequest {
            site: site.into(),
            start_mjd: N::from_float(&crate::source_text::extended(tokens[1])?),
            end_mjd: N::from_float(&crate::source_text::extended(tokens[2])?),
            freq_start_mhz: N::from_float(&crate::source_text::extended(tokens[3])?),
            freq_end_mhz: N::from_float(&crate::source_text::extended(tokens[4])?),
            segment_days: N::from_float(&days),
            overlap_days: N::from_float(&(days * 0.1f64)),
            time_coeffs: usize::try_from(dimensions.0).map_err(|_| boundary("predictor_order"))?,
            freq_coeffs: usize::try_from(dimensions.1).map_err(|_| boundary("predictor_order"))?,
        };
        let tim = io.directory.join("pred.tim");
        let mut set = crate::cheby2d_int::construct_set(&req, context, r, &tim)?;
        crate::tempo2pred::T2Predictor::Cheby(set.clone())
            .write(&io.directory.join("t2pred.dat"))
            .map_err(crate::predictor::error)?;
        let n = set.segments.len();
        let summary = crate::cheby2d_int::test_set(
            &mut set,
            context,
            r,
            req.time_coeffs * 5 * n,
            req.freq_coeffs * 5 * n,
            &tim,
        )?;
        let f0 = crate::predictor::spin_frequency(context)?;
        let rms = &summary.rms_turn / N::from_float(&f0);
        let mav = N::from_float(
            &(rug::Float::with_val(crate::PRECISION_BITS, summary.mav_turn.as_float()) / f0),
        );
        io.write(b"Predictive model constructed and written to t2pred.dat.\n")?;
        io.write(&format(
            "RMS error = %.3Lg s MAV= %.3Lg s\n",
            &[
                Argument::from(rms.as_float().clone()),
                Argument::from(mav.as_float().clone()),
            ],
        )?)?;
    }
    Ok(())
}
