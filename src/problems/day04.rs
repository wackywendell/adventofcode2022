use std::{io::Read, ops::RangeInclusive, str::FromStr};

use super::{solutions::parse_lines, Solver};

pub type Assignment = RangeInclusive<i64>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElfPair(Assignment, Assignment);

fn parse_pair(s: &str) -> anyhow::Result<Assignment> {
    let (a, b) = s
        .split_once('-')
        .ok_or_else(|| anyhow::anyhow!("Expected a dash"))?;

    let first: i64 = a.parse()?;
    let second: i64 = b.parse()?;
    Ok(first..=second)
}

impl FromStr for ElfPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Expected a comma"))?;

        let first = parse_pair(first)?;
        let second = parse_pair(second)?;
        Ok(ElfPair(first, second))
    }
}

impl ElfPair {
    pub fn fully_contained(&self) -> bool {
        let ElfPair(ref a, ref b) = self;
        (a.start() >= b.start() && a.end() <= b.end())
            || (a.start() <= b.start() && a.end() >= b.end())
    }

    pub fn overlapping(&self) -> bool {
        let ElfPair(ref a, ref b) = self;
        (a.start() <= b.end() && a.end() >= b.start())
            || (a.end() >= b.start() && a.start() <= b.end())
    }
}

pub struct Day04(Vec<ElfPair>);

impl Day04 {
    pub fn contained_pairs(&self) -> usize {
        self.0.iter().filter(|&p| p.fully_contained()).count()
    }

    pub fn overlapping_pairs(&self) -> usize {
        self.0.iter().filter(|&p| p.overlapping()).count()
    }
}

impl Solver for Day04 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let items = parse_lines::<ElfPair>(input)?;

        Ok(Day04(items))
    }

    fn part_one(&self) -> String {
        format!("{}", self.contained_pairs())
    }

    fn part_two(&self) -> String {
        format!("{}", self.overlapping_pairs())
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindent;

    use super::*;

    const EXAMPLE: &str = r"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn test_part_one() {
        let day = Day04::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.contained_pairs(), 2);
    }

    #[test]
    fn test_part_two() {
        let day = Day04::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.overlapping_pairs(), 4);
    }
}
