use crate::ntheory::factor::proper_divisors_sum;
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Non-abundant sums
    Problem 23

    A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example,
    the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

    A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this
    sum exceeds n.

    As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum
    of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can
    be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis
    even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is
    less than this limit.

    Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
    */

    let abundants: Vec<usize> = (1..MAX_N)
        .filter(|n| proper_divisors_sum(*n) > *n)
        .collect();
    // In a table identify all numbers that can be written by the sum of two abundant numbers
    let array_sum_of_2_abundants = get_array_of_sum_of_two_abundants(&abundants);
    let sum_of_non_two_abundants: usize = array_sum_of_2_abundants
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| if !(*x) { acc + i } else { acc });
    Ok(sum_of_non_two_abundants.to_string())
}

const MAX_N: usize = 28123;

fn get_array_of_sum_of_two_abundants(abundants: &[usize]) -> [bool; MAX_N] {
    let mut numbers_written_as_sum_of_2_abundants: [bool; MAX_N] = [false; MAX_N];
    for a in abundants {
        for b in abundants {
            let n = a + b;
            if n < MAX_N {
                numbers_written_as_sum_of_2_abundants[n] = true;
            }
        }
    }
    numbers_written_as_sum_of_2_abundants
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "4179871");
    }
}
