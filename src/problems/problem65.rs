use crate::utils::integers::int_to_vec_of_u8;
use crate::utils::timeit;
use num::BigUint;

fn convergent_of_e_numerator(m: usize) -> BigUint {
    /*
    the sequence of convergent can be computed as :
    e = h(n)/k(n)
    h(n) = a(n) * h(n-1) + h(n-2)
    k(n) = a(n) * k(n-1) + k(n-2)
    with h(0) = 2 and h(1) = 3
    with k(0) = 1 and k(1) = 1
    a(n) = alternating as follow 2,1,1,4,1,1,6,1,1,...
    Since we only use the numerator, we will skip the denominator calculations
    */
    let mut h: [BigUint; 2] = [BigUint::from(2u8), BigUint::from(3u8)];

    for n in 2..m {
        let a = match (n + 1) % 3 {
            0 => (n + 1) / 3 * 2,
            _ => 1,
        };
        (h[0], h[1]) = (h[1].clone(), a * h[1].clone() + h[0].clone());
    }
    h[1].clone()
}

fn p() -> String {
    /*
    Convergents of e
    Problem 65

    The square root of 2 can be written as an infinite continued fraction.
    sqrt{2} = 1 + dfrac{1}{2 + dfrac{1}{2 + dfrac{1}{2 + dfrac{1}{2 + ...}}}}

    The infinite continued fraction can be written, sqrt{2} = [1; (2)], indicates that 2 repeats ad infinitum.
    In a similar way, sqrt{23} = [4; (1, 3, 1, 8)]

    It turns out that the sequence of partial values of continued fractions for square roots provide the best rational
    approximations. Let us consider the convergents for sqrt{2}

    1 + dfrac{1}{2} = dfrac{3}{2}
    1 + dfrac{1}{2 + dfrac{1}{2}} = dfrac{7}{5}
    1 + dfrac{1}{2 + dfrac{1}{2 + dfrac{1}{2}}} = dfrac{17}{12}
    1 + dfrac{1}{2 + dfrac{1}{2 + dfrac{1}{2 + dfrac{1}{2}}}} = dfrac{41}{29}

    Hence the sequence of the first ten convergents for
    1, dfrac{3}{2}, dfrac{7}{5}, dfrac{17}{12}, dfrac{41}{29}, dfrac{99}{70}, dfrac{239}{169}, dfrac{577}{408},
        dfrac{1393}{985}, dfrac{3363}{2378}, ...

    are:

    e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ... , 1, 2k, 1, ...]

    What is most surprising is that the important mathematical constant,
    2, 3, dfrac{8}{3}, dfrac{11}{4}, dfrac{19}{7}, dfrac{87}{32}, dfrac{106}{39}, dfrac{193}{71},
        dfrac{1264}{465}, dfrac{1457}{536}, ...

    The first ten terms in the sequence of convergents for e are:
    2, 3, dfrac{8}{3}, dfrac{11}{4}, dfrac{19}{7}, dfrac{87}{32}, dfrac{106}{39}, dfrac{193}{71},
        dfrac{1264}{465}, dfrac{1457}{536}, ...

    The sum of digits in the numerator of the 10th convergent is 1 + 4 + 5 + 7 = 17

    Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
    */
    const M: usize = 100;
    let h = convergent_of_e_numerator(M);
    let h_digits = int_to_vec_of_u8(&h);
    h_digits
        .iter()
        .fold(0usize, |acc, &digit| acc + digit as usize)
        .to_string()
}

timeit::timeit!(Problem65, solve, p);
