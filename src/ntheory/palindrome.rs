pub fn is_palindrome(n: usize) -> bool {
    let n = n.to_string();
    if n.chars().count() > 1 {
        let n_reversed: String = n.chars().rev().collect();
        n == n_reversed
    } else {
        true
    }
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
