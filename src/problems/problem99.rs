use itertools::Itertools;

use crate::utils::timeit;
use std::fs;

fn p() -> String {
    /*
    Largest exponential
    Problem 99
    Comparing two numbers written in index form like 211 and 37 is not difficult, as any calculator would confirm that
    211 = 2048 < 37 = 2187.

    However, confirming that 632382518061 > 519432525806 would be much more difficult, as both numbers contain over three
    million digits.

    Using base_exp.txt (right click and 'Save Link/Target As...'), a 22K text file containing one thousand lines with a
    base/exponent pair on each line, determine which line number has the greatest numerical value.

    NOTE: The first two lines in the file represent the numbers in the example given above.

    It is much easier to calculate if we apply the log on both side else it would lead to too big numbers
    -> log(base1^exp1) > log(base2^exp2)
    -> exp1*log(base1) > exp2*log(base2)
    */
    let data =
        fs::read_to_string("src/problems/data/problem99.txt").expect("Problem opening the file");
    let numbers = data.lines();

    let calculated_nums = numbers.enumerate().map(|(line, number)| {
        let number_splitted = number.split_terminator(',').collect_vec();
        let (base, exp) = (
            number_splitted[0].parse::<f64>().unwrap(),
            number_splitted[1].parse::<f64>().unwrap(),
        );
        let calculated_num = exp * base.log10();
        (calculated_num, line + 1)
    });

    let (_, line) = calculated_nums
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap();
    line.to_string()
}

timeit::timeit!(Problem99, solve, p);
