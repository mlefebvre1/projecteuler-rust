use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Number spiral diagonals
    Problem 28

    Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

    21 22 23 24 25
    20  7  8  9 10
    19  6  1  2 11
    18  5  4  3 12
    17 16 15 14 13

    It can be verified that the sum of the numbers on the diagonals is 101.

    What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

    It is easy to see that each number on the diagonal is [1+2, 1+4, 1+6, 1+8, 1+12, 1+16, ...]
    So for a dimension of 3 we increment each time by 2 but for a dimension of 5 we increment each time by 4, etc..
    Since even dimension does not exist in this problem, we simply make a for loop skipping every even dimension and
    the increment is always dimension-1
    */
    /*
    max_dim = 1001
    total = 1
    base = 0
    for dim in range(3, max_dim + 1, 2):
        increment = dim - 1
        for corner in range(0, 4):
            base += increment
            total += 1 + base
    return total
    */
    const MAX_DIM: usize = 1001;
    let mut total = 1usize;
    let mut base = 0usize;
    for dim in (3..MAX_DIM + 1).step_by(2) {
        let increment = dim - 1;
        for _corner in 0..4 {
            base += increment;
            total += 1 + base;
        }
    }
    Ok(total.to_string())
}

timeit::timeit!(Problem28, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "669171001");
    }
}
