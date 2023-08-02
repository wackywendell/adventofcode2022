use std::{io::Read, str::FromStr};

use anyhow::bail;

use super::Solver;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crate(char);

impl FromStr for Crate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            bail!(
                "Expected len 3 for the form '[c]', got {} for '{}'",
                s.len(),
                s
            );
        }
        let mut cs = s.chars();
        let c1 = cs.next().unwrap();
        let c2 = cs.next().unwrap();
        let c3 = cs.next().unwrap();

        if c1 != '[' || c3 != ']' {
            bail!("Expected the form '[c]'");
        };
        if !c2.is_ascii_alphabetic() {
            bail!("Expected an alphabetic character");
        }

        Ok(Crate(c2))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stack {
    crates: Vec<Vec<Crate>>,
    instructions: Vec<Instruction>,
}

impl FromStr for Stack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = Vec::new();

        let mut lines = s.lines();
        for (n, line) in lines.by_ref().enumerate() {
            let line = line.trim_matches('\n');
            let len = line.len();
            if len % 4 != 3 {
                bail!("Invalid line length {}: {}", len, line);
            }
            let ncrates = (len + 1) / 4;
            if crates.is_empty() {
                crates.resize(ncrates, Vec::new());
            } else if ncrates != crates.len() {
                bail!("Invalid number of crates on line {}: {}", n, ncrates);
            }

            let mut finished = false;
            for (m, chunk) in line.as_bytes().chunks(4).enumerate() {
                let chunk = std::str::from_utf8(chunk)?;
                let trimmed = chunk.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if trimmed.len() == 1 {
                    let c = trimmed.chars().next().unwrap();
                    if (c as u8) == (m as u8 + b'1') {
                        finished = true;
                        continue;
                    }
                    bail!(
                        "Unexpected line after crates: could not parse '{}' of '{}'",
                        trimmed,
                        line
                    )
                }

                let c: Crate = trimmed.parse()?;
                crates[m].push(c);
            }
            if finished {
                break;
            }
        }

        // Reverse the crates, so the top crate is at the end
        for c in crates.iter_mut() {
            c.reverse()
        }

        let mut instructions = Vec::new();

        for line in lines {
            let line = line.trim();
            if line.trim().is_empty() {
                continue;
            }

            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 6 {
                bail!("Invalid instruction split: {}", line);
            }
            if parts[0] != "move" || parts[2] != "from" || parts[4] != "to" {
                bail!("Invalid instruction: {}", line);
            }

            let cnt = parts[1].parse::<usize>()?;
            let from = parts[3].parse::<usize>()?;
            let to = parts[5].parse::<usize>()?;
            let instr = Instruction {
                from,
                to,
                count: cnt,
            };
            instructions.push(instr);
        }

        Ok(Stack {
            crates,
            instructions,
        })
    }
}

fn get_two_refs<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    if a == b {
        panic!("Cannot get two references to the same element");
    }
    if a > b {
        let (b, a) = get_two_refs(slice, b, a);
        return (a, b);
    }

    let (left, right) = slice.split_at_mut(b);
    (&mut left[a], &mut right[0])
}

impl Stack {
    pub fn tops(&self) -> String {
        let mut tops = String::new();
        for c in self.crates.iter() {
            if let Some(top) = c.last() {
                tops.push(top.0);
            } else {
                tops.push(' ');
            }
        }
        tops
    }

    pub fn apply(&mut self, instr: Instruction) {
        let Instruction { from, to, count } = instr;

        let (stack, stack_to) = get_two_refs(&mut self.crates, from - 1, to - 1);

        for _ in 0..count {
            let c = stack.pop().unwrap();
            stack_to.push(c);
        }
    }

    pub fn apply_multiple(&mut self, instr: Instruction) {
        let Instruction { from, to, count } = instr;

        let (stack, stack_to) = get_two_refs(&mut self.crates, from - 1, to - 1);

        let mut boxes = Vec::new();
        for _ in 0..count {
            let c = stack.pop().unwrap();
            boxes.push(c);
        }

        stack_to.extend(boxes.into_iter().rev());
    }

    pub fn apply_all(&mut self) {
        let instructions = std::mem::take(&mut self.instructions);
        for instr in instructions {
            self.apply(instr);
        }
    }

    pub fn apply_all_multiple(&mut self) {
        let instructions = std::mem::take(&mut self.instructions);
        for instr in instructions {
            self.apply_multiple(instr);
        }
    }
}

pub struct Day05(Stack);

impl Solver for Day05 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let mut buf = String::new();
        let mut input = input;
        input.read_to_string(&mut buf)?;
        let stack: Stack = buf.parse()?;

        Ok(Day05(stack))
    }

    fn part_one(&self) -> String {
        let mut stack = self.0.clone();
        stack.apply_all();
        stack.tops()
    }

    fn part_two(&self) -> String {
        let mut stack = self.0.clone();
        stack.apply_all_multiple();
        stack.tops()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindent;

    use super::*;

    const EXAMPLE: &str = r"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        let mut day = Day05::from_input(unindent(EXAMPLE, 8).unwrap().as_bytes()).unwrap();
        let crates = &day.0.crates;
        let instructions = &day.0.instructions;
        assert_eq!(crates.len(), 3);
        assert_eq!(day.0.tops(), "NDP");

        assert_eq!(instructions.len(), 4);
        day.0.apply_all();
        assert_eq!(day.0.tops(), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let mut day = Day05::from_input(unindent(EXAMPLE, 8).unwrap().as_bytes()).unwrap();
        day.0.apply_all_multiple();
        assert_eq!(day.0.tops(), "MCD");
    }
}
