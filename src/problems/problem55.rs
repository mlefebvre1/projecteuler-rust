use crate::ntheory::palindrome::is_palindrome;
use crate::utils::integers::{int_to_vec_of_u8, vec_of_u8_to_int};
use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Lychrel numbers
    Problem 55

    If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.

    Not all numbers produce palindromes so quickly. For example,

    349 + 943 = 1292,
    1292 + 2921 = 4213
    4213 + 3124 = 7337

    That is, 349 took three iterations to arrive at a palindrome.

    Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome.
    A number that never forms a palindrome through the reverse and add process is called a Lychrel number.
    Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number
    is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand,
    it will either (i) become a palindrome in less than fifty iterations,
    or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome.
    In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome:
    4668731596684224866951378664 (53 iterations, 28-digits).

    Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.

    How many Lychrel numbers are there below ten-thousand?

    NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.
    */
    const MAX_N: u128 = 10000;
    Ok((1..MAX_N)
        .filter(|&n| is_lynchrel_number(n))
        .count()
        .to_string())
}

fn is_lynchrel_number(n: u128) -> bool {
    const MAX_ITER: u128 = 50;
    let mut _n = n as u128;
    for _ in 1..MAX_ITER {
        let n_vec = int_to_vec_of_u8(&_n);
        let n_vec_rev: Vec<u8> = n_vec.iter().rev().copied().collect();
        let n_rev = vec_of_u8_to_int::<u128>(&n_vec_rev);

        let original_plus_reverse = _n + n_rev;
        if is_palindrome::<u128>(&original_plus_reverse) {
            return false;
        }
        _n = original_plus_reverse;
    }
    true
}

timeit::timeit!(Problem55, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "249");
    }
}
