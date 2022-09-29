use crate::utils::timeit;
use std::fs;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Maximum path sum II
    Problem 67
    By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from
     top to bottom is 23.
    3
    7 4
    2 4 6
    8 5 9 3
    That is, 3 + 7 + 4 + 9 = 23.
    Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'), a 15K text
    file containing a triangle with one-hundred rows.
    NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route to solve this
    problem, as there are 299 altogether! If you could check one trillion (1012) routes every second it would take over
    twenty billion years to check them all. There is an efficient algorithm to solve it. ;o)
    */
    let mut matrix = prepare_matrix();
    for y in 1..matrix.len() {
        for x in 0..matrix[y].len() {
            // the first and last element of a line can only take 1 value
            if x == 0 {
                // first element of the line
                matrix[y][x] += matrix[y - 1][x];
            } else if x == matrix[y].len() - 1 {
                // last element of the line
                matrix[y][x] += matrix[y - 1][x - 1];
            } else {
                matrix[y][x] += matrix[y - 1][x - 1].max(matrix[y - 1][x]);
            }
        }
    }
    let last_line = &matrix[matrix.len() - 1];
    Ok((*last_line.iter().max().unwrap()).to_string())
}

fn prepare_matrix() -> Vec<Vec<usize>> {
    let mut matrix = Vec::new();

    let data =
        fs::read_to_string("src/problems/data/problem67.txt").expect("Problem opening the file");
    for line in data.lines() {
        let mut line_vec = Vec::new();
        for col in line.split_ascii_whitespace() {
            line_vec.push(col.parse::<usize>().unwrap());
        }
        matrix.push(line_vec);
    }

    matrix
}

timeit::timeit!(Problem67, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "7273");
    }
}
