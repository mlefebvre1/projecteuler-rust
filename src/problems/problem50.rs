use crate::ntheory::primes::{is_prime, sieves};
use crate::utils::timeit;

fn p() -> String {
    /*
    Consecutive prime sum
    Problem 50

    The prime 41, can be written as the sum of six consecutive primes:
    41 = 2 + 3 + 5 + 7 + 11 + 13

    This is the longest sum of consecutive primes that adds to a prime below one-hundred.

    The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to
     953.

    Which prime, below one-million, can be written as the sum of the most consecutive primes?
    */
    const MAX_PRIME: usize = 100000;
    const TARGET: usize = 1e6 as usize;
    let primes = sieves(MAX_PRIME);
    let mut consecutive_prime_sum = primes.iter().enumerate().filter_map(|(i, pi)| {
        let mut sum = *pi;
        let mut nb_primes = 1;
        for pj in primes.iter().skip(i + 1) {
            sum += pj;
            nb_primes += 1;
            if sum > TARGET {
                sum -= pj; // remove the last prime that sum bust the target
                nb_primes -= 1;
                break;
            }
        }
        if is_prime(sum) {
            // make sure we've got a prime number before adding the sum
            return Some((nb_primes, sum));
        }
        None
    });
    let (_, max_prime_sum) = consecutive_prime_sum.next().unwrap();
    max_prime_sum.to_string()
}

timeit::timeit!(Problem50, solve, p);
