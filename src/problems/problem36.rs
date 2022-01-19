use crate::ntheory::palindrome::is_palindrome;
use crate::utils::timeit;

fn p() -> String {
    /*
    Double-base palindromes
    Problem 36

    The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
    Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
    (Please note that the palindromic number, in either base, may not include leading zeros.)
    */
    const MAX_N: usize = 1e6 as usize;
    let palindroms = (0..MAX_N).filter(|&n| {
        let n_bin = format!("{:b}", n);
        is_palindrome(&n.to_string()) && is_palindrome(&n_bin)
    });
    palindroms.sum::<usize>().to_string()
}

timeit::timeit!(Problem36, solve, p);