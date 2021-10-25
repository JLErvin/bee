use crate::problem::GridSolver;
use crate::problem::ProblemStatement;
use std::collections::HashSet;
use std::iter::FromIterator;
pub struct SetSolver;

impl GridSolver for SetSolver {
    fn solve(&self, problem: &ProblemStatement) -> Vec<String> {
        let v = problem.dictionary.clone();
        let set: HashSet<u8> = HashSet::from_iter(problem.others.iter().cloned());

        v.into_iter()
            .filter(|s| SetSolver::is_legal(s, problem.center, &set))
            .collect()
    }

    fn name(&self) -> String {
        "Set Solver".to_string()
    }
}

impl SetSolver {
    fn is_legal(s: &str, center: u8, others: &HashSet<u8>) -> bool {
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
