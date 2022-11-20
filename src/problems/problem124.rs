use anyhow::Result;
use itertools::Itertools;

pub fn p() -> Result<String> {
    /*
    Ordered radicals
    Problem 124

    The radical of n, rad(n), is the product of the distinct prime factors of n. For example, 504 = 23 × 32 × 7, so
    rad(504) = 2 × 3 × 7 = 42.

    If we calculate rad(n) for 1 ≤ n ≤ 10, then sort them on rad(n), and sorting on n if the radical values are equal,
    we get:

        See : https://projecteuler.net/problem=124

    Let E(k) be the kth element in the sorted n column; for example, E(4) = 8 and E(6) = 9.

    If rad(n) is sorted for 1 ≤ n ≤ 100000, find E(10000).
    */
    const MAX_N: usize = 100000;
    let mut sorted_rad_n = calculate_rad(MAX_N)
        .into_iter()
        .enumerate()
        .sorted_by_key(|x| x.1);
    let (ans, _) = sorted_rad_n.nth(10000).unwrap();
    Ok(ans.to_string())
}

fn calculate_rad(max_n: usize) -> Vec<usize> {
    let mut rad = (0..=max_n).map(|_| 1).collect::<Vec<_>>();
    if max_n <= 2 {
        return vec![1, 1, 2];
    }
    if max_n <= 3 {
        return vec![1, 1, 1, 2, 3];
    }

    // Modified sieves to generate the product of distinct primes on the fly
    for n in 2..max_n {
        if rad[n] == 1 {
            rad[n] *= n;
            for x in (n + n..=max_n).step_by(n) {
                rad[x] *= n;
            }
        }
    }
    rad
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "21417");
    }
}
