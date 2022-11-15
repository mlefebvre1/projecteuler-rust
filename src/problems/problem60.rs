use crate::ntheory::primes::{is_prime, sieves};
use crate::utils::integers::{int_to_vec_of_u8, vec_of_u8_to_int};
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Prime pair sets
    Problem 60
    The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in any
    order the result will always be prime. For example, taking 7 and 109, both 7109 and 1097 are prime. The sum
    of these four primes, 792, represents the lowest sum for a set of four primes with this property.

    Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
    */
    const MAX_N: usize = 10000;
    let primes = sieves(MAX_N);
    for prime in primes.iter() {
        let ans = find_5_concat_primes(*prime, &primes, 1);
        if ans != 0 {
            return Ok(ans.to_string());
        }
    }
    panic!();
}

fn find_5_concat_primes(prime: usize, primes: &[usize], nb_primes: usize) -> usize {
    const TARGET: usize = 5;
    /*
        For a given prime, test each prime up to 10000 if the property of concatenation and generate a new primes holds.
        Keep every primes that holds the property. Pass that new list and the next prime in that new list to this function
        again. Keep doing this recursively until we get a sequence of 5 primes which holds the property. At that point
        de-stack and return the value of each prime in the list.
    */
    if nb_primes >= TARGET {
        // found five primes!!, start un-stacking the primes!
        return prime;
    }
    let prime_pair_and_is_prime: Vec<usize> = primes
        .iter()
        .filter(|&prime2| {
            let prime1_vec = int_to_vec_of_u8(&prime);
            let prime2_vec = int_to_vec_of_u8(prime2);
            let new_prime12_vec: Vec<u8> = prime1_vec
                .iter()
                .chain(prime2_vec.iter())
                .cloned()
                .collect();
            let new_prime21_vec: Vec<u8> = prime2_vec
                .iter()
                .chain(prime1_vec.iter())
                .cloned()
                .collect();
            let new_prime12: usize = vec_of_u8_to_int(&new_prime12_vec);
            let new_prime21: usize = vec_of_u8_to_int(&new_prime21_vec);
            is_prime(new_prime12) && is_prime(new_prime21)
        })
        .cloned()
        .collect();
    for prime2 in prime_pair_and_is_prime.iter() {
        let ans = find_5_concat_primes(*prime2, &prime_pair_and_is_prime, nb_primes + 1);
        if ans != 0 {
            return ans + prime;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "26033");
    }
}
