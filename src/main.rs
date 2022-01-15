mod ntheory;
mod problems;
mod utils;

const PROBLEMS: [fn() -> String; 38] = [
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
    problems::problem24::solve,
    problems::problem25::solve,
    problems::problem26::solve,
    problems::problem27::solve,
    problems::problem28::solve,
    problems::problem29::solve,
    problems::problem30::solve,
    problems::problem31::solve,
    problems::problem32::solve,
    problems::problem33::solve,
    problems::problem34::solve,
    problems::problem35::solve,
    problems::problem36::solve,
    problems::problem37::solve,
    problems::problem38::solve,
];

fn main() {
    for problem in PROBLEMS {
        problem();
    }
}

#[test]
fn test_regression() {
    const SOLUTIONS: [&str; 38] = [
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
        "2783915460",
        "4782",
        "983",
        "-59231",
        "669171001",
        "9183",
        "443839",
        "73682",
        "45228",
        "100",
        "40730",
        "55",
        "872187",
        "748317",
        "932718654",
    ];
    for (problem, solution) in PROBLEMS.iter().zip(SOLUTIONS.iter()) {
        let result = problem();
        println!("{}", result);
        assert_eq!(result, *solution);
    }
}
