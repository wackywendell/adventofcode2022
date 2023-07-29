mod day01;
mod day02;
mod solutions;
mod template;
#[cfg(test)]
mod testfns;

use std::io::Read;

use anyhow::{bail, Context};
use solutions::parse_lines;
pub use solutions::Solver;

pub use day01::Day01;
pub use day02::Day02;

fn unerr<S: Solver + 'static>(input: impl Read) -> anyhow::Result<Box<dyn Solver>> {
    Ok(Box::new(S::from_input(input).context("Failed to parse")?))
}

pub fn solver(day: usize, input: impl Read) -> anyhow::Result<Box<dyn Solver>> {
    Ok(match day {
        1 => unerr::<Day01>(input)?,
        2 => unerr::<Day02>(input)?,
        _ => bail!("No solution for day {}", day),
    })
}