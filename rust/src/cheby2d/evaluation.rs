//! Private arithmetic for cheby2d.c's single Clenshaw recurrence (0131).
//! Public predictor owners, model arithmetic and timing precision stay separate.

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod platform {
    use crate::predictor_number::N;
    use std::ops::{Add, Neg};

    pub type Input<'a> = f64;

    #[derive(Clone, Copy)]
    pub struct Number(f64);

    impl From<f64> for Number {
        fn from(value: f64) -> Self {
            // N's fresh add/multiply and source FMA results have positive NaN
            // sign. Rust leaves arithmetic NaN signs unspecified. Unary minus
            // below must still flip the sign, including zeros and NaNs.
            Self(if value.is_nan() { value.abs() } else { value })
        }
    }
    impl Number {
        pub fn source_mul_add_owned(self, b: &Self, c: &Self) -> Self {
            // Same single binary64 rounding as N's macOS source FMA; avoid
            // the three MPFR get_d calls and set_d for recurrence temporaries.
            Self::from(self.0.mul_add(b.0, c.0))
        }
    }
    impl Add for Number {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self::from(self.0 + rhs.0)
        }
    }
    impl Add<&N> for Number {
        type Output = Self;
        fn add(self, rhs: &N) -> Self {
            Self::from(self.0 + rhs.to_f64())
        }
    }
    impl Neg for Number {
        type Output = Self;
        fn neg(self) -> Self {
            Self(-self.0)
        }
    }
    pub fn input(value: &N) -> Input<'_> {
        // Validated native 53-bit values are exactly representable, including
        // subnormals and signed zero. NaN conversion matches N::to_f64.
        value.to_f64()
    }
    pub fn twice(value: Input<'_>) -> Number {
        // This multiply remains separately rounded before the source FMA.
        Number::from(value * 2.)
    }
    pub fn input_mul_add(a: Input<'_>, b: &Number, c: &Number) -> Number {
        Number::from(a.mul_add(b.0, c.0))
    }
    pub fn output(value: Number) -> N {
        // The final source FMA has already restored N's positive-NaN result.
        N::from(value.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[track_caller]
        fn same(actual: Number, expected: &N) {
            assert_eq!(actual.0.is_nan(), expected.as_float().is_nan());
            assert_eq!(
                actual.0.is_sign_negative(),
                expected.as_float().is_sign_negative()
            );
            if !actual.0.is_nan() {
                assert_eq!(actual.0.to_bits(), expected.to_f64().to_bits());
            }
        }

        #[test]
        fn predictor_recurrence_primitives_match_ordinary() {
            // Boundary neighbors plus reproducible bit patterns, using
            // the unchanged public N arithmetic as the independent oracle.
            let mut values = Vec::new();
            for bits in [
                0,
                1,
                2,
                0x000f_ffff_ffff_fffe,
                0x000f_ffff_ffff_ffff,
                0x0010_0000_0000_0000,
                0x0010_0000_0000_0001,
                0x3c90_0000_0000_0000,
                0x3fe0_0000_0000_0000,
                0x3fef_ffff_ffff_ffff,
                0x3ff0_0000_0000_0000,
                0x3ff0_0000_0000_0001,
                0x3ff0_0000_0000_0002,
                0x7fef_ffff_ffff_fffe,
                0x7fef_ffff_ffff_ffff,
                0x7ff0_0000_0000_0000,
            ] {
                let x = f64::from_bits(bits);
                values.extend([
                    (Number::from(x), N::from(x)),
                    (-Number::from(x), -N::from(x)),
                ]);
            }
            values.extend([
                (Number::from(f64::NAN), N::from(f64::NAN)),
                (-Number::from(f64::NAN), -N::from(f64::NAN)),
            ]);
            let mut bits = 0xa076_1d64_78bd_642fu64;
            for _ in 0..64 {
                bits ^= bits << 13;
                bits ^= bits >> 7;
                bits ^= bits << 17;
                let x = f64::from_bits(bits);
                values.push((Number::from(x), N::from(x)));
            }
            for (a, na) in &values {
                same(twice(input(na)), &(na * 2.));
                assert_eq!(
                    (-*a).0.is_sign_negative(),
                    !na.as_float().is_sign_negative()
                );
                if !a.0.is_nan() {
                    same(-*a, &(-na.clone()));
                }
                for (b, nb) in &values {
                    let expected = na + nb;
                    same(*a + *b, &expected);
                    same(*a + nb, &expected);
                    for (c, nc) in [
                        (Number::from(0.), N::from(0.)),
                        (Number::from(-0.), N::from(-0.)),
                        (Number::from(1.), N::from(1.)),
                        (Number::from(-1.), N::from(-1.)),
                        (Number::from(f64::INFINITY), N::from(f64::INFINITY)),
                        (Number::from(f64::NAN), N::from(f64::NAN)),
                        (-Number::from(f64::NAN), -N::from(f64::NAN)),
                    ] {
                        let expected = na.source_mul_add(nb, &nc);
                        same(a.source_mul_add_owned(b, &c), &expected);
                        same(input_mul_add(input(na), b, &c), &expected);
                    }
                }
            }
        }

        #[test]
        fn predictor_recurrence_keeps_rounding_and_contraction_sites() {
            let a = Number::from(f64::from_bits(0x0010_0000_0000_0001));
            let b = Number::from(f64::from_bits(0x3c9f_ffff_ffff_ffff));
            // Exact product lies above half the least subnormal. Hardware FMA
            // must agree with the established first-direction MPFR boundary.
            assert_eq!(a.source_mul_add_owned(&b, &Number::from(0.)).0.to_bits(), 1);
            assert_eq!(
                (-a).source_mul_add_owned(&b, &Number::from(-0.))
                    .0
                    .to_bits(),
                (1u64 << 63) | 1
            );
            let a = Number::from(1. + f64::EPSILON);
            let b = Number::from(1. - f64::EPSILON);
            assert_eq!(
                a.source_mul_add_owned(&b, &Number::from(-1.)).0,
                -f64::EPSILON * f64::EPSILON
            );
            let max = N::from(f64::MAX);
            // A source x*2 overflow must survive the subsequent FMA; fusing
            // that multiply too would incorrectly return the finite maximum.
            assert!(
                twice(input(&max))
                    .source_mul_add_owned(&Number::from(1.), &Number::from(-f64::MAX))
                    .0
                    .is_infinite()
            );
            assert_eq!(
                Number::from(f64::MAX)
                    .source_mul_add_owned(&Number::from(2.), &Number::from(-f64::MAX))
                    .0,
                f64::MAX
            );
        }
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod platform {
    use crate::predictor_number::N;

    pub type Input<'a> = &'a N;
    pub type Number = N;

    pub fn input(value: &N) -> Input<'_> {
        value
    }
    pub fn twice(value: Input<'_>) -> Number {
        value * 2.
    }
    pub fn input_mul_add(a: Input<'_>, b: &Number, c: &Number) -> Number {
        a.source_mul_add(b, c)
    }
    pub fn output(value: Number) -> N {
        value
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn predictor_recurrence_linux_keeps_wide_inputs() {
            for text in ["0x1.0000000000000002p0", "0x1p-16445", "0x1p16382"] {
                let x = N::parse_number(text).unwrap();
                assert_eq!(output(twice(input(&x))), &x * 2.);
                assert_eq!(
                    output(input_mul_add(input(&x), &N::from(1.), &N::from(0.))),
                    x
                );
            }
        }
    }
}

pub(super) use platform::{Number, input, input_mul_add, output, twice};
