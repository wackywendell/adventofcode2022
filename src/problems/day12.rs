use std::{
    collections::{BinaryHeap, HashMap},
    io::Read,
    ops::Index,
    str::FromStr,
};

use adventofcode2022::Position;
use anyhow::{anyhow, bail};

use super::{solutions::parse_from_read, Solver};

pub struct Day12(Grid);

impl Day12 {}

impl Solver for Day12 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let grid = parse_from_read(input)?;
        Ok(Day12(grid))
    }

    fn part_one(&self) -> String {
        let path = self.0.shortest_path_from_start();
        let steps = path.len() - 1;
        format!("{steps}")
    }

    fn part_two(&self) -> String {
        let path = self.0.shortest_path_from_lowest();
        let steps = path.len() - 1;
        format!("{steps}")
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid {
    rows: Vec<Vec<u8>>,
    start: Position,
    goal: Position,

    width: usize,
    height: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut goal = None;
        let mut width = 0;

        let mut rows = Vec::new();
        for (row, line) in s.trim().lines().enumerate() {
            let mut items = Vec::new();
            for (col, c) in line.trim().chars().enumerate() {
                let h = match c {
                    'S' => {
                        if start.is_some() {
                            bail!("multiple start positions");
                        }
                        start = Some(Position(col as i64, row as i64));
                        // start is at height 'a', or 0
                        0u8
                    }
                    'E' => {
                        if goal.is_some() {
                            bail!("multiple goal positions");
                        }
                        goal = Some(Position(col as i64, row as i64));
                        // Goal is at height 'z', or 25
                        25u8
                    }
                    'a'..='z' => (c as u8) - b'a',
                    _ => bail!("invalid character: {c}"),
                };
                items.push(h);
            }
            let row_width = items.len();
            rows.push(items);

            if row == 0 {
                width = row_width;
                continue;
            }
            if row_width != width {
                bail!("inconsistent row length");
            }
        }

        let start = start.ok_or_else(|| anyhow!("missing start position"))?;
        let goal = goal.ok_or_else(|| anyhow!("missing goal position"))?;
        let height = rows.len();

        Ok(Grid {
            rows,
            start,
            goal,
            width,
            height,
        })
    }
}

impl Index<Position> for Grid {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        let Position(c, r) = index;
        &self.rows[r as usize][c as usize]
    }
}

impl Grid {
    pub fn neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(4);
        let Position(x, y) = pos;

        if x > 0 {
            neighbors.push(Position(x - 1, y));
        }
        if x < self.width as i64 - 1 {
            neighbors.push(Position(x + 1, y));
        }
        if y > 0 {
            neighbors.push(Position(x, y - 1));
        }
        if y < self.height as i64 - 1 {
            neighbors.push(Position(x, y + 1));
        }

        neighbors
    }

    pub fn shortest_path_from_start(&self) -> Vec<Position> {
        self.shortest_path(vec![self.start])
    }

    pub fn shortest_path_from_lowest(&self) -> Vec<Position> {
        let mut starts = Vec::new();
        let lowest = self[self.start];
        for (r, row) in self.rows.iter().enumerate() {
            for (c, &h) in row.iter().enumerate() {
                if h > lowest {
                    continue;
                }
                starts.push(Position(c as i64, r as i64));
            }
        }

        self.shortest_path(starts)
    }

    pub fn shortest_path(&self, starts: Vec<Position>) -> Vec<Position> {
        // Position -> (Shortest distance, prev)
        let mut visited: HashMap<Position, (i64, Option<Position>)> = HashMap::new();
        let mut queue = BinaryHeap::new();

        for &start in &starts {
            queue.push(Visited {
                dist: 0,
                height: 0,
                prev: None,
                pos: start,
            });
        }

        while let Some(Visited {
            dist,
            height,
            prev,
            pos,
        }) = queue.pop()
        {
            if let Some(&(d, _)) = visited.get(&pos) {
                if d <= dist {
                    continue;
                }
            }

            visited.insert(pos, (dist, prev));

            if pos == self.goal {
                break;
            }

            for next in self.neighbors(pos) {
                let next_height = self[next];
                if next_height > height + 1 {
                    // Can't reach here
                    continue;
                }
                if Some(next) == prev {
                    // Don't go back, that's pointless
                    continue;
                }

                let next_dist = dist + 1;

                queue.push(Visited {
                    dist: next_dist,
                    height: next_height,
                    prev: Some(pos),
                    pos: next,
                });
            }
        }

        let mut path = Vec::new();
        let mut pos = Some(self.goal);
        while let Some(p) = pos {
            path.push(p);
            pos = visited[&p].1;
        }
        path.reverse();

        path
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Visited {
    pub dist: i64,
    pub height: u8,
    pub prev: Option<Position>,
    pub pos: Position,
}

impl Visited {
    // Key is (-dist, height, …) We want the "best" candidate to be the largest,
    // and that's the one with the shortest distance, tie-broken by the greatest
    // height
    fn as_key(&self) -> (i64, u8, Position, Option<Position>) {
        (-self.dist, self.height, self.pos, self.prev)
    }
}

impl Ord for Visited {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_key().cmp(&other.as_key())
    }
}

impl PartialOrd for Visited {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_key().cmp(&other.as_key()))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE: &str = r"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    fn example() -> Grid {
        unindented(EXAMPLE).unwrap().parse().unwrap()
    }

    #[test]
    fn test_parse() {
        let grid = example();
        assert_eq!(grid.start, Position(0, 0));
        assert_eq!(grid.goal, Position(5, 2));
        assert_eq!(grid.width, 8);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.rows.len(), 5);
        assert_eq!(grid[grid.start], 0);
        assert_eq!(grid[grid.goal], 25);
        assert_eq!(grid[Position(0, 0)], 0);
        assert_eq!(grid[Position(0, 1)], 0);
        assert_eq!(grid[Position(1, 0)], 0);
        assert_eq!(grid[Position(1, 1)], 1);
        assert_eq!(grid[Position(0, 2)], 0);
        assert_eq!(grid[Position(2, 0)], 1);
    }

    #[test]
    fn test_neighbors() {
        let grid = example();
        let neighbors = grid.neighbors(Position(0, 0));
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position(1, 0)));
        assert!(neighbors.contains(&Position(0, 1)));

        let mut neighbors = grid.neighbors(Position(5, 2));
        assert_eq!(neighbors.len(), 4);
        neighbors.sort();
        assert_eq!(
            neighbors,
            vec![
                Position(4, 2),
                Position(5, 1),
                Position(5, 3),
                Position(6, 2),
            ]
        );
    }

    #[test]
    fn test_shortest_path() {
        let grid = example();
        let path = grid.shortest_path_from_start();
        // should be done in 31 steps = 32 positions
        assert_eq!(path.len(), 32);
        assert_eq!(path[0], grid.start);
        assert_eq!(path[31], grid.goal);
    }

    #[test]
    fn shortest_path_from_lowest() {
        let grid = example();
        let path = grid.shortest_path_from_lowest();
        // should be done in 31 steps = 32 positions
        assert_eq!(path.len(), 30);
        assert_eq!(grid[path[0]], 0);
        assert_eq!(path[29], grid.goal);
    }

    #[test]
    fn test_part_one() {
        let day = Day12::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.part_one(), "31");
    }

    #[test]
    fn test_part_two() {
        let day = Day12::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.part_two(), "29");
    }
}
