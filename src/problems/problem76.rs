use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Counting summations
    Problem 76

    It is possible to write five as a sum in exactly six different ways:

    4 + 1
    3 + 2
    3 + 1 + 1
    2 + 2 + 1
    2 + 1 + 1 + 1
    1 + 1 + 1 + 1 + 1

    How many different ways can one hundred be written as a sum of at least two positive integers?

    Same strategy as problem 31, use dynamic programming, it can be seen that the current nnb of ways will be
    the current nb of ways + the nb of ways of closest multiple current multiple i
    */
    const MAX_N: usize = 100;
    let mut nb_ways = [0usize; MAX_N + 1];
    nb_ways[0] = 1;
    let pairs = (1..MAX_N).flat_map(|multiple| (multiple..=MAX_N).map(move |n| (multiple, n)));
    for (multiple, n) in pairs {
        nb_ways[n] += nb_ways[n - multiple];
    }
    Ok(nb_ways[MAX_N].to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "190569291");
    }
}
