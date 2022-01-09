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

pub fn gcd(a: usize, b: usize) -> usize {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t: usize = y;
        y = x % y;
        x = t;
    }
    x
}

#[test]
fn test_sieves() {
    assert_eq!(
        sieves(50),
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    );
    assert_eq!(
        sieves(97),
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}
#[test]
fn test_is_prime() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(983), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(100), false);
    assert_eq!(is_prime(10000), false);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(2, 8), 2);
    assert_eq!(gcd(100, 84), 4);
    assert_eq!(gcd(127426, 95874), 58);
}
