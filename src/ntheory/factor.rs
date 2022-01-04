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

/*
def factorize(n: int) -> List[int]:
    """
    Factorize a number n
    :param int n: A number which we will enumerate its factors
    :return: A list of factors for the number n
    """
    factors = [1]
    if n < 2:
        return factors
    sqrt_n = sqrt(n)
    for divider in range(2, ceil(sqrt_n)):
        if n % divider == 0:
            factors.append(divider)
            factors.append(int(n / divider))
    if n == (int(sqrt_n) * int(sqrt_n)):
        factors.append(int(sqrt_n))
    factors.append(n)
    return sorted(factors)
*/
