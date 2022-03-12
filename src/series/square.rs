use num::{Num, One, Zero};

pub struct Square<T> {
    n: T,
}

impl<T> Square<T>
where
    T: One + Zero,
    for<'a> &'a T: std::ops::Mul<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: &T) -> T {
        // s(n) = n^2
        n * n
    }
}

impl<T> Iterator for Square<T>
where
    T: Clone + One + Num,
    for<'a> &'a T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let t = Self::from_n(&self.n);
        self.n = &self.n + &num::one();
        Some(t)
    }
}

#[test]
fn test_square_usize() {
    let serie = Square::<usize>::new();
    let actual: Vec<usize> = serie.take(20).collect();
    assert_eq!(
        actual,
        [0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361]
    );
}

#[test]
fn test_square_biguint() {
    use num::BigUint;
    let actual: BigUint = Square::<BigUint>::new().nth(20).unwrap();
    let expected = BigUint::from(400usize);
    assert_eq!(actual, expected);

    let actual = Square::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
    let expected = BigUint::parse_bytes(b"328498451097686799925900568301054106978201", 10).unwrap();
    assert_eq!(actual, expected);
}
