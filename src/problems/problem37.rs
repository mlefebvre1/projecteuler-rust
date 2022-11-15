use crate::ntheory::primes::{is_prime, sieves};
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Truncatable primes
    Problem 37

    The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from
    left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797,
    379, 37, and 3.

    Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

    NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
    */
    fn is_truncatable(prime: usize) -> bool {
        let digits = prime.to_string();
        let nb_digits = digits.chars().count();
        for digit in 1..nb_digits {
            let trunc_left_to_right = digits[digit..nb_digits].parse::<usize>().unwrap();
            let trunc_right_to_left = digits[0..nb_digits - digit].parse::<usize>().unwrap();
            if !is_prime(trunc_left_to_right) || !is_prime(trunc_right_to_left) {
                return false;
            }
        }
        true
    }

    const MAX_N: usize = 1e6 as usize;
    const MAX_TRUNC: usize = 11;
    let primes = sieves(MAX_N);
    let primes_range = primes.iter().skip_while(|&p| *p < 10);
    let truncatables = primes_range
        .filter(|&prime| is_truncatable(*prime))
        .take(MAX_TRUNC);
    Ok(truncatables.sum::<usize>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "748317");
    }
}
