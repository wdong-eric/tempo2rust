//! Assignment AI observer: direct canonical APIs, real provider and PAR/TIM inputs.
use std::{error::Error, io::Write, path::Path};
use tempo2_rust::{
    pulsar_lifecycle::PulsarContext,
    t2accel as a,
    tk_matrix::BlasMatrix,
    tkfit::{self, FitNumber, FitOutputs},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn value(out: &mut std::fs::File, key: &str, kind: &str, v: impl Into<FitNumber>) -> Result<()> {
    writeln!(out, "{key}\t{kind}\t{}", v.into().as_float())?;
    Ok(())
}
fn matrix(out: &mut std::fs::File, key: &str, m: &BlasMatrix) -> Result<()> {
    for (i, row) in m.to_rows().iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            value(out, &format!("{key}/{i}/{j}"), "matrix", *v)?;
        }
    }
    Ok(())
}
fn snapshot(
    out: &mut std::fs::File,
    c: &mut PulsarContext,
    stage: &str,
    timing: bool,
) -> Result<()> {
    for p in 0..c.pulsars().len() {
        let (s, obs) = c.pulsar_observations_mut(p)?;
        for (i, o) in obs.iter().take(s.nobs.unwrap() as usize).enumerate() {
            let key = format!("{stage}/{p}/{i}");
            if !timing || s.useCalceph != Some(0) {
                writeln!(
                    out,
                    "{key}/tt\ttime\t{}",
                    o.correctionTT_calcEph.as_ref().unwrap()
                )?;
            }
            for (k, v) in [
                &o.earth_ssb,
                &o.sun_ssb,
                &o.jupiter_earth,
                &o.saturn_earth,
                &o.uranus_earth,
                &o.neptune_earth,
                &o.venus_earth,
                &o.earthMoonBary_ssb,
                &o.earthMoonBary_earth,
            ]
            .iter()
            .enumerate()
            {
                for (j, x) in v.iter().enumerate() {
                    value(
                        out,
                        &format!("{key}/v/{k}/{j}"),
                        if j < 3 { "position" } else { "velocity" },
                        x.unwrap(),
                    )?;
                }
            }
            if !timing {
                for k in 0..9 {
                    for j in 0..6 {
                        for (name, planets) in [
                            ("planet", &o.planet_ssb),
                            ("next", &o.planet_ssb_tmr),
                            ("diff", &o.planet_ssb_derv),
                        ] {
                            value(
                                out,
                                &format!("{key}/{name}/{k}/{j}"),
                                if j < 3 { "position" } else { "velocity" },
                                planets[k][j].unwrap(),
                            )?;
                        }
                    }
                }
            }
            if timing {
                for (name, kind, x) in [
                    ("bat", "mjd", &o.bat),
                    ("phase", "phase", &o.phase),
                    ("residual", "time", &o.residual),
                ] {
                    writeln!(out, "{key}/{name}\t{kind}\t{}", x.as_ref().unwrap())?;
                }
            }
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).ok_or("mode")?;
    let mut out = std::fs::File::create(args.get(2).ok_or("output")?)?;
    match mode.as_str() {
        "kernels" => {
            if cfg!(feature = "blas") {
                let x = BlasMatrix::from_rows(&[vec![1., 2., 3.], vec![-1., 0., 4.]])?;
                let y = BlasMatrix::from_rows(&[vec![2., -1.], vec![0., 3.], vec![5., 1.]])?;
                matrix(&mut out, "product", &a::accel_mult_matrix(&x, &y)?)?;
                for (i, v) in a::accel_mult_matrix_vec(&x, &[2., -1., 3.])?
                    .iter()
                    .enumerate()
                {
                    value(&mut out, &format!("vector/{i}"), "matrix", *v)?;
                }
            }
            if cfg!(feature = "lapack") {
                let original = vec![vec![4., 2., 1.], vec![2., 5., 2.], vec![1., 2., 6.]];
                let mut m = BlasMatrix::from_rows(&original)?;
                value(
                    &mut out,
                    "cholfac/status",
                    "exact",
                    f64::from(a::accel_cholfac(&mut m)?),
                )?;
                matrix(&mut out, "cholfac", &m)?;
                m = BlasMatrix::from_rows(&original)?;
                value(
                    &mut out,
                    "uinv/status",
                    "exact",
                    f64::from(a::accel_uinv(&mut m)?),
                )?;
                matrix(&mut out, "uinv", &m)?;
                let mut q = BlasMatrix::from_rows(&[
                    vec![1., -1.],
                    vec![1., 0.],
                    vec![1., 1.],
                    vec![0., 1.],
                ])?;
                let mut rhs = [1., 2., 4., 3.];
                let mut p = [0.; 2];
                let mut cov = BlasMatrix::malloc_uinv(2)?;
                value(
                    &mut out,
                    "qr/chi",
                    "matrix",
                    a::accel_lsq_qr(&mut q, &mut rhs, Some(&mut p), Some(&mut cov), true)?,
                )?;
                matrix(&mut out, "qr/a", &q)?;
                matrix(&mut out, "qr/c", &cov)?;
                for (i, v) in rhs.iter().enumerate() {
                    value(&mut out, &format!("qr/rhs/{i}"), "matrix", *v)?;
                }
                for (i, v) in p.iter().enumerate() {
                    value(&mut out, &format!("qr/p/{i}"), "matrix", *v)?;
                }
            }
        }
        "failures" => {
            if cfg!(feature = "lapack") {
                let mut m = BlasMatrix::from_rows(&[vec![-1., 0.], vec![0., 1.]])?;
                value(
                    &mut out,
                    "cholfac/failure",
                    "exact",
                    a::accel_cholfac(&mut m)? as f64,
                )?;
                m = BlasMatrix::from_rows(&[vec![4., 2.], vec![2., 5.]])?;
                value(
                    &mut out,
                    "cholfac/recovery",
                    "exact",
                    a::accel_cholfac(&mut m)? as f64,
                )?;
                let mut q = BlasMatrix::from_rows(&vec![vec![1., 0.]; 3])?;
                let mut rhs = [1., 2., 4.];
                assert!(a::accel_lsq_qr(&mut q, &mut rhs, None, None, false).is_err());
                value(&mut out, "qr/failure", "exact", -1.)?;
                q = BlasMatrix::from_rows(&[vec![1., -1.], vec![1., 0.], vec![1., 1.]])?;
                rhs = [1., 2., 4.];
                value(
                    &mut out,
                    "qr/recovery",
                    "matrix",
                    a::accel_lsq_qr(&mut q, &mut rhs, None, None, false)?,
                )?;
            }
        }
        "solver" => {
            let constraint = args[3] == "1";
            let scale = args[4] == "1";
            let mask: u8 = args[5].parse()?;
            let m = vec![vec![1., -1.], vec![1., 0.], vec![1., 1.]];
            let y = [1., 2., 4.];
            let c = if constraint {
                vec![vec![0., 1.]]
            } else {
                vec![]
            };
            let target = if constraint { vec![3.] } else { vec![] };
            let f = a::with_acceleration(2, || {
                tkfit::least_squares_outputs(
                    Some(&y),
                    Some(&y),
                    &m,
                    &m,
                    &c,
                    &target,
                    1e-27,
                    scale,
                    FitOutputs {
                        parameters: mask & 1 != 0,
                        errors: mask & 2 != 0,
                        covariance: mask & 4 != 0,
                    },
                    None,
                )
            })?;
            value(&mut out, "chi", "matrix", f.chi_square)?;
            for i in 0..2 {
                for (name, x) in [("p", &f.increments), ("e", &f.errors)] {
                    if let Some(v) = x.get(i) {
                        value(&mut out, &format!("{name}/{i}"), "matrix", v.clone())?;
                    }
                }
            }
            for (i, row) in f.covariance.into_iter().enumerate() {
                for (j, v) in row.into_iter().enumerate() {
                    value(&mut out, &format!("cov/{i}/{j}"), "matrix", v)?;
                }
            }
        }
        "calceph" => {
            let mut c = PulsarContext::unwritten(2);
            c.initialise(2, 4, 1)?;
            for p in 0..2 {
                c.pulsar_observations_mut(p)?.0.nobs = Some(p as i32 + 2);
                let (s, rows) = c.pulsar_observations_mut(p)?;
                s.units = Some(args[4].parse()?);
                s.eclCoord = Some(args[5].parse()?);
                for (i, v) in args[3].bytes().chain([0]).enumerate() {
                    s.JPL_EPHEMERIS[i] = Some(v);
                }
                use tempo2_rust::tempo2::Label::*;
                for (label, k, text) in [
                    (param_dphaseplanet, 4, b"0.0\n".as_slice()),
                    (param_dmassplanet, 5, b"0.00000095367431640625\n".as_slice()),
                ] {
                    let mut source = tempo2_rust::source_text::SourceText::new(
                        Path::new("direct"),
                        text.to_vec(),
                    );
                    tempo2_rust::source_par::read_value(s, b"AI_DIRECT", &mut source, label, k)?;
                }

                for (i, o) in rows.iter_mut().take(p + 2).enumerate() {
                    o.sat = Some(rug::Float::with_val(
                        128,
                        58000.125 + p as f64 * 20. + i as f64 / 8.,
                    ));
                    o.nclock_correction = Some(0);
                    o.correctionTT_Teph = Some(rug::Float::with_val(128, 0));
                    for k in 0..9 {
                        for j in 0..6 {
                            o.planet_ssb[k][j] = Some((k + 1) as f64 * 0.125 + j as f64);
                            o.planet_ssb_tmr[k][j] = Some(o.planet_ssb[k][j].unwrap() + 0.25);
                        }
                    }
                }
            }
            for pass in 0..2 {
                tempo2_rust::calceph::tt2tb_calceph(&mut c)?;
                tempo2_rust::calceph::read_ephemeris_calceph(&mut c)?;
                snapshot(&mut out, &mut c, &format!("direct{pass}"), false)?;
            }
        }
        "pipeline" => {
            let runtime = std::env::var("TEMPO2")?;
            let pairs = args[5..]
                .chunks_exact(2)
                .map(|p| tempo2_rust::bats::InputPair {
                    par: p[0].clone().into(),
                    tim: p[1].clone().into(),
                })
                .collect::<Vec<_>>();
            let mut input = tempo2_rust::bats::load_pairs(&pairs, Path::new(&runtime))?;
            tempo2_rust::preprocess::pre_process(
                &mut input.context,
                &[b"probe".to_vec()],
                Path::new(&runtime),
                &mut input.inputs,
            )?;
            let mut r = tempo2_rust::RuntimeContext::from_root(Path::new(&runtime))?;
            tempo2_rust::timing::evaluate(&mut input, &mut r, true)?;
            snapshot(&mut out, &mut input.context, "pre", true)?;
            let options = tempo2_rust::general_fitting::FitOptions {
                acceleration: args[3].parse()?,
                covariance: (args[4] != "-").then(|| args[4].clone()),
                ..Default::default()
            };
            let systems = tempo2_rust::general_fitting::t2_fit(&mut input.context, &options)?;
            for (p, s) in input.context.pulsars().iter().enumerate() {
                let k = format!("fit/{p}");
                value(&mut out, &format!("{k}/chi"), "chi", s.fitChisq.unwrap())?;
                value(
                    &mut out,
                    &format!("{k}/dof"),
                    "exact",
                    f64::from(s.fitNfree.unwrap()),
                )?;
                let f = &s.fitinfo.output;
                let nf = f.totalNfit.unwrap() as usize;
                value(&mut out, &format!("{k}/n"), "exact", nf as f64)?;
                let cov = input.context.matrix(s.covar.flatten().unwrap())?;
                for j in 0..nf {
                    let system = systems
                        .iter()
                        .find(|sys| sys.whitening.iter().any(|w| w.pulsar == p))
                        .ok_or("missing system")?;
                    let scale = system
                        .matrix
                        .iter()
                        .map(|r| r[j].as_float().to_f64().abs())
                        .fold(0.0, f64::max);
                    value(&mut out, &format!("{k}/scale/{j}"), "matrix", scale)?;
                    value(
                        &mut out,
                        &format!("{k}/p/{j}"),
                        "fit",
                        f.parameterEstimates[j].unwrap(),
                    )?;
                    value(
                        &mut out,
                        &format!("{k}/e/{j}"),
                        "error",
                        f.errorEstimates[j].unwrap(),
                    )?;
                    for l in 0..nf {
                        value(&mut out, &format!("{k}/c/{j}/{l}"), "cov", cov.row(j)?[l])?;
                    }
                }
            }
            std::fs::write(
                format!("{}.solver.json", args[2]),
                serde_json::to_vec_pretty(&systems)?,
            )?;
            tempo2_rust::timing::evaluate(&mut input, &mut r, true)?;
            snapshot(&mut out, &mut input.context, "post", true)?;
        }
        _ => return Err("unknown probe mode".into()),
    }
    Ok(())
}
