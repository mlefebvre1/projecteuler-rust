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

#[test]
fn test_int_to_vec_of_u8() {
    use num::BigUint;
    assert_eq!(int_to_vec_of_u8::<usize>(&0), [0]);
    assert_eq!(int_to_vec_of_u8::<usize>(&1), [1]);
    assert_eq!(int_to_vec_of_u8::<usize>(&123), [1, 2, 3]);
    assert_eq!(int_to_vec_of_u8::<usize>(&100), [1, 0, 0]);
    assert_eq!(int_to_vec_of_u8::<usize>(&957327), [9, 5, 7, 3, 2, 7]);
    assert_eq!(
        int_to_vec_of_u8::<BigUint>(&BigUint::from(957327usize)),
        [9, 5, 7, 3, 2, 7]
    );
}
