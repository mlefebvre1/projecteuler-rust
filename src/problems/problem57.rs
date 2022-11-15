use anyhow::Result;
use num::BigUint;

pub fn p() -> Result<String> {
    /*
    Square root convergents
    Problem 57
    It is possible to show that the square root of two can be expressed as an infinite continued fraction.
    In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?
    sqrt2 can be represented by the infinite fraction :
        sqrt(2) = h(n)/k(n)
        h(n) = a(n) * h(n-1) + h(n-2)
        k(n) = a(n) * k(n-1) + k(n-2)
        a(n) = 2

    Turns out that the first fraction is for n=7 and then a pattern alternate between 5 and 8..
    Meaning the problem can be solve by hand.. ceil((1000-7) / (5 + 8) * 2)
    That would be boring so I kept the code :)
    */
    const MAX_N: usize = 1000;
    let a = BigUint::from(2usize);
    let mut h = vec![BigUint::from(3usize), BigUint::from(7usize)];
    let mut k = vec![BigUint::from(2usize), BigUint::from(5usize)];
    let mut total: usize = 0;
    for n in 2..MAX_N {
        let hn = &a * &h[n - 1] + &h[n - 2];
        let kn = &a * &k[n - 1] + &k[n - 2];
        if hn.to_radix_be(10).len() > kn.to_radix_be(10).len() {
            total += 1;
        }
        h.push(hn);
        k.push(kn);
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "153");
    }
}
