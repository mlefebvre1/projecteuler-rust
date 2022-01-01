use utils::timeit;

fn p() -> usize {
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
    square_of_sum - sum_of_square
}

timeit::timeit!(Problem06, solve, p);
