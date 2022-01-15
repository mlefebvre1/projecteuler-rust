use crate::ntheory::palindrome;
use crate::utils::timeit;
use itertools::Itertools;

fn p() -> String {
    /*
    Largest palindrome product
    Problem 4
    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers
    is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers.
    */
    const RANGE: std::ops::Range<usize> = 100..999;
    let cartesian_prod = RANGE.cartesian_product(RANGE);
    let products = cartesian_prod.map(|(n1, n2)| n1 * n2);
    let candidates = products.filter(|&x| palindrome::is_palindrome(&x.to_string()));
    candidates.max().unwrap().to_string()
}

timeit::timeit!(Problem04, solve, p);
