use crate::series::square::Square;
use anyhow::Result;
use num::Integer;

pub fn p() -> Result<String> {
    /*
    Odd period square roots
    Problem 64

    All square roots are periodic when written as continued fractions and can be written in the form:
    For example, let us consider
    If we continue we would get the following expansion:
    The process can be summarised as follows:
    It can be seen that the sequence is repeating. For conciseness, we use the notation
    sqrt(23) = [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats indefinitely.
    The first ten continued fraction representations of (irrational) square roots are:
    See : https://projecteuler.net/problem=64
    Exactly four continued fractions, have an odd period.
    How many continued fractions for have an odd period?
    */
    const MAX_N: usize = 10000;
    let squares: Vec<usize> = Square::new().skip(2).take(MAX_N - 2).collect();
    let candidates = (2..=MAX_N).filter(|n| !squares.contains(n));
    let odd_period_sqrts =
        candidates.filter(|&n| sqrt_continued_fraction_expansion_len(n).is_odd());
    Ok(odd_period_sqrts.count().to_string())
}

fn sqrt_continued_fraction_expansion_len(n: usize) -> usize {
    let (m0, d0, a0) = (0, 1, (n as f64).sqrt().floor() as usize);
    let (mut mn, mut dn, mut an) = (m0, d0, a0);
    let criteria = 2 * an;
    let mut len = 0;
    while an != criteria {
        mn = dn * an - mn;
        dn = (n - mn.pow(2)) / dn;
        an = (a0 + mn) / dn;
        len += 1;
    }
    len
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1322");
    }
}
