mod ntheory;
mod problems;
mod utils;

const PROBLEMS: [fn() -> String; 23] = [
    problems::problem01::solve,
    problems::problem02::solve,
    problems::problem03::solve,
    problems::problem04::solve,
    problems::problem05::solve,
    problems::problem06::solve,
    problems::problem07::solve,
    problems::problem08::solve,
    problems::problem09::solve,
    problems::problem10::solve,
    problems::problem11::solve,
    problems::problem12::solve,
    problems::problem13::solve,
    problems::problem14::solve,
    problems::problem15::solve,
    problems::problem16::solve,
    problems::problem17::solve,
    problems::problem18::solve,
    problems::problem19::solve,
    problems::problem20::solve,
    problems::problem21::solve,
    problems::problem22::solve,
    problems::problem23::solve,
];

fn main() {
    for problem in PROBLEMS {
        problem();
    }
}

#[test]
fn test_regression() {
    const SOLUTIONS: [&str; 23] = [
        "233168",
        "4613732",
        "6857",
        "906609",
        "232792560",
        "25164150",
        "104743",
        "23514624000",
        "31875000",
        "142913828922",
        "70600674",
        "76576500",
        "5537376230",
        "837799",
        "137846528820",
        "1366",
        "21124",
        "1074",
        "171",
        "648",
        "31626",
        "871198282",
        "4179871",
    ];
    for (problem, solution) in PROBLEMS.iter().zip(SOLUTIONS.iter()) {
        let result = problem();
        println!("{}", result);
        assert_eq!(result, *solution);
    }
}
