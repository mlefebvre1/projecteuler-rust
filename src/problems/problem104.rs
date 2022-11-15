use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;

fn p() -> Result<String> {
    /*
    Pandigital Fibonacci ends
    Problem 104

    The Fibonacci sequence is defined by the recurrence relation:

        Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

    It turns out that F541, which contains 113 digits, is the first Fibonacci number for which the last nine digits are
    1-9 pandigital (contain all the digits 1 to 9, but not necessarily in order). And F2749, which contains 575 digits,
    is the first Fibonacci number for which the first nine digits are 1-9 pandigital.
    Given that Fk is the first Fibonacci number for which the first nine digits AND the last nine digits are 1-9
    pandigital, find k.
    */
    const M1: u64 = 1e9 as u64;
    const M2: u64 = 1e14 as u64; // Keeping more digits because of loss of precision
    let (mut tail, mut tailm1) = (2_u64, 1_u64);
    let (mut head, mut headm1) = (2_u64, 1_u64);
    let mut k = 3_u64;
    let ans = loop {
        k += 1;
        (tail, tailm1) = ((tail + tailm1) % M1, tail % M1);

        if headm1 > head {
            head = head * 10 + 9;
        }
        let next_head = head + headm1;
        (head, headm1) = (saturate(next_head, M2), saturate(head, M2));

        if is_pandigital(&int_to_vec_of_u8(&tail)) && is_pandigital(&int_to_vec_of_u8(&head)[0..9])
        {
            break k;
        }
    };

    Ok(ans.to_string())
}

fn is_pandigital(n: &[u8]) -> bool {
    let mut n = n.to_owned();
    n.sort();
    n == [1_u8, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn saturate(n: u64, max_value: u64) -> u64 {
    if n >= max_value {
        n / 10
    } else {
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "329468");
    }
}
