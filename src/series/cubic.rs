use num::{Num, One, Zero};

pub struct Cubic<T> {
    n: T,
}

impl<T> Cubic<T>
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
        // f(n) = n^3
        &(n * n) * n
    }
}

impl<T> Iterator for Cubic<T>
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cubic_usize() {
        let serie = Cubic::<usize>::new();
        let actual: Vec<usize> = serie.take(20).collect();
        assert_eq!(
            actual,
            [
                0, 1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096,
                4913, 5832, 6859,
            ]
        );
    }

    #[test]
    fn test_cubic_biguint() {
        use num::BigUint;
        let actual: BigUint = Cubic::<BigUint>::new().nth(20).unwrap();
        let expected = BigUint::from(8000usize);
        assert_eq!(actual, expected);

        let actual = Cubic::from_n(&BigUint::parse_bytes(b"573147844013817084101", 10).unwrap());
        let expected = BigUint::parse_bytes(
            b"188278179008517513490654244761257633062280617431713254190682301",
            10,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
}
