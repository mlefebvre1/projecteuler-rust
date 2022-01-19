use num::Integer;
use std::fmt;

#[derive(Debug, PartialEq)]
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
            if n.gcd(&m) == 1 && (m != n) && (m + n).is_odd() {
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
    triples
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
    triples
}

#[test]
fn test_primitives_pythagorean_triples() {
    let max_h = 100usize;
    let mut actual = primitive_pythagorean_triples(max_h);
    actual.sort_by_key(|k| k.0);
    let expected = vec![
        PythagoreanTriple(3, 4, 5),
        PythagoreanTriple(5, 12, 13),
        PythagoreanTriple(7, 24, 25),
        PythagoreanTriple(8, 15, 17),
        PythagoreanTriple(9, 40, 41),
        PythagoreanTriple(11, 60, 61),
        PythagoreanTriple(12, 35, 37),
        PythagoreanTriple(13, 84, 85),
        PythagoreanTriple(15, 112, 113),
        PythagoreanTriple(16, 63, 65),
        PythagoreanTriple(17, 144, 145),
        PythagoreanTriple(20, 21, 29),
        PythagoreanTriple(28, 45, 53),
        PythagoreanTriple(33, 56, 65),
        PythagoreanTriple(36, 77, 85),
        PythagoreanTriple(39, 80, 89),
        PythagoreanTriple(48, 55, 73),
        PythagoreanTriple(65, 72, 97),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_pythagorean_triples() {
    let max_h = 100usize;
    let mut actual = pythagorean_triples(max_h);
    actual.sort_by_key(|k| k.0);
    let expected = vec![
        PythagoreanTriple(3, 4, 5),
        PythagoreanTriple(5, 12, 13),
        PythagoreanTriple(6, 8, 10),
        PythagoreanTriple(7, 24, 25),
        PythagoreanTriple(8, 15, 17),
        PythagoreanTriple(9, 12, 15),
        PythagoreanTriple(9, 40, 41),
        PythagoreanTriple(10, 24, 26),
        PythagoreanTriple(11, 60, 61),
        PythagoreanTriple(12, 16, 20),
        PythagoreanTriple(12, 35, 37),
        PythagoreanTriple(13, 84, 85),
        PythagoreanTriple(14, 48, 50),
        PythagoreanTriple(15, 20, 25),
        PythagoreanTriple(15, 36, 39),
        PythagoreanTriple(16, 30, 34),
        PythagoreanTriple(16, 63, 65),
        PythagoreanTriple(18, 24, 30),
        PythagoreanTriple(18, 80, 82),
        PythagoreanTriple(20, 48, 52),
        PythagoreanTriple(20, 21, 29),
        PythagoreanTriple(21, 28, 35),
        PythagoreanTriple(21, 72, 75),
        PythagoreanTriple(24, 32, 40),
        PythagoreanTriple(24, 45, 51),
        PythagoreanTriple(24, 70, 74),
        PythagoreanTriple(25, 60, 65),
        PythagoreanTriple(27, 36, 45),
        PythagoreanTriple(28, 96, 100),
        PythagoreanTriple(28, 45, 53),
        PythagoreanTriple(30, 40, 50),
        PythagoreanTriple(30, 72, 78),
        PythagoreanTriple(32, 60, 68),
        PythagoreanTriple(33, 44, 55),
        PythagoreanTriple(33, 56, 65),
        PythagoreanTriple(35, 84, 91),
        PythagoreanTriple(36, 48, 60),
        PythagoreanTriple(36, 77, 85),
        PythagoreanTriple(39, 52, 65),
        PythagoreanTriple(39, 80, 89),
        PythagoreanTriple(40, 75, 85),
        PythagoreanTriple(40, 42, 58),
        PythagoreanTriple(42, 56, 70),
        PythagoreanTriple(45, 60, 75),
        PythagoreanTriple(48, 64, 80),
        PythagoreanTriple(48, 55, 73),
        PythagoreanTriple(51, 68, 85),
        PythagoreanTriple(54, 72, 90),
        PythagoreanTriple(57, 76, 95),
        PythagoreanTriple(60, 80, 100),
        PythagoreanTriple(60, 63, 87),
        PythagoreanTriple(65, 72, 97),
    ];
    assert_eq!(actual, expected);
}
