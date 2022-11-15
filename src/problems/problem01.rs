use anyhow::Result;
use num::Integer;

fn p() -> Result<String> {
    /*
    Multiples of 3 and 5
    Problem 1
    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these
    multiples is 23.

    Find the sum of all the multiples of 3 or 5 below 1000.
    */
    const MAX_N: usize = 1000;
    Ok((1..MAX_N)
        .filter(|n| n.is_multiple_of(&3) || n.is_multiple_of(&5))
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "233168");
    }
}
