//! Installed public Float borrows remain live across canonical evaluation.
use rug::Float;
use tempo2_rust::{
    cheby2d::Cheby2D,
    predictor_number::{NATIVE_BITS, PredictorNumber as N},
};

fn main() -> Result<(), String> {
    // Half-weighted constant axes give 3 + 4*x - y + x*y.
    let polynomial =
        Cheby2D::from_coefficients(2, 2, [12., 8., -2., 1.].into_iter().map(N::from).collect())?;
    let (x, y) = (N::from(0.25), N::from(-0.5));
    let input: &Float = x.as_float();
    let coefficient: &Float = polynomial.coefficients()[0].as_float();
    let result = polynomial.evaluate(&x, &y);
    let result_view: &Float = result.as_float();
    let derivative = polynomial.x_derivative()?.evaluate(&x, &y);

    // A later evaluation must not replace borrowed owners or their contents.
    let zero = N::from(-0.);
    let zero_view: &Float = zero.as_float();
    let at_zero = polynomial.evaluate(&zero, &zero);
    assert!(std::ptr::eq(input, x.as_float()));
    assert!(std::ptr::eq(
        coefficient,
        polynomial.coefficients()[0].as_float()
    ));
    assert!(std::ptr::eq(result_view, result.as_float()));
    assert_eq!(input, &0.25);
    assert_eq!(coefficient, &12.);
    assert_eq!(result_view, &4.375);
    assert_eq!(derivative.as_float(), &3.5);
    assert_eq!(at_zero.as_float(), &3.);
    assert!(zero_view.is_zero() && zero_view.is_sign_negative());
    assert_eq!(result_view.prec(), NATIVE_BITS);
    assert_eq!(N::from_float(result_view), result);
    println!(
        "precision={NATIVE_BITS} input={input} coefficient={coefficient} result={result_view} derivative={derivative} at_zero={at_zero} negative_zero={}",
        zero_view.is_sign_negative()
    );
    Ok(())
}
