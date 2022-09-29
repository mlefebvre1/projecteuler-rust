use crate::ntheory::primes::sieves;
use crate::utils::timeit;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Prime summations
    Problem 77

    It is possible to write ten as the sum of primes in exactly five different ways:

    7 + 3
    5 + 5
    5 + 3 + 2
    3 + 3 + 2 + 2
    2 + 2 + 2 + 2 + 2

    What is the first value which can be written as the sum of primes in over five thousand different ways?

    Same strategy as problem 76. Generate some n's above the target and find the smallest n above the target
    */
    const MAX_N: usize = 100;
    const TARGET: usize = 5000;
    let primes = sieves(MAX_N);
    let mut nb_ways = (0..=MAX_N).map(|_| 0).collect::<Vec<usize>>();
    nb_ways[0] = 1;
    let pairs = primes
        .into_iter()
        .flat_map(|prime| (prime..=MAX_N).map(move |n| (prime, n)));
    let candidates = pairs.filter_map(|(prime, n)| {
        nb_ways[n] += nb_ways[n - prime];
        if nb_ways[n] > TARGET {
            return Some(n);
        }
        None
    });
    Ok(candidates.min().unwrap().to_string())
}

timeit::timeit!(Problem77, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "71");
    }
}
