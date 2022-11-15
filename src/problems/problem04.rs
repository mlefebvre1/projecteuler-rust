use crate::ntheory::palindrome;
use anyhow::Result;
use itertools::Itertools;

pub fn p() -> Result<String> {
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
    let candidates = products.filter(|x| palindrome::is_palindrome(x));
    Ok(candidates.max().unwrap().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "906609");
    }
}
