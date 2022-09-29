use crate::ntheory::primes::{is_prime, sieves};
use crate::utils::integers::{int_to_vec_of_u8, vec_of_u8_to_int};
use crate::utils::timeit;
use itertools::Itertools;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Prime permutations
    Problem 49
    The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways:
     (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.
    There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is
     one other 4-digit increasing sequence.
    What 12-digit number do you form by concatenating the three terms in this sequence?
    */
    const MAX_N: usize = 9876;
    let primes = sieves(MAX_N);
    let four_digit_primes = primes.iter().filter(|&p| *p > 1000);

    let candidates = four_digit_primes.map(|prime| {
        let prime_vec_u8 = int_to_vec_of_u8(prime);
        let permutations = prime_vec_u8.iter().permutations(4);
        let permutations = permutations.map(|permutation| {
            let p = permutation.iter().map(|&c| *c).collect::<Vec<u8>>();
            vec_of_u8_to_int(&p)
        });
        let mut permutations = permutations
            .filter(|&permutation| permutation >= 1000 && is_prime(permutation))
            .collect::<Vec<usize>>();
        permutations.sort_unstable();
        permutations.dedup();
        permutations
    });

    let mut candidates_with_three_consecutive_terms_no_duplicates = candidates
        .filter(|candidate| candidate.len() >= 3)
        .collect::<Vec<Vec<usize>>>();
    candidates_with_three_consecutive_terms_no_duplicates.sort();
    candidates_with_three_consecutive_terms_no_duplicates.dedup();

    let mut candidates_with_all_criterias = candidates_with_three_consecutive_terms_no_duplicates
        .iter()
        .filter(|&candidate| {
            for (i, ki) in candidate.iter().enumerate() {
                for kj in candidate.iter().skip(i + 1) {
                    let increment = kj - ki;
                    let next_n = kj + increment;
                    if candidate.contains(&next_n) {
                        return true;
                    }
                }
            }
            false
        });

    let final_candidate = candidates_with_all_criterias
        .find(|&candidate| !candidate.contains(&1487))
        .unwrap();
    Ok(extract_the_correct_terms_and_concat(final_candidate))
}

fn extract_the_correct_terms_and_concat(candidate: &[usize]) -> String {
    let mut ans = String::from("");
    for (i, ki) in candidate.iter().enumerate() {
        for kj in candidate.iter().skip(i + 1) {
            let increment = kj - ki;
            let next_n = kj + increment;
            if candidate.contains(&next_n) {
                ans = ki.to_string() + &kj.to_string() + &next_n.to_string();
                break;
            }
        }
    }
    ans
}

timeit::timeit!(Problem49, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "296962999629");
    }
}
