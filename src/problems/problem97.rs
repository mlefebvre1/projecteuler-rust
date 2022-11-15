use anyhow::Result;
use num::BigUint;

pub fn p() -> Result<String> {
    /*
    Large non-Mersenne prime
    Problem 97

    The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime of the form
    26972593−1; it contains exactly 2,098,960 digits. Subsequently other Mersenne primes, of the form 2p−1, have been
    found which contain more digits.
    However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits: 28433×2^7830457+1.
    Find the last ten digits of this prime number.

    I guess in python this is way too easy...
    */
    let m = BigUint::from(1e10 as usize);
    let exp = BigUint::from(7830457usize);
    let base = BigUint::from(2usize);
    let ans = (BigUint::from(28433usize) * base.modpow(&exp, &m) + BigUint::from(1usize)) % m;
    Ok(ans.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "8739992577");
    }
}
