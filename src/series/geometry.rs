use num::traits::{One, Zero};
use num::Num;

pub struct Triangular<T> {
    n: T,
}

impl<T> Triangular<T>
where
    T: Zero + One + Num,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { n: num::zero() }
    }

    #[allow(dead_code)]
    pub fn from_n(n: T) -> T {
        &n * &(&n + &num::one()) / (&num::one() + &num::one())
    }
}

#[allow(clippy::just_underscores_and_digits)]
impl<T> Iterator for Triangular<T>
where
    T: Clone + One + Num,
    for<'a> &'a T:
        std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // t(n) = n * (n + 1) / 2
        let t = &self.n * &(&self.n + &num::one()) / (&num::one() + &num::one());
        self.n = &self.n + &num::one();
        Some(t)
    }
}

#[test]
fn test_triangular_usize() {
    let triangular_serie = Triangular::<usize>::new();
    let actual: Vec<usize> = triangular_serie.take(20).collect();
    assert_eq!(
        actual,
        [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190]
    );
}

#[test]
fn test_triangular_biguint() {
    use num::BigUint;
    let mut triangular_serie = Triangular::<BigUint>::new();
    let actual: BigUint = triangular_serie.nth(20).unwrap();
    assert_eq!(actual, BigUint::from(210usize));
    assert_eq!(
        Triangular::from_n(BigUint::parse_bytes(b"573147844013817084101", 10).unwrap()),
        BigUint::parse_bytes(b"164249225548843399963236858072533962031151", 10).unwrap()
    );
}
