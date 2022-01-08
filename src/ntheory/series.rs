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
