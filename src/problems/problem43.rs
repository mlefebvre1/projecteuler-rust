use crate::ntheory::primes::sieves;
use crate::utils::integers::vec_of_u8_to_int;
use crate::utils::timeit;
use itertools::Itertools;

fn p() -> String {
    /*
    Sub-string divisibility
    Problem 43

    The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some
    order, but it also has a rather interesting sub-string divisibility property.

    Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

    d2d3d4=406 is divisible by 2
    d3d4d5=063 is divisible by 3
    d4d5d6=635 is divisible by 5
    d5d6d7=357 is divisible by 7
    d6d7d8=572 is divisible by 11
    d7d8d9=728 is divisible by 13
    d8d9d10=289 is divisible by 17
    Find the sum of all 0 to 9 pandigital numbers with this property.
    */
    fn is_divisible(n: &[u8], primes: &[usize]) -> bool {
        if n[5] == 5 {
            // Digit 6 needs to always be 5 because it always divided by 5
            return primes
                .iter()
                .enumerate()
                .filter(|(d, &prime)| vec_of_u8_to_int(&n[d + 1..d + 4]) % prime != 0)
                .count()
                == 0;
        }
        false
    }

    let primes: Vec<usize> = sieves(17);
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let len = a.len();
    let permutations = a.into_iter().permutations(len);
    let divisibles =
        permutations.filter_map(|permutation| match is_divisible(&permutation, &primes) {
            true => Some(vec_of_u8_to_int(&permutation)),
            false => None,
        });
    divisibles.sum::<usize>().to_string()
}

timeit::timeit!(Problem43, solve, p);
