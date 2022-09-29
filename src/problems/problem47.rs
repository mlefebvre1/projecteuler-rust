use crate::ntheory::primes::{distinct_primes, is_prime, sieves};

use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Distinct primes factors
    Problem 47
    The first two consecutive numbers to have two distinct prime factors are:
        14 = 2 × 7
        15 = 3 × 5
    The first three consecutive numbers to have three distinct prime factors are:
        644 = 2² × 7 × 23
        645 = 3 × 5 × 43
        646 = 2 × 17 × 19.
    Find the first four consecutive integers to have four distinct prime factors each. What is the first of these
    numbers?
    */
    const CONSECUTIVE_TARGET: usize = 4;
    let primes = sieves::<usize>(75000);
    let mut n = 646;
    let mut consecutive_integers = Vec::<usize>::new();
    let ans = loop {
        if !is_prime(n) {
            let _distinct_primes = distinct_primes(n, &primes);
            if _distinct_primes.len() == CONSECUTIVE_TARGET {
                consecutive_integers.push(n);
                if consecutive_integers.len() == CONSECUTIVE_TARGET {
                    break consecutive_integers[0].to_string();
                }
            } else {
                consecutive_integers.clear();
            }
        } else {
            consecutive_integers.clear();
        }
        n += 1;
    };
    Ok(ans)
}

timeit::timeit!(Problem47, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "134043");
    }
}
