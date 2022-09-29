use crate::series::pentagonal::GeneralizedPentagonal;
use crate::utils::timeit;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Coin partitions
    Problem 78

    Let p(n) represent the number of different ways in which n coins can be separated into piles. For example, five
    coins can be separated into piles in exactly seven different ways, so p(5)=7.

    OOOOO
    OOOO   O
    OOO   OO
    OOO   O   O
    OO   OO   O
    OO   O   O   O
    O   O   O   O   O

    Find the least value of n for which p(n) is divisible by one million
    */
    const MAX_N: usize = 1e6 as usize;
    let mut partitions = (0..MAX_N).map(|_| 0).collect::<Vec<isize>>();
    let generalized_pentagonal = GeneralizedPentagonal::<isize>::new()
        .take(MAX_N)
        .collect::<Vec<isize>>();
    partitions[0] = 1;
    Ok((1..MAX_N)
        .find(|&n| {
            let part = partition(n as isize, &partitions, &generalized_pentagonal);
            if part == 0 {
                return true;
            }
            partitions[n] = part;
            false
        })
        .unwrap()
        .to_string())
}

fn partition(n: isize, partitions: &[isize], pentagonals: &[isize]) -> isize {
    /*
    Return Number partition p(n) using the Euler Function
    p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + p(n-15) - ...
    Keep going utils n-k is negative
    */
    let mut sign = 1;
    (1usize..)
        .map_while(|pentagonal_index| {
            let k = pentagonals[pentagonal_index];
            let index = n - k;
            if index < 0 {
                return None;
            }
            let result = sign * partitions[index as usize];
            let sign_counter = pentagonal_index % 4;
            sign = match sign_counter / 2 {
                n if n > 0 => -1,
                _ => 1,
            };
            Some(result)
        })
        .sum::<isize>()
        % 1e6 as isize
}

timeit::timeit!(Problem78, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "55374");
    }
}
