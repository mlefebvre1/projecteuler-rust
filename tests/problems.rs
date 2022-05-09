use projecteuler::problems;
#[test]
fn validate_solutions() {
    for (problem, solution) in problems::PROBLEMS.iter().zip(problems::SOLUTIONS.iter()) {
        let result = problem();
        println!("{}", result);
        assert_eq!(result, *solution);
    }
}
