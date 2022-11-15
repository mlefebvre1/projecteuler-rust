use crate::ntheory::factor::factorize;
use anyhow::Result;
use itertools::Itertools;
use num::integer::lcm;

fn p() -> Result<String> {
    /*
    Diophantine reciprocals I
    Problem 108

    In the following equation x, y, and n are positive integers.

     1     1      1
    --- + ---  = ---
     x     y      n

    For n = 4 there are exactly three distinct solutions

    What is the least value of n for which the number of distinct solutions exceeds one-thousand?

    NOTE: This problem is an easier version of Problem 110; it is strongly advised that you solve this one first.

    Factorize each numbers up until a limit (chosen through testing) and save the number of factors for each n.
    Sort the array with n with the greatest number of factors up to the less number of factors. From there check in
    order the array and stop if we have found a n for which the number of distinct solution is greater than 1000
    */
    let candidates = (0..200000)
        .map(|n| (n as u64, factorize(n).len() as u64))
        .sorted_by_key(|x| x.1);

    let (n, _nb_factor) = candidates
        .rev()
        .find(|(n, _nb_factor)| nb_reciprocal_diophantine(*n) > 1000)
        .unwrap();
    Ok(n.to_string())
}

fn nb_reciprocal_diophantine(n: u64) -> u64 {
    /*
         1     1      1         1     (x - n)             (x * n)
        --- + ---  = ---  -->  --- = ---------  -->  y = ---------
         x     y      n         y     (x * n)             (x - n)
    */
    (n + 1..)
        .map(|x| {
            let y = (x * n) / (x - n);
            (x, y)
        })
        .take_while(|(x, y)| x != y)
        .filter(|(x, y)| lcm(*x, n) == lcm(*y, n))
        .count() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "180180");
    }
}
