//! Replay 0049's identical native-parsed inputs through the current public API.
use rug::Float;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for line in std::fs::read_to_string(std::env::args().nth(1).expect("input TSV"))?.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let a = Float::with_val(128, Float::parse(fields[2])?);
        let value = if fields[1] == "nlong" {
            tempo2_rust::tempo2_util::fortran_nlong(&a)
        } else {
            let b = Float::with_val(128, Float::parse(fields[3])?);
            tempo2_rust::tempo2_util::fortran_mod(a, &b)?
        };
        if let Some((integer, exponent)) = value.to_integer_exp() {
            println!(
                "{}\tfinite\t{}\t{integer}\t{exponent}",
                fields[0],
                value.is_sign_negative()
            );
        } else {
            println!(
                "{}\t{}\t{}\t0\t0",
                fields[0],
                if value.is_nan() { "NaN" } else { "Infinity" },
                value.is_sign_negative()
            );
        }
    }
    Ok(())
}
