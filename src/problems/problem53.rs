use anyhow::Result;
use factorial::Factorial;
use num::BigUint;

pub fn p() -> Result<String> {
    /*
    Combinatoric selections
    Problem 53

    There are exactly ten ways of selecting three from five, 12345:

    123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

    It is not until n = 23 that the value exceed one-million c(23,10) = 1144066

    How many, not necessarily distinct, values of c(n,r) for 1 n < 100 are greater than one-million
    */
    const TARGET: usize = 1e6 as usize;
    let factorials = (0..=100)
        .map(|n: usize| BigUint::from(n).factorial())
        .collect::<Vec<BigUint>>();
    let combinations = (23..=100).flat_map(|n| (0..n).map(move |r| (n, r)));
    let combinatorics =
        combinations.map(|(n, r)| &factorials[n] / (&factorials[r] * &factorials[n - r]));
    let combinatorics_greater_than_target = combinatorics.filter(|c| c > &BigUint::from(TARGET));
    Ok(combinatorics_greater_than_target.count().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "4075");
    }
}
