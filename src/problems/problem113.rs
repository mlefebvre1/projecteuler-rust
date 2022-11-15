use anyhow::Result;

fn p() -> Result<String> {
    /*
    Non-bouncy numbers
    Problem 113

    Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number; for
    example, 134468.

    Similarly if no digit is exceeded by the digit to its right it is called a decreasing number; for example, 66420.

    We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number; for example, 155349.

    As n increases, the proportion of bouncy numbers below n increases such that there are only 12951 numbers below
    one-million that are not bouncy and only 277032 non-bouncy numbers below 1010.

    How many numbers below a googol (10^100) are not bouncy?

    To solve this problem, we generate the number of increasing, decreasing numbers ans print the total for each
    10**n up to n=5

            increment   decrement    flat   total
    10:         0           0         9       9
    100:        45          45        9       99
    1000:       210         255       9       474
    10000:      705         960       9       1674
    100000:     1992        2952      9       4953

    From this table we can see that we can generate decrement numbers using the previous decrement + the current
    increment ex: 255 = 210 + 45. So the goal will be to generate quickly increment numbers and than we can generate
    solutions very quickly.

    To generate increments quickly, one can observe the following table :
                exponents ...
                1       2      3
        1       1       9      45
        2       1       8      36
    d   3       1       7      28
    i   4       1       6      21
    g   5       1       5      15
    i   6       1       4      10
    t   7       1       3      6
        8       1       2      3
        9       1       1      1

    for each leading number, and for each exponent we can extract the relation that the leading digit 1 will generate
    the same amount of number than total increasing number of the previous exponent. ex: for exponent 3 and leading
    digit 1, we get 45 which is 9+8+7+6+5+4+3+2+1, for leading digit 2 we get : 36 which is 8+7+6+5+4+3+2+1, and we
    repeat this up to leading digit 9. Once we have completed that simple algorithm, the number of increasing numbers
    is simply the sum of the new table (for exponent 3 : 45+36+28+21+15+10+6+3+1)
    */
    const MAX_EXPONENT: u64 = 100;

    let mut incresing_numbers = [1; 10];
    let (mut total_non_bouncy, mut total_increasing, mut last_decreasing) = (0_u64, 0_u64, 0_u64);
    for _exponent in 2..=MAX_EXPONENT {
        for base in 1..9 {
            incresing_numbers[base] = incresing_numbers[base..].iter().sum();
        }
        total_increasing += incresing_numbers[1..].iter().sum::<u64>();
        total_non_bouncy = 2 * total_increasing + last_decreasing + 9;
        last_decreasing += total_increasing;
    }
    Ok(total_non_bouncy.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "51161058134250");
    }
}
