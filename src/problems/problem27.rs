use crate::ntheory::primes::is_prime;
use crate::utils::timeit;
use itertools::Itertools;

fn quadratic_consecutive_primes(a: isize, b: isize) -> isize {
    let mut n = 0;
    loop {
        let s = num::pow(n, 2) + a * n + b;
        if s % 2 != 0 {
            if !is_prime(s) {
                break;
            }
        } else {
            break;
        }
        n += 1;
    }
    n
}

fn p() -> String {
    /*
    Quadratic primes
    Problem 27

    Euler discovered the remarkable quadratic formula:

            n^2 + n + 41

    It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39.
    However, when  n=40, 40^2 + 40 + 41 = 40(40+1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41
    is clearly divisible by 41.

    The incredible formula n^2 -79n +1601 was discovered, which produces 80 primes for the consecutive values
    0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.

    Considering quadratics of the form:

        n^2 + an + b, where |a|<1000 and |b|<1000

    where |n| is the modulus/absolute value of
    e.g. |11| = 11 and |-4| = 4

    Find the product of the coefficients, a nd b, for the quadratic expression that produces the maximum number of
    primes for consecutive values of n, starting with n=0.
    */
    const RANGE: std::ops::Range<isize> = -1000..1000;
    let candidates = RANGE.cartesian_product(RANGE);
    let quad_consecutive_primes =
        candidates.map(|(a, b)| (a, b, quadratic_consecutive_primes(a, b)));
    let (a, b, _) = quad_consecutive_primes.max_by_key(|k| k.2).unwrap();

    (a * b).to_string()
}

timeit::timeit!(Problem27, solve, p);
