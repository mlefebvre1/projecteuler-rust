use num::Integer;

pub fn factorize(n: usize) -> Vec<usize> {
    let mut factors = vec![1];
    if n < 2 {
        return factors;
    }
    let sqrt_n = (n as f64).sqrt();
    for divider in 2..sqrt_n.ceil() as usize {
        if n.is_multiple_of(&divider) {
            factors.push(divider);
            factors.push(n / divider);
        }
    }
    if n == (sqrt_n.floor() as usize * sqrt_n.floor() as usize) {
        factors.push(sqrt_n.floor() as usize);
    }
    factors.push(n);
    factors.sort_unstable();
    factors
}

pub fn proper_divisors_sum(n: usize) -> usize {
    let mut proper_div_sum = 1usize;
    if n < 2 {
        return proper_div_sum;
    }

    let sqrt_n = (n as f64).sqrt();
    for divider in 2..sqrt_n.ceil() as usize {
        if n.is_multiple_of(&divider) {
            // if it's divisible, we found 2 divisors at once!
            proper_div_sum += divider;
            proper_div_sum += n / divider;
        }
    }
    let sqrt_n_int = sqrt_n as usize;
    if (sqrt_n_int * sqrt_n_int) == n {
        proper_div_sum += sqrt_n_int;
    }
    proper_div_sum
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1,&[1])]
    #[case(2,&[1,2])]
    #[case(10, &[1, 2, 5, 10])]
    #[case(13, &[1, 13])]
    #[case(28, &[1, 2, 4, 7, 14, 28])]
    #[case(788, &[1, 2, 4, 197, 394, 788])]
    #[case(512, &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512])]
    fn test_factorize(#[case] n: usize, #[case] output: &[usize]) {
        assert_eq!(factorize(n), output);
    }

    #[rstest]
    #[case(2, 1)]
    #[case(10, 8)]
    #[case(13, 1)]
    #[case(28, 28)]
    #[case(788, 598)]
    #[case(512, 511)]
    fn test_proper_divisors_sum(#[case] input: usize, #[case] output: usize) {
        assert_eq!(proper_divisors_sum(input), output);
    }
}
