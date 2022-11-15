use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;
use itertools::iproduct;

fn p() -> Result<String> {
    /*
    Digit power sum
    Problem 119

    The number 512 is interesting because it is equal to the sum of its digits raised to some power: 5 + 1 + 2 = 8, and
    83 = 512. Another example of a number with this property is 614656 = 284.

    We shall define an to be the nth term of this sequence and insist that a number must contain at least two digits to
    have a sum.

    You are given that a2 = 512 and a10 = 614656.

    Find a30.
    */
    const MAX_POW: usize = 30;
    const MAX_BASE: usize = 100;
    const GOAL: usize = 30;
    let mut candidates = iproduct!(2..MAX_BASE, 0..MAX_POW)
        .filter_map(|(base, power)| {
            let (n, overflowed) = base.overflowing_pow(power as u32);
            if !overflowed {
                let digits = int_to_vec_of_u8(&n);
                let digit_sum = digits.iter().map(|d| *d as usize).sum::<usize>();
                if digit_sum == base && digits.len() >= 2 {
                    Some(n)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    candidates.sort();
    let ans = candidates[GOAL - 1];
    Ok(ans.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "248155780267521");
    }
}
