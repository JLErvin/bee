use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct ProblemStatement {
    pub center: u8,
    pub others: Vec<u8>,
    pub dictionary: Vec<String>
}

pub trait GridSolver {
    fn solve(&self, problem: &ProblemStatement) -> Vec<String>;
    fn name(&self) -> String;
}

impl ProblemStatement {
    pub fn new() -> ProblemStatement {
        let center = ProblemStatement::parse_center();
        let others = ProblemStatement::parse_others();
        let dictionary = ProblemStatement::parse_dictionary();
        ProblemStatement { center, others, dictionary }
    }

    fn parse_center() -> u8 {
        let c = std::io::stdin().lock().lines().next().unwrap().unwrap();
        c.chars().next().unwrap() as u8
    }

    fn parse_others() -> Vec<u8> {
        let o = std::io::stdin().lock().lines().next().unwrap().unwrap();
        o.chars().map(|c| c as u8).collect::<Vec<u8>>()
    }

    fn parse_dictionary() -> Vec<String> {
        let file = File::open("words.txt").unwrap();
        let buffer = BufReader::new(file);
        buffer.lines().map(|l| l.unwrap()).collect::<Vec<String>>()
    }
}