use crate::utils::timeit;

fn p() -> String {
    /*
    Almost equilateral triangles
    Problem 94

    It is easily proved that no equilateral triangle exists with integral length sides and integral area. However, the
    almost equilateral triangle 5-5-6 has an area of 12 square units.
    We shall define an almost equilateral triangle to be a triangle for which two sides are equal and the third differs
    by no more than one unit.
    Find the sum of the perimeters of all almost equilateral triangles with integral side lengths and area and whose
    perimeters do not exceed one billion (1,000,000,000).

    Solution :  By cutting the triangle in half, it is easy to see that we are looking for pythagorean triples. More
    precisely, we need primitive ones, by inspection, one can see that it is impossible to obtain the smallest side to
    be a factor of 2 minus 1 of the biggest side.
    Here in the example, we've got a triangle [5,5,6]. By cutting it in half, we get two triangles [3,4,5] which
    corresponds exactly to a primitive pythagorean triples. By finding every pythagorean triples and checking if
    the smallest side multiplied by 2 minus 1 equals to the biggest side, we get all the possibilities. Afterward, the
    perimeter is easy to calculate by multiplying by 2 both the smallest and biggest side of the triples.

            /|\
           / | \
      5   /  |  \
         /   |   \
        /___ | ___\
          3     3
    p = a*2 + c*2 = 3*2 + 5*2

    Primitive pythagoreans are given by :
    a = m*m - n*n
    b = 2*m*n
    c = m*m + n*n

    with constraint :
        2*a = c + 1

            2*(m*m-n*n) = (m*m+n*n)+1
            2*m*m-2*n*n = (m*m + n*n) + 1
            m*m = 3*n*n + 1
        solve for m:
         --> m = sqrt(3*n*n + 1)

        2*b = c-1

            2*(2*m*n) = m*m +n*n - 1 = 0
            4*m*n = m*m + n*n - 1
            m*m -4*m*n +n*n -1 = 0

        solve for m :
         --> m = sqrt(3*n*n+1) + 2n

    We can simply make the value of n change and compute the new values of m with the constraints.
    */
    let perimeters = get_all_almost_equilateral_perimeters();
    perimeters.sum::<usize>().to_string()
}
type Triplet = [usize; 3];

fn get_all_almost_equilateral_perimeters() -> impl Iterator<Item = usize> {
    const MAX_PERIMETER: usize = 1e9 as usize;
    // test constraint 2*a = c+
    let perimeters_constraint_1 = (1usize..)
        .filter_map(|n| {
            let m = ((3 * n * n + 1) as f64).sqrt();
            if m.fract() == 0.0 {
                return Some((n, m));
            }
            None
        })
        .map_while(|(n, m)| {
            let [a, _, c] = calc_triplet(m as usize, n);
            let perimeter = calc_perimeter(a, c);
            if perimeter >= MAX_PERIMETER {
                return None;
            }
            Some(perimeter)
        });
    // test constraint 2*b = c-1
    let perimeters_constraint_2 = (1usize..)
        .filter_map(|n| {
            //m = sqrt(3 * n * n + 1) + 2 * n
            let m = ((3 * n * n + 1) as f64).sqrt() + 2.0 * n as f64;
            if m.fract() == 0.0 {
                return Some((n, m));
            }
            None
        })
        .map_while(|(n, m)| {
            let [_, b, c] = calc_triplet(m as usize, n);
            let perimeter = calc_perimeter(b, c);
            if perimeter >= MAX_PERIMETER {
                return None;
            }
            Some(perimeter)
        });
    perimeters_constraint_1.chain(perimeters_constraint_2)
}

fn calc_triplet(m: usize, n: usize) -> Triplet {
    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;
    [a, b, c]
}
fn calc_perimeter(shortest: usize, largest: usize) -> usize {
    2 * shortest + 2 * largest
}

timeit::timeit!(Problem94, solve, p);
