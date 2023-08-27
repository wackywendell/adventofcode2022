use std::fmt;
use std::hash::Hash;
use std::ops::{self, Add, Mul, Neg};

/// The integer Value type used by the maps herein
pub type Value = i64;

/// 2D positions, in the form (East, South)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Position(pub Value, pub Value);

impl Position {
    pub fn manhattan(self) -> i64 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(Value, Value)> for Position {
    fn from((x, y): (Value, Value)) -> Self {
        Position(x, y)
    }
}

/// 2D compass directions
///
/// ```
/// use adventofcode2022::positions::{Position,Compass};
///
/// let origin = Position(0, 0);
/// assert_eq!(origin + Compass::North, Position(0, -1));
/// assert_eq!(origin + Compass::East, Position(1, 0));
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

impl Compass {
    pub const fn all() -> [Compass; 4] {
        [Compass::North, Compass::East, Compass::South, Compass::West]
    }
}

impl Mul<Value> for Compass {
    type Output = (Value, Value);

    fn mul(self, rhs: Value) -> Self::Output {
        match self {
            Compass::North => (0, -rhs),
            Compass::South => (0, rhs),
            Compass::East => (rhs, 0),
            Compass::West => (-rhs, 0),
        }
    }
}

impl Add<(Value, Value)> for Compass {
    type Output = (Value, Value);

    fn add(self, rhs: (Value, Value)) -> Self::Output {
        let (e, s) = rhs;
        match self {
            Compass::North => (e, s - 1),
            Compass::South => (e, s + 1),
            Compass::East => (e + 1, s),
            Compass::West => (e - 1, s),
        }
    }
}

impl From<Compass> for (Value, Value) {
    fn from(value: Compass) -> Self {
        match value {
            Compass::North => (0, -1),
            Compass::South => (0, 1),
            Compass::East => (1, 0),
            Compass::West => (-1, 0),
        }
    }
}

impl fmt::Display for Compass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Compass::North => "N",
            Compass::South => "S",
            Compass::East => "E",
            Compass::West => "W",
        };
        f.write_str(c)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Add<Compass> for Position {
    type Output = Self;

    fn add(self, rhs: Compass) -> Self {
        let Position(x, y) = self;
        match rhs {
            Compass::North => Position(x, y - 1),
            Compass::South => Position(x, y + 1),
            Compass::East => Position(x + 1, y),
            Compass::West => Position(x - 1, y),
        }
    }
}

impl ops::Add<(Value, Value)> for Position {
    type Output = Self;

    fn add(self, (dx, dy): (Value, Value)) -> Self {
        let Position(x, y) = self;
        Position(x + dx, y + dy)
    }
}

impl ops::Sub<Position> for Position {
    type Output = (Value, Value);

    fn sub(self, Position(x2, y2): Position) -> Self::Output {
        let Position(x1, y1) = self;
        (x1 - x2, y1 - y2)
    }
}

impl ops::Add<Turn> for Compass {
    type Output = Self;

    fn add(self, rhs: Turn) -> Self {
        match (self, rhs) {
            (Compass::North, Turn::Left) => Compass::West,
            (Compass::North, Turn::Right) => Compass::East,
            (Compass::South, Turn::Left) => Compass::East,
            (Compass::South, Turn::Right) => Compass::West,
            (Compass::East, Turn::Left) => Compass::North,
            (Compass::East, Turn::Right) => Compass::South,
            (Compass::West, Turn::Left) => Compass::South,
            (Compass::West, Turn::Right) => Compass::North,
            (Compass::North, Turn::Straight) => Compass::North,
            (Compass::South, Turn::Straight) => Compass::South,
            (Compass::East, Turn::Straight) => Compass::East,
            (Compass::West, Turn::Straight) => Compass::West,
            (Compass::North, Turn::Reverse) => Compass::South,
            (Compass::South, Turn::Reverse) => Compass::North,
            (Compass::East, Turn::Reverse) => Compass::West,
            (Compass::West, Turn::Reverse) => Compass::East,
        }
    }
}

/// Represents a turn in direction
///
/// ```
/// use adventofcode2022::positions::{Compass,Turn};
///
/// let dir = Compass::North;
/// assert_eq!(dir + Turn::Right, Compass::East);
/// assert_eq!(dir + Turn::Straight, Compass::North);
/// assert_eq!(dir + Turn::Reverse, Compass::South);
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Turn {
    Right,
    Straight,
    Left,
    Reverse,
}

impl Turn {
    fn as_i8(self) -> i8 {
        match self {
            Turn::Straight => 0,
            Turn::Left => 1,
            Turn::Reverse => 2,
            Turn::Right => -1,
        }
    }

    fn from_i8(n: i8) -> Self {
        match n.rem_euclid(4) {
            0 => Turn::Straight,
            1 => Turn::Left,
            2 => Turn::Reverse,
            3 => Turn::Right,
            _ => unreachable!("Unexpected value {n}"),
        }
    }
}

impl ops::Add<Turn> for Turn {
    type Output = Self;

    fn add(self, rhs: Turn) -> Self {
        let n = self.as_i8() + rhs.as_i8();
        Turn::from_i8(n)
    }
}

impl Neg for Turn {
    type Output = Turn;

    fn neg(self) -> Self::Output {
        match self {
            Turn::Right => Turn::Left,
            Turn::Straight => Turn::Reverse,
            Turn::Left => Turn::Right,
            Turn::Reverse => Turn::Straight,
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Turn::Right => "R",
            Turn::Straight => ">",
            Turn::Left => "L",
            Turn::Reverse => "<",
        };
        f.write_str(c)
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_compass_add() {
        let origin = Position(0, 0);
        assert_eq!(origin + Compass::North, Position(0, -1));
        assert_eq!(origin + Compass::East, Position(1, 0));
        assert_eq!(origin + Compass::South, Position(0, 1));
        assert_eq!(origin + Compass::West, Position(-1, 0));
    }

    #[test]
    fn test_compass_turn() {
        assert_eq!(Compass::North + Turn::Right, Compass::East);
        assert_eq!((Compass::North + Turn::Right) + Turn::Right, Compass::South);
        assert_eq!(Compass::North + (Turn::Right + Turn::Right), Compass::South);
        assert_eq!(Compass::West + Turn::Left, Compass::South);
        assert_eq!((Compass::West + Turn::Left) + Turn::Left, Compass::East);
        assert_eq!(Compass::West + (Turn::Left + Turn::Left), Compass::East);
        assert_eq!(
            Compass::West + (Turn::Left + Turn::Left + Turn::Left),
            Compass::North
        );
        assert_eq!(
            Compass::West + (Turn::Left + Turn::Left) + Turn::Left,
            Compass::North
        );
        assert_eq!(
            (Compass::West + Turn::Left) + (Turn::Left + Turn::Left),
            Compass::North
        );
        assert_eq!(
            (Compass::West + Turn::Left) + Turn::Left + (Turn::Left + Turn::Left),
            Compass::West
        );
    }
}
