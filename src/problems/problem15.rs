use crate::utils::timeit;

fn p() -> String {
    /*
    Lattice paths
    Problem 15

    Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly
    6 routes to the bottom right corner.

    How many such routes are there through a 20×20 grid?

    The solution is to re-use previously calculated solution : [col, row] = [col-1, row] + [col, row-1]

    Easy to visualize with the following grid..

         1   2   3   4
      ---------------------
    1 |  2   3   4   5   ...
    2 |  3   6   10  15  ...
    3 |  4   10  20  35  ...
    4 |  5   15  35  70  ...
    */
    let mut tab = [[0usize; 21]; 21];
    // Initialize the first row and column
    for col in 1..=20 {
        tab[1][col] = col + 1;
    }
    for row in 1..=20 {
        tab[row][1] = row + 1;
    }
    for row in 2..=20 {
        for col in 2..=20 {
            tab[row][col] = tab[row - 1][col] + tab[row][col - 1];
        }
    }
    return tab[20][20].to_string();
}

timeit::timeit!(Problem15, solve, p);
