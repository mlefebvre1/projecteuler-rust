use crate::utils::integers::int_to_vec_of_u8;

pub fn is_palindrome_str(n: &str) -> bool {
    if n.chars().count() > 1 {
        let n_reversed: String = n.chars().rev().collect();
        return n == n_reversed;
    }
    true
}

pub fn is_palindrome(n: usize) -> bool {
    let n_vec = int_to_vec_of_u8(n);
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
    assert!(!is_palindrome(123));
    assert!(is_palindrome(121));
    assert!(is_palindrome(1245421));
    assert!(is_palindrome(111));
    assert!(is_palindrome(3333));
    assert!(is_palindrome(987656789));
    assert!(!is_palindrome(987656779));
}
