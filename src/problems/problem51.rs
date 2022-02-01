use crate::ntheory::primes::is_prime;
use crate::utils::integers::{int_to_vec_of_u8, vec_of_u8_to_int};
use crate::utils::timeit;
use bitvec::prelude::*;

fn p() -> String {
    /*
    Prime digit replacements
    Problem 51

    By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values:
     13, 23, 43, 53, 73, and 83, are all prime.

    By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having
    seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and
    56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

    Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same
    digit, is part of an eight prime value family.

    Candidates will be constructed like this :

    i: some number in a defined range. If i is 3 digits then i will be tested for 100 to 999
    x : replace all * on the number with the numbers 1 to 9
    j : the lat digit should be either 1,3,7 or 9

    i i i x j
    i i x i j
    i i x x j
    i x i i j
    i x i x j
    i x x i j
    i x x x j
    x i i i j
    x i i x j
    x i x i j
    x i x x j
    x x i i j
    x x i x j
    x x x i j
    x x x x j
    */
    const NB_DIGITS: usize = 6; // Nb digits is atleast > 5

    fn make_number(
        i_vec: &[u8],
        j: u8,
        x: u8,
        bitstring: &bitvec::slice::BitSlice<u8, bitvec::order::Msb0>,
    ) -> usize {
        let mut iindex = 0;
        let mut number = [0u8; NB_DIGITS];
        number[NB_DIGITS - 1] = j;
        for (index, bit) in bitstring.iter().enumerate() {
            if *bit {
                number[index] = x;
            } else {
                number[index] = i_vec[iindex];
                iindex += 1;
            }
        }
        vec_of_u8_to_int(&number)
    }

    const TARGET: usize = 8;

    let bitstrings = (1..31u8).map(BitVec::<_, Msb0>::from_element);
    for bitstring in bitstrings {
        let _bitstring = &bitstring[3..];
        let nb_zeroes = _bitstring.iter().fold(0, |acc, b| {
            if !b {
                return acc + 1;
            }
            acc
        });
        for i in match nb_zeroes {
            1 => Some(0usize..=9),
            2 => Some(10usize..=99),
            3 => Some(100usize..=999),
            4 => Some(1000usize..=9999),
            _ => None,
        }
        .unwrap()
        {
            let i_vec = int_to_vec_of_u8(i);
            for j in [1, 3, 7, 9] {
                let mut combinations = (0..=9).filter_map(|x| {
                    let n = make_number(&i_vec, j, x, _bitstring);
                    if is_prime(n) {
                        return Some(n);
                    }
                    None
                });
                let ans = combinations.next();
                // Here avoid collecting for every number, instead grab the first value and check TARGET-1
                if combinations.count() == TARGET - 1 {
                    return ans.unwrap().to_string();
                }
            }
        }
    }
    String::from("Answer not found..")
}

timeit::timeit!(Problem51, solve, p);
