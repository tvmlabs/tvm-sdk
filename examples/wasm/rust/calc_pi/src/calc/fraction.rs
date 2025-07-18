pub mod fraction {
    #![allow(clippy::suspicious_arithmetic_impl)]
    use core::ops::{Add, Div, Mul, Neg, Sub};
    use num_bigint::*;
    use num_traits::*;
    use std::ops::Not;
    use std::*; // 0.2.8

    #[derive(Debug, Clone, PartialEq, Eq, Ord)]
    pub struct Fraction {
        pub numerator: BigInt,

        pub denominator: BigUint,
    }

    impl From<i32> for Fraction {
        #[inline]
        fn from(x: i32) -> Self {
            Self::new(x.into(), 1_u32.into())
        }
    }

    impl PartialOrd for Fraction {
        fn partial_cmp(self: &'_ Self, other: &'_ Self) -> Option<cmp::Ordering> {
            (self - other).numerator.partial_cmp(&BigInt::zero())
        }
    }

    impl Fraction {
        pub fn new(numerator: BigInt, denominator: BigUint) -> Self {
            assert!(denominator.is_zero().not(), "Division by zero");
            let mut ret = Self {
                numerator,
                denominator,
            };
            ret.simplify();
            ret
        }

        pub fn simplify(self: &'_ mut Self) {
            let (sign, abs) = self.numerator.split();
            let gcd = gcd(&abs, &self.denominator);
            self.numerator = BigInt::from_biguint(sign, abs / &gcd);
            self.denominator /= gcd;
        }

        pub fn inverse(self: &'_ Self) -> Self {
            if let Some(numerator) = self.numerator.to_biguint() {
                Fraction::new(
                    self.denominator.to_bigint().unwrap(), // why ???
                    numerator,
                )
            } else {
                Fraction::new(
                    BigInt::from_biguint(self.numerator.sign(), self.denominator.clone()),
                    self.numerator.clone().neg().to_biguint().unwrap(),
                )
            }
        }

        pub fn abs(self: &'_ Self) -> Self {
            Self {
                numerator: self.numerator.abs(),
                denominator: self.denominator.clone(),
            }
        }

        pub fn decimal(self: &'_ Self, precision: usize) -> String {
            use core::fmt::Write;
            use num_integer::Integer;
            let mut ret = String::new();
            let Self {
                numerator,
                denominator,
            } = self.clone();
            let (sign, mut numerator) = numerator.split();
            if let Sign::Minus = sign {
                ret.push('-');
            }
            let base = BigUint::from(10_u32);
            let (q, r) = numerator.div_mod_floor(&denominator);
            write!(&mut ret, "{}", q).unwrap();
            if r.is_zero() {
                return ret;
            } else {
                ret.reserve(1 + precision);
                ret.push('.');
            }
            numerator = r * &base;
            for _ in 0..precision {
                let (q, r) = numerator.div_mod_floor(&denominator);
                write!(&mut ret, "{}", q).unwrap();
                if r.is_zero() {
                    break;
                }
                numerator = r * &base;
            }
            ret
        }
    }

    macro_rules! derive_op {(
        impl $Op:ident for Fraction {
            type Output = Fraction;

            fn $op:ident (&$self:tt, &$other:tt) -> Self::Output
            $body:block
        }
    ) => (
        impl<'a> $Op for &'a Fraction {
            type Output = Fraction;

            fn $op ($self: &'a Fraction, $other: &'a Fraction) -> Self::Output
            $body
        }

        impl<'a> $Op<&'a Fraction> for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: &'a Fraction) -> Self::Output
            {
                $Op::$op(&$self, $other)
            }
        }

        impl<'a> $Op<Fraction> for &'a Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: &'a Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op($self, &$other)
            }
        }

        impl $Op for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op(&$self, &$other)
            }
        }
    )}

    derive_op! {
        impl Add for Fraction {
            type Output = Fraction;

            fn add (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs + rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Sub for Fraction {
            type Output = Fraction;

            fn sub (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs - rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Mul for Fraction {
            type Output = Fraction;

            fn mul (&self, &other) -> Self::Output
            {
                Fraction::new(
                    &self.numerator * &other.numerator,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Div for Fraction {
            type Output = Fraction;

            fn div (&self, &other) -> Self::Output
            {
                self * other.inverse()
            }
        }
    }

    impl fmt::Display for Fraction {
        fn fmt(self: &'_ Self, stream: &'_ mut fmt::Formatter<'_>) -> fmt::Result {
            write!(stream, "{} / {}", self.numerator, self.denominator,)
        }
    }

    fn gcd(a: &'_ BigUint, b: &'_ BigUint) -> BigUint {
        let mut a = a.clone();
        let mut b = b.clone();
        while b.is_zero().not() {
            let r = a % &b;
            a = b;
            b = r;
        }
        a
    }

    trait SignSplit {
        fn split(self: &'_ Self) -> (Sign, BigUint);
    }
    impl SignSplit for BigInt {
        fn split(self: &'_ BigInt) -> (Sign, BigUint) {
            fn to_biguint_lossy(this: &'_ BigInt) -> BigUint {
                this.to_biguint()
                    .unwrap_or_else(|| this.neg().to_biguint().unwrap())
            }
            (self.sign(), to_biguint_lossy(self))
        }
    }
}
