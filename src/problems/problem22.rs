use anyhow::Result;
use std::fs;

pub fn p() -> Result<String> {
    /*
    Names scores
    Problem 22

    Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first
    names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply
    this value by its alphabetical position in the list to obtain a name score.

    For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is
    the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
    What is the total of all the name scores in the file?
    */
    let data =
        fs::read_to_string("src/problems/data/problem22.txt").expect("Problem opening the file");
    let mut names_raw: Vec<&str> = data.split(',').collect();
    names_raw.sort_unstable();
    let names = names_raw.iter().map(|&name| name.replace('\"', ""));
    let names_score = names
        .enumerate()
        .map(|(index, name)| calc_name_score(&name, index));
    let total: usize = names_score.sum();
    Ok(total.to_string())
}

fn calc_name_score(name: &str, index: usize) -> usize {
    const REBASE: usize = 'A' as usize - 1; // 'A' is worth 1 instead of 65 so rebase every characters
    let name_score = (name.chars().map(|c| c as usize).sum::<usize>()
        - REBASE * name.chars().count())
        * (index + 1);
    name_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "871198282");
    }
}
