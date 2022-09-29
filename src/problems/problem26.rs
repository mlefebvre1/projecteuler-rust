use crate::ntheory::{arithmetic, primes};
use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Reciprocal cycles
    Problem 26

    A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators
    2 to 10 are given:

        1/2	= 	0.5
        1/3	= 	0.(3)
        1/4	= 	0.25
        1/5	= 	0.2
        1/6	= 	0.1(6)
        1/7	= 	0.(142857)
        1/8	= 	0.125
        1/9	= 	0.(1)
        1/10	= 	0.1

    Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring
    cycle.

    Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
    */
    const LIMIT: usize = 1000;
    let primes = primes::sieves(LIMIT);
    let decimal_recurring = primes
        .iter()
        .map(|p| arithmetic::nb_decimal_recurring_len(*p));
    let both = primes.iter().zip(decimal_recurring);
    let (max_prime, _) = both.max_by_key(|k| k.1).unwrap();
    Ok((*max_prime).to_string())
}

timeit::timeit!(Problem26, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "983");
    }
}
