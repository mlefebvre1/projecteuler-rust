use num::traits::{One, Zero};
use num::{FromPrimitive, Num};

pub struct Octagonal<T> {
    n: T,
}

impl<T> Octagonal<T>
where
    T: Zero + One + Num + FromPrimitive,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: &T) -> T {
        // o(n) = n * (3*n - 2)
        let two: T = FromPrimitive::from_usize(2).unwrap();
        let three: T = FromPrimitive::from_usize(3).unwrap();

        n * &(&three * n - two)
    }
}

impl<T> Iterator for Octagonal<T>
where
    T: Clone + One + Num + FromPrimitive,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
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
fn test_octagonal_usize() {
    let serie = Octagonal::<usize>::new();
    let actual: Vec<usize> = serie.take(20).collect();
    assert_eq!(
        actual,
        [
            0, 1, 8, 21, 40, 65, 96, 133, 176, 225, 280, 341, 408, 481, 560, 645, 736, 833, 936,
            1045,
        ]
    );
}

#[test]
fn test_octagonal_biguint() {
    use num::BigUint;
    let actual: BigUint = Octagonal::<BigUint>::new().nth(20).unwrap();
    let expected = BigUint::from(1160usize);
    assert_eq!(actual, expected);

    let actual = Octagonal::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
    let expected = BigUint::parse_bytes(b"985495353293060399776555409215134686766401", 10).unwrap();
    assert_eq!(actual, expected);
}
