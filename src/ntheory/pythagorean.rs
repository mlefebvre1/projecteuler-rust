use crate::ntheory::primes;
use std::fmt;

pub struct PythagoreanTriple(usize, usize, usize);

impl PythagoreanTriple {
    pub fn sum(&self) -> usize {
        self.0 + self.1 + self.2
    }
    pub fn prod(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

impl fmt::Display for PythagoreanTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

pub fn primitive_pythagorean_triples(max_h: usize) -> Vec<PythagoreanTriple> {
    let end: usize = (max_h as f64).sqrt().ceil() as usize;
    let mut triples = Vec::new();
    for n in 1..end {
        for m in n..end {
            if primes::gcd(n, m) == 1 && (m != n) && ((m + n) % 2 != 0) {
                let mut a = m * m - n * n;
                let mut b = 2 * m * n;
                if b < a {
                    std::mem::swap(&mut a, &mut b);
                }
                let c = m * m + n * n;
                triples.push(PythagoreanTriple(a, b, c));
            }
        }
    }
    return triples;
}

pub fn pythagorean_triples(max_h: usize) -> Vec<PythagoreanTriple> {
    let mut triples = Vec::new();
    let mut primitive_triples = primitive_pythagorean_triples(max_h);
    primitive_triples.sort_by_key(|k| k.0);
    for primitive_triple in primitive_triples {
        let mut m = 1;
        loop {
            let triple_m = PythagoreanTriple(
                primitive_triple.0 * m,
                primitive_triple.1 * m,
                primitive_triple.2 * m,
            );
            if triple_m.2 > max_h {
                break;
            }
            triples.push(triple_m);
            m += 1;
        }
    }
    return triples;
}
