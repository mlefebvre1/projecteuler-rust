use crate::ntheory::primes::is_prime;
use crate::utils::timeit;
use itertools::Itertools;

fn p() -> String {
    /*
    Pandigital prime
    Problem 41

    We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once.
    For example, 2143 is a 4-digit pandigital and is also prime.

    What is the largest n-digit pandigital prime that exists?
    */
    let numbers = [
        1, 12, 123, 1234, 12345, 123456, 1234567, 12345678, 123456789,
    ];
    let candidates = numbers.map(|n| {
        let n_str = n.to_string();
        let permutations = n_str.chars().permutations(n_str.chars().count());
        let prime_permutations = permutations.filter_map(|permutation_vec| {
            let permutation = permutation_vec
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            match is_prime(permutation) {
                true => Some(permutation),
                false => None,
            }
        });
        prime_permutations.collect::<Vec<usize>>()
    });
    candidates.into_iter().flatten().max().unwrap().to_string()
}

timeit::timeit!(Problem41, solve, p);
