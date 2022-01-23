use num::traits::{One, Zero};
use num::Num;

pub struct Hexagonal<T> {
    n: T,
}

impl<T> Hexagonal<T>
where
    T: Zero + One + Num,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: &T) -> T {
        // n * (2 * n - 1)
        let two = &num::one() + &num::one();
        n * &(&(&two * n) - &num::one())
    }
}

impl<T> Iterator for Hexagonal<T>
where
    T: Clone + One + Num,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let t = match &self.n {
            n if n == &num::zero() => num::zero(),
            n => Self::from_n(n),
        };
        self.n = &self.n + &num::one();
        Some(t)
    }
}

#[test]
fn test_hexagonal_usize() {
    let hexagonal_series = Hexagonal::<usize>::new();
    let actual: Vec<usize> = hexagonal_series.take(20).collect();
    assert_eq!(
        actual,
        [
            0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630,
            703,
        ]
    );
}

#[test]
fn test_hexagonal_biguint() {
    use num::BigUint;
    let actual: BigUint = Hexagonal::<BigUint>::new().nth(20).unwrap();
    let expected = BigUint::from(780usize);
    assert_eq!(actual, expected);

    let actual = Hexagonal::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
    let expected = BigUint::parse_bytes(b"656996902195373599851227988758094396872301", 10).unwrap();
    assert_eq!(actual, expected);
}

#[allow(dead_code)]
pub fn is_hexagonal(n: usize) -> bool {
    let (a, b, c) = (2isize, -1isize, -(n as isize));
    let discriminant = (b.pow(2) - (4 * a * c)) as f64;
    let x = (-b as f64 + discriminant.sqrt()) / (2 * a) as f64;
    if x == (x as usize) as f64 {
        return true;
    }
    false
}

#[test]
fn test_is_hexagonal() {
    assert!(is_hexagonal(1));
    assert!(is_hexagonal(6));
    assert!(is_hexagonal(15));
    assert!(is_hexagonal(40755));
    assert!(!is_hexagonal(3));
}
