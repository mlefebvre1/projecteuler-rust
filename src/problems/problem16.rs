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
    return n
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>() as usize;
}

timeit::timeit!(Problem16, solve, p);
