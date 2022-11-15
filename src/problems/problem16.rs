use anyhow::Result;
use num::BigUint;

pub fn p() -> Result<String> {
    /*
    Power digit sum
    Problem 16

    2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

    What is the sum of the digits of the number 2^1000?
    */
    let n: BigUint = BigUint::from(2usize).pow(1000);
    Ok(n.to_radix_be(10)
        .iter()
        .map(|x| *x as usize)
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1366");
    }
}
