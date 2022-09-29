use crate::utils::timeit;

use anyhow::Result;

fn p() -> Result<String> {
    /*
    Right triangles with integer coordinates
    Problem 91

    The points P (x1, y1) and Q (x2, y2) are plotted at integer co-ordinates and are joined to the origin, O(0,0),
    to form ΔOPQ.
    There are exactly fourteen triangles containing a right angle that can be formed when each co-ordinate lies between
    0 and 2 inclusive; that is,
    0 ≤ x1, y1, x2, y2 ≤ 2.
    Given that 0 ≤ x1, y1, x2, y2 ≤ 50, how many right triangles can be formed?

    Solution : For every set of coordinate possible P[x1,y1] and Q[x2,y2], determine the distance power of 2 from the
    origin O(0,0). of P and Q, namely OP and OQ. Then find the distance power of 2 between P and Q (PQ). Finally,
    determine the hypotenuse which is max(OP, OQ, PQ) and check the equality --> hyp power of two == the sum of the two
    other distance power of 2 (a^2 = b^2 + c^2). If the equation is satisfied, this tells us that the triangle has a
    right angle.
    */
    const K: isize = 50;
    let mut nb_triangles: usize = 0;
    for y2 in 0..=K {
        for y1 in 0..=K {
            for x1 in 0..=K {
                for x2 in 0..=K {
                    let op2: isize = x1 * x1 + y1 * y1;
                    let oq2: isize = x2 * x2 + y2 * y2;
                    let qp2: isize = (x2 - x1).pow(2) + (y2 - y1).pow(2);

                    if op2 != 0 && oq2 != 0 && qp2 != 0 {
                        if op2 > oq2 && op2 > qp2 {
                            //OP is the hyp.
                            if op2 == (oq2 + qp2) {
                                nb_triangles += 1;
                            }
                        } else if oq2 > op2 && oq2 > qp2 {
                            // OQ is the Hyp
                            if oq2 == (op2 + qp2) {
                                nb_triangles += 1;
                            }
                        } else {
                            //QP is the Hyp
                            if qp2 == (op2 + oq2) {
                                nb_triangles += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((nb_triangles / 2).to_string())
}

timeit::timeit!(Problem91, solve, p);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve().unwrap(), "14234");
    }
}
