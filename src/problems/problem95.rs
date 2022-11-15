use anyhow::Result;
use num::Integer;

pub fn p() -> Result<String> {
    /*
    Amicable chains
    Problem 95

    The proper divisors of a number are all the divisors excluding the number itself. For example, the proper divisors
    of 28 are 1, 2, 4, 7, and 14. As the sum of these divisors is equal to 28, we call it a perfect number.

    Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors of 284 is 220,
    forming a chain of two numbers. For this reason, 220 and 284 are called an amicable pair.

    Perhaps less well known are longer chains. For example, starting with 12496, we form a chain of five numbers:

    12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)

    Since this chain returns to its starting point, it is called an amicable chain.

    Find the smallest member of the longest amicable chain with no element exceeding one million.

    Solution : First we find the sum proper divisor for each entry. Using this we can layout the chains without
               calculating twice or more the same sum proper divisor. By observation we only check even numbers
               because no amicable chain exists starting with odd numbers. The chain is not amicable if a number
               in the chain repeats but it's not the starting number. The chain is also considered invalid if any
               number of the chain is greater than 1e6.
    */
    const MAX_N: usize = 1e6 as usize;
    let proper_divisor_sums = get_proper_divisor_sums(MAX_N);
    let chains = make_chains(MAX_N, &proper_divisor_sums);
    let (n, _) = chains.iter().rev().max_by_key(|k| k.1).unwrap();
    Ok(n.to_string())
}

fn get_proper_divisor_sums(max_n: usize) -> Vec<usize> {
    /*
    Basically a modified sieves, but instead of marking true for visited,
    we add the value of the prime visiting
    */
    let mut sums: Vec<usize> = (0..=max_n).map(|_| 0).collect();
    for n in 1..=max_n {
        for x in ((n + n)..=max_n).step_by(n) {
            sums[x] += n;
        }
    }
    sums
}

fn make_chains(max_n: usize, div_sums: &[usize]) -> Vec<(usize, usize)> {
    let mut chain_starting_n_and_len = Vec::new();
    for n in 0..=max_n {
        if n.is_even() {
            let mut current = n;
            let mut cur_chain = vec![n];
            loop {
                let div_sum = div_sums[current];
                if div_sum > max_n {
                    // Not amicable with all under 1M in the chain
                    chain_starting_n_and_len.push((n, 0));
                    break;
                }
                if cur_chain.contains(&div_sum) {
                    if div_sum == n {
                        chain_starting_n_and_len.push((n, cur_chain.len()))
                    } else {
                        chain_starting_n_and_len.push((n, 0));
                    }
                    break;
                }
                cur_chain.push(div_sum);
                current = div_sum;
            }
        }
    }
    chain_starting_n_and_len
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "14316");
    }
}
