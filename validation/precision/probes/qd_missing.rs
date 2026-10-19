use qd::Quad;
fn main() {
    let x: Quad = "56000.78912398509389208687235".parse().unwrap();
    println!("{x:.36}");
    std::hint::black_box((x.sin(), x.cos(), x.atan2(Quad::ONE), x.floor(), x.round()));
}
