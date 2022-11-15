use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Product-sum numbers
    Problem 88
    A natural number, N, that can be written as the sum and product of a given set of at least two natural numbers,
    {a1, a2, ... , ak} is called a product-sum number: N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.
    For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.
    For a given set of size, k, we shall call the smallest N with this property a minimal product-sum number. The
    minimal product-sum numbers for sets of size, k = 2, 3, 4, 5, and 6 are as follows.
    k=2: 4 = 2 × 2 = 2 + 2
    k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
    k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
    k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
    k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
    Hence for 2≤k≤6, the sum of all the minimal product-sum numbers is 4+6+8+12 = 30; note that 8 is only counted once
    in the sum.
    In fact, as the complete set of minimal product-sum numbers for 2≤k≤12 is {4, 6, 8, 12, 15, 16}, the sum is 61.
    What is the sum of all the minimal product-sum numbers for 2≤k≤12000?
    */
    let max_k = 12000;
    let max_accepted = 2 * max_k;
    let mut prod_sum_table = (0..=max_k).map(|_| usize::MAX).collect::<Vec<usize>>();
    for arr_len in 2..15 {
        comb_k(max_accepted, arr_len, &mut prod_sum_table, max_k);
    }
    let mut mem = (0..=max_accepted).map(|_| false).collect::<Vec<bool>>();
    for k in 2..prod_sum_table.len() {
        mem[prod_sum_table[k]] = true;
    }
    Ok(mem
        .iter()
        .enumerate()
        .filter_map(|(i, m)| {
            if *m {
                return Some(i);
            }
            None
        })
        .sum::<usize>()
        .to_string())
}

fn calc_k(arr: &[usize]) -> (usize, usize) {
    let sum: usize = arr.iter().sum();
    let prod: usize = arr.iter().product();
    let k = (prod - sum) + arr.len();
    (k, prod)
}

fn update_prod_sum_table(k: usize, prod: usize, prod_sum_table: &mut [usize], max_k: usize) {
    if k <= max_k && prod < prod_sum_table[k] {
        prod_sum_table[k] = prod;
    }
}

fn comb_k(max_accepted: usize, arr_len: usize, table: &mut [usize], max_k: usize) {
    let mut indexes = (0..arr_len).map(|_| 2).collect::<Vec<usize>>();
    let indexes_stop = (0..arr_len)
        .map(|n| (max_accepted as f64).powf(1.0 / (n as f64 + 1.0)) as usize)
        .collect::<Vec<usize>>();
    let (mut k, mut prod) = calc_k(&indexes);
    update_prod_sum_table(k, prod, table, max_k);
    loop {
        // inner loop
        let mut inner_loop_stop_mult = 1;
        for item in indexes.iter().take(arr_len).skip(1) {
            inner_loop_stop_mult *= item;
        }
        let inner_loop_stop = max_accepted / inner_loop_stop_mult;
        for _ in indexes[0]..inner_loop_stop {
            indexes[0] += 1;
            (k, prod) = calc_k(&indexes);
            update_prod_sum_table(k, prod, table, max_k);
        }
        let mut stop_index = usize::MAX;
        // outer loops
        for i in 1..arr_len {
            indexes[i] += 1;
            (k, prod) = calc_k(&indexes);
            update_prod_sum_table(k, prod, table, max_k);
            if indexes[i] < indexes_stop[i] {
                stop_index = i;
                break;
            }
        }
        if stop_index == usize::MAX {
            return;
        }
        for i in 0..stop_index {
            indexes[i] = indexes[stop_index];
        }
        (k, prod) = calc_k(&indexes);
        update_prod_sum_table(k, prod, table, max_k);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "7587457");
    }
}
