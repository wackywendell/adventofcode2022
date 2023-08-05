mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
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
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;

fn unerr<S: Solver + 'static>(input: impl Read) -> anyhow::Result<Box<dyn Solver>> {
    Ok(Box::new(S::from_input(input).context("Failed to parse")?))
}

pub fn solver(day: usize, input: impl Read) -> anyhow::Result<Box<dyn Solver>> {
    Ok(match day {
        1 => unerr::<Day01>(input)?,
        2 => unerr::<Day02>(input)?,
        3 => unerr::<Day03>(input)?,
        4 => unerr::<Day04>(input)?,
        5 => unerr::<Day05>(input)?,
        6 => unerr::<Day06>(input)?,
        _ => bail!("No solution for day {}", day),
    })
}
