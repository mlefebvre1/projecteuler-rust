use crate::ntheory::primes;
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Summation of primes
    Problem 10

    The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

    Find the sum of all the primes below two million
    */
    const MAX_PRIME: usize = 2e6 as usize;
    let primes = primes::sieves(MAX_PRIME);
    let sum: usize = primes.iter().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "142913828922");
    }
}
