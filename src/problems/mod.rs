mod day01;
mod solutions;

use std::io::Read;

pub use solutions::Solver;

pub use day01::Day01;

pub fn solver(day: usize, input: impl Read) -> Box<dyn Solver> {
    match day {
        1 => Box::new(Day01::from_input(input)),
        _ => unimplemented!(),
    }
}
