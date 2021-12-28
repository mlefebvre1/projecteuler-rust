use utils::timeit;

fn p() -> i32 {
    /*
    Even Fibonacci numbers
    Problem 2
    Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2,
    the first 10 terms will be:

    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

    By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the
    even-valued terms.
    */
    const MAX_F_VALUE: i32 = 4e6 as i32;

    let mut total: i32 = 0;

    let mut a = 0;
    let mut b = 1;
    loop {
        if a >= MAX_F_VALUE {
            break;
        }
        if a % 2 == 0 {
            total += a;
        }
        let c = a + b;
        a = b;
        b = c;
       }

    return total;
}

timeit::timeit!(Problem02, solve, p);
