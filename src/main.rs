mod graph;
mod ntheory;
mod problems;
mod series;
mod utils;

use clap::Parser;

use problems::{run_all_problems, PROBLEMS};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    number: Option<usize>,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = Args::parse();
    if let Some(problem_number) = args.number {
        PROBLEMS[problem_number - 1].0();
    } else {
        run_all_problems();
    }
}

#[cfg(test)]
mod test {
    use crate::problems::{Problem, PROBLEMS};

    #[test]
    fn test_regression() {
        PROBLEMS.iter().for_each(|problem| {
            let Problem(solve, solution) = problem;
            let ans = solve();
            println!("{ans}");
            assert_eq!(ans, *solution);
        });
    }
}
