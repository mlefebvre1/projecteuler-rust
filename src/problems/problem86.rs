use crate::ntheory::pythagorean::{pythagorean_triples, PythagoreanTriple};
use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Cuboid route
    Problem 86
    A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits in the opposite corner.
    By travelling on the surfaces of the room the shortest "straight line" distance from S to F is 10 and the path is
    shown on the diagram.
    However, there are up to three "shortest" path candidates for any given cuboid and the shortest route doesn't always
    have integer length.
    It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer dimensions, up to a
    maximum size of M by M by M, for which the shortest route has integer length when M = 100. This is the least value
    of M for which the number of solutions first exceeds two thousand; the number of solutions when M = 99 is 1975.
    Find the least value of M such that the number of solutions first exceeds one million.
    */
    const TRIPLET_MAX: usize = 4000;
    let mut prev = 0;
    let mut triplets = all_pythagorean_splits(TRIPLET_MAX);
    triplets.sort_unstable();
    for (nb, solution) in triplets.into_iter().enumerate() {
        if prev != solution {
            if nb > 1e6 as usize {
                break;
            }
            prev = solution;
        }
    }
    Ok(prev.to_string())
}

fn all_pythagorean_splits(m: usize) -> Vec<usize> {
    let mut triples = pythagorean_triples(m);
    triples.sort_by_key(|x| x.0);
    let mut v = Vec::new();
    for triple in triples {
        v.push(pythagorean_splits(&triple));
    }
    v.into_iter().flatten().collect::<Vec<usize>>()
}

fn pythagorean_splits(triple: &PythagoreanTriple) -> Vec<usize> {
    let PythagoreanTriple(a, b, _) = triple;
    let mut v = Vec::new();
    for n1 in 1..*a {
        let n2 = b - n1;
        if n1 > n2 {
            break;
        }
        if n2 <= *a {
            v.push(*a);
        }
        if n1 == n2 {
            break;
        }
    }

    for n1 in 1..*b {
        let n2 = a - n1;
        if n1 > n2 {
            break;
        }
        if n2 <= *a {
            v.push(*b);
        }
        if n1 == n2 {
            break;
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1818");
    }
}
