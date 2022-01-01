use utils::timeit;

fn p() -> usize {
    /*
    Multiples of 3 and 5
    Problem 1
    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these
    multiples is 23.

    Find the sum of all the multiples of 3 or 5 below 1000.
    */
    use std::collections::HashSet;

    let max_n = 1000;
    let mut multiples = HashSet::new();
    for multiplier in [3, 5].iter() {
        for n in (*multiplier..max_n).step_by(*multiplier) {
            multiples.insert(n);
        }
    }
    multiples.iter().sum()
}

timeit::timeit!(Problem01, solve, p);
