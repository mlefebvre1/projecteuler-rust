use num::{Num, NumCast, ToPrimitive};

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
        primes.push(_2);
    }

    if k <= _3 {
        primes.push(_3);
    }

    let end = k.to_f64().unwrap().sqrt().ceil() as usize;

    for n in 2..end {
        if !sieved[n] {
            for x in (n + n..=k.to_usize().unwrap()).step_by(n) {
                sieved[x] = true;
            }
        }
    }

    for n in 2..=k.to_usize().unwrap() {
        if !sieved[n] {
            primes.push(NumCast::from(n).unwrap());
        }
    }
    return primes;
}

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
    return true;
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Num + NumCast + PartialOrd + Copy,
{
    let mut x = a;
    let mut y = b;
    while y != num::zero() {
        let t = NumCast::from(y).unwrap();
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
