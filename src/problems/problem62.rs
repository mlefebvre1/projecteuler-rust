use crate::series::cubic::Cubic;
use crate::utils::integers::int_to_vec_of_u8;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub fn p() -> Result<String> {
    /*
    Cubic permutations
    Problem 62
    The cube, 41063625 (3453), can be permuted to produce two other cubes: 56623104 (3843) and 66430125 (4053). In fact,
    41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.
    Find the smallest cube for which exactly five permutations of its digits are cube.
    */
    const NB_PERMUTATIONS_TARGET: usize = 5;
    let mut cubics_gen = CubicsGenerator::new();
    Ok(cubics_gen
        .find_map(|cubics| {
            let hashmap = map_by_sorted_digits(&cubics);
            let candidates =
                search_hash_with_correct_nb_permutations(hashmap, NB_PERMUTATIONS_TARGET);
            if !candidates.is_empty() {
                return Some(candidates.iter().min().unwrap().to_string());
            }
            None
        })
        .unwrap())
}

struct CubicsGenerator {
    nb_digits: usize,
}

impl CubicsGenerator {
    fn new() -> CubicsGenerator {
        CubicsGenerator { nb_digits: 1 }
    }
}

impl Iterator for CubicsGenerator {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let cubics = Cubic::<usize>::new();
        let ans = cubics
            .take_while(|cubic| {
                let cubic_str = int_to_vec_of_u8(cubic);
                cubic_str.len() <= self.nb_digits
            })
            .collect::<Vec<usize>>();
        self.nb_digits += 1;
        Some(ans)
    }
}
fn map_by_sorted_digits(cubics: &[usize]) -> HashMap<String, Vec<usize>> {
    let mut hashmap = HashMap::<String, Vec<usize>>::new();
    for cubic in cubics {
        let cubic_str = cubic.to_string().chars().sorted().collect::<String>();
        if let std::collections::hash_map::Entry::Vacant(e) = hashmap.entry(cubic_str.clone()) {
            e.insert(vec![*cubic]);
        } else {
            hashmap.get_mut(&cubic_str).unwrap().push(*cubic);
        }
    }
    hashmap
}

fn search_hash_with_correct_nb_permutations(
    hashmap: HashMap<String, Vec<usize>>,
    nb_permutations: usize,
) -> Vec<usize> {
    let mut ans = Vec::new();
    for (_, vec) in hashmap {
        if vec.len() == nb_permutations {
            ans.push(vec[0]);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "127035954683");
    }
}
