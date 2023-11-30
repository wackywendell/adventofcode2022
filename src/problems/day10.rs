use std::{collections::VecDeque, io::Read, str::FromStr};

use super::{solutions::parse_lines, Solver};

use anyhow::{anyhow, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        let (op, arg) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid line: {s}"))?;
        let arg = arg.parse::<i64>()?;

        let op = match op {
            "addx" => Instruction::Addx(arg),
            _ => bail!("invalid operation: {s}"),
        };

        Ok(op)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Computer {
    instructions: VecDeque<Instruction>,
    stack: Option<Instruction>,
    register: i64,
    cycles: usize,
    width: usize,
}

impl Computer {
    pub fn new<T: Into<VecDeque<Instruction>>>(instructions: T, width: usize) -> Self {
        Computer {
            instructions: instructions.into(),
            stack: None,
            register: 1,
            cycles: 0,
            width,
        }
    }

    pub fn step(&mut self) -> Option<i64> {
        self.cycles += 1;
        if let Some(op) = self.stack.take() {
            match op {
                Instruction::Noop => {
                    unreachable!("noop should never be on the stack")
                }
                Instruction::Addx(n) => self.register += n,
            }
            return Some(self.register);
        }

        let op = self.instructions.pop_front()?;
        if op == Instruction::Noop {
            return Some(self.register);
        }

        self.stack = Some(op);
        Some(self.register)
    }

    pub fn cycle(&mut self) -> Option<bool> {
        let reg = self.register;
        self.step()?;
        let loc = (self.cycles % self.width) as i64;
        Some(loc.abs_diff(reg) <= 1)
    }

    pub fn draw(&mut self) -> String {
        let mut output = String::new();
        while let Some(d) = self.cycle() {
            if self.cycles > 1 && (self.cycles - 1) % self.width == 0 {
                // XXX: Why > 1? It seemed to work, _shrug_
                output.push('\n');
            }

            let c = if d { '#' } else { '.' };
            output.push(c);
        }

        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day10(Computer);

impl Day10 {
    pub fn strengths(&mut self, init: usize, step: usize) -> Vec<i64> {
        let computer = &mut self.0;

        for _ in 0..init - 1 {
            if computer.step().is_none() {
                return vec![];
            }
        }

        let mut values = vec![computer.register * (init as i64)];
        let mut cycle_no = init;
        loop {
            for _ in 0..step {
                if computer.step().is_none() {
                    return values;
                }
                cycle_no += 1;
            }
            values.push(computer.register * (cycle_no as i64));
        }
    }

    pub fn strength_sum(&mut self, init: usize, step: usize) -> i64 {
        self.strengths(init, step).iter().sum()
    }
}

impl Solver for Day10 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let instructions = parse_lines::<Instruction>(input)?;

        Ok(Day10(Computer::new(instructions, 40)))
    }

    fn part_one(&self) -> String {
        let mut day = self.clone();
        let sum = day.strength_sum(20, 40);
        format!("{sum}")
    }

    fn part_two(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE_SMALL: &str = r"
        noop
        addx 3
        addx -5
    ";

    const EXAMPLE_1: &str = r"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    ";

    #[test]
    fn test_parse() {
        let day = Day10::from_input(unindented(EXAMPLE_SMALL).unwrap().as_bytes()).unwrap();
        let instructions = day.0.instructions;
        let expected = VecDeque::from(vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ]);
        assert_eq!(expected, instructions);

        let day = Day10::from_input(unindented(EXAMPLE_1).unwrap().as_bytes()).unwrap();
        let instructions = day.0.instructions;
        let expected = VecDeque::from(vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
        ]);

        let first_bit: VecDeque<Instruction> = instructions.into_iter().take(3).collect();

        assert_eq!(expected, first_bit);
    }

    #[test]
    fn test_apply() {
        let mut day = Day10::from_input(unindented(EXAMPLE_1).unwrap().as_bytes()).unwrap();
        let values = day.strengths(20, 40);
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], values);
    }

    const EXPECTED_1: &str = r"
        ##..##..#..##...##.##..##..##..##..##..#
        ###..####..###...###...####..####..###..
        ##.....#####...###.....####.....##.....#
        ####.....##.#......#####.....####.......
        ###.##.....##.###......######......##.#.
        ###.##.......####.#........########.....
    ";

    #[test]
    fn test_part_one() {
        let mut day = Day10::from_input(unindented(EXAMPLE_1).unwrap().as_bytes()).unwrap();
        let output = day.0.draw();
        // println!("{}", output);
        assert_eq!(unindented(EXPECTED_1).unwrap(), output);
    }
}
