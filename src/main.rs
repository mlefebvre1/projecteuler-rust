mod ntheory;
mod problems;
mod series;
mod utils;

const PROBLEMS: [fn() -> String; 80] = [
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
    problems::problem39::solve,
    problems::problem40::solve,
    problems::problem41::solve,
    problems::problem42::solve,
    problems::problem43::solve,
    problems::problem44::solve,
    problems::problem45::solve,
    problems::problem46::solve,
    problems::problem47::solve,
    problems::problem48::solve,
    problems::problem49::solve,
    problems::problem50::solve,
    problems::problem51::solve,
    problems::problem52::solve,
    problems::problem53::solve,
    problems::problem54::solve,
    problems::problem55::solve,
    problems::problem56::solve,
    problems::problem57::solve,
    problems::problem58::solve,
    problems::problem59::solve,
    problems::problem60::solve,
    problems::problem61::solve,
    problems::problem62::solve,
    problems::problem63::solve,
    problems::problem64::solve,
    problems::problem65::solve,
    problems::problem66::solve,
    problems::problem67::solve,
    problems::problem68::solve,
    problems::problem69::solve,
    problems::problem70::solve,
    problems::problem71::solve,
    problems::problem72::solve,
    problems::problem73::solve,
    problems::problem74::solve,
    problems::problem75::solve,
    problems::problem76::solve,
    problems::problem77::solve,
    problems::problem78::solve,
    problems::problem79::solve,
    problems::problem80::solve,
];

#[cfg(not(tarpaulin_include))]
fn parser_arguments() -> i32 {
    use argparse::{ArgumentParser, Store};

    let mut problem_number = -1;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("ProjectEuler-Rust - Enter a problem number to run that problem or leave it empty to run all problems");

        parser
            .refer(&mut problem_number)
            .add_option(&["-n"], Store, "Problem number to execute");
        parser.parse_args_or_exit();
    }
    problem_number
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let problem_number = parser_arguments();
    if problem_number == -1 {
        run_all_problems();
    } else {
        PROBLEMS[(problem_number - 1) as usize]();
    }
}

#[cfg(not(tarpaulin_include))]
fn run_all_problems() {
    for problem in PROBLEMS {
        problem();
    }
}

#[test]
fn test_regression() {
    const SOLUTIONS: [&str; 80] = [
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
        "840",
        "210",
        "7652413",
        "162",
        "16695334890",
        "5482660",
        "1533776805",
        "5777",
        "134043",
        "9110846700",
        "296962999629",
        "997651",
        "121313",
        "142857",
        "4075",
        "376",
        "249",
        "972",
        "153",
        "26241",
        "129448",
        "26033",
        "28684",
        "127035954683",
        "49",
        "1322",
        "272",
        "661",
        "7273",
        "6531031914842725",
        "510510",
        "8319823",
        "428570",
        "303963552391",
        "7295372",
        "402",
        "161667",
        "190569291",
        "71",
        "55374",
        "73162890",
        "40886",
    ];
    for (problem, solution) in PROBLEMS.iter().zip(SOLUTIONS.iter()) {
        let result = problem();
        println!("{}", result);
        assert_eq!(result, *solution);
    }
}
