use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    io::Read,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::{bail, Context};

use super::Solver;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsOutput {
    File(String, i64),
    Dir(String),
}

impl FromStr for LsOutput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("Expected a space in ls output {}", s))?;
        if first == "dir" {
            return Ok(LsOutput::Dir(second.to_owned()));
        }
        let size: i64 = first.parse().with_context(|| {
            format!(
                "Expected ls output to start with a number or \"dir\" on line '{}'",
                s
            )
        })?;

        Ok(LsOutput::File(second.to_owned(), size))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path(Vec<String>);

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.strip_prefix('/') {
            None => bail!("Expected path {s} to start with a slash"),
            Some(s) => s,
        };
        Ok(Path(
            s.split('/')
                // .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect(),
        ))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "/");
        }
        for part in &self.0 {
            write!(f, "/{part}")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathChange {
    Root,
    Parent,
    Relative(String),
}

impl AddAssign<PathChange> for Path {
    fn add_assign(&mut self, rhs: PathChange) {
        match rhs {
            PathChange::Root => {
                self.0.clear();
            }
            PathChange::Parent => {
                self.0.pop();
            }
            PathChange::Relative(s) => {
                self.0.push(s);
            }
        }
    }
}

impl Add<PathChange> for Path {
    type Output = Path;

    fn add(mut self, rhs: PathChange) -> Self::Output {
        self += rhs;
        self
    }
}

impl FromStr for PathChange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "/" {
            return Ok(PathChange::Root);
        }
        if s == ".." {
            return Ok(PathChange::Parent);
        }
        Ok(PathChange::Relative(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    // cd _to_ path
    CD(PathChange),
    // ls, with output
    Ls(Vec<LsOutput>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Session {
    pub commands: Vec<Command>,
}

impl FromStr for Session {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();

        let mut commands = Vec::new();

        while let Some(cmd_line) = lines.next() {
            let cmd_line = cmd_line.trim();
            if let Some(path_str) = cmd_line.strip_prefix("$ cd ") {
                let path = str::parse(path_str.trim())?;
                commands.push(Command::CD(path));
                continue;
            }

            if cmd_line == "$ ls" {
                let mut ls_output = Vec::new();
                while let Some(line) = lines.peek() {
                    if line.starts_with('$') {
                        break;
                    }
                    ls_output.push(str::parse(line.trim())?);
                    lines.next();
                }
                commands.push(Command::Ls(ls_output));
                continue;
            }

            bail!("Unknown command {}", cmd_line);
        }

        Ok(Session { commands })
    }
}

impl Session {
    pub fn run(&self) -> Filesystem {
        let mut fs = Filesystem {
            files: HashMap::new(),
        };

        let mut current_dir = Path(Vec::new());

        for cmd in &self.commands {
            match cmd {
                Command::CD(path) => {
                    current_dir += path.clone();
                }
                Command::Ls(ls_output) => {
                    fs.files.insert(current_dir.clone(), ls_output.clone());
                }
            }
        }

        fs
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Filesystem {
    pub files: HashMap<Path, Vec<LsOutput>>,
}

impl Filesystem {
    pub fn total_sizes(&self) -> HashMap<Path, Option<i64>> {
        let mut sizes = HashMap::new();
        let mut seen = HashSet::new();
        let mut queue = vec![Path(Vec::new())];

        while let Some(mut path) = queue.pop() {
            log::debug!("Processing {}", path);
            seen.insert(path.clone());

            let mut size = 0;
            for output in self.files.get(&path).unwrap_or(&Vec::new()) {
                match output {
                    LsOutput::File(_, s) => size += s,
                    LsOutput::Dir(c) => {
                        let mut child = path.clone();
                        child += PathChange::Relative(c.to_string());
                        queue.push(child);
                    }
                }
            }

            let entry = sizes.entry(path.clone());
            match entry {
                Entry::Occupied(_e) => {
                    unreachable!("Paths should not be seen twice: {:?}", path);
                }
                Entry::Vacant(e) => e.insert(Some(size)),
            };

            while path.0.pop().is_some() {
                let sz: &mut Option<i64> = sizes.entry(path.clone()).or_default();
                // If we haven't seen the parent yet... it's because we visited
                // it with a cd but didn't do an ls. So we can't know the true size.
                if let Some(sz) = sz {
                    *sz += size;
                }
            }
        }

        sizes
    }
}

pub struct Day07(Filesystem);

impl Day07 {
    pub fn small_dir_sum(&self, cutoff: i64) -> i64 {
        let sizes = self.0.total_sizes();

        let mut sum = 0;
        for (path, &size) in &sizes {
            if let Some(size) = size {
                if size < cutoff {
                    log::debug!("{}: {}", path, size);
                    sum += size;
                }
            }
        }

        sum
    }
}

impl Solver for Day07 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let mut s = String::new();
        let mut input = input;
        input.read_to_string(&mut s)?;
        let session: Session = s.parse()?;
        let fs = session.run();

        Ok(Day07(fs))
    }

    fn part_one(&self) -> String {
        let sum = self.small_dir_sum(100_000);
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

    const EXAMPLE: &str = r"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    ";

    #[test]
    fn test_session() {
        let session = Session::from_str(unindented(EXAMPLE).unwrap().as_str()).unwrap();
        assert_eq!(session.commands.len(), 10);

        let fs = session.run();
        let sizes = fs.total_sizes();
        assert_eq!(sizes.len(), 4);
        let p = Path::from_str("/a/e").unwrap();
        assert_eq!(sizes[&p], Some(584));
        let p = Path::from_str("/a/e").unwrap();
        assert_eq!(sizes[&p], Some(584));
    }

    #[test]
    fn test_part_one() {
        let session = Session::from_str(unindented(EXAMPLE).unwrap().as_str()).unwrap();
        let fs = session.run();
        let day = Day07(fs);
        assert_eq!(day.small_dir_sum(100_000), 95437);
    }

    #[test]
    fn test_part_two() {
        // let day = Day07::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        // assert_eq!(day.sum(), 14);
    }
}
