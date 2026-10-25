//! Read-only reuse of AB's complete direct-helper result and output-array cases.
use crate::{ifunc, shapelet, solar_wind as sw};
use serde_json::{Value, json};
fn scalar(x: f64) -> Value {
    json!(if x.is_nan() {
        "NaN".into()
    } else if x.is_infinite() {
        if x > 0.0 {
            "+Inf".into()
        } else {
            "-Inf".into()
        }
    } else {
        x.to_le_bytes()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    })
}
pub(crate) fn evaluate(a: &[&str]) -> Result<Vec<Value>, crate::TimingError> {
    let n = |i: usize| a[i].parse::<f64>().unwrap();
    let k = |i: usize| a[i].parse::<i32>().unwrap();
    let answer: Vec<Value> = match a[0] {
        "ifunc" | "basis" | "sinc" => {
            let t: Vec<_> = (0..6).map(|z| n(4 + z * 2)).collect();
            let v: Vec<_> = (0..6).map(|z| n(5 + z * 2)).collect();
            let x = match a[0] {
                "ifunc" => ifunc::ifunc(&t, &v, n(1), k(2)),
                "basis" => ifunc::ifunc_basis(&t, n(1), k(2), k(3)),
                _ => ifunc::sinfunc(&t, n(1), k(3) as usize),
            };
            vec![scalar(x?)]
        }
        "factorial" => vec![scalar(shapelet::iter_factorial(k(1) as u32))],
        "hermite" => {
            let mut v: Vec<f64> = (91..103).map(f64::from).collect();
            shapelet::tnothpl(k(1), n(2), &mut v)?;
            v.into_iter().map(scalar).collect()
        }
        "recurrence" => {
            let (mut y0, mut y1) = (n(3), n(4));
            let ans = shapelet::iterative_tnothpl(k(1), n(2), &mut y0, &mut y1);
            vec![scalar(ans), scalar(y0), scalar(y1)]
        }
        "shapelet" => {
            let v: Vec<_> = (0..12).map(|z| n(5 + z)).collect();
            vec![scalar(shapelet::evaluate_shapelet(
                k(1),
                n(2),
                n(3),
                &v,
                n(4),
            )?)]
        }
        "angle" => vec![scalar(sw::find_angle(n(1), n(2), n(3), n(4)))],
        "amod" => vec![scalar(sw::amod(n(1), n(2))?)],
        "ecliptic" => {
            let (x, y) = sw::convert_ecliptic(n(1), n(2));
            vec![scalar(x), scalar(y)]
        }
        "elsun" => {
            let (x, y, z, w) = sw::elsun2(k(1), k(2), n(3))?;
            vec![scalar(x), scalar(y), scalar(z), scalar(w)]
        }
        "date" => {
            let mut out = sw::SolarDate {
                iyr: 81,
                year: 82,
                month: 83,
                day: 84,
                day_of_year: Some(85),
            };
            sw::mjd2date(k(1), &mut out)?;
            vec![
                json!(out.iyr),
                json!(out.year),
                json!(out.month),
                json!(out.day),
                json!(out.day_of_year.unwrap()),
            ]
        }
        "rotation" => {
            let (x, y, b) = sw::calc_rot_n(n(1), n(2))?;
            vec![json!(x), json!(y), scalar(b)]
        }
        "mcl" => {
            let mut out = sw::MclOutput {
                helat: [81.0; 36],
                crlon: [82.0; 36],
                rots: [83.0; 36],
                helate: 84.0,
                crlne: 84.0,
                rote: 84.0,
                elong: 84.0,
                beta: 84.0,
                dlon: 84.0,
                delng: 84.0,
                line_of_sight: -1,
            };
            sw::mcl2(n(1), n(2), k(3), k(4), n(5), n(6), &mut out)?;
            let mut v = vec![json!(out.line_of_sight)];
            v.extend(
                out.helat
                    .into_iter()
                    .chain(out.crlon)
                    .chain(out.rots)
                    .chain([
                        out.helate, out.crlne, out.rote, out.elong, out.beta, out.dlon, out.delng,
                    ])
                    .map(scalar),
            );
            v
        }
        "sheet" => {
            let mut lon = [81.0; 500];
            let mut lat = [82.0; 500];
            let p = std::path::Path::new(a[1]);
            let count = sw::read_current_sheet(p, &mut lon, &mut lat, &mut vec![])?;
            let mut v = vec![json!(count)];
            for (x, y) in lon.into_iter().zip(lat) {
                for x in [x, y] {
                    v.push(json!(if x.is_nan() {
                        "NaN".into()
                    } else if x.is_infinite() {
                        if x > 0.0 {
                            "+Inf".into()
                        } else {
                            "-Inf".into()
                        }
                    } else {
                        x.to_le_bytes()
                            .iter()
                            .map(|b| format!("{b:02x}"))
                            .collect::<String>()
                    }));
                }
            }
            v
        }
        "output" => {
            let lon = std::array::from_fn(|i| (i as f64).mul_add(n(2), n(1)));
            let lat = std::array::from_fn(|i| (i as f64).mul_add(n(4), n(3)));
            sw::output_results(&lon, &lat)?;
            vec![]
        }
        _ => panic!("unknown selected helper"),
    };
    Ok(answer)
}
#[test]
#[ignore = "validation/input_to_bat/check_helpers.py supplies AB evidence"]
fn ab_direct_helpers() {
    let path = std::env::var_os("AC_HELPER_CASES").unwrap();
    let path = std::path::Path::new(&path);
    let cases: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let mut failures = vec![];
    for case in cases.as_array().unwrap() {
        for action in case["actions"].as_array().unwrap() {
            let a: Vec<&str> = action["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let answer = evaluate(&a).unwrap();
            if answer != *action["expected"].as_array().unwrap() {
                let differences: Vec<_> = answer
                    .iter()
                    .zip(action["expected"].as_array().unwrap())
                    .enumerate()
                    .filter(|(_, (x, y))| x != y)
                    .map(|(i, (x, y))| json!({"index":i,"rust":x,"native":y}))
                    .collect();
                failures.push(format!("{} {}: {}", case["id"], a[0], json!(differences)));
            }
        }
    }
    std::fs::write(
        path.with_file_name("helper-differences.json"),
        serde_json::to_vec_pretty(&failures).unwrap(),
    )
    .unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
