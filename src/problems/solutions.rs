use std::{
    io::{BufRead, Read},
    str::FromStr,
};

use anyhow::Context;

pub trait Solver {
    fn from_input(input: impl Read) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

pub fn parse_lines<I>(input: impl Read) -> Result<Vec<I>, anyhow::Error>
where
    I: FromStr,
    Result<I, I::Err>: anyhow::Context<I, I::Err>,
{
    std::io::BufReader::new(input)
        .lines()
        .map(|r| {
            r.context("Failed to read line")
                .and_then(|s| I::from_str(&s).context("Failed parsing line"))
        })
        .collect()
}
