use projecteuler::problems;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t=-1)]
    number: isize,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = Args::parse();
    if args.number == -1 {
        problems::run_all_problems();
    } else {
        problems::PROBLEMS[(args.number - 1) as usize]();
    }
}
