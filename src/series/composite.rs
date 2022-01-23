use crate::ntheory::primes::is_prime;
use num::{Integer, NumCast, One};

pub struct OddComposite<T> {
    n: T,
}

impl<T> OddComposite<T>
where
    T: Integer + One + Copy + NumCast,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::one() }
    }
}

impl<T> Iterator for OddComposite<T>
where
    T: Integer + One + Copy + NumCast,
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let two = NumCast::from(2).unwrap();
        loop {
            self.n = self.n + two;
            if !is_prime(self.n) {
                break;
            }
        }
        Some(self.n)
    }
}

#[test]
fn test_composite_iter() {
    let actual: Vec<usize> = OddComposite::new().take(10).collect();
    let expected = [9, 15, 21, 25, 27, 33, 35, 39, 45, 49];
    assert_eq!(actual, expected);
}
