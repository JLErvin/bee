use crate::problem::GridSolver;
use crate::problem::ProblemStatement;
pub struct NaiveSolver;

impl GridSolver for NaiveSolver {
    fn solve(&self, problem: &ProblemStatement) -> Vec<String> {
        let v = problem.dictionary.clone();

        v.into_iter().filter(|s| NaiveSolver::is_legal(s, problem.center, problem.others.clone())).collect()
    }

    fn name(&self) -> String {
        "Naive Solver".to_string()
    }
}

impl NaiveSolver {
    fn is_legal(s: &str, center: u8, others: Vec<u8>) -> bool {
        if !s.contains(center as char) {
            return false;
        }

        for c in s.as_bytes() {
            if !others.contains(c) && *c != center {
                return false;
            }
        }

        true
    }
}