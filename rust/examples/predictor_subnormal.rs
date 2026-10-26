use rug::Float;
use tempo2_rust::predictor_number::PredictorNumber as N;
fn main() {
    let x = (Float::with_val(128, 1) >> 1075) + (Float::with_val(128, 1) >> 1180);
    let got = N::from_float(&x).to_f64();
    println!(
        "conversion native={:016x} rust={:016x}",
        x.to_f64().to_bits(),
        got.to_bits()
    );
    let a = f64::from_bits(0x0010000000000001);
    let b = f64::from_bits(0x3c9fffffffffffff);
    let product = (N::from(a) * N::from(b)).to_f64();
    println!(
        "multiply native={:016x} rust={:016x}",
        (a * b).to_bits(),
        product.to_bits()
    );
    for (name, x) in [
        ("normal", 1.5),
        ("min_subnormal", f64::from_bits(1)),
        ("min_normal", f64::MIN_POSITIVE),
        ("neg_zero", -0.0),
    ] {
        println!(
            "neighbor {} native={:016x} rust={:016x}",
            name,
            x.to_bits(),
            N::from(x).to_f64().to_bits()
        );
    }
}
