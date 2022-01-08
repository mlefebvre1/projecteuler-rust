use crate::utils::timeit;
use itertools::Itertools;

fn p() -> String {
    /*
    Lexicographic permutations
    Problem 24
    A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits
    1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
    The lexicographic permutations of 0, 1 and 2 are:

        012   021   102   120   201   210

    What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
    */
    const TARGET: usize = 1e6 as usize;
    let mut permutations = (0..10).permutations(10);
    let permutation_target = permutations.nth(TARGET - 1).unwrap();
    let permutation_target_as_utf8: Vec<u8> = permutation_target.iter().map(|c| *c + 48).collect(); // convert digits int to utf8
    String::from(String::from_utf8_lossy(&permutation_target_as_utf8))
}

timeit::timeit!(Problem24, solve, p);
