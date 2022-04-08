use crate::series::triangular::Triangular;
use crate::utils::timeit;

fn p() -> String {
    /*
    Counting rectangles
    Problem 85

    By counting carefully it can be seen that a rectangular grid measuring 3 by 2 contains eighteen rectangles:

    https://projecteuler.net/problem=85

    Although there exists no rectangular grid that contains exactly two million rectangles, find the area of the
    grid with the nearest solution.

    For 1D rectangle the number of rectangles is given by the triangle serie
    */
    const K: usize = 100;
    let mut best_rec = BestRectangle {
        err: 2e6 as usize,
        shape: (0, 0),
    };
    let triangles = Triangular::new().take(K).collect::<Vec<usize>>();
    for m in 2..K {
        for n in m..K {
            let rect2d = triangles[m] * triangles[n]; // For 2D rect it is simply the multiplication of both value in 1D rect.
            let err: usize = (2e6 as isize - rect2d as isize).abs() as usize;
            if err < best_rec.err {
                best_rec.err = err;
                best_rec.shape = (m, n);
            }
        }
    }
    let (m, n) = best_rec.shape;
    let area = m * n;
    area.to_string()
}

struct BestRectangle {
    err: usize,
    shape: (usize, usize),
}

timeit::timeit!(Problem85, solve, p);
