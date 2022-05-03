use crate::utils::integers::{int_to_vec_of_u8, vec_of_u8_to_int};
use crate::utils::timeit;
use itertools::Itertools;
use std::fs;

fn p() -> String {
    /*
    Anagramic squares
    Problem 98

    By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a square number:
    1296 = 36^2. What is remarkable is that, by using the same digital substitutions, the anagram, RACE, also forms a
    square number: 9216 = 96^2. We shall call CARE (and RACE) a square anagram word pair and specify further that leading
    zeroes are not permitted, neither may a different letter have the same digital value as another letter.

    Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common
    English words, find all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of
    itself).

    What is the largest square number formed by any member of such a pair?

    NOTE: All anagrams formed must be contained in the given text file.
    */
    let data =
        fs::read_to_string("src/problems/data/problem98.txt").expect("Problem opening the file");
    let words = data.split_terminator(',');
    let words: Vec<String> = words.map(|word| word.replace('\"', "")).collect();
    let words_and_len: Vec<(String, usize)> = words
        .iter()
        .map(|word| (word.clone(), word.chars().count()))
        .collect();
    let word_max_len = words_and_len.iter().max_by_key(|x| x.1).unwrap().1;
    let mut word_stored_per_nb_letters: Vec<Vec<String>> = vec![Vec::new(); word_max_len];
    for (word, len) in words_and_len.into_iter() {
        word_stored_per_nb_letters[len - 1].push(word);
    }
    // For each list of words, find those that are anagrams
    let mut largest_square = 0;
    for word_list in word_stored_per_nb_letters.iter() {
        if !word_list.is_empty() {
            let nb_letters = word_list[0].len();
            let anagram_candidates = get_anagram_candidates(word_list);
            if !anagram_candidates.is_empty() {
                let square_candidates = get_square_candidates(nb_letters);
                for anagrams in anagram_candidates.iter() {
                    for candidate_a in square_candidates.iter() {
                        let candidate_b = anagram_squared_validate(anagrams, candidate_a);
                        if square_candidates.contains(&candidate_b) {
                            let candidate_b_int = vec_of_u8_to_int::<usize>(&candidate_b);
                            if candidate_b_int > largest_square {
                                largest_square = candidate_b_int;
                            }
                        }
                    }
                }
            }
        }
    }
    largest_square.to_string()
}

type NumDigits = Vec<u8>;

fn get_anagram_candidates(words: &[String]) -> Vec<Vec<String>> {
    // From a list of words build combination of words that are anagrams
    let mut anagrams_candidates = Vec::new();
    let mut visited = vec![false; words.len()];
    for (index_a, word_a) in words.iter().enumerate() {
        visited[index_a] = true;
        let mut anagrams = vec![word_a.to_string()];
        // Out of this loop, we have the anagrams for "word_a"
        for (index_b, word_b) in words.iter().enumerate() {
            if word_a != word_b && !visited[index_b] && is_anagram(word_a, word_b) {
                visited[index_b] = true;
                anagrams.push(word_b.to_string());
            }
        }
        if anagrams.len() > 1 {
            anagrams_candidates.push(anagrams.clone());
        }
    }
    anagrams_candidates
}

fn is_anagram(word_a: &str, word_b: &str) -> bool {
    if word_a.chars().count() != word_b.chars().count() || word_a == word_b {
        return false;
    }
    // Two words are anagram when sorted they are equal!
    let word_a_sorted: String = word_a.chars().sorted().collect();
    let word_b_sorted: String = word_b.chars().sorted().collect();
    word_a_sorted == word_b_sorted
}

fn get_square_candidates(nb_digits: usize) -> Vec<NumDigits> {
    let mut candidates = Vec::new();
    let square_per_nb_digits = get_squares_per_nb_digits(10_usize.pow(nb_digits as u32));
    for category in square_per_nb_digits.iter() {
        for n in category.iter() {
            if validate_no_dulicate(n, nb_digits) {
                candidates.push(n.clone());
            }
        }
    }
    candidates
}

fn validate_no_dulicate(n: &[u8], nb_digits: usize) -> bool {
    if n.len() != nb_digits {
        return false;
    }
    let mut presence = [false; 10];
    for digit in n.iter() {
        if presence[*digit as usize] {
            return false;
        } else {
            presence[*digit as usize] = true;
        }
    }
    true
}

fn get_squares_per_nb_digits(max_n: usize) -> Vec<Vec<NumDigits>> {
    let squares = (1_usize..).map_while(|n| {
        let n2 = n * n;
        if n2 > max_n {
            return None;
        }
        Some(n2)
    });

    let nb_digits_max = int_to_vec_of_u8(&max_n).len();
    let mut categories = vec![Vec::new(); nb_digits_max];
    for square in squares {
        let square_as_vec = int_to_vec_of_u8(&square);
        let nb_digits = square_as_vec.len();
        categories[nb_digits - 1].push(square_as_vec);
    }
    categories
}

fn anagram_squared_validate(anagrams: &[String], square_a: &NumDigits) -> NumDigits {
    let mut square_b = vec![0_u8; square_a.len()];
    for (idx, c1) in anagrams[1].chars().enumerate() {
        let letter_idx = anagrams[0].chars().position(|c0| c0 == c1).unwrap();
        square_b[idx] = square_a[letter_idx];
    }
    square_b
}

timeit::timeit!(Problem98, solve, p);
