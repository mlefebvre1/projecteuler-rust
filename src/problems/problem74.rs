use crate::utils::integers::int_to_vec_of_u8;
use crate::utils::timeit;
use factorial::Factorial;
use phf::phf_map;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Digit factorial chains
    Problem 74

    The number 145 is well known for the property that the sum of the factorial of its digits is equal to 145:

    1! + 4! + 5! = 1 + 24 + 120 = 145

    Perhaps less well known is 169, in that it produces the longest chain of numbers that link back to 169; it turns out
    that there are only three such loops that exist:

    169 → 363601 → 1454 → 169
    871 → 45361 → 871
    872 → 45362 → 872

    It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For example,

    69 → 363600 → 1454 → 169 → 363601 (→ 1454)
    78 → 45360 → 871 → 45361 (→ 871)
    540 → 145 (→ 145)

    Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating chain with a starting
    number below one million is sixty terms.

    How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?
    */
    const MAX_N: usize = 1e6 as usize;
    const NB_TERMS: usize = 60;
    let chain_map_size: usize = 6 * 9.factorial(); // worst case is the starting number 999999 -> 6 * 9!
    let mut chain_map = ChainMap::new(chain_map_size);
    Ok((0..MAX_N)
        .filter(|&n| chain_map.chain(n) == NB_TERMS)
        .count()
        .to_string())
}

static CHAIN_ENDING_VALUE: phf::Map<&'static str, usize> = phf_map! {
    "1"=> 1,
    "2"=> 1,
    "169"=> 3,
    "363601"=> 3,
    "1454"=> 3,
    "871"=> 2,
    "45361"=> 2,
    "872"=> 2,
    "45362"=> 2,
    "145"=> 0,
    "40585"=> 0,
};
const CHAIN_ENDING: [usize; 11] = [1, 2, 145, 169, 363601, 1454, 871, 45361, 872, 45362, 40585];

struct ChainMap {
    chain_map: Vec<usize>,
    factorials: Vec<usize>,
}

impl ChainMap {
    fn new(chain_map_size: usize) -> Self {
        let chain_map = (0..=chain_map_size).map(|_| 0).collect::<Vec<usize>>();
        let factorials = (0..10).map(|n| n.factorial()).collect::<Vec<usize>>();
        ChainMap {
            chain_map,
            factorials,
        }
    }

    fn chain(&mut self, n: usize) -> usize {
        let mut n_ = n;
        let cnt = (1..)
            .take_while(|_| {
                n_ = self.get_next_n(n_);
                !CHAIN_ENDING.contains(&n_)
            })
            .count();
        cnt + CHAIN_ENDING_VALUE[&n_.to_string()] + 1
    }

    fn get_next_n(&mut self, n: usize) -> usize {
        if self.chain_map[n] == 0 {
            let n_vec = int_to_vec_of_u8(&n);
            let next_n = n_vec
                .into_iter()
                .fold(0usize, |acc, digit| acc + self.factorials[digit as usize]);
            self.chain_map[n] = next_n;
            return next_n;
        }
        self.chain_map[n]
    }
}

timeit::timeit!(Problem74, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "402");
    }
}
