use std::io::Read;

pub trait Solver {
    fn from_input(input: impl Read) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}
