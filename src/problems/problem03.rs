use ntheory::primes;
use utils::timeit;

fn p() -> i32 {
    /*
    Largest prime factor
    Problem 3
    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
    */
    const K: usize = 600851475143;
    let numbers = 1..(K as f64).sqrt() as usize;
    let candidates = numbers.filter(|x| primes::is_prime(*x) && (K % x == 0));
    return candidates.max().unwrap() as i32;
}

timeit::timeit!(Problem03, solve, p);
