use anyhow::Result;
use num::Integer;

pub fn p() -> Result<String> {
    /*
    Square remainders
    Problem 120

    Let r be the remainder when (a−1)^n + (a+1)^n is divided by a2.

    For example, if a = 7 and n = 3, then r = 42: 63 + 83 = 728 ≡ 42 mod 49. And as n varies, so too will r, but for
    a = 7 it turns out that rmax = 42.

    For 3 ≤ a ≤ 1000, find ∑ rmax.

    Solution : By inspection, it was found that rmax is actually the value of "a" multiplied by the last even value of a
               Or in other words, if a is odd => rmax = a*(a-1) and if a is even rmax = a*(a-2)
               For example if a = 3, the last even value of a would be 2 meaning that rmax would be rmax=3*2=6
    */
    let sum_rmax = (3_u64..=1000)
        .scan(2, |last_even, a| {
            let rmax = a * *last_even;
            if a.is_even() {
                *last_even = a;
            }
            Some(rmax)
        })
        .sum::<u64>();
    Ok(sum_rmax.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "333082500");
    }
}
