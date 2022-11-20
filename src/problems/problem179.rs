use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Consecutive positive divisors
    Problem 179

    Find the number of integers 1 < n < 10^7, for which n and n + 1 have the same number of positive divisors. For
    example, 14 has the positive divisors 1, 2, 7, 14 while 15 has 1, 3, 5, 15.
    */
    const MAX_N: usize = 1e7 as usize;
    let divs_per_n = nb_divisors_per_n(MAX_N);
    let nb_consecutives_pos_dividers = (2..MAX_N)
        .filter(|&n| divs_per_n[n] == divs_per_n[n - 1])
        .count();
    Ok(nb_consecutives_pos_dividers.to_string())
}

fn nb_divisors_per_n(max_n: usize) -> Vec<usize> {
    let mut nb_divisors = vec![1; max_n];
    for n in 2..max_n {
        for x in (n..max_n).step_by(n) {
            nb_divisors[x] += 1;
        }
    }
    nb_divisors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "986262");
    }
}
