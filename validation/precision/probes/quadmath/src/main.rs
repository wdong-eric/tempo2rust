fn main() {
    let x = f128::f128::parse("56000.78912398509389208687235").unwrap();
    let day = f128::f128::parse("86400").unwrap();
    let tolerance = f128::f128::parse("1e-12").unwrap();
    let formatted = format!("{x:.36}");
    assert_eq!(f128::f128::parse(&formatted).unwrap(), x);
    println!("case\tvalue");
    println!("epoch_36_digits\t{formatted}");
    println!("epoch_default_format\t{x}");
    for input in ["1e-9", "-1e-9", "4e-9", "-4e-9"] {
        let delta = f128::f128::parse(input).unwrap();
        let recovered = ((std::hint::black_box(x) + delta/day)-x)*day;
        let error = recovered-delta;
        assert!(error < tolerance && error > -tolerance);
        println!("recovered_{input}_s\t{recovered:.36}");
    }
    // The crate checks interior NUL, but strtoflt128 gets no end pointer.
    for input in ["bad", "1junk", "1D-3", "NaN", "inf", "1\0junk"] {
        match f128::f128::parse(input) {
            Ok(value) => println!("parse_{input:?}\tOK:{value:.36}"),
            Err(error) => println!("parse_{input:?}\tERROR:{error}"),
        }
    }
}
