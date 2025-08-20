use std::convert::TryFrom;
use std::ops::Not;
use std::ops::Sub;
use std::*;

use num_bigint::{
    // 0.2.2
    BigInt,
    BigUint,
};

use super::fraction::fraction::Fraction;

/// PI = 16 * atan(1/5) - 4 * atan(1/239)
pub fn pi(precision: u64) -> String {
    /// atan(x) = x - x^3/3 + x^5/5 - x^7/7 + x^9/9...
    fn atan(x: Fraction, precision: u64) -> Fraction {
        use num_traits::pow::Pow;
        let end: BigUint = BigUint::from(10_u32).pow(precision);
        let target = Fraction::new(1.into(), end);

        let mut current_term = x.clone();
        let mut ret = Fraction::from(0);
        let mut sign = BigInt::from(1);
        let mut n = BigUint::from(1_u32);
        let mut x_pow_n = x.clone();
        let two = BigUint::from(2_u32);
        let x_square = &x * &x;

        while current_term.abs() > target {
            ret = ret + current_term;
            // eprintln!(
            //     "atan({}) ~ {}",
            //     x,
            //     ret.decimal(precision as usize),
            // );
            n += &two;
            sign = -sign;
            x_pow_n = x_pow_n * &x_square;
            current_term = &x_pow_n * Fraction::new(sign.clone(), n.clone());
        }
        ret
    }

    let precision_usize = usize::try_from(precision).expect("Overflow");
    let pi_approx = Fraction::sub(
        Fraction::from(16)
            * atan(
                Fraction::new(1.into(), 5_u32.into()),
                precision + 2, // 16 -> 10 ^ 2
            ),
        Fraction::from(4)
            * atan(
                Fraction::new(1.into(), 239_u32.into()),
                precision + 1, // 4 -> 10 ^ 1
            ),
    );
    pi_approx.decimal(precision_usize)
}
