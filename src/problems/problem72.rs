use crate::utils::timeit;
use num::Integer;

fn p() -> String {
    /*
    Counting fractions
    Problem 72

    Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced
    proper fraction.

    If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:

    1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8

    It can be seen that there are 21 elements in this set.

    How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?

    The problem can be reformulated as find the sum of phi(n) : 2 <= n <= 1000000
    */
    const MAX_N: usize = 1e6 as usize;
    let distinct_primes_by_n = make_distinct_primes_by_n(MAX_N);
    let phi_by_n =
        distinct_primes_by_n
            .into_iter()
            .enumerate()
            .filter_map(|(n, distinct_primes)| {
                if n >= 2 {
                    let phi = distinct_primes.into_iter().fold(n as f64, |acc, prime| {
                        if n.is_multiple_of(&prime) {
                            return acc * (1.0 - 1.0 / prime as f64);
                        }
                        acc
                    });
                    return Some(phi as usize);
                }
                None
            });
    phi_by_n.sum::<usize>().to_string()
}

#[allow(clippy::needless_range_loop)]
fn make_distinct_primes_by_n(max_n: usize) -> Vec<Vec<usize>> {
    let mut sieved: Vec<Vec<usize>> = Vec::new();
    sieved.resize(max_n + 1, Vec::new());
    for n in 2..=max_n {
        if sieved[n].is_empty() {
            for x in ((n + n)..=max_n).step_by(n) {
                sieved[x].push(n);
            }
        }
    }
    for n in 2..=max_n {
        if sieved[n].is_empty() {
            sieved[n].push(n);
        }
    }
    sieved
}

timeit::timeit!(Problem72, solve, p);
