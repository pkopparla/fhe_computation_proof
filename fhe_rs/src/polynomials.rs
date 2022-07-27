use sunscreen::types::{bfv::Fractional, Cipher};

use crate::{FILL_VALUE, INT_BITS, POLY_SIZE};
use std::ops::{Add, Mul};

pub fn poly_evaluate<T, U>(powers: &[T], coefs: &[U]) -> T
where
    T: Mul<U, Output = T> + Add<Output = T> + Copy,
    U: Copy,
{
    let mut sum = powers[0] * coefs[0];
    for count in 1..powers.len() {
        sum = sum + powers[count] * coefs[count];
    }
    sum
}

pub fn poly_multiply<T, const N: usize>(first: [T; N], second: [T; N]) -> [T; N]
where
    T: Mul<Output = T> + Add<Output = T> + Copy + From<f64> + std::cmp::PartialEq,
{
    let mut out_poly: [T; N] = [0_f64.into(); N];
    let mut first_size = N;
    for i in (0..N).rev() {
        if first[i] != 0_f64.into() {
            first_size = i + 1;
            break;
        }
    }
    let mut second_size = N;
    for i in (0..N).rev() {
        if second[i] != 0_f64.into() {
            second_size = i + 1;
            break;
        }
    }
    if first_size + second_size > POLY_SIZE {
        panic!(
            "Polynomial multiplication cannot fit within program POLY_SIZE {:}",
            POLY_SIZE
        );
    }
    for i in 0..first_size {
        for j in 0..second_size {
            out_poly[i + j] = out_poly[i + j] + first[i] * second[j];
        }
    }
    out_poly
}

#[cfg(test)]
mod tests {
    use crate::{
        polynomials::{poly_evaluate, poly_multiply},
        POLY_SIZE,
    };
    #[test]
    fn test_polynomial_evaluate() {
        let powers = [1.0, 2.0, 4.0];
        let coefs = [1.0, 2.0, 3.0];
        let result = poly_evaluate(&powers, &coefs);
        assert_eq!(result, 17_f64);
    }

    #[test]
    fn test_poly_multiply() {
        let mut first = [0_f64; POLY_SIZE];
        first[1] = 1.into();
        let mut second = [0_f64; POLY_SIZE];
        second[2] = 2.5.into();
        let result: [f64; POLY_SIZE] = poly_multiply(first, second);
        let mut expected_result = [0_f64; POLY_SIZE];
        expected_result[3] = 2.5.into();
        assert_eq!(result, expected_result);
    }
}
