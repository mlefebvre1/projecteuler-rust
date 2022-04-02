use crate::utils::matrix::load_matrix2d_from_file;
use crate::utils::timeit;
use ndarray::s;

fn p() -> String {
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
    let matrix = load_matrix2d_from_file("src/problems/data/problem81.txt", ',');
    let (y_len, x_len) = (matrix.shape()[0], matrix.shape()[1]);
    let mut _matrix = matrix.clone();

    for y in 0..y_len {
        for x in 1..x_len {
            _matrix[[y, x]] += _matrix[[y, x - 1]];
        }
    }

    let min_path_stable_threshold = 10;
    let mut min_path_stable_cnt = 0;
    let mut previous_min_path = 0;
    loop {
        for y in 0..y_len {
            for x in 1..x_len {
                if y == 0 {
                    _matrix[[y, x]] = [_matrix[[y + 1, x]], _matrix[[y, x - 1]]]
                        .into_iter()
                        .min()
                        .unwrap()
                        + matrix[[y, x]];
                } else if y == y_len - 1 {
                    _matrix[[y, x]] = [_matrix[[y - 1, x]], _matrix[[y, x - 1]]]
                        .into_iter()
                        .min()
                        .unwrap()
                        + matrix[[y, x]];
                } else {
                    _matrix[[y, x]] = [
                        _matrix[[y + 1, x]],
                        _matrix[[y - 1, x]],
                        _matrix[[y, x - 1]],
                    ]
                    .into_iter()
                    .min()
                    .unwrap()
                        + matrix[[y, x]];
                }
            }
        }

        let matrix_slice = _matrix.slice(s![.., x_len - 1]);
        let min_path = matrix_slice.iter().min().unwrap();
        if *min_path == previous_min_path {
            min_path_stable_cnt += 1;
        } else {
            min_path_stable_cnt = 0;
        }
        if min_path_stable_cnt > min_path_stable_threshold {
            break;
        }
        previous_min_path = *min_path;
    }

    let matrix_slice = _matrix.slice(s![.., x_len - 1]);
    matrix_slice.into_iter().min().unwrap().to_string()
}

timeit::timeit!(Problem82, solve, p);
