use crate::ntheory::primes::sieves;
use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Totient permutation
    Problem 70
    Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number of positive
    numbers less than or equal to n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less
    than nine and relatively prime to nine, φ(9)=6.
    The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.

    Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.

    Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a minimum.
    */
    let candidates = phi_from_2_primes();
    Ok(candidates
        .into_iter()
        .filter_map(|(n, phi)| {
            let (mut n_vec, mut phi_vec) = (int_to_vec_of_u8(&n), int_to_vec_of_u8(&phi));
            n_vec.sort_unstable();
            phi_vec.sort_unstable();
            if n_vec == phi_vec {
                return Some((n, (n as f64 / phi as f64)));
            }
            None
        })
        .min_by(|(_, ratio_a), (_, ratio_b)| ratio_a.partial_cmp(ratio_b).unwrap())
        .unwrap()
        .0
        .to_string())
}

fn phi_from_2_primes() -> Vec<(usize, usize)> {
    /*
    For n/phi(n) to be minimal, we need a large number with not many factors, hence 2. This means, we are
    looking for the product of 2 primes which produces the closest number possible to 10^7. The answer should be 2
    primes near sqrt(10^7). To give us room, we extend the search space with primes up to 2*sqrt(10^7) which is large
    enough to find the answer we are looking for.
    */
    const MAX_N: usize = 1e7 as usize;
    let max_n_sqrt: usize = (MAX_N as f64).sqrt() as usize;
    let primes = sieves(2 * max_n_sqrt);
    let mut phi_from_2_primes_vec = Vec::new();
    for (i, p1) in primes.iter().enumerate() {
        for p2 in primes[i..].iter() {
            let (n, phi) = (p1 * p2, (p1 - 1) * (p2 - 1));
            if n > MAX_N {
                break;
            }
            phi_from_2_primes_vec.push((n, phi));
        }
    }
    phi_from_2_primes_vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "8319823");
    }
}
