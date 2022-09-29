use crate::ntheory::primes;
use crate::utils::timeit;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    Smallest multiple
    Problem 5
    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

    Solution : Find the prime numbers up to 20. For each prime number, use the distinct prime numbers of each number
    from 1 to 20 to find the greatest number of occurences. Finally, the smallest positive number is the multiplication
    of all the primes numbers up to 20 with their greatest occurance.
    */
    const MAX_N: usize = 20;
    let primes = primes::sieves(MAX_N);
    let prime_occurences = primes
        .iter()
        .map(|&x| (MAX_N as f64).log(x as f64).floor() as usize);
    let primes_and_occurences = primes.iter().zip(prime_occurences);
    Ok(primes_and_occurences
        .fold(1, |acc, (prime, occurence)| {
            acc * prime.pow(occurence as u32)
        })
        .to_string())
}

timeit::timeit!(Problem05, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "232792560");
    }
}
