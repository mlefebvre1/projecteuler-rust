use crate::utils::timeit;
use num::BigUint;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Factorial digit sum
    Problem 20
    n! means n × (n − 1) × ... × 3 × 2 × 1
    For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
    and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
    Find the sum of the digits in the number 100!
    */
    const N: usize = 100;
    Ok(factorial(N)
        .to_radix_be(10)
        .iter()
        .map(|x| *x as usize)
        .sum::<usize>()
        .to_string())
}

fn factorial(n: usize) -> BigUint {
    let mut fact = BigUint::from(1usize);
    for i in 1..=n {
        fact *= i;
    }
    fact
}
timeit::timeit!(Problem20, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "648");
    }
}
