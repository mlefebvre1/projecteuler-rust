use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Digit fifth powers
    Problem 30

    Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

        1634 = 1^4 + 6^4 + 3^4 + 4^4
        8208 = 8^4 + 2^4 + 0^4 + 8^4
        9474 = 9^4 + 4^4 + 7^4 + 4^4

    As 1 = 1^4 is not a sum it is not included.

    The sum of these numbers is 1634 + 8208 + 9474 = 19316.

    Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
    */
    const EXP: usize = 5;
    const MAX_N: usize = 1000000;
    const RANGE: std::ops::Range<usize> = 2..MAX_N;
    let powers: Vec<usize> = (0..10).map(|n| num::pow(n, EXP)).collect();
    let range_as_str = RANGE.map(|n| n.to_string());
    let fifth_powers_sum = range_as_str.map(|s| {
        s.chars()
            .fold(0, |acc, c| acc + powers[c.to_digit(10).unwrap() as usize])
    });
    let solutions = fifth_powers_sum
        .zip(RANGE)
        .filter(|(fifth_power, n)| *fifth_power == *n);
    Ok(solutions.fold(0, |acc, (_, n)| acc + n).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "443839");
    }
}
