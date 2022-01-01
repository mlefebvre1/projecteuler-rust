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
