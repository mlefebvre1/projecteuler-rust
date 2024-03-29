use crate::ntheory::factor;
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Amicable numbers
    Problem 21

    Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
    If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable
    numbers.

    For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
    The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

    Evaluate the sum of all the amicable numbers under 10000.
    */
    const MAX_A: usize = 10000;
    Ok((1..MAX_A)
        .map(|a| {
            let b = factor::proper_divisors_sum(a);
            if (factor::proper_divisors_sum(b) == a) && (a != b) {
                b
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "31626");
    }
}
