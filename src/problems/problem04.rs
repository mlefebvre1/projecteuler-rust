use ntheory::palindrome;
use utils::timeit;

fn p() -> i64 {
    /*
    Largest palindrome product
    Problem 4
    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers
    is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers.
    */
    let mut products = Vec::new();
    for n1 in 100..999 {
        for n2 in n1..999 {
            products.push(n1 * n2);
        }
    }
    let candidates = products.iter().filter(|&x| palindrome::is_palindrome(*x));
    return *candidates.max().unwrap() as i64;
}

timeit::timeit!(Problem04, solve, p);
