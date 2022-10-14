use num::{pow, FromPrimitive, Integer, ToPrimitive, Unsigned};

pub fn vec_of_u8_to_int<T>(vec: &[u8]) -> T
where
    T: FromPrimitive + Unsigned + Integer + Clone,
{
    vec.iter()
        .rev()
        .enumerate()
        .fold(num::zero(), |acc, (exp, digit)| {
            let _digit: T = FromPrimitive::from_u8(*digit).unwrap();
            let ten: T = FromPrimitive::from_usize(10).unwrap();
            acc + _digit * pow(ten, exp)
        })
}

#[test]
fn test_vec_of_u8_to_int() {
    use num::BigUint;
    assert_eq!(vec_of_u8_to_int::<usize>(&[1, 2, 3]), 123);
    assert_eq!(
        vec_of_u8_to_int::<usize>(&[4, 5, 9, 0, 0, 5, 4, 7, 2]),
        459005472
    );
    assert_eq!(vec_of_u8_to_int::<usize>(&[1]), 1);
    assert_eq!(vec_of_u8_to_int::<usize>(&[0, 0, 0]), 0);
    assert_eq!(vec_of_u8_to_int::<usize>(&[0, 5, 8]), 58);
    assert_eq!(vec_of_u8_to_int::<usize>(&[5, 0, 0]), 500);
    assert_eq!(
        vec_of_u8_to_int::<BigUint>(&[1, 2, 3]),
        BigUint::from(123usize)
    );
}

pub fn int_to_vec_of_u8<T>(n: &T) -> Vec<u8>
where
    T: FromPrimitive + ToPrimitive + Unsigned + Integer + Clone,
    for<'a> &'a T: std::ops::Rem<T> + std::ops::Div<T> + PartialEq,
{
    if n == &num::zero() {
        return vec![0];
    }
    let mut vec: Vec<u8> = Vec::new();
    let mut _n = &mut n.clone();
    while *_n > num::zero() {
        let n_mod_10 = _n.clone() % FromPrimitive::from_usize(10).unwrap();
        // push n % 10
        vec.push(ToPrimitive::to_u8(&n_mod_10).unwrap());
        // n /= 10
        *_n = _n.clone() / FromPrimitive::from_usize(10).unwrap();
    }
    vec.reverse();
    vec
}

pub fn powmod(base: u64, exp: u64, m: u64) -> u64 {
    let mut exp = exp;
    let mut base = base % m;
    let mut ans = 1;
    while exp > 0 {
        if exp.is_odd() {
            ans = (ans * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }
    ans
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, &[0])]
    #[case(1, &[1])]
    #[case(123, &[1,2,3])]
    #[case(100, &[1,0,0])]
    #[case(957327, &[9,5,7,3,2,7])]
    fn test_int_to_vec_of_u8_for_usize(#[case] input: usize, #[case] output: &[u8]) {
        assert_eq!(int_to_vec_of_u8::<usize>(&input), output);
    }

    #[test]
    fn test_int_to_vec_of_u8_for_bigint() {
        use num::BigUint;
        assert_eq!(
            int_to_vec_of_u8::<BigUint>(&BigUint::from(957327usize)),
            [9, 5, 7, 3, 2, 7]
        );
    }

    #[rstest]
    #[case(2, 1236, 100000, 48736)]
    #[case(148, 148, 148148, 28120)]
    #[case(0, 0, 99, 1)]
    #[case(0, 99, 99, 0)]
    #[case(1, 99, 101, 1)]
    fn test_powmod(#[case] base: u64, #[case] exp: u64, #[case] modulus: u64, #[case] ans: u64) {
        assert_eq!(powmod(base, exp, modulus), ans)
    }
}
