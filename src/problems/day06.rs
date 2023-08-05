use std::{collections::VecDeque, io::Read, str::FromStr};

use log::{debug, info};

use super::Solver;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Message(String);

impl FromStr for Message {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Message(s.to_owned()))
    }
}

impl Message {
    pub fn find_no_repeats(&self, duplicates: usize) -> Option<usize> {
        info!("Checking {}", self.0);
        let mut seen = VecDeque::with_capacity(duplicates);
        let mut last_dup: isize = -1;
        for (ix, c) in self.0.chars().enumerate() {
            seen.push_back(c);
            if seen.len() > duplicates {
                seen.pop_front();
            }

            for (pix, &p) in seen.iter().rev().skip(1).enumerate() {
                if p == c {
                    last_dup = last_dup.max((ix - pix) as isize - 1);
                    debug!("Found duplicate {c}-{p} at {last_dup} = {ix} - {pix} - 1");
                    break;
                }
            }

            debug!("Finishing {ix}:{c}, with last at {last_dup}");
            if (last_dup + duplicates as isize) <= (ix as isize) {
                return Some(ix + 1);
            }
        }

        None
    }
}

pub struct Day06(Message);

impl Solver for Day06 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let mut input = input;
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;

        Ok(Day06(Message(buf.trim().to_owned())))
    }

    fn part_one(&self) -> String {
        let ix = self.0.find_no_repeats(4).unwrap_or(self.0 .0.len());
        format!("{ix}")
    }

    fn part_two(&self) -> String {
        let ix = self.0.find_no_repeats(14).unwrap_or(self.0 .0.len());
        format!("{ix}")
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE: &str = r"
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
        bvwbjplbgvbhsrlpgdmjqwftvncz
        nppdvjthqldpwncqszvftbrmjlhg
        nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
        zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
    ";

    #[test]
    fn test_part_one() {
        let unexamples = unindented(EXAMPLE).unwrap();
        let messages = unexamples
            .lines()
            .map(|l| Message(l.trim().to_owned()))
            .collect::<Vec<Message>>();
        let expected = vec![7, 5, 6, 10, 11];

        for (s, e) in messages.iter().zip(expected) {
            let g = s.find_no_repeats(4);
            assert_eq!(g, Some(e), "Analyzing {}: Got {:?}, expected {}", s.0, g, e);
        }
    }

    #[test]
    fn test_part_two() {
        let unexamples = unindented(EXAMPLE).unwrap();
        let messages = unexamples
            .lines()
            .map(|l| Message(l.trim().to_owned()))
            .collect::<Vec<Message>>();
        let expected = vec![19, 23, 23, 29, 26];

        for (s, e) in messages.iter().zip(expected) {
            let g = s.find_no_repeats(14);
            assert_eq!(g, Some(e), "Analyzing {}: Got {:?}, expected {}", s.0, g, e);
        }
    }
}
