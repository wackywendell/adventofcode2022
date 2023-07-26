mod day01;
mod solutions;
#[cfg(test)]
mod testfns;

use std::io::Read;

use anyhow::{bail, Context};
pub use solutions::Solver;

pub use day01::Day01;

pub fn solver(day: usize, input: impl Read) -> anyhow::Result<Box<dyn Solver>> {
    Ok(Box::new(match day {
        1 => Day01::from_input(input).context("Failed to parse")?,
        _ => bail!("No solution for day {}", day),
    }))
}
