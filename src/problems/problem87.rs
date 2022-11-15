use crate::ntheory::primes::sieves;
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Prime power triples
    Problem 87
    The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power is 28. In fact,
    there are exactly four numbers below fifty that can be expressed in such a way:
    28 = 2^2 + 2^3 + 2^4
    33 = 32 + 23 + 24
    49 = 52 + 23 + 24
    47 = 22 + 33 + 24
    How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth
    power?
    Solution : Create list 3 of primes for each power using the fact that for x^2 < 50e6 and x^3 < 50e6 and x^4 < 50e6
                Next simply use 3 nested loops to calculate each sum. For each sum below 50e6 make sure the sum was not
                written already.
    */
    const MAX: usize = 50e6 as usize;
    let mut cache = (0..=MAX).map(|_| 0).collect::<Vec<usize>>();
    let primes_pow2 = sieves((MAX as f64).powf(1.0 / 2.0) as usize);
    let primes_pow3 = sieves((MAX as f64).powf(1.0 / 3.0) as usize);
    let primes_pow4 = sieves((MAX as f64).powf(1.0 / 4.0) as usize);
    for prime2 in primes_pow2.iter() {
        for prime3 in primes_pow3.iter() {
            for prime4 in primes_pow4.iter() {
                let sum = prime2.pow(2) + prime3.pow(3) + prime4.pow(4);
                if sum <= MAX && cache[sum] == 0 {
                    cache[sum] = 1;
                }
            }
        }
    }
    Ok(cache.into_iter().filter(|&n| n != 0).count().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1097343");
    }
}
