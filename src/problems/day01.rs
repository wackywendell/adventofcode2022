use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{BufRead, BufReader, Read},
};

use super::Solver;

pub struct Elves {
    calories: Vec<Vec<i64>>,
}

impl Elves {
    pub fn read(input: impl Read) -> anyhow::Result<Self> {
        let mut calories = Vec::new();
        let buf = BufReader::new(input);

        let mut cur = Vec::new();
        for line in buf.lines() {
            let line = line?;
            let line = line.trim_end_matches('\n');
            if line.is_empty() {
                calories.push(std::mem::take(&mut cur));
                continue;
            }

            let n: i64 = line.parse()?;
            cur.push(n);
        }

        calories.push(cur);
        Ok(Elves { calories })
    }

    pub fn sums(&self) -> impl Iterator<Item = i64> + '_ {
        self.calories.iter().map(|c| c.iter().sum())
    }

    pub fn maxes(&self, n: usize) -> Vec<i64> {
        let mut maxes = BinaryHeap::new();
        for cals in self.sums() {
            maxes.push(Reverse(cals));
            if maxes.len() > n {
                maxes.pop();
            }
        }

        let mut out: Vec<_> = maxes.into_iter().map(|Reverse(c)| c).collect();
        out.sort();
        out
    }
}

pub struct Day01(Elves);

impl Solver for Day01 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        Ok(Day01(Elves::read(input)?))
    }

    fn part_one(&self) -> String {
        self.0.sums().max().unwrap_or_default().to_string()
    }

    fn part_two(&self) -> String {
        self.0.maxes(3).iter().sum::<i64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::super::testfns::unindent;
    use super::*;

    const EXAMPLE: &str = r"
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
    ";

    #[test]
    pub fn parse() {
        let elves = Elves::read(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();

        assert_eq!(elves.calories.len(), 5);
    }

    #[test]
    pub fn basic() {
        let elves = Elves::read(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();
        let maxes = elves.maxes(3);

        assert_eq!(maxes, vec![24000, 11000, 10000]);
    }

    #[test]
    pub fn solving() {
        let solver = Day01::from_input(unindent(EXAMPLE).unwrap().as_bytes()).unwrap();

        let out = solver.part_one();
        assert_eq!(out, "24000");

        let out = solver.part_two();
        assert_eq!(out, "45000");
    }
}
