use crate::series::triangular::Triangular;
use crate::utils::timeit;
use std::fs;

fn p() -> String {
    /*
    Coded triangle numbers
    Problem 42

    The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:

    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

    By converting each letter in a word to a number corresponding to its alphabetical position and adding these values
    we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a
    triangle number then we shall call the word a triangle word.

    Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common
    English words, how many are triangle words?
    */
    let data =
        fs::read_to_string("src/problems/data/problem42.txt").expect("Problem opening the file");
    let words: Vec<&str> = data.split(',').collect();
    let words_sum: Vec<usize> = words
        .iter()
        .map(|&word| {
            word.chars().fold(0, |acc, c| {
                if c != '"' {
                    acc + (c as usize) - 64
                } else {
                    acc
                }
            })
        })
        .collect();
    let max_word_sum = words_sum.iter().max().unwrap();
    let triangulars: Vec<usize> = Triangular::<usize>::new().take(*max_word_sum).collect();
    let triangular_word_sum = words_sum
        .iter()
        .filter(|word_sum| triangulars.contains(word_sum));
    triangular_word_sum.count().to_string()
}

timeit::timeit!(Problem42, solve, p);
