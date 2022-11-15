use anyhow::Result;
use std::fs;

pub fn p() -> Result<String> {
    /*
    Passcode derivation
    Problem 79

    A common security method used for online banking is to ask the user for three random characters from a passcode. For
    example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th characters; the expected reply would be:
    317.

    The text file, keylog.txt, contains fifty successful login attempts.

    Given that the three characters are always asked for in order, analyse the file so as to determine the shortest
    possible secret passcode of unknown length.
    */
    let logins =
        fs::read_to_string("src/problems/data/problem79.txt").expect("Problem opening the file");
    let logins = logins.lines().collect::<Vec<&str>>();
    let mut digits_to_place = generate_digit_candidates(&logins);
    let mut passcode = String::from("");
    while !digits_to_place.is_empty() {
        let digit = find_largest_digit(&logins, &digits_to_place);
        let index = digits_to_place.iter().position(|&x| x == digit).unwrap();
        digits_to_place.remove(index);
        passcode += &digit.to_string();
    }
    Ok(passcode)
}

fn generate_digit_candidates(logins: &[&str]) -> Vec<usize> {
    let mut digit_presence = (0..10).map(|_| false).collect::<Vec<bool>>();
    for login in logins.iter() {
        for digit in login
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
        {
            digit_presence[digit] = true;
        }
    }
    digit_presence
        .into_iter()
        .enumerate()
        .filter_map(|(digit, presence)| {
            if presence {
                return Some(digit);
            }
            None
        })
        .collect::<Vec<usize>>()
}

fn find_largest_digit(logins: &[&str], digits_to_place: &[usize]) -> usize {
    let mut largest_digit = (0..10)
        .map(|digit| digits_to_place.contains(&digit))
        .collect::<Vec<bool>>();
    for login in logins.iter() {
        let login_len = login.chars().count();
        // taking len - 1 because the left digit cannot be the last digit, we need a digit to compare with..
        for (i, digit_to_the_left) in login
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
            .take(login_len - 1)
        {
            // try every digit until we get one that is a digit to place
            if digits_to_place.contains(&digit_to_the_left) {
                for digit_to_the_right in login
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .skip(i + 1)
                {
                    // if a digit is to the right of another digit, that means it can't be the largest digit..
                    largest_digit[digit_to_the_right] = false;
                }
            }
        }
    }
    largest_digit
        .into_iter()
        .enumerate()
        .find_map(|(digit, still_true)| {
            if still_true {
                return Some(digit);
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "73162890");
    }
}
