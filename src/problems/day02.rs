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

impl Hand {
    pub fn next(self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
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
    fn assume_simple(self) -> Play {
        let Pair(l, r) = self;
        let r = match r {
            Right::X => Hand::Rock,
            Right::Y => Hand::Paper,
            Right::Z => Hand::Scissors,
        };
        Play(l, r)
    }

    fn assume_solution(self) -> Play {
        let Pair(l, r) = self;
        let rh = match r {
            Right::X => l.next().next(),
            Right::Y => l,
            Right::Z => l.next(),
        };
        Play(l, rh)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Play(Hand, Hand);

impl Play {
    pub fn winner(self) -> Winner {
        let Play(l, r) = self;
        if l == r {
            Winner::Draw
        } else if l.next() == r {
            Winner::Right
        } else {
            Winner::Left
        }
    }

    pub fn score(self) -> i64 {
        let winner = self.winner();
        let Play(_l, r) = self;
        let pair_score = match r {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
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
        self.0
            .iter()
            .map(|p| p.assume_simple().score())
            .sum::<i64>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .map(|p| p.assume_solution().score())
            .sum::<i64>()
            .to_string()
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
        assert_eq!(pairs[0].assume_simple().score(), 8);
        assert_eq!(pairs[1].assume_simple().score(), 1);
        assert_eq!(pairs[2].assume_simple().score(), 6);

        assert_eq!(solver.part_one(), "15");
    }

    #[test]
    pub fn solution() {
        let solver = Day02::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();

        let pairs = &solver.0;
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].assume_solution().score(), 4);
        assert_eq!(pairs[1].assume_solution().score(), 1);
        assert_eq!(pairs[2].assume_solution().score(), 7);

        assert_eq!(solver.part_two(), "12");
    }
}
