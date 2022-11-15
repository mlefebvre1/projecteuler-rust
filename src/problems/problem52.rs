use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Permuted multiples
    Problem 52
    It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.

    Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
    */
    const MAX_N: usize = 1659999999; // Max will start with 16XXXXXXXX else 6x it would add a digit !!
    let ans = (100..MAX_N)
        .take_while(|&n| {
            if digit_uniqueness(n) {
                for mult in 2..7 {
                    if !same_digits(n * mult, n) {
                        return true;
                    }
                }
                return false;
            }
            true
        })
        .last()
        .unwrap()
        + 1; // +1 because take_while returns the iterator up to n-1 that found the predicate
    Ok(ans.to_string())
}

fn digit_uniqueness(n: usize) -> bool {
    let mut digit_array = [0usize; 10];
    for digit in int_to_vec_of_u8(&n) {
        digit_array[digit as usize] += 1;
    }
    for nb_rep in digit_array {
        if nb_rep > 1 {
            return false;
        }
    }
    true
}

fn same_digits(n1: usize, n2: usize) -> bool {
    for digit in int_to_vec_of_u8(&n1) {
        if !int_to_vec_of_u8(&n2).contains(&(digit)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "142857");
    }
}
