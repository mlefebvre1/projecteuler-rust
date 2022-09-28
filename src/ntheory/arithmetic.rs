pub fn decimal_division(n: usize, nb_digits: usize) -> Vec<u8> {
    /*
    Generate the decimal representation of 1/n up to 'nb_digits' digits
    */
    let mut decimals = Vec::new();
    let mut a = 10usize;
    for _ in 0..nb_digits {
        decimals.push((a / n) as u8);
        a = (a % n) * 10;
    }
    decimals
}

pub fn nb_decimal_recurring_len(n: usize) -> usize {
    let nb_digits = n.to_string().chars().count();
    let decimals = decimal_division(n, n * 2);
    let target = &decimals[0..nb_digits];
    for i in nb_digits..decimals.len() {
        let slice = &decimals[i..(i + nb_digits)];
        if slice == target {
            return i;
        }
    }
    1
}

pub fn pandigital_validation(n: &str, start: usize, stop: usize) -> bool {
    let mut presence_status = vec![0usize; stop + 1];
    for digit in n.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        presence_status[digit] += 1;
    }
    for (digit, presence) in presence_status.iter().enumerate() {
        if digit < start {
            if *presence != 0 {
                return false;
            }
        } else if *presence != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(3,5, &[3,3,3,3,3])]
    #[case(8, 4, &[1, 2, 5, 0])]
    #[case(13,15,&[0, 7, 6, 9, 2, 3, 0, 7, 6, 9, 2, 3, 0, 7, 6])]
    fn test_decimal_division(#[case] n: usize, #[case] nb_digits: usize, #[case] output: &[u8]) {
        assert_eq!(decimal_division(n, nb_digits), output);
    }

    #[rstest]
    #[case(2, 1)]
    #[case(3, 1)]
    #[case(5, 1)]
    #[case(7, 6)]
    #[case(13, 6)]
    #[case(983, 982)]
    fn test_nb_decimal_recurring_len(#[case] input: usize, #[case] output: usize) {
        assert_eq!(nb_decimal_recurring_len(input), output);
    }

    #[rstest]
    #[case("123", 1, 3, true)]
    #[case("1213", 1, 3, false)]
    #[case("987654321", 1, 9, true)]

    fn test_pandigital_validation(
        #[case] n: &str,
        #[case] start: usize,
        #[case] stop: usize,
        #[case] output: bool,
    ) {
        assert_eq!(pandigital_validation(n, start, stop), output);
    }
}
