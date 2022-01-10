use crate::utils::timeit;

fn chain(n: usize) -> usize {
    let mut chain_len = 0usize;
    let mut n = n;
    while n != 1 {
        if n % 2 != 0 {
            // Skip a step since for an odd number : 3*n+1 always gives a even number
            n = (3 * n + 1) / 2;
            chain_len += 2
        } else {
            n /= 2;
            chain_len += 1
        }
    }
    chain_len
}

fn p() -> String {
    /*
    Longest Collatz sequence
    Problem 14

    The following iterative sequence is defined for the set of positive integers:

    n → n/2 (n is even)
    n → 3n + 1 (n is odd)

    Using the rule above and starting with 13, we generate the following sequence:
    13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

    It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been
    proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

    Which starting number, under one million, produces the longest chain?

    NOTE: Once the chain starts the terms are allowed to go above one million.
    */
    const END: usize = 1e6 as usize;
    // We don't need to check anything below half of the candidates since chain(2*n) = 1 + chain(n)
    const RANGE: std::ops::Range<usize> = (END / 2)..END;
    let chains = RANGE.map(chain).zip(RANGE);
    let (_, max_n) = chains.max().unwrap();
    max_n.to_string()
}

timeit::timeit!(Problem14, solve, p);
