use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Investigating the behaviour of a recursively defined sequence
    Problem 197
    Given is the function f(x) = ⌊2^(30.403243784-x^2)⌋ × 10^-9 ( ⌊ ⌋ is the floor-function),
    the sequence un is defined by u0 = -1 and un+1 = f(un).

    Find un + un+1 for n = 10^12.
    Give your answer with 9 digits after the decimal point.

    The recursive function converge and alternate between 0.681175878 and 1.029461839
    */
    const MAX_N: usize = 1000;
    let mut un = f(-1.0);
    let mut unm1 = un;
    let mut settled_count = 0;
    let mut prev_s = 0.0;
    while settled_count < 200 {
        unm1 = un;
        un = f(un);
        let s = un + unm1;
        if s == prev_s {
            settled_count += 1;
        } else {
            settled_count = 0;
        }
        prev_s = s;
    }
    Ok((unm1 + un).to_string())
}
fn f(x: f64) -> f64 {
    2.0_f64.powf(30.403243784 - x.powf(2.0)).floor() / 1e9
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1.710637717");
    }
}
