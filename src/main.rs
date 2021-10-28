use crate::problem::GridSolver;
use std::time::Instant;

mod bitvec;
mod naive;
mod problem;
mod regex;
mod set;

fn main() {
    let problem = problem::ProblemStatement::new();
    println!("{}", problem.center);
    println!("{:?}", problem.others);

    let bit_solver = Box::new(bitvec::BitSolver {});
    let set_solver = Box::new(set::SetSolver {});
    let naive_solver = Box::new(naive::NaiveSolver {});
    let regex_solver = Box::new(regex::RegexSolver {});
    let solvers: Vec<Box<dyn GridSolver>> =
        vec![naive_solver, set_solver, bit_solver, regex_solver];

    for solver in solvers.into_iter() {
        let now = Instant::now();
        let v = solver.solve(&problem);
        let elapsed = now.elapsed().as_millis();
        println!("{}", solver.name());
        println!("  Found {} words", v.len());
        println!("  Took {:>10} ns", elapsed);
    }
}
