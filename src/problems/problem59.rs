use crate::utils::timeit;
use std::fs;

use anyhow::Result;
fn p() -> Result<String> {
    /*
    XOR decryption
    Problem 59

    Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code
     for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
    A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given
    value, taken from a secret key. The advantage with the XOR function is that using the same encryption key on the
    cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.
    For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random
    bytes. The user would keep the encrypted message and the encryption key in different locations, and without both
    "halves", it is impossible to decrypt the message.
    Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key. If
    the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message.
    The balance for this method is using a sufficiently long password key for security, but short enough to be
    memorable.
    Your task has been made easy, as the encryption key consists of three lower case characters. Using p059_cipher.txt
    (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes, and the knowledge that the
    plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the
    original text
    */
    let data =
        fs::read_to_string("src/problems/data/problem59.txt").expect("Problem opening the file");
    let cipher = data.split(',');
    let streams = seperate_cipher_into_3_streams(&cipher);

    let mut keys = [0u8; 3];
    for (i, stream) in streams.iter().enumerate() {
        let repetitions = (0usize..256).zip(count_repetitions(stream));
        let encoded_space = repetitions.max_by_key(|ch| ch.1).unwrap();
        keys[i] = encoded_space.0 as u8 ^ b' ';
    }
    Ok(decode_text(&cipher, &keys)
        .chars()
        .fold(0usize, |acc, ch| acc + ch as usize)
        .to_string())
}

fn seperate_cipher_into_3_streams<'a>(cipher: &'a std::str::Split<char>) -> [Vec<&'a str>; 3] {
    let mut streams: [Vec<&str>; 3] = Default::default();
    for (i, value) in cipher.clone().enumerate() {
        streams[i % 3].push(value);
    }
    streams
}

fn decode_text<'a>(cipher: &'a std::str::Split<char>, keys: &'a [u8; 3]) -> String {
    let mut decoded_text = String::from("");
    for (index, char_) in cipher.clone().enumerate() {
        let value = String::from(char_).parse::<u8>().unwrap();
        let decoded_value = value ^ keys[index % 3];
        decoded_text.push(decoded_value as char);
    }
    decoded_text
}
fn count_repetitions(stream: &[&str]) -> [usize; 256] {
    let mut repetitions = [0usize; 256];
    for x in stream {
        let digit = x.parse::<usize>().unwrap();
        repetitions[digit] += 1;
    }
    repetitions
}

timeit::timeit!(Problem59, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "129448");
    }
}
