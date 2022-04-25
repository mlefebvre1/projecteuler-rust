use crate::utils::timeit;
use eval::{eval, Value::Null};
use itertools::Itertools;

fn p() -> String {
    /*
    Arithmetic expressions
    Problem 93

    By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and making use of the four arithmetic
    operations (+, −, *, /) and brackets/parentheses, it is possible to form different positive integer targets.
    For example,
    8 = (4 * (1 + 3)) / 2
    14 = 4 * (3 + 1 / 2)
    19 = 4 * (2 + 3) − 1
    36 = 3 * 4 * (2 + 1)
    Note that concatenations of the digits, like 12 + 34, are not allowed.
    Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different target numbers of which 36 is the
    maximum , and each of the numbers 1 to 28 can be obtained before encountering the first non-expressible number.
    Find the set of four distinct digits, a < b < c < d, for which the longest set of consecutive positive integers,
    1 to n, can be obtained, giving your answer as a string: abcd.
    */

    const MAX_DIGIT: u8 = 9;
    let mut best_score = BestScore {
        score: 0,
        digits: [0, 0, 0, 0],
    };
    let (a, b) = (1u8, 2u8);
    for c in (b + 1u8)..=MAX_DIGIT {
        for d in (c + 1u8)..=MAX_DIGIT {
            let digit_set = [a, b, c, d].map(|c| c + 48);
            let expressions = make_expressions(&digit_set);
            let score = get_score(&expressions);
            if score > best_score.score {
                best_score.digits = [a, b, c, d];
                best_score.score = score;
            }
        }
    }
    let [a, b, c, d] = best_score.digits.map(|c| c + 48);
    String::from_utf8([a, b, c, d].to_vec()).unwrap()
}

type DigitSet = [u8; 4];
type MathOperations = [u8; 3];

struct BestScore {
    score: usize,
    digits: DigitSet,
}

fn make_expressions(set: &DigitSet) -> Vec<String> {
    let digits_set = set.map(|c| c);
    let digits_set_list = digits_set
        .iter()
        .permutations(4)
        .map(|p| [*p[0], *p[1], *p[2], *p[3]]);
    let math_operators_list = make_math_operators();
    let mut expressions = Vec::new();
    for digits_set in digits_set_list {
        for math_operators in math_operators_list.iter() {
            let expr = make_expressions_from_digit_set_and_operations(&digits_set, math_operators);
            expressions.push(expr);
        }
    }
    expressions.into_iter().flatten().collect::<Vec<String>>()
}

fn make_math_operators() -> Vec<MathOperations> {
    const OPERATIONS: [u8; 4] = [b'+', b'-', b'*', b'/'];
    let base_expr = make_base_expr();
    let mut vec_of_operators = Vec::new();

    for expr in base_expr {
        vec_of_operators.push([
            OPERATIONS[expr[0]],
            OPERATIONS[expr[1]],
            OPERATIONS[expr[2]],
        ]);
    }
    vec_of_operators
}

fn make_base_expr() -> Vec<[usize; 3]> {
    const MAX_N: usize = 64;
    const BASE: usize = 4;
    const NB_DIMENSIONS: usize = 3; // ceil(log(64,4)) = 3
    fn next_value(value: usize) -> usize {
        let mut next = value + 1;
        if next >= BASE {
            next = 0;
        }
        next
    }

    let mut set = [0; NB_DIMENSIONS];
    let mut digits_sets = vec![set];
    for n in 1..MAX_N {
        for dim in 0..NB_DIMENSIONS {
            if n % BASE.pow(dim as u32) == 0 {
                let digit_index = NB_DIMENSIONS - 1 - dim;
                set[digit_index] = next_value(set[digit_index]);
            }
            if BASE.pow(dim as u32) > n {
                break;
            }
        }
        digits_sets.push(set);
    }
    digits_sets
}

fn make_expressions_from_digit_set_and_operations(
    digits: &DigitSet,
    operations: &MathOperations,
) -> Vec<String> {
    /*
    Generating the following expressions : (+ can be any operations of +,-,*,/)
    a+b+c+d
    (a+b)+c+d
    (a+b)+(c+d)
    (a+b+c)+d
    a+(b+c+d)
    a+(b+c)+d
    a+((b,c)+d)
    a+(b+(c+d))
    (a+(b+c))+d
    ((a+b)+c)+d
    */
    let [a, b, c, d] = digits.map(|c| c);
    let [op1, op2, op3] = operations.map(|c| c);
    let expressions = [
        vec![a, op1, b, op2, c, op3, d],
        vec![b'(', a, op1, b, b')', op2, c, op3, d],
        vec![b'(', a, op1, b, b')', op2, b'(', c, op3, d, b')'],
        vec![b'(', a, op1, b, op2, c, b')', op3, d],
        vec![a, op1, b'(', b, op2, c, op3, d, b')'],
        vec![a, op1, b'(', b, op2, c, b')', op3, d],
        vec![b'(', a, op1, b'(', b, op2, c, b')', b')', op3, d],
        vec![a, op1, b'(', b'(', b, op2, c, b')', op3, d, b')'],
        vec![b'(', b'(', a, op1, b, b')', op2, c, b')', op3, d],
        vec![a, op1, b'(', b, op2, b'(', c, op3, d, b')', b')'],
    ];
    expressions
        .map(|expr| String::from_utf8(expr).unwrap())
        .into_iter()
        .collect::<Vec<String>>()
}

fn get_results(expression: &[String]) -> Vec<bool> {
    const MAX_SCORE: usize = 10000;
    let mut results: Vec<bool> = (0..=MAX_SCORE).map(|_| false).collect();
    for expr in expression {
        let result = eval(expr).unwrap();
        if result != Null {
            let value = result.as_f64().unwrap();
            if value.fract() == 0.0 {
                let value = value as usize;
                if value > 0 && value <= MAX_SCORE {
                    results[value - 1] = true;
                }
            }
        }
    }
    results
}

fn get_score(expressions: &[String]) -> usize {
    let results = get_results(expressions);
    let score = results.iter().take_while(|&result| *result).count();
    score
}

timeit::timeit!(Problem93, solve, p);
