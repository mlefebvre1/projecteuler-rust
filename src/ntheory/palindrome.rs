pub fn is_palindrome(n: usize) -> bool {
    let n = n.to_string();
    if n.chars().count() > 1 {
        let n_reversed: String = n.chars().rev().collect();
        if n == n_reversed {
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(123), false);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(1245421), true);
    assert_eq!(is_palindrome(111), true);
    assert_eq!(is_palindrome(3333), true);
    assert_eq!(is_palindrome(987656789), true);
    assert_eq!(is_palindrome(987656779), false);
}
