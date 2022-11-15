use crate::utils::integers::PowMod;
use anyhow::Result;
use lazy_static::lazy_static;

const NB_EULER_NUMBERS: usize = 24680;
const M: u64 = 1_020_202_009;

fn p() -> Result<String> {
    /*
        Even Stevens
        Problem 709

        Every day for the past n days Even Stevens brings home his groceries in a plastic bag.
        He stores these plastic bags in a cupboard. He either puts the plastic bag into the cupboard with the rest,
        or else he takes an even number of the existing bags (which may either be empty or previously filled with other
        bags themselves) and places these into the current bag.

        After 4 days there are 5 possible packings and if the bags are numbered 1 (oldest), 2, 3, 4, they are:

        Four empty bags,
        1 and 2 inside 3, 4 empty,
        1 and 3 inside 4, 2 empty,
        1 and 2 inside 4, 3 empty,
        2 and 3 inside 4, 1 empty.
        Note that 1, 2, 3 inside 4 is invalid because every bag must contain an even number of bags.

        Define f(n) to be the number of possible packings of n bags. Hence f(4)=5. You are also given f(8)=1385.

        Find f(24680) giving your answer modulo 1 020 202 009.

        The solution to the problem is to calculate the euler numbers up to n=24680
        using the following recurrence equation :

            E(n+1) = sum(Comb(n,k) * E(n) * E(n-k)) / 2  for k 0..(n+1)
    */
    let mut euler_numbers = EulerNumbers::new();
    Ok(euler_numbers.nth(NB_EULER_NUMBERS - 2).unwrap().to_string())
}

lazy_static! {
    static ref FACTORIAL_MODULUS: Vec<u64> = {
        let mut factorial = vec![0; NB_EULER_NUMBERS + 1];
        factorial[0] = 1;
        factorial[1] = 1;
        factorial[2] = 2;
        for n in 1..NB_EULER_NUMBERS {
            factorial[n + 1] = (factorial[n] * (n + 1) as u64) % M;
        }
        factorial
    };
    static ref DIV2_MODULUS: u64 = u64::powmod(2, M - 2, M);
}

struct EulerNumbers {
    euler_number_cache: Vec<u64>,
    n: usize,
}

impl EulerNumbers {
    pub fn new() -> Self {
        let mut cache = vec![0; NB_EULER_NUMBERS + 1];
        cache[0] = 1;
        cache[1] = 1;
        cache[2] = 1;
        EulerNumbers {
            euler_number_cache: cache,
            n: 1,
        }
    }
    fn calculate_combinations(&self, n: usize, r: usize) -> u64 {
        // c(n,r) % M = (n! / (r! (n-r)!)) % M
        let inverse_mult = u64::powmod(FACTORIAL_MODULUS[r] * FACTORIAL_MODULUS[n - r], M - 2, M);
        FACTORIAL_MODULUS[n] * inverse_mult % M
    }
}

impl Iterator for EulerNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let mut alt_permutations = 0;
        for k in 0..=n {
            let comb = self.calculate_combinations(n, k);
            alt_permutations = (alt_permutations
                + ((comb * self.euler_number_cache[k]) % M * self.euler_number_cache[n - k]))
                % M;
        }
        let next_euler_number = ((alt_permutations % M) * *DIV2_MODULUS) % M;
        self.euler_number_cache[n + 1] = next_euler_number;
        self.n += 1;
        Some(next_euler_number)
        // sum() / 2 in mod M for M being a prime number is:
        //     // -> (sum() * (2^-1)) % M
        //     // -> ((sum() % M) * (2^(M-2) % M)) % M
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "773479144");
    }
}
