use crate::utils::matrix::load_matrix2d_from_file;
use anyhow::Result;
use std::cmp::min;

pub fn p() -> Result<String> {
    /*
    Path sum: two ways
    Problem 81
    In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right,
    by only moving to the right and down, is indicated in bold red and is equal to 2427.

    https://projecteuler.net/problem=81

    Find the minimal path sum from the top left to the bottom right by only moving right and down
    in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

    The original solution was using dijkstra shortest path algorithm. But it turns out it was much slower than
    the matrix dynamic programming solution.
    */
    let mut matrix = load_matrix2d_from_file::<usize>("src/problems/data/problem81.txt", ',');
    let shape = matrix.shape();
    let [y_len, x_len] = [shape[0], shape[1]];
    for y in 0..y_len {
        for x in 0..x_len {
            if x == 0 && y == 0 {
                continue;
            } else if y == 0 {
                matrix[[y, x]] += matrix[[y, x - 1]];
            } else if x == 0 {
                matrix[[y, x]] += matrix[[y - 1, x]];
            } else {
                matrix[[y, x]] += min(matrix[[y, x - 1]], matrix[[y - 1, x]])
            }
        }
    }
    Ok(matrix[[y_len - 1, x_len - 1]].to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "427337");
    }
}
