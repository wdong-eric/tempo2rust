#![cfg_attr(nightly_probe, feature(f128))]

fn main() {
    let epoch = std::hint::black_box(56000.78912398509389208687235_f128);
    let delay = std::hint::black_box(1e-9_f128);
    let recovered = ((epoch + delay / 86400.0) - epoch) * 86400.0;
    println!("epoch={:032x} delay={:032x} recovered={:032x} error={:032x}", epoch.to_bits(), delay.to_bits(), recovered.to_bits(), (recovered-delay).to_bits());
    assert!((recovered - delay).abs() < 1e-12_f128);
    println!("f128 arithmetic passed; recovered bits={:032x}", recovered.to_bits());
}
