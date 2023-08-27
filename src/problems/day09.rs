use std::{collections::HashSet, io::Read, str::FromStr};

use adventofcode2022::{Compass, Position};
use anyhow::{anyhow, bail};

use super::Solver;

pub struct Instructions(Vec<(Compass, usize)>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let rows = s
            .lines()
            .map(|line| {
                let (d, n) = line
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("invalid line: {line}"))?;
                let dir = match d {
                    "L" => Compass::West,
                    "R" => Compass::East,
                    "U" => Compass::North,
                    "D" => Compass::South,
                    _ => bail!("invalid direction: {d}"),
                };
                let n = n.parse::<usize>()?;

                Ok((dir, n))
            })
            .collect::<Result<Vec<(Compass, usize)>, anyhow::Error>>()?;
        Ok(Instructions(rows))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rope(Position, Position);

impl Rope {
    pub fn step(self, dir: Compass) -> Self {
        let Rope(head, tail) = self;
        let new_head = head + dir;
        let (dx, dy) = new_head - tail;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            return Rope(new_head, tail);
        }

        let new_tail = tail + (dx.signum(), dy.signum());

        Rope(new_head, new_tail)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LongRope(Vec<Position>);

impl LongRope {
    pub fn initial(size: usize) -> LongRope {
        assert!(size > 1);
        let positions = (0..size).map(|_| Position(0, 0)).collect::<Vec<Position>>();
        LongRope(positions)
    }

    pub fn step(&mut self, dir: Compass) -> usize {
        self.0[0] = *self.0.first().unwrap() + dir;
        for ix in 1..self.0.len() {
            let prev = self.0[ix - 1];
            let cur = self.0[ix];
            let (dx, dy) = prev - cur;

            if dx.abs() <= 1 && dy.abs() <= 1 {
                return ix;
            }

            self.0[ix] = cur + (dx.signum(), dy.signum());
        }

        self.0.len()
    }
}

pub struct Track {
    tail_visited: HashSet<Position>,
}

impl Track {
    pub fn follow(start: Rope, instructions: &Instructions) -> Self {
        let mut tail_visited = HashSet::new();
        let mut rope = start;
        tail_visited.insert(rope.1);

        for &(dir, n) in &instructions.0 {
            for _ in 0..n {
                rope = rope.step(dir);
                tail_visited.insert(rope.1);
            }
        }

        Track { tail_visited }
    }
    pub fn follow_long(start: LongRope, instructions: &Instructions) -> Self {
        let mut tail_visited = HashSet::new();
        let mut rope = start;

        tail_visited.insert(rope.0.last().copied().unwrap());

        for &(dir, n) in &instructions.0 {
            for _ in 0..n {
                rope.step(dir);
                tail_visited.insert(rope.0.last().copied().unwrap());
            }
        }

        Track { tail_visited }
    }
}

pub struct Day09(Instructions);

impl Solver for Day09 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let mut s = String::new();
        let mut input = input;
        input.read_to_string(&mut s)?;
        let instructions: Instructions = s.parse()?;

        Ok(Day09(instructions))
    }

    fn part_one(&self) -> String {
        let track = Track::follow(Rope::default(), &self.0);
        let visits = track.tail_visited.len();
        format!("{visits}")
    }

    fn part_two(&self) -> String {
        let track = Track::follow_long(LongRope::initial(10), &self.0);
        let visits = track.tail_visited.len();
        format!("{visits}")
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE1: &str = r"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";

    const EXAMPLE2: &str = r"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    ";

    #[test]
    fn test_rope_step() {
        let mut rope = Rope::default();
        rope = rope.step(Compass::East);
        assert_eq!(rope, Rope(Position(1, 0), Position(0, 0)));
        rope = rope.step(Compass::East);
        assert_eq!(rope, Rope(Position(2, 0), Position(1, 0)));
        rope = rope.step(Compass::South);
        assert_eq!(rope, Rope(Position(2, 1), Position(1, 0)));
        rope = rope.step(Compass::South);
        assert_eq!(rope, Rope(Position(2, 2), Position(2, 1)));
        rope = rope.step(Compass::West).step(Compass::West);
        assert_eq!(rope, Rope(Position(0, 2), Position(1, 2)));
    }

    #[test]
    fn test_long_rope_step() {
        let mut rope = LongRope::initial(2);
        rope.step(Compass::East);
        assert_eq!(rope, LongRope(vec![Position(1, 0), Position(0, 0)]));
        rope.step(Compass::East);
        assert_eq!(rope, LongRope(vec![Position(2, 0), Position(1, 0)]));
        rope.step(Compass::South);
        assert_eq!(rope, LongRope(vec![Position(2, 1), Position(1, 0)]));
        rope.step(Compass::South);
        assert_eq!(rope, LongRope(vec![Position(2, 2), Position(2, 1)]));
        rope.step(Compass::West);
        rope.step(Compass::West);
        assert_eq!(rope, LongRope(vec![Position(0, 2), Position(1, 2)]));
    }

    #[test]
    fn test_part_one() {
        let day = Day09::from_input(unindented(EXAMPLE1).unwrap().as_bytes()).unwrap();
        let track = Track::follow(Rope::default(), &day.0);
        let visits = track.tail_visited.len();
        assert_eq!(visits, 13);
    }

    #[test]
    fn test_part_two() {
        let day = Day09::from_input(unindented(EXAMPLE1).unwrap().as_bytes()).unwrap();
        let track = Track::follow_long(LongRope::initial(2), &day.0);
        let visits = track.tail_visited.len();
        assert_eq!(visits, 13);

        let day = Day09::from_input(unindented(EXAMPLE2).unwrap().as_bytes()).unwrap();
        let track = Track::follow_long(LongRope::initial(10), &day.0);
        let visits = track.tail_visited.len();
        assert_eq!(visits, 36);
    }
}
