use num::{Integer, Num, NumCast, ToPrimitive};

#[allow(clippy::just_underscores_and_digits)]
pub fn sieves<T>(k: T) -> Vec<T>
where
    T: Num + NumCast + PartialOrd + ToPrimitive + Copy,
{
    /* Generate the prime numbers in the range 2 <= n <= k */

    let _1 = num::one();
    let _2 = NumCast::from(2usize).unwrap();
    let _3 = NumCast::from(3usize).unwrap();

    let mut sieved = vec![false; NumCast::from(k + _1).unwrap()];
    let mut primes = Vec::new();

    if k <= _2 {
        primes.extend_from_slice(&[_2]);
        return primes;
    }

    if k <= _3 {
        primes.extend_from_slice(&[_2, _3]);
        return primes;
    }

    let end = k.to_f64().unwrap().sqrt().ceil() as usize;

    for n in 2..end {
        if !sieved[n] {
            for x in (n + n..=k.to_usize().unwrap()).step_by(n) {
                sieved[x] = true;
            }
        }
    }
    for (n, sieve_entry) in sieved
        .iter()
        .enumerate()
        .take(k.to_usize().unwrap() + 1)
        .skip(2)
    {
        if !sieve_entry {
            primes.push(NumCast::from(n).unwrap());
        }
    }
    primes
}

#[test]
fn test_sieves() {
    assert_eq!(sieves(2), [2]);
    assert_eq!(sieves(3), [2, 3]);
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

#[allow(clippy::just_underscores_and_digits)]
pub fn is_prime<T>(n: T) -> bool
where
    T: Integer + Num + Copy + NumCast,
{
    let zero = NumCast::from(0).unwrap();
    let one = NumCast::from(1).unwrap();
    let two = NumCast::from(2).unwrap();
    let three = NumCast::from(3).unwrap();
    let five = NumCast::from(5).unwrap();
    let six = NumCast::from(6).unwrap();

    if n <= one {
        return false;
    }
    if n <= three {
        return true;
    }
    if n.is_even() || n.is_multiple_of(&three) {
        return false;
    }
    let mut i = five;
    while (i * i) <= n {
        if ((n % i) == zero) || ((n % (i + two)) == zero) {
            return false;
        }
        i = i + six;
    }
    true
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(983));
    assert!(!is_prime(4));
    assert!(!is_prime(100));
    assert!(!is_prime(10000));
}

pub fn distinct_primes<T>(n: T, primes: &[T]) -> Vec<T>
where
    T: Integer + Num + NumCast + Copy,
{
    let mut n = n;
    let mut distincts: Vec<T> = Vec::new();
    let mut prime_iter = primes.iter();
    loop {
        let prime = prime_iter.next().unwrap();
        if n.is_multiple_of(prime) {
            distincts.push(*prime)
        }
        while n.is_multiple_of(prime) {
            n = n / *prime;
            if n == NumCast::from(1).unwrap() {
                return distincts;
            }
        }
    }
}

#[test]
fn test_distinct_primes() {
    assert_eq!(distinct_primes::<usize>(28, &sieves(28)), [2, 7]);
    assert_eq!(distinct_primes(30030, &sieves(30030)), [2, 3, 5, 7, 11, 13]);
    assert_eq!(distinct_primes(1009, &sieves(1009)), [1009]);
}
/*
  primes = iter(primes)
    while 1:
        prime = next(primes)
        if n % prime == 0:
            yield prime
        while n % prime == 0:
            n /= prime
            if n == 1:
                return
*/
