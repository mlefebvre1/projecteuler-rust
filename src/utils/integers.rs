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
