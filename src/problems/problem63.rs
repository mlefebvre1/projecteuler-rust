use crate::utils::integers::int_to_vec_of_u8;
use crate::utils::timeit;
use num::BigUint;

fn nb_digit_power_match(power: usize) -> usize {
    (1..10usize)
        .filter(|&digit| {
            let n = BigUint::from(digit).pow(power as u32);
            if int_to_vec_of_u8(&n).len() == power {
                return true;
            }
            false
        })
        .count()
}

fn p() -> String {
    /*
    Powerful digit counts
    Problem 63

    The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number, 134217728=8^9, is a ninth
    power.

    How many n-digit positive integers exist which are also an nth power?
    */
    (1usize..)
        .map_while(|power| {
            let max_value = BigUint::from(9usize).pow(power as u32);
            if int_to_vec_of_u8(&max_value).len() != power {
                return None;
            }
            Some(nb_digit_power_match(power))
        })
        .sum::<usize>()
        .to_string()
}

timeit::timeit!(Problem63, solve, p);
