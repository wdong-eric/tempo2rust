//! Source-body and state probes through public AG APIs, with explicit shared inputs.
mod analysis_support;
use analysis_support::*;
use tempo2_rust::{
    gw_sim::*,
    pulsar_lifecycle::PulsarContext,
    sigma_z::{SigmaZ, indexx8, mat20},
    spectrum::{self, Options},
};
fn spectrum_components() -> Result<(), Box<dyn std::error::Error>> {
    let x: Vec<_> = (0..16).map(|i| i as f64 * 1.25).collect();
    let y: Vec<_> = (0..16)
        .map(|i| ((i * i + 3 * i) % 11) as f64 * 1e-6)
        .collect();
    let e: Vec<_> = (0..16).map(|i| (1.0 + (i % 3) as f64) * 1e-6).collect();
    for est in 1..=5 {
        for kind in 0..=4 {
            for prewhite in 0..=2 {
                let mut xx = x.clone();
                let mut yy = y.clone();
                let mut ee = e.clone();
                let out = spectrum::analyze(
                    &mut xx,
                    &mut yy,
                    &mut ee,
                    &Options {
                        estimator: est,
                        output_kind: kind,
                        prewhite,
                        ..Default::default()
                    },
                )?;
                let key = format!("driver.{est}.{kind}.{prewhite}");
                emit(&format!("{key}.count"), "exact", out.reported_count);
                vector(&format!("{key}.x"), "frequency_day", &out.frequency_per_day);
                vector(&format!("{key}.y"), out.units, &out.values);
                vector(&format!("{key}.re"), "coefficient_s", &out.real);
                vector(&format!("{key}.im"), "coefficient_s", &out.imaginary);
            }
        }
    }
    for transform in 0..3 {
        for est in [2, 4, 5] {
            let options = Options {
                estimator: est,
                average_time: if transform == 0 { 3 } else { 0 },
                smooth_type: if transform == 1 { 1 } else { 0 },
                smooth_width: 3,
                prewhite: if transform == 2 { 1 } else { 0 },
                ..Default::default()
            };
            let out = spectrum::analyze(&mut x.clone(), &mut y.clone(), &mut e.clone(), &options)?;
            let key = format!("transform.{transform}.{est}");
            emit(&format!("{key}.count"), "exact", out.reported_count);
            vector(&format!("{key}.x"), "frequency_day", &out.frequency_per_day);
            vector(&format!("{key}.y"), out.units, &out.values);
        }
    }
    for mode in [0, 1] {
        let out = spectrum::analyze(
            &mut x.clone(),
            &mut y.clone(),
            &mut vec![1e-6; 4096],
            &Options {
                fit_spline: 1,
                estimator: if mode == 0 { 3 } else { 4 },
                ..Default::default()
            },
        )?;
        let key = format!("spline_driver.{mode}");
        emit(&format!("{key}.count"), "exact", out.reported_count);
        vector(&format!("{key}.x"), "frequency_day", &out.frequency_per_day);
        vector(&format!("{key}.y"), out.units, &out.values);
    }
    let mut white_y = y.clone();
    let out = spectrum::analyze(
        &mut x.clone(),
        &mut white_y,
        &mut e.clone(),
        &Options {
            smooth_type: 1,
            smooth_width: 3,
            calc_white: 1,
            ..Default::default()
        },
    )?;
    extended("white.scalar", "seconds", out.scalar.as_float());
    vector("white.y", "seconds", &white_y);
    for n in [1, 2, 5] {
        let coeff = spectrum::cmonot(&x[..n], &y[..n])?;
        for (i, c) in coeff.iter().enumerate() {
            vector(&format!("cmonot.{n}.{i}"), "coefficient_s", c);
        }
        let z =
            spectrum::spline_interpolate(&x[..n], &coeff, &[-1.0, 0.0, 0.5, 1.25, 4.0, 5.0, 20.0])?;
        vector(&format!("interp.{n}"), "coefficient_s", &z);
    }
    for width in [1, 2, 3, 5] {
        let (a, b) = spectrum::boxcar(&x, &y, width)?;
        vector(&format!("box.{width}.x"), "days", &a);
        vector(&format!("box.{width}.y"), "coefficient_s", &b);
        let (a, b) = spectrum::hann(&x, &y, width)?;
        vector(&format!("hann.{width}.x"), "days", &a);
        vector(&format!("hann.{width}.y"), "coefficient_s", &b);
    }
    let (a, b, var) = spectrum::lomb(&x, &y, 2.0, 0.5)?;
    vector("lomb.x", "frequency_day", &a);
    vector("lomb.y", "dimensionless", &b);
    num("lomb.var", "s^2", var);
    for dir in [-1, 0, 1] {
        let mut re = y.clone();
        let mut im: Vec<_> = y.iter().rev().copied().collect();
        spectrum::fft(dir, &mut re, &mut im)?;
        vector(&format!("fft.{dir}.re"), "coefficient_s", &re);
        vector(&format!("fft.{dir}.im"), "coefficient_s", &im);
    }
    Ok(())
}
fn sigma_components() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = PulsarContext::unwritten(1);
    ctx.initialise_one(0, 32, 1, 1)?;
    ctx.pulsar_observations_mut(0)?.0.nobs = Some(24);
    let (_, rows) = ctx.pulsar_observations_mut(0)?;
    for (i, o) in rows[..24].iter_mut().enumerate() {
        let t = i as f64;
        o.sat = Some(ld(54000.0 + t * 30.0));
        o.residual = Some(ld(((i * i + 5 * i) % 13) as f64 * 1e-6 + t * t * t * 1e-9));
        o.toaErr = Some((1 + i % 4) as f64);
        o.deleted = Some(if i == 7 { 1 } else { 0 });
    }
    let mut sigma = SigmaZ::default();
    for (call, weights) in [-2, -1, 0, -3, 1, 2, -2].iter().enumerate() {
        let points = sigma.calculate(&mut ctx, 0, *weights, 0.03)?;
        let key = format!("sigmaz.{call}");
        emit(&format!("{key}.count"), "exact", points.len());
        for (i, p) in points.iter().enumerate() {
            for (k, v) in [
                ("tau", p.log10_tau_year),
                ("sigma", p.log10_sigma_bias),
                ("plus", p.error_plus),
                ("minus", p.error_minus),
            ] {
                num(&format!("{key}.{i}.{k}"), "log_sigma", v);
            }
        }
        num(&format!("{key}.addvar"), "seconds", sigma.addvar);
        emit(&format!("{key}.ncubics"), "exact", sigma.ncubics);
        emit(&format!("{key}.ntau"), "exact", sigma.ntau);
    }
    let a = [0.0, 3.0, 1.0, 1.0, 2.0, -0.0];
    for (i, v) in indexx8(&a)?.iter().enumerate().skip(1) {
        emit(&format!("index.{i}"), "exact", v);
    }
    let mut a = [[0.0; 21]; 21];
    a[1][1] = 2.0;
    a[1][2] = 1.0;
    a[2][1] = 1.0;
    a[2][2] = 3.0;
    let (inv, det, bad) = mat20(&a, 2)?;
    num("matrix.det", "log_sigma", det);
    emit("matrix.bad", "exact", bad);
    for (i, r) in inv.iter().enumerate().take(3).skip(1) {
        for (j, &v) in r.iter().enumerate().take(3).skip(1) {
            num(&format!("matrix.{i}.{j}"), "geometry", v);
        }
    }
    Ok(())
}
fn gw_components() -> Result<(), Box<dyn std::error::Error>> {
    let kp = setup_pulsar(&ld(0.3), &ld(-0.8));
    for (i, v) in kp.iter().enumerate() {
        extended(&format!("kp.{i}"), "geometry", v);
    }
    let mut t =
        TensorSource::from_prefix([0.5, 1.2, 3e-8, 0.7, 0.2, 1e-15, 2e-16, -3e-16, 4e-16].map(ld));
    t.setup();
    let mut g = GeneralSource::from_prefix(
        [
            0.5, 1.2, 3e-8, 0.7, 0.2, 1e-15, 2e-16, -3e-16, 4e-16, 2e-15, 1e-15, 3e-15, 2e-15,
            4e-15, 3e-15, 5e-15, 4e-15,
        ]
        .map(ld),
    );
    g.setup();
    for (name, geometry) in [
        ("tensor", t.geometry.as_ref().unwrap()),
        ("general", g.geometry.as_ref().unwrap()),
    ] {
        for i in 0..3 {
            extended(&format!("{name}.kg.{i}"), "geometry", &geometry.kg[i]);
            for j in 0..3 {
                extended(&format!("{name}.h.{i}.{j}"), "amplitude", &geometry.h[i][j]);
                extended(
                    &format!("{name}.him.{i}.{j}"),
                    "amplitude",
                    &geometry.h_im[i][j],
                );
            }
        }
    }
    for d in [0.0, 3e19] {
        extended(
            &format!("tensor.res.{d}"),
            "seconds",
            &t.residual(&kp, &ld(5e7), &ld(d))?,
        );
        extended(
            &format!("general.res.{d}"),
            "seconds",
            &g.residual(&kp, &ld(5e7), &ld(d))?,
        );
    }
    for l in 0..5 {
        for m in 0..=l {
            num(&format!("harm.{l}.{m}"), "geometry", sphharm(l, m, 0.3)?);
        }
    }
    num("angle", "radians", psrangle(0.1, 0.2, 0.3, -0.4));
    num("phi", "radians", find_phi(0.31, 0.12, 0.8));
    let mut session = GwSession {
        ngrid: 4,
        ..Default::default()
    };
    let h = [
        Harmonic {
            l: 0.0,
            m: 0.0,
            amplitude: 0.1,
        },
        Harmonic {
            l: 2.0,
            m: -1.0,
            amplitude: 1.0,
        },
    ];
    let spec = GeneralSpectrum {
        tensor_amp: 1e-15,
        st_amp: 2e-15,
        sl_amp: 3e-15,
        vl_amp: 4e-15,
        tensor_alpha: -0.5,
        st_alpha: 0.0,
        sl_alpha: 0.5,
        vl_alpha: -1.0,
    };
    let mut seed = -42;
    for loglin in [0, 1] {
        let o = PopulationOptions {
            flo: ld(1e-9),
            fhi: ld(1e-7),
            loglin,
        };
        let sources = session.general_anisotropic_background(
            [2, 3, 0, 17],
            &mut seed,
            &o,
            spec,
            &std::array::from_fn(|_| h.to_vec()),
        )?;
        for (i, g) in sources.iter().enumerate() {
            for (j, v) in g.prefix().iter().enumerate() {
                extended(
                    &format!("population.{loglin}.{i}.{j}"),
                    if j == 2 {
                        "angular_frequency"
                    } else if j < 5 {
                        "radians"
                    } else {
                        "amplitude"
                    },
                    v,
                );
            }
        }
        emit(
            &format!("population.{loglin}.draws"),
            "exact",
            session.rng.state().2,
        );
    }
    emit("population.warnings", "exact", session.warnings.len());
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    match required(&a, "--case")? {
        "spectrum" => spectrum_components()?,
        "sigma" => sigma_components()?,
        "gw" => gw_components()?,
        _ => return Err("unknown case".into()),
    }
    Ok(())
}
