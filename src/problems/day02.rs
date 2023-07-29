use anyhow::{anyhow, bail};
use log::debug;

use std::{io::Read, str::FromStr};

use super::{parse_lines, Solver};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Right {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Winner {
    Left,
    Right,
    Draw,
}

impl Winner {
    pub fn score(self) -> i64 {
        match self {
            Winner::Left => 0,
            Winner::Right => 6,
            Winner::Draw => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair(Hand, Right);

impl Pair {
    pub fn winner(self) -> Winner {
        let Pair(l, r) = self;
        match (l, r) {
            (Hand::Rock, Right::Y) => Winner::Right,
            (Hand::Paper, Right::Z) => Winner::Right,
            (Hand::Scissors, Right::X) => Winner::Right,
            (Hand::Rock, Right::X) => Winner::Draw,
            (Hand::Paper, Right::Y) => Winner::Draw,
            (Hand::Scissors, Right::Z) => Winner::Draw,
            (Hand::Rock, Right::Z) => Winner::Left,
            (Hand::Paper, Right::X) => Winner::Left,
            (Hand::Scissors, Right::Y) => Winner::Left,
        }
    }

    pub fn score(self) -> i64 {
        let winner = self.winner();
        let Pair(_l, r) = self;
        let pair_score = match r {
            Right::X => 1,
            Right::Y => 2,
            Right::Z => 3,
        };

        debug!("{} {:?} {:?} {}", pair_score, self, winner, winner.score());

        pair_score + winner.score()
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid input {}", s))?;
        let left = match left {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => bail!("Invalid input {}", s),
        };
        let right = match right {
            "X" => Right::X,
            "Y" => Right::Y,
            "Z" => Right::Z,
            _ => bail!("Invalid input {}", s),
        };
        Ok(Pair(left, right))
    }
}

pub struct Day02(Vec<Pair>);

impl Solver for Day02 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let pairs = parse_lines::<Pair>(input)?;

        Ok(Day02(pairs))
    }

    fn part_one(&self) -> String {
        self.0.iter().map(|p| p.score()).sum::<i64>().to_string()
    }

    fn part_two(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::super::testfns::unindent;
    use super::*;

    const EXAMPLE: &str = r"
        A Y
        B X
        C Z";

    #[test]
    pub fn basic() {
        let solver = Day02::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();

        let pairs = &solver.0;
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].score(), 8);
        assert_eq!(pairs[1].score(), 1);
        assert_eq!(pairs[2].score(), 6);

        assert_eq!(solver.part_one(), "15");
    }
}
