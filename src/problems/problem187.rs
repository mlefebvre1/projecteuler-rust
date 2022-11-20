use anyhow::Result;

use crate::ntheory::primes::sieves;

pub fn p() -> Result<String> {
    /*
    Semiprimes
    Problem 187

    A composite is a number containing at least two prime factors. For example, 15 = 3 × 5; 9 = 3 × 3; 12 = 2 × 2 × 3.

    There are ten composites below thirty containing precisely two, not necessarily distinct, prime factors: 4, 6, 9,
    10, 14, 15, 21, 22, 25, 26.

    How many composite integers, n < 10^8, have precisely two, not necessarily distinct, prime factors?
    */
    const MAX_N: usize = 1e8 as usize;
    let primes = sieves(MAX_N / 2);
    let mut nb_composites = 0;
    for (i, p1) in primes.iter().enumerate() {
        nb_composites += primes
            .iter()
            .skip(i)
            .map(|p2| p1 * p2)
            .take_while(|&product| product < MAX_N)
            .count();
    }
    Ok(nb_composites.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "17427258");
    }
}
