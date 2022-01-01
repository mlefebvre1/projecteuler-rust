pub struct Fibonacci {
    a: usize,
    b: usize,
    max: usize,
}

impl Fibonacci {
    pub fn new(max: usize) -> Self {
        Self {
            a: 0,
            b: 1,
            max: max,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let fib = self.a + self.b;
        self.a = self.b;
        self.b = fib;
        if fib >= self.max {
            None
        } else {
            Some(fib)
        }
    }
}
