use std::{io::Read, str::FromStr};

use super::{solutions::parse_lines, Solver};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item(char);

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            anyhow::bail!("Expected a single character");
        }
        let c = s.chars().next().unwrap();
        if !c.is_ascii_alphabetic() {
            anyhow::bail!("Expected an alphabetic character");
        }
        Ok(Item(c))
    }
}

impl Item {
    pub fn priority(self) -> i64 {
        let Item(c) = self;
        if c.is_ascii_lowercase() {
            self.0 as i64 - 'a' as i64 + 1
        } else if c.is_ascii_uppercase() {
            self.0 as i64 - 'A' as i64 + 27
        } else {
            panic!("Unexpected character '{c}'")
        }
    }
}

pub struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    pub fn splits(&self) -> (&[Item], &[Item]) {
        let len = self.items.len();
        let mid = len / 2;
        (&self.items[..mid], &self.items[mid..])
    }

    pub fn find_duplicate(&self) -> Option<Item> {
        let (first, second) = self.splits();

        for c in second {
            if first.contains(c) {
                return Some(*c);
            }
        }

        None
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    Ok(Item(c))
                } else {
                    Err(anyhow::anyhow!("Expected alphabetic character, got {c}"))
                }
            })
            .collect::<Result<Vec<Item>, _>>()?;
        Ok(Rucksack { items })
    }
}

pub struct Day03(Vec<Rucksack>);

impl Solver for Day03 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let rucksacks = parse_lines::<Rucksack>(input)?;

        Ok(Day03(rucksacks))
    }

    fn part_one(&self) -> String {
        let mut total = 0;
        for r in &self.0 {
            let c = r.find_duplicate().unwrap();
            total += c.priority();
        }

        format!("{}", total)
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
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn basic() {
        let solver = Day03::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        let rucksacks = &solver.0;

        let scores = vec![16, 38, 42, 22, 20, 19];
        assert_eq!(rucksacks.len(), scores.len());

        for (r, s) in rucksacks.iter().zip(scores) {
            assert_eq!(r.find_duplicate().unwrap().priority(), s);
        }

        let s = solver.part_one();
        assert_eq!(&s, "157");
    }
}
