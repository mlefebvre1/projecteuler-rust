use crate::utils::timeit;
use itertools::Itertools;
use num::Integer;

fn p() -> String {
    /*
    Digit cancelling fractions
    Problem 33

    The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may
    incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

    We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

    There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two
    digits in the numerator and denominator.

    If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
    */
    fn is_non_trivial_two_digits_curious_fraction(num: usize, denom: usize) -> bool {
        let fraction = num as f64 / denom as f64;
        let num_digits = [num / 10, num % 10];
        let denom_digits = [denom / 10, denom % 10];
        if num_digits[0] == denom_digits[1] && num_digits[1] == denom_digits[0] {
            return false;
        }
        let simplified_fraction: f64 = {
            if num_digits[0] == denom_digits[1] && denom_digits[0] != 0 {
                num_digits[1] as f64 / denom_digits[0] as f64
            } else if num_digits[1] == denom_digits[0] && denom_digits[1] != 0 {
                num_digits[0] as f64 / denom_digits[1] as f64
            } else {
                0.0f64
            }
        };
        simplified_fraction == fraction
    }

    const MAX: usize = 99;
    let two_digits_fractions = (10..=MAX).combinations_with_replacement(2);
    let non_trivial_two_digits_curious_fractions = two_digits_fractions
        .filter(|fraction| {
            let (num, denom) = (fraction[0], fraction[1]);
            is_non_trivial_two_digits_curious_fraction(num, denom)
        })
        .map(|fraction| {
            let (num, denom) = (fraction[0], fraction[1]);
            (num, denom)
        });
    let fraction_product = non_trivial_two_digits_curious_fractions
        .fold((1, 1), |(acc_num, acc_denom), (num, denom)| {
            (acc_num * num, acc_denom * denom)
        });
    let (num_prod, denom_prod) = fraction_product;
    (denom_prod / denom_prod.gcd(&num_prod)).to_string()
}

timeit::timeit!(Problem33, solve, p);
