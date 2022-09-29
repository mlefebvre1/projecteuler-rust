use crate::utils::integers::vec_of_u8_to_int;
use crate::utils::timeit;
use itertools::Itertools;
use ndarray::{self, s};
use std::fs;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Su Doku
    Problem 96

    Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept. Its origin is unclear, but
    credit must be attributed to Leonhard Euler who invented a similar, and much more difficult, puzzle idea called
    Latin Squares. The objective of Su Doku puzzles, however, is to replace the blanks (or zeros) in a 9 by 9 grid in
    such that each row, column, and 3 by 3 box contains each of the digits 1 to 9. Below is an example of a typical
    starting puzzle grid and its solution grid. See https://projecteuler.net/problem=96

    A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it may be necessary to
    employ "guess and test" methods in order to eliminate options (there is much contested opinion over this). The
    complexity of the search determines the difficulty of the puzzle; the example above is considered easy because it
    can be solved by straight forward direct deduction.

    The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty different Su Doku puzzles
    ranging in difficulty, but all with unique solutions (the first puzzle in the file is the example above).

    By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner of each solution grid;
    for example, 483 is the 3-digit number found in the top left corner of the solution grid above.
    */
    let grids = grids_from_file();
    let grid_solved_digits = grids.iter().map(|grid| {
        let mut sudoku = Sudoku::new(grid);
        let solved_grid = sudoku.solve();
        let digits: Vec<u8> = solved_grid
            .row(0)
            .iter()
            .take(3)
            .map(|d| *d as u8)
            .collect();
        vec_of_u8_to_int::<usize>(&digits)
    });
    Ok(grid_solved_digits.sum::<usize>().to_string())
}

type Grid = ndarray::Array2<usize>;

fn grids_from_file() -> Vec<Grid> {
    let file =
        fs::read_to_string("src/problems/data/problem96.txt").expect("Problem opening the file");
    let mut grids = Vec::new();
    let mut grid = ndarray::Array2::<usize>::zeros([9, 9]);
    let mut y = 0;
    for line in file.lines() {
        if line.starts_with("Grid") {
            if line != "Grid 01" {
                grids.push(grid.clone());
                y = 0;
            }
        } else {
            for (x, col) in line.chars().enumerate() {
                grid[[y, x]] = col.to_digit(10).unwrap() as usize;
            }
            y += 1;
        }
    }
    // push the last grid because we won't hit a line that starts with "Grid*"!
    grids.push(grid.clone());
    grids
}

struct Sudoku {
    grid: Grid,
    x_max: usize,
    y_max: usize,
}

impl Sudoku {
    pub fn new(grid: &Grid) -> Self {
        let shape = grid.shape();
        let [y_max, x_max] = [shape[0], shape[1]];
        Self {
            grid: grid.clone(),
            x_max,
            y_max,
        }
    }

    pub fn solve(&mut self) -> Grid {
        self.backtrack(0, 0);
        self.grid.clone()
    }

    fn backtrack(&mut self, x: usize, y: usize) -> bool {
        let (mut x_, mut y_) = (x, y);
        loop {
            if self.grid[[y_, x_]] == 0 {
                for n in 1..=self.x_max {
                    if self.is_possible(n, x_, y_) {
                        self.grid[[y_, x_]] = n;
                        let next_square = match (x_, y_) {
                            (_x, _y) if _x < self.x_max - 1 => Some((_x + 1, _y)),
                            (_, _y) if _y < self.y_max - 1 => Some((0, _y + 1)),
                            _ => None,
                        };
                        if next_square == None {
                            return true;
                        }
                        let (next_x, next_y) = next_square.unwrap();
                        if self.backtrack(next_x, next_y) {
                            return true;
                        }
                        self.grid[[y_, x_]] = 0;
                    }
                }
                return false;
            }
            if x_ < self.x_max - 1 {
                x_ += 1;
            } else if y_ < self.y_max - 1 {
                (x_, y_) = (0, y_ + 1);
            } else {
                return true;
            }
        }
    }

    fn is_possible(&self, n: usize, x: usize, y: usize) -> bool {
        // Check if it is possible to insert n at the location provided in the grid
        // Line Validation
        if self.grid.row(y).iter().contains(&n) {
            return false;
        }

        // Column Validation
        if self.grid.column(x).iter().contains(&n) {
            return false;
        }

        // Square Validation
        let (sqr_x, sqr_y) = (x / 3, y / 3);
        if self
            .grid
            .slice(s![sqr_y * 3..sqr_y * 3 + 3, sqr_x * 3..sqr_x * 3 + 3])
            .iter()
            .contains(&n)
        {
            return false;
        }
        true
    }
}

timeit::timeit!(Problem96, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "24702");
    }
}
