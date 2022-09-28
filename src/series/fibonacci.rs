use num::traits::{One, Zero};

pub struct Fibonacci<T> {
    a: T,
    b: T,
}

impl<T> Fibonacci<T>
where
    T: Zero + One,
{
    pub fn new() -> Self {
        Self {
            a: num::zero(),
            b: num::one(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Clone,
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        std::mem::swap(&mut self.a, &mut self.b);
        self.b = &self.b + &self.a;
        Some(self.a.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_usize() {
        let fib = Fibonacci::<usize>::new();
        let actual: Vec<usize> = fib.take(20).collect();
        assert_eq!(
            actual,
            [
                1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
                6765
            ]
        );
    }

    #[test]
    fn test_fibonacci_biguint() {
        use num::BigUint;
        let mut fib = Fibonacci::<BigUint>::new();
        let actual = fib.nth(100).unwrap();
        let expected = BigUint::parse_bytes(b"573147844013817084101", 10).unwrap();
        assert_eq!(actual, expected);
    }
}
