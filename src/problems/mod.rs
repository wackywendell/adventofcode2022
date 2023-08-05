mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
pub use day07::Day07;
pub use day08::Day08;
pub use day09::Day09;
pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;
pub use day23::Day23;
pub use day24::Day24;
pub use day25::Day25;

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
        7 => unerr::<Day07>(input)?,
        8 => unerr::<Day08>(input)?,
        9 => unerr::<Day09>(input)?,
        10 => unerr::<Day10>(input)?,
        11 => unerr::<Day11>(input)?,
        12 => unerr::<Day12>(input)?,
        13 => unerr::<Day13>(input)?,
        14 => unerr::<Day14>(input)?,
        15 => unerr::<Day15>(input)?,
        16 => unerr::<Day16>(input)?,
        17 => unerr::<Day17>(input)?,
        18 => unerr::<Day18>(input)?,
        19 => unerr::<Day19>(input)?,
        20 => unerr::<Day20>(input)?,
        21 => unerr::<Day21>(input)?,
        22 => unerr::<Day22>(input)?,
        23 => unerr::<Day23>(input)?,
        24 => unerr::<Day24>(input)?,
        25 => unerr::<Day25>(input)?,

        _ => bail!("No solution for day {}", day),
    })
}
