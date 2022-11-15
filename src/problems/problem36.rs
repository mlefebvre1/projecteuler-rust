use crate::ntheory::palindrome::{is_palindrome, is_palindrome_str};
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Double-base palindromes
    Problem 36

    The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
    Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
    (Please note that the palindromic number, in either base, may not include leading zeros.)
    */
    const MAX_N: usize = 1e6 as usize;
    let palindroms = (0..MAX_N).filter(|n| {
        let n_bin = format!("{:b}", n);
        is_palindrome(n) && is_palindrome_str(&n_bin)
    });
    Ok(palindroms.sum::<usize>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "872187");
    }
}
