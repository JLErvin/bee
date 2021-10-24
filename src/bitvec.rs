use crate::problem::GridSolver;
use crate::problem::ProblemStatement;

type BitVector = u32;

pub struct BitSolver;

impl GridSolver for BitSolver {
    fn solve(&self, problem: &ProblemStatement) -> Vec<String> {
        let v = problem.dictionary.clone();

        let center_mask = from_char(problem.center);
        let other_mask = BitSolver::calc_mask(&problem.others);

        v.into_iter()
            .filter(|s| is_legal(s, center_mask, other_mask))
            .collect()
    }

    fn name(&self) -> String {
        "Bit Vector Solver".to_string()
    }
}

impl BitSolver {
    fn calc_mask(chars: &Vec<u8>) -> BitVector {
        chars
            .into_iter()
            .map(|c| from_char(*c))
            .reduce(|a, b| a | b)
            .unwrap()
    }
}

fn from_str(s: &str) -> BitVector {
    s.as_bytes()
        .into_iter()
        .map(|c| from_char(*c))
        .reduce(|a, b| a | b)
        .unwrap()
}

fn from_char(char: u8) -> BitVector {
    1 << ((char as u32) - 97)
}

fn is_legal(s: &str, center_mask: BitVector, other_mask: BitVector) -> bool {
    let b = from_str(s);
    if b & center_mask == 0 {
        return false;
    }

    b & !(center_mask | other_mask) == 0
}
