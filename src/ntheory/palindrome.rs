use crate::utils::integers::int_to_vec_of_u8;
use num::{FromPrimitive, Integer, ToPrimitive, Unsigned};

pub fn is_palindrome_str(n: &str) -> bool {
    if n.chars().count() > 1 {
        let n_reversed: String = n.chars().rev().collect();
        return n == n_reversed;
    }
    true
}

pub fn is_palindrome<T>(n: &T) -> bool
where
    T: FromPrimitive + ToPrimitive + Integer + Unsigned + Clone,
    for<'a> &'a T: std::ops::Rem<T> + std::ops::Div<T> + PartialEq,
{
    let n_vec = int_to_vec_of_u8::<T>(n);
    if n_vec.len() > 1 {
        let n_vec_reversed: Vec<&u8> = n_vec.iter().rev().collect();
        for (n1, n2) in n_vec.iter().zip(n_vec_reversed) {
            if n1 != n2 {
                return false;
            }
        }
        return true;
    }
    true
}

#[test]
fn test_is_palindrome_str() {
    assert!(!is_palindrome_str("123"));
    assert!(is_palindrome_str("121"));
    assert!(is_palindrome_str("1245421"));
    assert!(is_palindrome_str("111"));
    assert!(is_palindrome_str("3333"));
    assert!(is_palindrome_str("987656789"));
    assert!(!is_palindrome_str("987656779"));
}
#[test]
fn test_is_palindrome() {
    assert!(!is_palindrome::<usize>(&123));
    assert!(is_palindrome::<usize>(&121));
    assert!(is_palindrome::<usize>(&1245421));
    assert!(is_palindrome::<usize>(&111));
    assert!(is_palindrome::<usize>(&3333));
    assert!(is_palindrome::<usize>(&987656789));
    assert!(!is_palindrome::<usize>(&987656779));
}
