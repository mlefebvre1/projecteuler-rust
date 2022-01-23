use crate::ntheory::primes::sieves;
use crate::series::composite::OddComposite;
use crate::utils::timeit;

fn p() -> String {
    /*
    Goldbach's other conjecture
    Problem 46

    It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and
    twice a square.

        9 = 7 + 2×1^2
        15 = 7 + 2×2^2
        21 = 3 + 2×3^2
        25 = 7 + 2×3^2
        27 = 19 + 2×2^2
        33 = 31 + 2×1^2

    It turns out that the conjecture was false.

    What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
    */
    const MAX_N: usize = 10000;

    let square_times_2_vec: Vec<usize> = (1..)
        .map(|n| 2 * n * n)
        .take_while(|&s| s < MAX_N)
        .collect();
    let primes = sieves(MAX_N);
    let mut odd_composites = OddComposite::<usize>::new();

    odd_composites
        .find(|&composite| {
            for prime in primes.iter() {
                for square_times2 in square_times_2_vec.iter() {
                    if prime + square_times2 == composite {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()
        .to_string()
}

timeit::timeit!(Problem46, solve, p);
