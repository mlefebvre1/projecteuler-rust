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
    factors.sort();
    return factors;
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
    return proper_div_sum;
}
