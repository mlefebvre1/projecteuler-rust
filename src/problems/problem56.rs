use anyhow::Result;
use num::BigUint;

pub fn p() -> Result<String> {
    /*
    Powerful digit sum
    Problem 56

    A googol (10100) is a massive number: one followed by one-hundred zeros; 100100 is almost unimaginably large: one
    followed by two-hundred zeros. Despite their size, the sum of the digits in each number is only 1.

    Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?
    */
    let ab = (0..100u32).flat_map(|a| (0..100u32).map(move |b| (a, b)));
    let digit_sums = ab.map(|(a, b)| {
        let power = BigUint::from(a).pow(b).to_str_radix(10);
        let digit_sum = power.chars().fold(0usize, |acc, digit| {
            acc + digit.to_digit(10).unwrap() as usize
        });
        digit_sum
    });
    Ok(digit_sums.max().unwrap().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "972");
    }
}
