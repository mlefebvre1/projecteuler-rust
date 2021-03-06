use crate::ntheory::primes;
use crate::utils::timeit;
use num::Integer;

fn p() -> String {
    /*
    Largest prime factor
    Problem 3
    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
    */
    const K: usize = 600851475143;
    let numbers = 1..(K as f64).sqrt() as usize;
    let candidates = numbers.filter(|&x| primes::is_prime(x) && K.is_multiple_of(&x));
    candidates.max().unwrap().to_string()
}

timeit::timeit!(Problem03, solve, p);
