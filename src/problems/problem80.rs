use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;
use num::BigUint;

fn p() -> Result<String> {
    /*
    Square root digital expansion
    Problem 80

    It is well known that if the square root of a natural number is not an integer, then it is irrational. The decimal
    expansion of such square roots is infinite without any repeating pattern at all.

    The square root of two is 1.41421356237309504880..., and the digital sum of the first one hundred decimal digits is
    475.

    For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal
    digits for all the irrational square roots.

    Solution : First we find all rational sqrt ( [1*1, 2*2, 3*3, ...]. Afterward for each irrational sqrt
    we do the sqrt using a special function that use integers to find the sqrt up to n digits by avoiding floating point
    because the precision is not good enough for 100 digits. It's important to note that the digit sum actually includes
    left and right digits from the comma. My initial error was to take only digits at the right of the comma.
    */
    const NB_DIGITS: usize = 100;
    const NB_NUMBERS: usize = 100;
    let nb_numbers_sqrt: usize = (NB_NUMBERS as f64).sqrt().floor() as usize;
    let exclude = (1..nb_numbers_sqrt).map(|i| i * i).collect::<Vec<usize>>();
    let numbers = (1..NB_NUMBERS).filter(|n| !exclude.contains(n));
    Ok(numbers
        .map(|n| {
            let mut sqrt_digit_expansion = SqrtDigitExpansion::new(NB_DIGITS);
            let digits = sqrt_digit_expansion.calculate(n);
            digits.into_iter().map(|d| d as usize).sum::<usize>()
        })
        .sum::<usize>()
        .to_string())
}

struct SqrtDigitExpansion {
    nb_digits: usize,
    n: usize,
    r: BigUint,
    ans: Vec<u8>,
}

impl SqrtDigitExpansion {
    pub fn new(nb_digits: usize) -> Self {
        SqrtDigitExpansion {
            nb_digits,
            n: 0,
            r: BigUint::from(0usize),
            ans: Vec::new(),
        }
    }

    pub fn calculate(&mut self, n: usize) -> Vec<u8> {
        self.n = n;
        self.r = BigUint::from(n * 100);
        self.ans
            .append(&mut int_to_vec_of_u8(&self.find_first_digit()));
        for _ in 1..(self.nb_digits) {
            self.generate_next_digit();
            self.r *= BigUint::from(100usize);
        }
        self.ans.clone()
    }

    fn find_first_digit(&self) -> usize {
        for i in 1..=self.n {
            if i * i > self.n {
                return i - 1;
            }
        }
        panic!()
    }

    fn generate_next_digit(&mut self) {
        for digit in 1..=9 {
            self.ans.push(digit);
            let last_index = self.ans.len() - 1;
            let n = BigUint::from_radix_be(&self.ans, 10).unwrap();
            if (&n * &n) > self.r {
                self.ans[last_index] = digit - 1;
                return;
            }
            self.ans.remove(last_index);
        }
        self.ans.push(9u8);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "40886");
    }
}
