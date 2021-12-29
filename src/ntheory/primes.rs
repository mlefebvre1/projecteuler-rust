#[allow(dead_code)]
pub fn sieves(k: usize) -> Vec<usize> {
    /* Generate the prime numbers in the range 2 <= n <= k */
    let mut sieved = vec![false; (k + 1) as usize];
    let mut primes = Vec::new();

    if k <= 2 {
        primes.push(2);
    }

    if k <= 3 {
        primes.push(3);
    }

    let end = (k as f64).sqrt().ceil() as usize;

    for n in 2..end {
        if !sieved[n] {
            for x in (n + n..=k).step_by(n) {
                sieved[x] = true;
            }
        }
    }

    for n in 2..=k {
        if !sieved[n] {
            primes.push(n);
        }
    }
    return primes;
}

pub fn is_prime(n: usize) -> bool {
    let mut i = 5;
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if ((n % 2) == 0) || ((n % 3) == 0) {
        return false;
    }
    while (i * i) <= n {
        if ((n % i) == 0) || ((n % (i + 2)) == 0) {
            return false;
        }
        i += 6;
    }
    return true;
}
