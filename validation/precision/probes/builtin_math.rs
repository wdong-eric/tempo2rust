#![feature(f128)]
fn main() {
    let zero = std::hint::black_box(0.0_f128);
    let one = std::hint::black_box(1.0_f128);
    assert_eq!(zero.sin().to_bits(), zero.to_bits());
    assert_eq!(zero.cos().to_bits(), one.to_bits());
    assert_eq!((one*4.0).sqrt().to_bits(), 2.0_f128.to_bits());
    assert_eq!(one.ln().to_bits(), zero.to_bits());
    assert_eq!(one.atan2(one).to_bits(), 0.7853981633974483096156608458198757_f128.to_bits());
    println!("f128 sin/cos/sqrt/ln/atan2 smoke passed (not an accuracy survey)");
}
