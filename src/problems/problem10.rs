use crate::ntheory::primes;
use crate::utils::timeit;

fn p() -> usize {
    /*
    Summation of primes
    Problem 10

    The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

    Find the sum of all the primes below two million
    */
    const MAX_PRIME: usize = 2e6 as usize;
    let primes = primes::sieves(MAX_PRIME);
    let sum: usize = primes.iter().sum();
    sum
}

timeit::timeit!(Problem10, solve, p);
