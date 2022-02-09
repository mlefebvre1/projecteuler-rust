use num::traits::{One, Zero};
use num::Num;

pub struct Pentagonal<T> {
    n: T,
}

impl<T> Pentagonal<T>
where
    T: Zero + One + Num,
    for<'a> &'a T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: &T) -> T {
        // n * (3 * n - 1) / 2
        let two = &num::one() + &num::one();
        let three = &two + &num::one();
        n * &(&(&three * n) - &num::one()) / two
    }
}

impl<T> Iterator for Pentagonal<T>
where
    T: Clone + One + Num,
    for<'a> &'a T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>,
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
fn test_pentagonal_usize() {
    let pentagonal_series = Pentagonal::<usize>::new();
    let actual: Vec<usize> = pentagonal_series.take(20).collect();
    assert_eq!(
        actual,
        [0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477, 532,]
    );
}

#[test]
fn test_pentagonal_biguint() {
    use num::BigUint;
    let actual: BigUint = Pentagonal::<BigUint>::new().nth(20).unwrap();
    let expected = BigUint::from(590usize);
    assert_eq!(actual, expected);

    let actual = Pentagonal::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
    let expected = BigUint::parse_bytes(b"492747676646530199888564278529574251925251", 10).unwrap();
    assert_eq!(actual, expected);
}

#[allow(dead_code)]
#[allow(clippy::many_single_char_names)]
pub fn is_pentagonal(n: usize) -> bool {
    let (a, b, c) = (3isize, -1isize, -(2 * n as isize));
    let discriminant = (b.pow(2) - (4 * a * c)) as f64;
    let x = (-b as f64 + discriminant.sqrt()) / (2 * a) as f64;
    if (x - (x as usize) as f64).abs() < f64::EPSILON {
        return true;
    }
    false
}

#[test]
fn test_is_pentagonal() {
    assert!(is_pentagonal(1));
    assert!(is_pentagonal(5));
    assert!(is_pentagonal(12));
    assert!(is_pentagonal(40755));
    assert!(!is_pentagonal(3));
}
