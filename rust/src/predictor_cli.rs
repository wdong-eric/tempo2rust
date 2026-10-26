//! tempo2.C predictor branches / getInputs.C argument routing, Rust JSON adapter.
use std::{
    collections::BTreeMap,
    ffi::OsString,
    io::{self, Write},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};
use tempo2_rust::{
    RuntimeContext, Scalar,
    cheby2d_int::{self, ChebyRequest},
    polyco::{self, PolycoRequest},
    predictor,
    predictor_number::PredictorNumber as N,
    tempo2pred::T2Predictor,
};
pub fn run(args: &[OsString]) -> Result<(), Box<dyn std::error::Error>> {
    let route = args
        .first()
        .ok_or("predictor requires cheby, polyco, evaluate or plan")?
        .to_str()
        .ok_or("non-UTF8 route")?;
    let allowed: &[&str] = match route {
        "evaluate" => &["input", "mjd", "frequency-mhz"],
        "plan" => &[
            "input",
            "start-mjd",
            "end-mjd",
            "step-seconds",
            "frequency-mhz",
        ],
        "cheby" => &[
            "par",
            "runtime",
            "site",
            "start-mjd",
            "end-mjd",
            "freq-start-mhz",
            "freq-end-mhz",
            "time-coeffs",
            "freq-coeffs",
            "segment-seconds",
            "output-dir",
        ],
        "polyco" => &[
            "par",
            "runtime",
            "site",
            "start-mjd",
            "end-mjd",
            "span-minutes",
            "coefficients",
            "max-hour-angle",
            "frequency-mhz",
            "output-dir",
        ],
        _ => return Err("unknown predictor route".into()),
    };
    let mut values = BTreeMap::new();
    let mut pre = vec![];
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--" {
            pre = args[i + 1..]
                .iter()
                .map(|a| a.as_bytes().to_vec())
                .collect();
            break;
        }
        let key = args[i]
            .to_str()
            .and_then(|s| s.strip_prefix("--"))
            .ok_or("expected --option")?;
        if !allowed.contains(&key) || values.contains_key(key) {
            return Err(format!("unknown/duplicate predictor option {key}").into());
        }
        let value = args.get(i + 1).ok_or("missing predictor option value")?;
        values.insert(key, value);
        i += 2;
    }
    let text = |key: &str| -> Result<&str, Box<dyn std::error::Error>> {
        values
            .get(key)
            .ok_or_else(|| format!("missing --{key}"))?
            .to_str()
            .ok_or_else(|| format!("non-UTF8 --{key}").into())
    };
    let path = |key: &str| -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(PathBuf::from(
            values.get(key).ok_or_else(|| format!("missing --{key}"))?,
        ))
    };
    let number = |key: &str| -> Result<N, Box<dyn std::error::Error>> { Ok(text(key)?.parse()?) };
    let json = if route == "evaluate" || route == "plan" {
        if !pre.is_empty() {
            return Err("preprocessing applies only to generation".into());
        }
        let file = T2Predictor::read(&path("input")?)?;
        let freq = number("frequency-mhz")?;
        if route == "evaluate" {
            let mjd = number("mjd")?;
            serde_json::json!({"schema":"tempo2-predictor-evaluate-v1","input":file,"metadata":file.predictor.metadata()?,"mjd":mjd,"frequency_mhz":freq,"prediction":file.predictor.evaluate(&mjd,&freq)?})
        } else {
            let result = file.predictor.plan(
                &number("start-mjd")?,
                &number("end-mjd")?,
                &number("step-seconds")?,
                &freq,
            )?;
            serde_json::json!({"schema":"tempo2-predictor-plan-v1","input":file,"segment_count":result.segments.len(),"plan":result})
        }
    } else {
        let root = path("runtime")?;
        let mut runtime = RuntimeContext::from_root(&root)?;
        let mut input = predictor::load_par(&path("par")?, &root, &pre)?;
        let site = text("site")?;
        let warnings = predictor::cli_defaults(&mut input.context, site)?;
        let output = path("output-dir")?;
        std::fs::create_dir_all(&output)?;
        let result = if route == "cheby" {
            let requested_nt: usize = text("time-coeffs")?.parse()?;
            let requested_nf: usize = text("freq-coeffs")?.parse()?;
            let nt = requested_nt
                .checked_add(requested_nt % 2)
                .ok_or("time count overflow")?;
            let nf = requested_nf
                .checked_add(requested_nf % 2)
                .ok_or("frequency count overflow")?;
            // CLI parses timing longdouble first, divides seconds there, then
            // converts arguments to the native-long-double constructor boundary.
            let days = text("segment-seconds")?
                .parse::<Scalar>()?
                .as_float()
                .clone()
                / 86400f64;
            let request = ChebyRequest {
                site: site.into(),
                start_mjd: number("start-mjd")?,
                end_mjd: number("end-mjd")?,
                segment_days: N::from_float(&days),
                overlap_days: N::from_float(&(days * 0.1f64)),
                freq_start_mhz: number("freq-start-mhz")?,
                freq_end_mhz: number("freq-end-mhz")?,
                time_coeffs: nt,
                freq_coeffs: nf,
            };
            let tim = output.join("pred.tim");
            let mut set =
                cheby2d_int::construct_set(&request, &mut input.context, &mut runtime, &tim)?;
            T2Predictor::Cheby(set.clone()).write(&output.join("t2pred.dat"))?;
            let count = set.segments.len();
            let summary = cheby2d_int::test_set(
                &mut set,
                &mut input.context,
                &mut runtime,
                nt * 5 * count,
                nf * 5 * count,
                &tim,
            )?;
            serde_json::json!({"predictor":T2Predictor::Cheby(set),"requested_coefficients":[requested_nt,requested_nf],"effective_coefficients":[nt,nf],"source_test":summary,"f0_hz":predictor::spin_frequency(&input.context)?.to_string_radix(10,None),"files":[output.join("t2pred.dat"),tim]})
        } else {
            let request = PolycoRequest {
                start_mjd: text("start-mjd")?.parse::<Scalar>()?.as_float().clone(),
                end_mjd: text("end-mjd")?.parse::<Scalar>()?.as_float().clone(),
                span_minutes: text("span-minutes")?.parse()?,
                coefficients: text("coefficients")?.parse()?,
                max_hour_angle: text("max-hour-angle")?
                    .parse::<Scalar>()?
                    .as_float()
                    .clone(),
                site: site.into(),
                frequency_mhz: text("frequency-mhz")?.parse::<Scalar>()?.as_float().clone(),
                true_dm: true,
                file_prefix: output.join(""),
            };
            // PathBuf removes a trailing separator; source prefix concatenation
            // is explicit. CLI uses directory + slash, direct callers supply any prefix.
            let mut request = request;
            request.file_prefix = PathBuf::from(format!("{}/", output.display()));
            serde_json::to_value(polyco::polyco(&mut input.context, &mut runtime, &request)?)?
        };
        let diagnostics = tempo2_rust::take_clock_diagnostics();
        for d in &diagnostics {
            io::stderr().lock().write_all(&d.bytes)?;
        }
        for w in &warnings {
            eprintln!("{w}");
        }
        serde_json::json!({"schema":"tempo2-predictor-generation-v1","status":"ok","kind":route,"inputs":input.inputs,"resources":runtime.resources(),"warnings":warnings,"diagnostics":diagnostics.iter().map(|d|serde_json::json!({"bytes":d.bytes})).collect::<Vec<_>>(),"result":result,"observations":predictor::snapshot(&input.context)?})
    };
    let mut bytes = serde_json::to_vec_pretty(&json)?;
    bytes.push(b'\n');
    io::stdout().lock().write_all(&bytes)?;
    Ok(())
}
