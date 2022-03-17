use crate::series::square::Square;
use crate::utils::timeit;
use ndarray::{arr2, Array2};
use num::BigInt;

enum ShiftType {
    Right,
    Left,
}

fn left_step(a: &BigInt, b: &BigInt, c: &BigInt) -> (BigInt, BigInt, BigInt) {
    (a + BigInt::from(2isize) * b + c, b + c, c.clone())
}
fn right_step(a: &BigInt, b: &BigInt, c: &BigInt) -> (BigInt, BigInt, BigInt) {
    (a.clone(), a + b, a + BigInt::from(2isize) * b + c)
}

fn apply_shifts(init: (BigInt, BigInt, BigInt)) -> Vec<ShiftType> {
    let mut shifts = Vec::new();
    let (mut a, mut b, mut c) = init.clone();
    loop {
        let t: BigInt = &a + BigInt::from(2isize) * &b + &c;
        if t < BigInt::from(0isize) {
            // The total is less than 0, we need to apply a right shift
            (a, b, c) = right_step(&a, &b, &c);
            shifts.push(ShiftType::Right);
        } else {
            // The total is greater than 0, we need to apply a left shift
            (a, b, c) = left_step(&a, &b, &c);
            shifts.push(ShiftType::Left);
        }
        if a == init.0 && b == init.1 && c == init.2 {
            // If we recover the initial matrix, this indicates we are done
            break;
        }
    }
    shifts
}
fn get_matrix_from_step(step: ShiftType) -> Array2<BigInt> {
    let l_step_matrix = arr2(&[
        [BigInt::from(1), BigInt::from(0)],
        [BigInt::from(1), BigInt::from(1)],
    ]);
    let r_step_matrix = arr2(&[
        [BigInt::from(1), BigInt::from(1)],
        [BigInt::from(0), BigInt::from(1)],
    ]);
    match step {
        ShiftType::Left => l_step_matrix,
        ShiftType::Right => r_step_matrix,
    }
}

fn matrix_multiply_2d(matrix_a: &Array2<BigInt>, matrix_b: &Array2<BigInt>) -> Array2<BigInt> {
    let (a00, a01, a10, a11) = (
        &matrix_a[[0, 0]],
        &matrix_a[[0, 1]],
        &matrix_a[[1, 0]],
        &matrix_a[[1, 1]],
    );
    let (b00, b01, b10, b11) = (
        &matrix_b[[0, 0]],
        &matrix_b[[0, 1]],
        &matrix_b[[1, 0]],
        &matrix_b[[1, 1]],
    );
    let m00 = a00 * b00 + a01 * b10;
    let m01 = a00 * b01 + a01 * b11;
    let m10 = a10 * b00 + a11 * b10;
    let m11 = a10 * b01 + a11 * b11;
    arr2(&[[m00, m01], [m10, m11]])
}

fn p() -> String {
    /*
    Diophantine equation

    Problem 66

    Consider quadratic Diophantine equations of the form:

    x^2 – Dy^2 = 1

    For example, when D=13, the minimal solution in x is 649^2 – 13×180^2 = 1.

    It can be assumed that there are no solutions in positive integers when D is square.

    By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:

    32 – 2×22 = 1
    22 – 3×12 = 1
    92 – 5×42 = 1
    52 – 6×22 = 1
    82 – 7×32 = 1

    Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.

    Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.

    The solution is based on this paper by N.J. Wilberger : https://arxiv.org/pdf/0806.2490.pdf
    */
    const MAX_D: usize = 1000;
    let max_d_sqrt: usize = ((MAX_D as f64).sqrt().ceil()) as usize;
    let squares: Vec<usize> = Square::new().skip(1).take(max_d_sqrt - 1).collect();
    let ds = (2..=MAX_D as isize).filter(|&d| !squares.contains(&(d as usize)));
    ds.map(|d| {
        let d_ = BigInt::from(d);
        let mut steps = apply_shifts((BigInt::from(1isize), BigInt::from(0isize), -d_)).into_iter();
        let matrix_a = get_matrix_from_step(steps.next().unwrap());
        let matrix_b = get_matrix_from_step(steps.next().unwrap());
        let mut matrix_n = matrix_multiply_2d(&matrix_a, &matrix_b);
        for step in steps {
            let matrix_a = get_matrix_from_step(step);
            matrix_n = matrix_multiply_2d(&matrix_n, &matrix_a);
        }
        (d, matrix_n[[0, 0]].clone())
    })
    .max_by_key(|elem| elem.1.clone())
    .unwrap()
    .0
    .to_string()
}

timeit::timeit!(Problem66, solve, p);
