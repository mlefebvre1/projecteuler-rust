use anyhow::Result;
use num::BigUint;

fn p() -> Result<String> {
    /*
    Self powers
    Problem 48

    The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.

    Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
    */
    const MAX_K: usize = 1000;
    let m: BigUint = BigUint::from(1e10 as usize); // to keep only 10 digits..
    let self_powers = (1..=MAX_K).map(|n| {
        let _n = BigUint::from(n);
        _n.modpow(&_n, &m)
    });

    let sum: BigUint = self_powers.sum();
    let last_ten_digits = sum % m;
    Ok(last_ten_digits.to_str_radix(10))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "9110846700");
    }
}
