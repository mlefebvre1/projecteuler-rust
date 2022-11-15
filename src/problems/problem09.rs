use crate::ntheory::pythagorean;
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Special Pythagorean triplet
    Problem 9

    A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
    a^2 + b^2 = c^2

    For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

    There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    Find the product abc.
    */
    let triples = pythagorean::pythagorean_triples(2000);
    let triple = triples.iter().find(|triple| triple.sum() == 1000).unwrap();
    Ok(triple.prod().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "31875000");
    }
}
