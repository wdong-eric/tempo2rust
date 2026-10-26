//! Direct AF component consumer: no timing runtime or native engine.
use tempo2_rust::{cheby2d::Cheby2D, predictor_number::PredictorNumber as N};
fn n(x: f64) -> N {
    x.into()
}
fn callback(x: &[N], y: &[N], kind: &str) -> Result<Vec<N>, String> {
    Ok(y.iter()
        .flat_map(|y| {
            x.iter().map(move |x| match kind {
                "constant" => n(3.),
                "linear" => n(3.) + x * 2. - y * 0.5,
                _ => n(3.) + x * 2. - y * 0.5 + x * 0.25 * y + x * 0.125 * x,
            })
        })
        .collect())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kind = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "polynomial".into());
    let c = Cheby2D::construct(8, 6, |x, y| callback(x, y, &kind))?;
    let d = c.x_derivative()?;
    let mut phase = vec![];
    let mut frequency = vec![];
    for i in 0..13 {
        let x = n(-1.) + n(2.) * i as f64 / 12.;
        let y = "0.73".parse::<N>()? - n(0.1) * i as f64;
        phase.push(c.evaluate(&x, &y));
        frequency.push(d.evaluate(&x, &y));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"coeff":c.coefficients(),"derivative":d.coefficients(),"phase":phase,"frequency":frequency,"test":c.test(17,11,|x,y|callback(x,y,&kind))?})
        )?
    );
    Ok(())
}
