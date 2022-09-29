mod graph;
mod ntheory;
mod problems;
mod series;
mod utils;

use clap::Parser;

use anyhow::Result;
use problems::{run_all_problems, PROBLEMS};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    number: Option<usize>,
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(problem_number) = args.number {
        PROBLEMS[problem_number - 1].0()?;
    } else {
        run_all_problems()?;
    }
    Ok(())
}
