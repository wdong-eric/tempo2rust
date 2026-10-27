//! AL repeats the pinned Lomb contraction witness through the public Rust API.
fn emit(case: usize, kind: &str, i: usize, value: f64) {
    println!(
        "{case}\t{kind}\t{i}\t{:016x}\t{value:.17e}",
        value.to_bits()
    );
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for case in 0..2 {
        let n = if case == 0 { 16 } else { 17 };
        let x: Vec<_> = (0..n)
            .map(|i| {
                if case == 0 {
                    i as f64 * 1.25
                } else {
                    (20 * i + i % 3) as f64 / 16.0
                }
            })
            .collect();
        let y: Vec<_> = (0..n)
            .map(|i| ((i * i + 3 * i) % 11) as f64 * 1e-6)
            .collect();
        for i in 0..n {
            emit(case, "x", i, x[i]);
            emit(case, "y", i, y[i]);
        }
        let (frequencies, power, variance) = tempo2_rust::spectrum::lomb(&x, &y, 1.0, 1.0)?;
        emit(case, "count", 0, frequencies.len() as f64);
        emit(case, "variance", 0, variance);
        for i in 0..frequencies.len() {
            emit(case, "frequency", i, frequencies[i]);
            emit(case, "power", i, power[i]);
        }
    }
    Ok(())
}
