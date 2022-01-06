use crate::utils::timeit;
use num_bigint::BigUint;

fn p() -> usize {
    /*
    Power digit sum
    Problem 16

    2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

    What is the sum of the digits of the number 2^1000?
    */
    let n: BigUint = BigUint::from(2usize).pow(1000);
    n.to_radix_be(10).iter().map(|x| *x as usize).sum::<usize>()
}

timeit::timeit!(Problem16, solve, p);
