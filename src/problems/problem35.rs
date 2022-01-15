use crate::ntheory::primes::{is_prime, sieves};
use crate::utils::timeit;

struct DigitRotations {
    n: Vec<char>,
}

impl DigitRotations {
    pub fn new(mut _n: &[char]) -> Self {
        Self { n: _n.to_vec() }
    }
    fn rotation(_n: &mut Vec<char>) {
        for i in 0..(_n.len() - 1) {
            _n.swap(i, i + 1);
        }
    }
}

impl Iterator for DigitRotations {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Vec<char>> {
        let _n = self.n.clone();
        Self::rotation(&mut self.n);
        Some(_n)
    }
}

#[test]
fn rotation_test() {
    let n = vec!['9', '0', '7'];
    let rotations: Vec<Vec<char>> = DigitRotations::new(&n).take(n.len()).collect();
    assert_eq!(
        rotations,
        [['9', '0', '7'], ['0', '7', '9'], ['7', '9', '0']]
    )
}

fn p() -> String {
    /*
    Circular primes
    Problem 35

    The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves
    prime.

    There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

    How many circular primes are there below one million?
    */
    fn get_all_rotations(n: usize) -> std::iter::Take<DigitRotations> {
        let _n: Vec<char> = n.to_string().chars().collect();
        DigitRotations::new(&_n).take(_n.len())
    }
    fn is_circular(n: usize) -> bool {
        let rotations = get_all_rotations(n).map(|r| r.into_iter().collect::<String>());
        let rotations_not_prime = rotations.filter(|x| !is_prime(x.parse::<usize>().unwrap()));
        rotations_not_prime.count() == 0
    }

    const MAX_PRIME: usize = 1e6 as usize;
    let primes = sieves(MAX_PRIME);
    let circulars = primes.iter().filter(|&n| is_circular(*n));
    let nb_circulars = circulars.count();
    nb_circulars.to_string()
}

timeit::timeit!(Problem35, solve, p);
