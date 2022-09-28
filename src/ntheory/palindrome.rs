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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("123", false)]
    #[case("121", true)]
    #[case("1245421", true)]
    #[case("111", true)]
    #[case("3333", true)]
    #[case("987656789", true)]
    #[case("987656779", false)]
    fn test_is_palindrome_str(#[case] input: &str, #[case] output: bool) {
        assert_eq!(is_palindrome_str(input), output);
    }

    #[rstest]
    #[case(123, false)]
    #[case(121, true)]
    #[case(1245421, true)]
    #[case(111, true)]
    #[case(3333, true)]
    #[case(987656789, true)]
    #[case(987656779, false)]
    fn test_is_palindrome(#[case] input: usize, #[case] output: bool) {
        assert_eq!(is_palindrome(&input), output);
    }
}
