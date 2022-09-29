use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Sum square difference
    Problem 6

    The sum of the squares of the first ten natural numbers is 385,

    The square of the sum of the first ten natural numbers is 3025,

    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
    2640

    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the
    sum.
    */
    const MAX_N: usize = 100;
    let range = 0..=MAX_N;
    let sum_of_square: usize = range.map(|x| x * x).sum();
    let square_of_sum = (MAX_N * (MAX_N + 1) / 2).pow(2);
    Ok((square_of_sum - sum_of_square).to_string())
}

timeit::timeit!(Problem06, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "25164150");
    }
}
