use std::{collections::HashSet, io::Read, panic, str::FromStr};

use anyhow::bail;

use super::Solver;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid {
    rows: Vec<Vec<u8>>,
}
impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|n| n as u8)
                            .ok_or_else(|| anyhow::anyhow!("invalid digit: {c}"))
                    })
                    .collect::<Result<Vec<u8>, anyhow::Error>>()
            })
            .collect::<Result<Vec<Vec<u8>>, anyhow::Error>>()?;

        if rows.is_empty() {
            bail!("empty grid");
        }

        let width = rows[0].len();
        if !rows.iter().all(|row| row.len() == width) {
            bail!("inconsistent row widths");
        }

        Ok(Grid { rows })
    }
}

impl Grid {
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.rows[0].len()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.rows.get(row).and_then(|row| row.get(col)).copied()
    }

    pub fn row_iter(&self, row: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
        self.rows[row].iter().copied()
    }

    pub fn col_iter(&self, col: usize) -> impl DoubleEndedIterator<Item = u8> + '_ {
        self.rows.iter().map(move |row| row[col])
    }

    pub fn maxes(&self, arr: impl Iterator<Item = u8>) -> impl Iterator<Item = (usize, u8)> {
        let mut max = None;
        arr.enumerate().filter_map(move |(ix, val)| {
            let ret: Option<(usize, u8)>;
            (max, ret) = match max {
                Some(mx) if mx >= val => (max, None),
                _ => (Some(val), Some((ix, val))),
            };
            ret
        })
    }

    pub fn visible(&self) -> HashSet<(usize, usize, u8)> {
        let rows = self.row_count();
        let mut vis = HashSet::new();

        let checker = |row_ix, col_ix, val| {
            #[cfg(debug_assertions)]
            {
                let found = self.get(row_ix, col_ix);
                if found != Some(val) {
                    panic!(
                        "expected {:?} at ({}, {}), found {:?}",
                        val, row_ix, col_ix, found
                    );
                }
            }
        };

        for row_ix in 0..rows {
            for (col_ix, val) in self.maxes(self.row_iter(row_ix)) {
                checker(row_ix, col_ix, val);
                vis.insert((row_ix, col_ix, val));
            }
            for (col_rev_ix, val) in self.maxes(self.row_iter(row_ix).rev()) {
                let col_ix = self.col_count() - col_rev_ix - 1;
                checker(row_ix, col_ix, val);
                vis.insert((row_ix, col_ix, val));
            }
        }

        let cols = self.col_count();
        for col_ix in 0..cols {
            for (row_ix, val) in self.maxes(self.col_iter(col_ix)) {
                checker(row_ix, col_ix, val);
                vis.insert((row_ix, col_ix, val));
            }
            for (row_rev_ix, val) in self.maxes(self.col_iter(col_ix).rev()) {
                let row_ix = self.row_count() - row_rev_ix - 1;
                checker(row_ix, col_ix, val);
                vis.insert((row_ix, col_ix, val));
            }
        }

        vis
    }

    fn visibility(iter: impl Iterator<Item = u8>, height: u8) -> usize {
        let mut vis: usize = 0;
        for t in iter {
            vis += 1;
            if t >= height {
                break;
            }
        }
        vis
    }

    // Find viewable trees in each direction, starting above and going clockwise.
    pub fn find_distances(&self, row_ix: usize, col_ix: usize) -> [usize; 4] {
        let height = self.rows[row_ix][col_ix];

        let rc = self.row_count();
        let above = self.col_iter(col_ix).rev().skip(rc - row_ix);
        let above = Self::visibility(above, height);
        let below = self.col_iter(col_ix).skip(row_ix + 1);
        let below = Self::visibility(below, height);

        let cc: usize = self.col_count();
        let left = self.row_iter(row_ix).rev().skip(cc - col_ix);
        let left = Self::visibility(left, height);
        let right = self.row_iter(row_ix).skip(col_ix + 1);
        let right = Self::visibility(right, height);

        [above, right, below, left]
    }

    pub fn scenic_score(&self, row_ix: usize, col_ix: usize) -> usize {
        let dists = self.find_distances(row_ix, col_ix);
        dists.iter().product()
    }

    // Find the most scenic spot on the map. Returns (row, col, score)
    pub fn most_scenic(&self) -> (usize, usize, usize) {
        let mut best = (0, 0, 0);
        for row_ix in 0..self.row_count() {
            for col_ix in 0..self.col_count() {
                let score = self.scenic_score(row_ix, col_ix);
                if score > best.2 {
                    best = (row_ix, col_ix, score);
                }
            }
        }
        best
    }
}

pub struct Day08(Grid);

impl Day08 {
    pub fn visible_count(&self) -> usize {
        self.0.visible().len()
    }
}

impl Solver for Day08 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let mut s = String::new();
        let mut input = input;
        input.read_to_string(&mut s)?;
        let grid: Grid = s.parse()?;

        Ok(Day08(grid))
    }

    fn part_one(&self) -> String {
        let vis = self.visible_count();
        format!("{vis}")
    }

    fn part_two(&self) -> String {
        let (_r, _c, s) = self.0.most_scenic();
        format!("{s}")
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE: &str = r"
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn test_part_one() {
        let day = Day08::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        let grid = &day.0;
        let vis = grid.visible();
        for (rix, cix, v) in vis {
            let found = grid.get(rix, cix).unwrap();
            assert_eq!(v, found, "({rix}, {cix}) = {found} != {v}")
        }

        let vc = day.visible_count();
        assert_eq!(vc, 21);
    }

    #[test]
    fn test_part_two() {
        let day = Day08::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        let grid = &day.0;

        let dists = grid.find_distances(0, 2);
        assert_eq!(dists, [0, 1, 1, 2]);

        let dists = grid.find_distances(1, 2);
        assert_eq!(dists, [1, 2, 2, 1]);

        let dists = grid.find_distances(3, 2);
        assert_eq!(dists, [2, 2, 1, 2]);
        assert_eq!(grid.scenic_score(3, 2), 8);
        assert_eq!(grid.most_scenic(), (3, 2, 8));
    }
}
