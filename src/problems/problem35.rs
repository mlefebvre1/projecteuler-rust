use crate::ntheory::primes::{is_prime, sieves};
use crate::utils::timeit;

struct DigitRotations {
    n: String,
}

impl DigitRotations {
    pub fn new(_n: &str) -> Self {
        Self {
            n: String::from(_n),
        }
    }
    fn rotation(_n: &str) -> String {
        let mut digits: Vec<char> = _n.chars().collect();
        let nb_digits = digits.len();
        for i in 0..(nb_digits - 1) {
            digits.swap(i, i + 1);
        }
        digits.into_iter().collect::<String>()
    }
}

impl Iterator for DigitRotations {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let _n = self.n.clone();
        self.n = Self::rotation(&self.n);
        Some(_n)
    }
}

#[test]
fn rotation_test() {
    let n = "907";
    assert_eq!(
        DigitRotations::new(n)
            .take(n.chars().count())
            .collect::<Vec<String>>(),
        ["907", "079", "790"]
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
    fn get_all_rotations(n: &str) -> std::iter::Take<DigitRotations> {
        let nb_rotations = n.chars().count();
        DigitRotations::new(n).take(nb_rotations)
    }
    fn is_circular(n: &str) -> bool {
        let rotations = get_all_rotations(n);
        let rotations_not_prime = rotations.filter(|x| !is_prime(x.parse::<usize>().unwrap()));
        rotations_not_prime.count() == 0
    }

    const MAX_PRIME: usize = 1e6 as usize;
    let primes = sieves(MAX_PRIME);
    let circulars = primes.iter().filter(|n| is_circular(&(n.to_string())));
    let nb_circulars = circulars.count();
    nb_circulars.to_string()
}

timeit::timeit!(Problem35, solve, p);
