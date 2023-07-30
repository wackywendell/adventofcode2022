use std::io::Read;

use super::{solutions::parse_lines, Solver};

pub struct Day04(Vec<i64>);

impl Day04 {
    pub fn max(&self) -> i64 {
        self.0.iter().copied().max().unwrap_or_default()
    }

    pub fn sum(&self) -> i64 {
        self.0.iter().copied().sum::<i64>()
    }
}

impl Solver for Day04 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let items = parse_lines::<i64>(input)?;

        Ok(Day04(items))
    }

    fn part_one(&self) -> String {
        self.max();
        unimplemented!()
    }

    fn part_two(&self) -> String {
        self.sum();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindent;

    use super::*;

    const EXAMPLE: &str = r"
        1
        9
        4
    ";

    #[test]
    fn test_part_one() {
        let day = Day04::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.max(), 9);
    }

    #[test]
    fn test_part_two() {
        let day = Day04::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.sum(), 14);
    }
}
