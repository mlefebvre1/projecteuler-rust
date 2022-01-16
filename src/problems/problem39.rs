use crate::ntheory::pythagorean::pythagorean_triples;
use crate::utils::timeit;

fn p() -> String {
    /*
    Integer right triangles
    Problem 39
    If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three
    solutions for p = 120.

    For which value of p ≤ 1000, is the number of solutions maximised?
    */
    const MAX_P: usize = 1000;
    let triples = pythagorean_triples(MAX_P);
    let perimeters_under_max_p = triples.iter().filter_map(|triple| {
        let sum = triple.sum();
        match sum {
            p if p <= 1000 => Some(p),
            _ => None,
        }
    });

    // An HashMap would have been cleaner here, but for performance we reason we prefer an array.
    // The array is faster because the size is small and known at compile time thus will be allocated
    // on the stack instead of the heap (in the case of an HashMap)
    let mut perimeters = [0usize; MAX_P + 1];
    for p in perimeters_under_max_p {
        perimeters[p] += 1;
    }
    let (max_index, _) = perimeters.iter().enumerate().max_by_key(|p| p.1).unwrap();
    max_index.to_string()
}

timeit::timeit!(Problem39, solve, p);
