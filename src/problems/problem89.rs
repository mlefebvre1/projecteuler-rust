use crate::utils::timeit;
use phf::phf_map;
use std::fs;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    For a number written in Roman numerals to be considered valid there are basic rules which must be followed.
    Even though the rules allow some numbers to be expressed in more than one way there is always a "best" way of
    writing a particular number.

    For example, it would appear that there are at least six ways of writing the number sixteen:

    IIIIIIIIIIIIIIII
    VIIIIIIIIIII
    VVIIIIII
    XIIIIII
    VVVI
    XVI

    However, according to the rules only XIIIIII and XVI are valid, and the last example is considered to be the most
    efficient, as it uses the least number of numerals.

    The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains one thousand numbers written in
    valid, but not necessarily minimal, Roman numerals; see About... Roman Numerals for the definitive rules for this
    problem.

    Find the number of characters saved by writing each of these in their minimal form.

    Note: You can assume that all the Roman numerals in the file contain no more than four consecutive identical units.
    */
    let data =
        fs::read_to_string("src/problems/data/problem89.txt").expect("Problem opening the file");
    let numbers = data.lines();

    let mut nb_initial_character = 0;
    let mut nb_character_after_opt = 0;
    for number in numbers {
        nb_initial_character += number.chars().count();
        let arab_number = roman_to_arab_number(number);
        let roman_number_optimized = arab_to_roman_number(arab_number);
        nb_character_after_opt += roman_number_optimized.chars().count();
    }
    Ok((nb_initial_character - nb_character_after_opt).to_string())
}

static ROMAN_VALUES: phf::Map<char, usize> = phf_map! {
    'M'=> 1000,
    'D'=> 500,
    'C'=> 100,
    'L'=> 50,
    'X'=> 10,
    'V'=> 5,
    'I'=> 1,
};

fn roman_to_arab_number(roman_number: &str) -> usize {
    let mut last_character_value = 1001;
    let mut arab_number = 0;
    for c in roman_number.chars() {
        let current_character_value = ROMAN_VALUES[&c];
        if last_character_value < current_character_value {
            arab_number += current_character_value - (2 * last_character_value);
        } else {
            arab_number += current_character_value;
        }
        last_character_value = current_character_value;
    }
    arab_number
}

fn arab_to_roman_number(arab_number: usize) -> String {
    let mut arab_number_ = arab_number;
    let mut roman_number = String::new();
    while arab_number_ > 0 {
        let next_char = match arab_number_ {
            x if x >= 1000 => {
                arab_number_ -= 1000;
                "M"
            }
            x if x >= 900 => {
                arab_number_ -= 900;
                "CM"
            }
            x if x >= 500 => {
                arab_number_ -= 500;
                "D"
            }
            x if x >= 400 => {
                arab_number_ -= 400;
                "CD"
            }
            x if x >= 100 => {
                arab_number_ -= 100;
                "C"
            }
            x if x >= 90 => {
                arab_number_ -= 90;
                "XC"
            }
            x if x >= 50 => {
                arab_number_ -= 50;
                "L"
            }
            x if x >= 40 => {
                arab_number_ -= 40;
                "XL"
            }
            x if x >= 10 => {
                arab_number_ -= 10;
                "X"
            }
            x if x >= 9 => {
                arab_number_ -= 9;
                "IX"
            }
            x if x >= 5 => {
                arab_number_ -= 5;
                "V"
            }
            x if x >= 4 => {
                arab_number_ -= 4;
                "IV"
            }
            x if x >= 1 => {
                arab_number_ -= 1;
                "I"
            }
            _ => "",
        };
        roman_number.push_str(next_char);
    }
    roman_number
}

timeit::timeit!(Problem89, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "743");
    }
}
