use crate::utils::{integers::int_to_vec_of_u8, timeit};
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number; for
    example, 134468.

    Similarly if no digit is exceeded by the digit to its right it is called a decreasing number; for example, 66420.

    We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number; for example, 155349.

    Clearly there cannot be any bouncy numbers below one-hundred, but just over half of the numbers below one-thousand (525)
    are bouncy. In fact, the least number for which the proportion of bouncy numbers first reaches 50% is 538.

    Surprisingly, bouncy numbers become more and more common and by the time we reach 21780 the proportion of bouncy numbers
    is equal to 90%.

    Find the least number for which the proportion of bouncy numbers is exactly 99%.
    */
    let (ans, _) = (0_u64..)
        .filter(is_bouncy)
        .scan(0, |nb_bouncy, n| {
            *nb_bouncy += 1;
            Some((n, *nb_bouncy as f64 / n as f64))
        })
        .find(|(_n, prop)| *prop >= 0.99)
        .unwrap();
    Ok(ans.to_string())
}

fn is_bouncy(n: &u64) -> bool {
    let digits = int_to_vec_of_u8(n);

    let mut incr_sort = digits.clone();
    incr_sort.sort();
    let decr_sort = incr_sort.iter().rev().copied().collect::<Vec<_>>();
    incr_sort != digits && decr_sort != digits
}

timeit::timeit!(Problem108, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "1587000");
    }
}
