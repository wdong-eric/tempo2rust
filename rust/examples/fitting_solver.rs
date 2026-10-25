//! Identical-input generalized solver, using binary64 input text shared with C++.
use tempo2_rust::tkfit::{FitDiagnostics, FitOutputs, least_squares_outputs};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let path = args.first().ok_or("expected input file")?;
    let mask: u8 = args.get(1).map(|v| v.parse()).transpose()?.unwrap_or(7);
    let has_data = args.get(2).is_none_or(|v| v != "0");
    let robust = args.get(3).map_or(0, |v| v.as_bytes()[0]);
    let io = args
        .get(4)
        .map(|path| -> Result<_, Box<dyn std::error::Error>> {
            Ok(FitDiagnostics {
                directory: path.into(),
                write_residuals: args.get(5).ok_or("expected writeResiduals")?.parse()?,
                debug: args.get(6).is_some_and(|v| v == "1"),
            })
        })
        .transpose()?;
    let text = std::fs::read_to_string(path)?;
    let mut tokens = text.split_whitespace();
    let mut number = || -> Result<f64, Box<dyn std::error::Error>> {
        Ok(tokens.next().ok_or("incomplete solver input")?.parse()?)
    };
    let n = number()? as usize;
    let nf = number()? as usize;
    let nc = number()? as usize;
    let rescale = number()? != 0.0;
    let (mut y, mut wy, mut a, mut w) = (vec![], vec![], vec![], vec![]);
    for _ in 0..n {
        y.push(number()?);
        wy.push(number()?);
        let (mut row, mut wr) = (vec![], vec![]);
        for _ in 0..nf {
            row.push(number()?);
            wr.push(number()?);
        }
        a.push(row);
        w.push(wr);
    }
    let (mut targets, mut c) = (vec![], vec![]);
    for _ in 0..nc {
        targets.push(number()?);
        c.push((0..nf).map(|_| number()).collect::<Result<Vec<_>, _>>()?);
    }
    if tokens.next().is_some() {
        return Err("extra solver input".into());
    }
    let fit = if robust > 48 {
        if mask != 7 || !has_data {
            return Err("source TKrobust requires all outputs and data".into());
        }
        tempo2_rust::tkrobust::robust_least_squares(
            &y,
            &wy,
            &a,
            &w,
            &c,
            &targets,
            1e-27,
            rescale,
            robust,
            io.as_ref(),
        )?
    } else {
        least_squares_outputs(
            has_data.then_some(y.as_slice()),
            has_data.then_some(wy.as_slice()),
            &a,
            &w,
            &c,
            &targets,
            1e-27,
            rescale,
            FitOutputs {
                parameters: mask & 1 != 0,
                errors: mask & 2 != 0,
                covariance: mask & 4 != 0,
            },
            io.as_ref(),
        )?
    };
    println!("{}", serde_json::to_string_pretty(&fit)?);
    Ok(())
}
