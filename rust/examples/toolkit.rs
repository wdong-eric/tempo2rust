//! Public toolkit/statistics/associated sorting and reproducible MT/Gaussian stream.
mod analysis_support;
use analysis_support::*;
use tempo2_rust::toolkit as tk;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args()?;
    let mut seed = value(&args, "--seed", -12345i64)?;
    let draws = value(&args, "--draws", 1300usize)?;
    let x = [2.0, -3.0, 0.0, -0.0, 5.25, 2.0, 1e-5];
    let e = [1.0, 2.0, 0.5, 3.0, 0.75, 1.25, 2.5];
    let f = tk::convert_float1(&x);
    for (name, v) in [
        ("min", tk::find_min_d(&x)?),
        ("max", tk::find_max_d(&x)?),
        ("median", tk::find_median_d(&x)?),
        ("mean", tk::mean_d(&x)),
        ("rms", tk::find_rms_d(&x)),
        ("variance", tk::variance_d(&x)),
        ("range", tk::range_d(&x)?),
    ] {
        num(
            &format!("stat.{name}"),
            if matches!(name, "min" | "max") {
                "f64_exact"
            } else {
                "stat"
            },
            v,
        );
    }
    for (name, v) in [
        ("min", tk::find_min_f(&f)?),
        ("max", tk::find_max_f(&f)?),
        ("median", tk::find_median_f(&f)?),
        ("mean", tk::mean_f(&f)),
        ("rms", tk::find_rms_f(&f)),
        ("range", tk::range_f(&f)?),
        ("weighted_rms", tk::find_rms_weight_d(&x, &e)?),
    ] {
        num(
            &format!("float.{name}"),
            if matches!(name, "min" | "max") {
                "f32_exact"
            } else {
                "f32"
            },
            v as f64,
        );
    }
    let (mut a, mut b, mut c) = (x, e, x);
    tk::sort_3d(&mut a, &mut b, &mut c)?;
    for (name, v) in [("sort.x", a), ("sort.y", b), ("sort.z", c)] {
        for (i, v) in v.iter().enumerate() {
            emit(&format!("{name}.{i}"), "bits", v.to_bits());
        }
    }
    let (mut a, mut b) = tk::convert_float2(&x, &e)?;
    tk::sort_2f(&mut a, &mut b)?;
    for (name, v) in [("sort2.x", a), ("sort2.y", b)] {
        for (i, v) in v.iter().enumerate() {
            emit(&format!("{name}.{i}"), "bits", v.to_bits());
        }
    }
    let mut a = x;
    tk::zero_mean_d(&mut a);
    vector("zero_mean", "stat", &a);
    for (a, b) in [(1.0, -0.0), (-2.0, 1.0), (3.0, -1.0), (1.0, f64::NAN)] {
        let prefix = format!("ret.{}.{}", a, b);
        for (name, v) in [
            ("sign", tk::sign_d(a, b)),
            ("max", tk::ret_max_d(a, b)),
            ("min", tk::ret_min_d(a, b)),
        ] {
            num(&format!("{prefix}.{name}"), "class", v);
        }
    }
    emit("seed.clock", "exact", tk::set_seed_at(1793000000)?);
    emit("capacity", "exact", tk::max_observations()?);
    let mut rng = tk::ToolkitRng::default();
    for i in 0..draws {
        emit(&format!("default.{i}"), "exact", rng.genrand_int32());
    }
    for i in 0..draws {
        if i == 621 {
            seed = -17;
        }
        if i == 1000 {
            seed = -12345;
        }
        let v = if i % 3 == 0 {
            rng.gauss_dev(&mut seed)?
        } else {
            rng.ran_dev(&mut seed)?
        };
        num(
            &format!("rng.{i}"),
            if i % 3 == 0 { "gaussian" } else { "uniform" },
            v,
        );
        emit(&format!("rng.{i}.seed"), "exact", seed);
        emit(&format!("rng.{i}.index"), "exact", rng.state().1);
        emit(&format!("rng.{i}.draws"), "exact", rng.state().2);
    }
    for (i, w) in rng.state().0.iter().enumerate() {
        emit(&format!("state.{i}"), "exact", w);
    }
    Ok(())
}
