use crate::ntheory::primes::sieves;
use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Sum square difference
    Problem 6

    The sum of the squares of the first ten natural numbers is 385,

    The square of the sum of the first ten natural numbers is 3025,

    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
    2640

    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the
    sum.
    */
    const MAX_PRIME: usize = 1e6 as usize;
    const PRIME_INDEX: usize = 10001;
    let primes = sieves(MAX_PRIME);
    Ok(primes[PRIME_INDEX - 1].to_string())
}

timeit::timeit!(Problem07, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "104743");
    }
}
