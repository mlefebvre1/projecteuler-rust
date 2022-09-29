use crate::utils::timeit;
use factorial::Factorial;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Digit factorials
    Problem 34
    145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
    Find the sum of all numbers which are equal to the sum of the factorial of their digits.
    Note: As 1! = 1 and 2! = 2 are not sums they are not included.
    */
    fn is_curious_number(n: usize) -> bool {
        let digits_fractorial_sum = n.to_string().chars().fold(0usize, |acc, c| {
            let digit = c.to_digit(10).unwrap() as usize;
            acc + digit.factorial()
        });
        digits_fractorial_sum == n
    }

    const MAX_N: usize = 50000;
    Ok((3..=MAX_N)
        .filter(|n| is_curious_number(*n))
        .sum::<usize>()
        .to_string())
}

timeit::timeit!(Problem34, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "40730");
    }
}
