use crate::problem::GridSolver;
use std::time::{Duration, Instant};

mod bitvec;
mod naive;
mod problem;

fn main() {
    let problem = problem::ProblemStatement::new();
    println!("{}", problem.center);
    println!("{:?}", problem.others);

    let bit_solver = Box::new(bitvec::BitSolver {});
    let set_solver = Box::new(bitvec::BitSolver {});
    let naive_solver = Box::new(naive::NaiveSolver {});
    let solvers: Vec<Box<dyn GridSolver>> = vec![bit_solver, naive_solver];

    for solver in solvers.into_iter() {
        let now = Instant::now();
        let v = solver.solve(&problem);
        let elapsed = now.elapsed().as_nanos();
        println!("{}", solver.name());
        println!("  Found {} words", v.len());
        println!("  Took {:>10} ns", elapsed);
    }
}
