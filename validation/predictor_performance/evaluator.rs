//! AO 42 direct public Rust leaves. No alternate polynomial implementation.
use std::{hint::black_box, path::Path, time::Instant};
use tempo2_rust::{predictor_number::PredictorNumber as N, tempo2pred::T2Predictor};

fn value(name: &str, i: usize, x: N) {
    println!("{name} {i} {x} negative={} nan={}",
        x.as_float().is_sign_negative(), x.as_float().is_nan());
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    let predictor = T2Predictor::read(Path::new(&args[1])).unwrap().predictor;
    let T2Predictor::Cheby(set) = predictor else { panic!("Cheby input required") };
    let m = &set.segments[0];
    let iterations: usize = args[3].parse().unwrap();
    assert!(iterations > 0);
    // Dimensionless off-grid leaf inputs; model inputs use the same seven
    // epoch expressions and 1400 MHz as the installed C workload.
    let x: Vec<N> = (0..7).map(|i| N::from(-0.75)+N::from(i as f64)*0.125).collect();
    let epoch: Vec<N> = (0..7).map(|i| "58000.01".parse::<N>().unwrap()
        + N::from(i as f64)*"0.000001".parse::<N>().unwrap()).collect();
    let y = N::from(0.25);
    let frequency = N::from(1400.);
    for i in 0..7 {
        value("phase-polynomial-turn", i, m.cheby.evaluate(&x[i], &y));
        value("phase-derivative-dx", i, m.frequency_cheby.evaluate(&x[i], &y));
        value("model-phase-turn", i, m.phase(&epoch[i], &frequency));
        value("model-frequency-hz", i, m.frequency(&epoch[i], &frequency));
    }
    let started = Instant::now();
    match args[2].as_str() {
        "from" => for _ in 0..iterations { black_box(N::from(black_box(0.125))); },
        "owned" => for _ in 0..iterations { black_box(black_box(x[0].clone()) * black_box(&y)); },
        "leaf" => for i in 0..iterations {
            black_box(m.cheby.evaluate(black_box(&x[i%7]), black_box(&y)));
            black_box(m.frequency_cheby.evaluate(black_box(&x[i%7]), black_box(&y)));
        },
        "model" => for i in 0..iterations {
            black_box(m.phase(black_box(&epoch[i%7]), black_box(&frequency)));
            black_box(m.frequency(black_box(&epoch[i%7]), black_box(&frequency)));
        },
        mode => panic!("unknown observation scope {mode}"),
    }
    println!("TIME {} {} {:.9}", args[2], iterations, started.elapsed().as_secs_f64());
}
