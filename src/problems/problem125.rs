use anyhow::Result;

use crate::ntheory::palindrome::is_palindrome;

pub fn p() -> Result<String> {
    /*
    Palindromic sums
    Problem 125

    The palindromic number 595 is interesting because it can be written as the sum of consecutive squares: 6^2 + 7^2 +
    8^2 + 9^2 + 10^2 + 11^2 + 12^2.

    There are exactly eleven palindromes below one-thousand that can be written as consecutive square sums, and the sum
    of these palindromes is 4164. Note that 1 = 02 + 12 has not been included as this problem is concerned with the
    squares of positive integers.

    Find the sum of all the numbers less than 10^8 that are both palindromic and can be written as the sum of consecutive
    squares.
    */
    const MAX_N: f32 = 1e8;
    let max_square = MAX_N.sqrt().ceil() as usize;
    let squares = (0..max_square).map(|n| n.pow(2)).collect::<Vec<_>>();
    let mut solutions = Vec::new();
    for (i, square1) in squares.iter().enumerate() {
        let mut square_sum = *square1;
        for square2 in squares[i + 1..].iter() {
            square_sum += square2;
            if square_sum < MAX_N as usize {
                if is_palindrome(&square_sum) {
                    solutions.push(square_sum);
                }
            } else {
                break;
            }
        }
    }
    solutions.sort();
    solutions.dedup();
    let ans = solutions.iter().sum::<usize>() - 1;
    Ok(ans.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "2906969179");
    }
}
