use crate::{ntheory::primes::sieves, utils::integers::PowMod};
use anyhow::Result;
use num::Integer;

pub fn p() -> Result<String> {
    /*
    Prime square remainders
    Problem 123
    Let pn be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when (pn−1)^n + (pn+1)^n is divided by
    pn^2.
    For example, when n = 3, p3 = 5, and 4^3 + 6^3 = 280 ≡ 5 mod 25.
    The least value of n for which the remainder first exceeds 109 is 7037.
    Find the least value of n for which the remainder first exceeds 1010.
    Solution : Simply apply the modulo on every operation to keep every results in the set p(n)**2
    */
    let primes = sieves::<u128>(1e6 as u128);

    let (_, ans) = primes
        .into_iter()
        .enumerate()
        .filter_map(|(n, prime)| {
            let np1 = (n + 1) as u128;
            if np1.is_odd() {
                Some((prime, np1))
            } else {
                None
            }
        })
        .find(|(prime, np1)| {
            let p2 = prime.pow(2);
            let pn_m1 = u128::powmod(prime - 1, *np1, p2); // Finite field exponential : (p(n)-1)^n) % p(n)^2
            let pn_p1 = u128::powmod(prime + 1, *np1, p2); // Finite field exponential : (p(n)+1)^n) % p(n)^2
            let r = (pn_m1 + pn_p1) % p2;
            r > 1e10 as u128
        })
        .unwrap();

    Ok(ans.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "21035");
    }
}
