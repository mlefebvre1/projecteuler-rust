use anyhow::Result;
use num::Integer;

pub fn p() -> Result<String> {
    /*
    Counting fractions in a range
    Problem 73

    Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced
    proper fraction.

    If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:

    1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8

    It can be seen that there are 3 fractions between 1/3 and 1/2.

    How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for d ≤ 12,000?

    For each denominator, we establish a lower/upper (1/3 and 1/2) bound to check for the numerator. We then check is the either
    the numerator or denominator is odd and if the gcd between d and n is 1.
    */
    const MAX_D: usize = 12000;
    let lower_bound = 1.0 / 3.0;
    let upper_bound = 1.0 / 2.0;

    Ok((4..=MAX_D)
        .flat_map(|d| {
            let n_lower_bound = (d as f64 * lower_bound).ceil() as usize;
            let n_upper_bound = (d as f64 * upper_bound).ceil() as usize;
            (n_lower_bound..n_upper_bound).filter(move |n| {
                if (n.is_odd() || d.is_odd()) && n.gcd(&d) == 1 {
                    return true;
                }
                false
            })
        })
        .count()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "7295372");
    }
}
