use anyhow::Result;
use num::Integer;
pub fn p() -> Result<String> {
    /*
    How many reversible numbers are there below one-billion?
    Problem 145

    Some positive integers n have the property that the sum [ n + reverse(n) ] consists entirely of odd (decimal)
    digits. For instance, 36 + 63 = 99 and 409 + 904 = 1313. We will call such numbers reversible; so 36, 63, 409,
    and 904 are reversible. Leading zeroes are not allowed in either n or reverse(n).

    There are 120 reversible numbers below one-thousand.

    How many reversible numbers are there below one-billion (109)?
    */
    let mut nb_greater_than_10_and_odd_and_b_not_zero: u64 = 0;
    let mut nb_greater_than_10_and_odd: u64 = 0;
    let mut nb_lesser_than_10_and_odd_and_both_not_zero: u64 = 0;
    let mut nb_lesser_than_10_and_odd: u64 = 0;
    let mut nb_lesser_than_10_and_even: u64 = 0;
    for a in 0..10 {
        for b in 0..10 {
            let ab = a + b;
            if ab > 10 && ab.is_odd() && b > 0 {
                nb_greater_than_10_and_odd_and_b_not_zero += 1;
            }
            if ab < 10 && ab.is_odd() && b > 0 && a > 0 {
                nb_lesser_than_10_and_odd_and_both_not_zero += 1;
            }

            if ab > 10 && ab.is_odd() {
                nb_greater_than_10_and_odd += 1;
            }

            if ab < 10 && ab.is_odd() {
                nb_lesser_than_10_and_odd += 1;
            }

            if ab < 10 && ab.is_even() {
                nb_lesser_than_10_and_even += 1;
            }
        }
    }
    let nb_reversibles = [
        0,                                                                              // 10
        nb_greater_than_10_and_odd_and_b_not_zero,                                      // 100
        nb_greater_than_10_and_odd_and_b_not_zero * 5,                                  // 1_000
        nb_lesser_than_10_and_odd_and_both_not_zero * nb_lesser_than_10_and_odd,        // 10_000
        0,                                                                              // 100_000
        nb_lesser_than_10_and_odd_and_both_not_zero * nb_lesser_than_10_and_odd.pow(2), // 1_000_000
        nb_greater_than_10_and_odd_and_b_not_zero
            * nb_greater_than_10_and_odd
            * nb_lesser_than_10_and_even
            * 5,
        nb_lesser_than_10_and_odd_and_both_not_zero * nb_lesser_than_10_and_odd.pow(3), // 10_000_000
        0, // 100_000_000
    ];
    Ok(nb_reversibles.iter().sum::<u64>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "608720");
    }
}
