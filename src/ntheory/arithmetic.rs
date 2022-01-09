/*
def decimal_recurring_len(n: int) -> int:
    """
    Calculate the number of digits of the recurrence of decimal
    """
    nb_digits = len(str(n))
    decimal = decimal_division(n, n * 2)
    # Trying to find this sequence again, for relatively small numbers (< 1000) checking 3 digits only is enough
    target = decimal[0:nb_digits]
    for n in range(nb_digits, len(decimal)):
        if decimal[n : n + nb_digits] == target:
            return n
    else:
        return 1
*/

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
    return decimals;
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
    return 1;
}

#[test]
fn test_decimal_division() {
    assert_eq!(decimal_division(3, 5), [3, 3, 3, 3, 3]);
    assert_eq!(decimal_division(8, 4), [1, 2, 5, 0]);
    assert_eq!(
        decimal_division(13, 15),
        [0, 7, 6, 9, 2, 3, 0, 7, 6, 9, 2, 3, 0, 7, 6]
    )
}
#[test]
fn test_nb_decimal_recurring_len() {
    assert_eq!(nb_decimal_recurring_len(2), 1);
    assert_eq!(nb_decimal_recurring_len(3), 1);
    assert_eq!(nb_decimal_recurring_len(5), 1);
    assert_eq!(nb_decimal_recurring_len(7), 6);
    assert_eq!(nb_decimal_recurring_len(13), 6);
    assert_eq!(nb_decimal_recurring_len(983), 982);
}
