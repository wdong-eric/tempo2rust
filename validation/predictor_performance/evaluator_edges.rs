//! Complete canonical-leaf edge results; compare frozen before/after processes.
use rug::Float;
use tempo2_rust::{cheby2d::Cheby2D, predictor_number::{NATIVE_BITS, PredictorNumber as N}};

fn scalar(i: usize) -> N {
    let normal = if NATIVE_BITS == 53 { -1022 } else { -16382 };
    let maximum = if NATIVE_BITS == 53 { 1024 } else { 16384 };
    match i {
        0 => N::from(0.), 1 => N::from(-0.), 2 => N::from(1.), 3 => N::from(-1.),
        4 => N::from(f64::INFINITY), 5 => N::from(f64::NEG_INFINITY),
        6 => N::from(f64::NAN), 7 => -N::from(f64::NAN),
        8 => N::from_float(&(Float::with_val(128, 1) << (normal-(NATIVE_BITS as i32-1)))),
        9 => -scalar(8),
        10 => N::from_float(&(Float::with_val(128, 1) << normal)),
        11 => -scalar(10),
        12 => N::from_float(&((Float::with_val(128, 1)-(Float::with_val(128, 1)>>NATIVE_BITS)) << maximum)),
        13 => -scalar(12),
        _ => unreachable!(),
    }
}

fn main() {
    for (nx,ny) in [(1,1), (1,4), (2,1), (3,2), (8,4)] {
        for k in 0..14 {
            let mut coeff: Vec<N> = (0..nx*ny).map(|i| N::from((i+1) as f64/64.)).collect();
            coeff[k%(nx*ny)] = scalar(k);
            let p = Cheby2D::from_coefficients(nx, ny, coeff).unwrap();
            for i in 0..14 {
                for j in [0, 1, 2, 5, 7, 8] {
                    let result = p.evaluate(&scalar(i), &scalar(j));
                    println!("{nx} {ny} {k} {i} {j} {result} negative={} nan={}",
                        result.as_float().is_sign_negative(), result.as_float().is_nan());
                }
            }
        }
    }
}
