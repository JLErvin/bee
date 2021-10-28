use crate::problem::GridSolver;
use crate::problem::ProblemStatement;
use regex::Regex;

pub struct RegexSolver;

impl GridSolver for RegexSolver {
    fn solve(&self, problem: &ProblemStatement) -> Vec<String> {
        let v = problem.dictionary.clone();

        let fo = String::from_utf8(problem.others.clone()).unwrap();
        let formatted = format!(
            "^[{}{}]*{}[{}{}]*$",
            fo, problem.center as char, problem.center as char, fo, problem.center as char
        );
        let re = Regex::new(&formatted).unwrap();

        v.into_iter().filter(|s| re.is_match(s)).collect()
    }

    fn name(&self) -> String {
        "Regex Solver".to_string()
    }
}
