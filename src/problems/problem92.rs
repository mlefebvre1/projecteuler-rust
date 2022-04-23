use crate::utils::integers::int_to_vec_of_u8;
use crate::utils::timeit;

fn generate_digit_sum_values(max_digit_sum: usize) -> Vec<usize> {
    let mut mem: Vec<usize> = (0..=max_digit_sum).map(|_| 0).collect();

    for n in 2..=max_digit_sum {
        let mut _n = n;
        let mut stack = Vec::new();
        loop {
            let digit_sum: usize = int_to_vec_of_u8(&_n)
                .into_iter()
                .map(|d| (d as usize).pow(2))
                .sum();
            if digit_sum == 89 || mem[digit_sum] == 89 {
                mem[n] = 89;
                for id in stack.iter() {
                    mem[*id] = 89;
                }
                break;
            }
            if digit_sum == 1 || mem[digit_sum] == 1 {
                mem[n] = 1;
                for id in stack.iter() {
                    mem[*id] = 1;
                }
                break;
            }
            if mem[digit_sum] == 0 {
                stack.push(digit_sum);
                _n = digit_sum;
            }
        }
    }
    mem
}

fn p() -> String {
    /*
    Square digit chains
    Problem 92
    A number chain is created by continuously adding the square of the digits in a number to form a new number
    until it has been seen before.

    For example,

    44 → 32 → 13 → 10 → 1 → 1
    85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89

    Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most amazing is that
    EVERY starting number will eventually arrive at 1 or 89.

    How many starting numbers below ten million will arrive at 89?
    */
    const MAX_N: usize = 10e6 as usize;
    let max_digit_sum = (9 * 9) * (int_to_vec_of_u8(&MAX_N).len() - 1); // max digit square sum would be 9^2 * 7
    let mem = generate_digit_sum_values(max_digit_sum);
    let nb_89_max_digit_sum_to_max_n = (max_digit_sum + 1..MAX_N)
        .filter(|n| {
            let digit_sum: usize = int_to_vec_of_u8(n)
                .iter()
                .map(|d| (*d as usize).pow(2))
                .sum();
            if mem[digit_sum] == 89 {
                return true;
            }
            false
        })
        .count();
    let nb_89_in_mem = mem.iter().filter(|&x| *x == 89).count();
    let nb_89 = nb_89_in_mem + nb_89_max_digit_sum_to_max_n;
    nb_89.to_string()
}

timeit::timeit!(Problem92, solve, p);
