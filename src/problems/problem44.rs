use crate::series::pentagonal::Pentagonal;
use crate::utils::timeit;

fn p() -> String {
    /*
    Pentagon numbers
    Problem 44

    Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2. The first ten pentagonal numbers are:

    1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...

    It can be seen that P4 + P7 = 22 + 70 = 92 = P8. However, their difference, 70 − 22 = 48, is not pentagonal.

    Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal and D = |Pk − Pj|
    is minimised; what is the value of D?
    */

    fn pentagonal_cache(max_n: usize) -> Vec<bool> {
        let pentagonals = Pentagonal::<usize>::new().take(max_n);
        let mut cache = vec![false; Pentagonal::from_n(&max_n)];
        for p in pentagonals {
            cache[p] = true;
        }
        cache
    }

    #[allow(clippy::collapsible_if)]
    fn is_sum_and_diff_pentagonal(s: usize, d: usize, cache: &[bool]) -> bool {
        let cache_size = cache.len();
        if d < cache_size && s < cache_size {
            if cache[d] && cache[s] {
                return true;
            }
        }
        false
    }

    const MAX_N: usize = 2500;
    let cache = pentagonal_cache(MAX_N);
    let pentagonals: Vec<usize> = Pentagonal::new().take(MAX_N).collect();

    let minimal_pentagon: usize = {
        let mut result = 0;
        for (j, pj) in pentagonals.iter().enumerate() {
            for pk in &pentagonals[j + 1..] {
                let d = pk - pj;
                let s = pk + pj;
                if is_sum_and_diff_pentagonal(s, d, &cache) {
                    result = d;
                    break;
                }
            }
        }
        result
    };
    minimal_pentagon.to_string()
}

timeit::timeit!(Problem44, solve, p);