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

#[allow(clippy::just_underscores_and_digits)]
pub fn is_prime<T>(n: T) -> bool
where
    T: Integer + Num + Copy + NumCast,
{
    let one = NumCast::from(1).unwrap();
    let three = NumCast::from(3).unwrap();

    if n <= one {
        return false;
    }
    if n <= three {
        return true;
    }
    if n.is_even() || n.is_multiple_of(&three) {
        return false;
    }
    (5..)
        .step_by(6)
        .take_while(|&i| {
            let _i: T = NumCast::from(i).unwrap();
            (_i * _i) <= n
        })
        .filter(|&i| {
            n.is_multiple_of(&NumCast::from(i).unwrap())
                || n.is_multiple_of(&NumCast::from(i + 2).unwrap())
        })
        .count()
        == 0
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
