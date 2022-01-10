pub fn factorize(n: usize) -> Vec<usize> {
    let mut factors = vec![1];
    if n < 2 {
        return factors;
    }
    let sqrt_n = (n as f64).sqrt();
    for divider in 2..sqrt_n.ceil() as usize {
        if n % divider == 0 {
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
        if n % divider == 0 {
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

#[test]
fn test_factorize() {
    assert_eq!(factorize(2), [1, 2]);
    assert_eq!(factorize(10), [1, 2, 5, 10]);
    assert_eq!(factorize(13), [1, 13]);
    assert_eq!(factorize(28), [1, 2, 4, 7, 14, 28]);
    assert_eq!(factorize(788), [1, 2, 4, 197, 394, 788]);
    assert_eq!(factorize(512), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
}

#[test]
fn test_proper_divisors_sum() {
    assert_eq!(proper_divisors_sum(2), 1);
    assert_eq!(proper_divisors_sum(10), 8);
    assert_eq!(proper_divisors_sum(13), 1);
    assert_eq!(proper_divisors_sum(28), 28);
    assert_eq!(proper_divisors_sum(788), 598);
    assert_eq!(proper_divisors_sum(512), 511);
}
