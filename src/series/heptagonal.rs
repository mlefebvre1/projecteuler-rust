use num::traits::{One, Zero};
use num::{FromPrimitive, Num};

pub struct Heptagonal<T> {
    n: T,
}

impl<T> Heptagonal<T>
where
    T: Zero + One + Num + FromPrimitive,
    for<'a> &'a T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: &T) -> T {
        // h(n) = n * (5*n - 3) / 2
        let two: T = FromPrimitive::from_usize(2).unwrap();
        let three: T = FromPrimitive::from_usize(3).unwrap();
        let five: T = FromPrimitive::from_usize(5).unwrap();
        n * &((&five * n) - three) / two
    }
}

impl<T> Iterator for Heptagonal<T>
where
    T: Clone + One + Num + FromPrimitive,
    for<'a> &'a T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
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
fn test_heptagonal_usize() {
    let serie = Heptagonal::<usize>::new();
    let actual: Vec<usize> = serie.take(20).collect();
    assert_eq!(
        actual,
        [
            0, 1, 7, 18, 34, 55, 81, 112, 148, 189, 235, 286, 342, 403, 469, 540, 616, 697, 783,
            874,
        ]
    );
}

#[test]
fn test_heptagonal_biguint() {
    use num::BigUint;
    let actual: BigUint = Heptagonal::<BigUint>::new().nth(20).unwrap();
    let expected = BigUint::from(970usize);
    assert_eq!(actual, expected);

    let actual = Heptagonal::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
    let expected = BigUint::parse_bytes(b"821246127744216999813891698986614541819351", 10).unwrap();
    assert_eq!(actual, expected);
}
