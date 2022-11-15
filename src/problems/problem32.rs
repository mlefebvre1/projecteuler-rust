use crate::ntheory::arithmetic::pandigital_validation;
use anyhow::Result;
use itertools::Itertools;

pub fn p() -> Result<String> {
    /*
    Pandigital products
    Problem 32

    We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for
    example, the 5-digit number, 15234, is 1 through 5 pandigital.

    The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1
    through 9 pandigital.

    Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9
    pandigital.

    HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
    */
    let range = (1..100).cartesian_product(1..10000);
    let products = range.map(|(m1, m2)| {
        let prod = m1 * m2;
        let n = m1.to_string() + &m2.to_string() + &prod.to_string();
        (n, prod)
    });
    let pandigital_products = products
        .filter(|(n, _prod)| pandigital_validation(n, 1, 9))
        .map(|(_n, prod)| prod);
    Ok(pandigital_products
        .collect::<std::collections::HashSet<usize>>()
        .iter()
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "45228");
    }
}
