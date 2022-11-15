use crate::ntheory::pythagorean::{pythagorean_triples, PythagoreanTriple};
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Singular integer right triangles
    Problem 75

    It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided right angle
    triangle in exactly one way, but there are many more examples.

    12 cm: (3,4,5)
    24 cm: (6,8,10)
    30 cm: (5,12,13)
    36 cm: (9,12,15)
    40 cm: (8,15,17)
    48 cm: (12,16,20)

    In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right angle triangle, and
    other lengths allow more than one solution to be found; for example, using 120 cm it is possible to form exactly
    three different integer sided right angle triangles.

    120 cm: (30,40,50), (20,48,52), (24,45,51)

    Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one integer sided right
    angle triangle be formed?
    */
    const MAX_LEN: usize = 1.5e6 as usize;
    let triples = pythagorean_triples(MAX_LEN);
    let l_cnt = nb_repetitions_for_each_length(&triples, MAX_LEN);
    let l_cnt_singular = l_cnt.into_iter().filter(|&len| len == 1);
    Ok(l_cnt_singular.sum::<usize>().to_string())
}

fn nb_repetitions_for_each_length(triples: &[PythagoreanTriple], max_len: usize) -> Vec<usize> {
    // For each triple, get the sum of the triple and count how many time the sum is repeated
    let mut l_cnt = (0..=max_len).map(|_| 0).collect::<Vec<usize>>();
    for triple in triples {
        let triple_sum = triple.sum();
        if triple_sum < max_len {
            l_cnt[triple_sum] += 1;
        }
    }
    l_cnt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "161667");
    }
}
