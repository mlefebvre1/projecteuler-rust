use ntheory::primes::sieves;
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
    const MAX_PRIME: usize = 1e6 as usize;
    const PRIME_INDEX: usize = 10001;
    let primes = sieves(MAX_PRIME);
    primes[PRIME_INDEX - 1]
}

timeit::timeit!(Problem07, solve, p);
