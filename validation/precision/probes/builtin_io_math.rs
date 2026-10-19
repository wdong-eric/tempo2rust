#![feature(f128)]
fn main() {
    let x: f128 = "56000.78912398509389208687235".parse().unwrap();
    println!("{x:.36}");
    std::hint::black_box((x.sin(), x.cos(), x.atan2(1.0), x.sqrt(), x.ln()));
}
