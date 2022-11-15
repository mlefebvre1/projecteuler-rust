use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Champernowne's constant
    Problem 40

    An irrational decimal fraction is created by concatenating the positive integers:

    0.123456789101112131415161718192021...

    It can be seen that the 12th digit of the fractional part is 1.

    If dn represents the nth digit of the fractional part, find the value of the following expression.

    d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
    */
    fn make_irrational_decimal_faction() -> String {
        const MAX_N: usize = 1000000;
        let mut irrational_decimal_fraction = String::from("");
        let mut n = 1;
        let mut nb_digits_appended = 0;
        loop {
            let n_str = n.to_string();
            irrational_decimal_fraction += &n_str;
            nb_digits_appended += n_str.chars().count();
            if nb_digits_appended >= MAX_N {
                break;
            }
            n += 1;
        }
        irrational_decimal_fraction
    }

    const TARGETS: [usize; 7] = [1, 10, 100, 1000, 10000, 100000, 1000000];

    let irrational_decimal_fraction = make_irrational_decimal_faction();
    let ds = TARGETS.map(|i| {
        irrational_decimal_fraction
            .chars()
            .nth(i - 1)
            .unwrap()
            .to_digit(10)
            .unwrap()
    });
    let prod: u32 = ds.iter().product();
    Ok(prod.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "210");
    }
}
