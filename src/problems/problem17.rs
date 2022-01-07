use crate::utils::timeit;

const NUM_0_10: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUM_10_20: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const NUM_20_100: [&str; 10] = [
    "unused", "unused", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
    "ninety",
];

const ONE: &str = "one";
const THOUSAND: &str = "thousand";
const HUNDRED: &str = "hundred";
const AND: &str = "and";

fn count_base10(n: usize) -> usize {
    let mut nb_letters = 0;
    if n < 10 {
        nb_letters += NUM_0_10[n].chars().count();
    } else if n >= 10 && n < 20 {
        nb_letters += NUM_10_20[n % 10].chars().count();
    } else if n >= 20 && n < 100 {
        let n_str = n.to_string();
        let first_digit: usize = n_str[0..1].parse::<usize>().unwrap();
        let second_digit: usize = n_str[1..2].parse::<usize>().unwrap();
        if second_digit == 0 {
            // 20, 30, 40 .. etc.
            nb_letters += NUM_20_100[first_digit].chars().count();
        } else {
            // 21, 22.. 31, 32 .. 91.. 99
            nb_letters +=
                NUM_20_100[first_digit].chars().count() + NUM_0_10[second_digit].chars().count();
        }
    }
    return nb_letters;
}

fn count_letters(n: usize) -> usize {
    let mut nb_letters = 0;
    if n < 100 {
        // 10 .. 99
        nb_letters += count_base10(n);
    } else if n < 1000 {
        let n_str = n.to_string();
        let base10 = n_str[1..3].parse::<usize>().unwrap();
        let base100 = n_str[0..1].parse::<usize>().unwrap();
        if base10 == 0 {
            // 100, 200, 300 .. etc.
            nb_letters += NUM_0_10[base100].chars().count() + HUNDRED.chars().count();
        } else {
            // 101, 102 .. etc.
            nb_letters +=
                NUM_0_10[base100].chars().count() + HUNDRED.chars().count() + AND.chars().count();
            nb_letters += count_base10(base10);
        }
    } else {
        // 1000
        nb_letters += ONE.chars().count() + THOUSAND.chars().count();
    }

    return nb_letters;
}

fn p() -> String {
    /*
    Number letter counts
    Problem 17

    If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19
    letters used in total.
    If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be
    used?
    NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115
    (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with
    British usage.
    */
    const MAX_N: usize = 1000;
    let nb_letters = (1..=MAX_N).map(|n| count_letters(n));
    nb_letters.sum::<usize>().to_string()
}

timeit::timeit!(Problem17, solve, p);
