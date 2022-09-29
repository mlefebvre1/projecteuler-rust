use crate::ntheory::primes;
use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Largest prime factor
    Problem 3
    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
    */
    const K: usize = 600851475143;
    let k_sqrt: usize = (K as f64).sqrt() as usize;

    Ok((1..k_sqrt)
        .filter(|&n| K % n == 0 && primes::is_prime(n))
        .max()
        .unwrap()
        .to_string())
}

timeit::timeit!(Problem03, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "6857");
    }
}
