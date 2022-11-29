use anyhow::Result;

use crate::ntheory::primes::sieves;

pub fn p() -> Result<String> {
    /*
    Squarefree Binomial Coefficients
    Problem 203
    The binomial coefficients  (n, k) can be arranged in triangular form, Pascal's triangle, like this:

        0       1       2       3       4       5       6       7

    0    1
    1    1		1
    2    1		2		1
    3    1		3		3		1
    4    1		4		6		4		1
    5    1		5		10		10		5		1
    6    1		6		15		20		15		6		1
    7    1		7		21		35		35		21		7		1


    It can be seen that the first eight rows of Pascal's triangle contain twelve distinct numbers: 1, 2, 3, 4, 5, 6, 7,
    10, 15, 20, 21 and 35.

    A positive integer n is called squarefree if no square of a prime divides n. Of the twelve distinct numbers in the
    first eight rows of Pascal's triangle, all except 4 and 20 are squarefree. The sum of the distinct squarefree
    numbers in the first eight rows is 105.

    Find the sum of the distinct squarefree numbers in the first 51 rows of Pascal's triangle.
    */
    const MAX_ROW: usize = 51;
    let pascal_triangle = create_pascale_triangle(MAX_ROW);
    let squared_primes = create_squared_primes(MAX_ROW);
    let ans = find_sum_of_squarefree(pascal_triangle, squared_primes);
    Ok(ans.to_string())
}

#[allow(clippy::needless_range_loop)]
fn create_pascale_triangle(max_row: usize) -> Vec<Vec<usize>> {
    let mut pascal_triangle = (0..max_row)
        .map(|y| vec![0; y + 1])
        .collect::<Vec<Vec<usize>>>();
    for y in 0..max_row {
        pascal_triangle[y][0] = 1;
        pascal_triangle[y][y] = 1;
    }
    for y in 2..max_row {
        for x in 1..y {
            pascal_triangle[y][x] = pascal_triangle[y - 1][x - 1] + pascal_triangle[y - 1][x];
        }
    }
    pascal_triangle
}

fn create_squared_primes(max_prime: usize) -> Vec<usize> {
    sieves(max_prime).iter().map(|prime| prime.pow(2)).collect()
}

fn find_sum_of_squarefree(pascal_triangle: Vec<Vec<usize>>, squares: Vec<usize>) -> usize {
    let mut square_free = Vec::new();
    for row in pascal_triangle {
        for item in row {
            if !squares.iter().any(|square| item % square == 0) {
                square_free.push(item);
            }
        }
    }
    square_free.sort();
    square_free.dedup();
    square_free.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "34029210557338");
    }
}
