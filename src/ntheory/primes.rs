use num::{Num, NumCast, ToPrimitive};

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

#[allow(clippy::just_underscores_and_digits)]
pub fn is_prime<T>(n: T) -> bool
where
    T: Num + PartialOrd + NumCast + Copy,
{
    let _0 = num::zero();
    let _1 = num::one();
    let _2 = NumCast::from(2usize).unwrap();
    let _3 = NumCast::from(3usize).unwrap();
    let _6 = NumCast::from(6usize).unwrap();

    let mut i = NumCast::from(5usize).unwrap();
    if n <= _1 {
        return false;
    } else if n <= _3 {
        return true;
    } else if ((n % _2) == _0) || ((n % _3) == _0) {
        return false;
    }
    while (i * i) <= n {
        if ((n % i) == _0) || ((n % (i + _2)) == _0) {
            return false;
        }
        i = i + _6;
    }
    true
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
