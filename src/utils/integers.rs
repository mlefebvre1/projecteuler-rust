pub fn vec_of_u8_to_int(vec: &[u8]) -> usize {
    vec.iter().rev().enumerate().fold(0, |acc, (exp, digit)| {
        acc + (*digit as usize) * 10usize.pow(exp as u32)
    })
}

#[test]
fn test_vec_of_u8_to_int() {
    assert_eq!(vec_of_u8_to_int(&[1, 2, 3]), 123);
    assert_eq!(vec_of_u8_to_int(&[4, 5, 9, 0, 0, 5, 4, 7, 2]), 459005472);
    assert_eq!(vec_of_u8_to_int(&[1]), 1);
    assert_eq!(vec_of_u8_to_int(&[0, 0, 0]), 0);
    assert_eq!(vec_of_u8_to_int(&[0, 5, 8]), 58);
    assert_eq!(vec_of_u8_to_int(&[5, 0, 0]), 500);
}

pub fn int_to_vec_of_u8(n: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let mut _n = n;
    while _n > 0 {
        vec.push((_n % 10) as u8);
        _n /= 10;
    }
    vec.reverse();
    vec
}

#[test]
fn test_int_to_vec_of_u8() {
    assert_eq!(int_to_vec_of_u8(123), [1, 2, 3]);
    assert_eq!(int_to_vec_of_u8(100), [1, 0, 0]);
    assert_eq!(int_to_vec_of_u8(957327), [9, 5, 7, 3, 2, 7]);
}
