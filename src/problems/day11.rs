use std::{
    collections::{BTreeMap, VecDeque},
    io::Read,
    str::FromStr,
};

use super::{solutions::parse_from_read, Solver};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "problems/day11.pest"]
pub struct MonkeyParser;

pub struct Day11(Monkeys);

impl Solver for Day11 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let monkeys: Monkeys = parse_from_read(input)?;

        Ok(Day11(monkeys))
    }

    fn part_one(&self) -> String {
        let mut monkeys = self.0.clone();
        monkeys.rounds(3).take(20).count();

        let (mx1, mx2) = monkeys.two_maxes();
        let mul = mx1 * mx2;
        format!("{mul}")
    }

    fn part_two(&self) -> String {
        let mut monkeys = self.0.clone();
        monkeys.rounds(1).take(10000).count();

        let (mx1, mx2) = monkeys.two_maxes();
        let mul = mx1 * mx2;
        format!("{mul}")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => anyhow::bail!("Unknown operation: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord, Hash)]
pub struct MonkeyId(i64);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Monkey {
    id: MonkeyId,
    items: VecDeque<i64>,
    operation: Operation,
    // Operand is None if it is 'old'
    operand: Option<i64>,
    test: i64,
    true_branch: MonkeyId,
    false_branch: MonkeyId,
}

fn to_number(pair: Pair<Rule>) -> anyhow::Result<i64> {
    if pair.as_rule() != Rule::number {
        anyhow::bail!("Expected number, got {:?}", pair.as_rule());
    }
    pair.as_str().parse::<i64>().map_err(Into::into)
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = MonkeyParser::parse(Rule::Monkey, s)?.next().unwrap();
        Ok(Self::process(pair))
    }
}

impl Monkey {
    fn catch(&mut self, worry: i64) {
        self.items.push_back(worry);
    }

    // Take a Pair<Rule> where the rule is Monkey and return a Monkey struct.
    //
    // Panics if the input rule is not Rule::Monkey.
    // (Other panics possible if there are bugs in conversion from AST to Monkey.)
    fn process(pair: Pair<Rule>) -> Monkey {
        if pair.as_rule() != Rule::Monkey {
            panic!("Expected Monkey, got {:?}", pair.as_rule());
        }

        let mut lines = pair.into_inner();

        let id_line = lines.next().expect("Expected MonkeyId");
        if id_line.as_rule() != Rule::monkeyId {
            panic!("Expected MonkeyId, got {:?}", id_line.as_rule());
        }

        let id = MonkeyId(to_number(id_line.into_inner().next().unwrap()).unwrap());

        let starting_line = lines.next().expect("Expected MonkeyId");

        if starting_line.as_rule() != Rule::startingLine {
            panic!("Expected StartingItems, got {:?}", starting_line.as_rule());
        }

        let starting_items = starting_line
            .into_inner()
            .map(to_number)
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap();

        let operation_line = lines.next().expect("Expected OperationLine");

        if operation_line.as_rule() != Rule::operationLine {
            panic!("Expected OperationLine, got {:?}", operation_line.as_rule());
        }

        let mut op_elements = operation_line.into_inner();
        let operation = op_elements
            .next()
            .expect("Expected Operation, got")
            .as_str()
            .parse::<Operation>()
            .unwrap();

        let op_string = op_elements.next().expect("Expected Operation");
        let operand = if op_string.as_str() == "old" {
            None
        } else {
            Some(to_number(op_string).unwrap())
        };

        let test_line = lines.next().expect("Expected TestLine");

        if test_line.as_rule() != Rule::testLine {
            panic!("Expected TestLine, got {:?}", test_line.as_rule());
        }

        let mut test_elements = test_line.into_inner();
        let test = to_number(test_elements.next().unwrap()).unwrap();
        let true_branch = MonkeyId(to_number(test_elements.next().unwrap()).unwrap());
        let false_branch = MonkeyId(to_number(test_elements.next().unwrap()).unwrap());

        Monkey {
            id,
            items: VecDeque::from(starting_items),
            operation,
            operand,
            test,
            true_branch,
            false_branch,
        }
    }

    // Inspect an item, process its worry level, and throw it to a new level.
    //
    // Returns None if there are no items left.
    // Returns Some((id, worry_level)) if there are items left.
    fn inspect(&mut self, relief: i64) -> Option<(MonkeyId, i64)> {
        let item = self.items.pop_front()?;

        // Inspect
        let rhs = self.operand.unwrap_or(item);
        let worry = match self.operation {
            Operation::Add => item + rhs,
            Operation::Mul => item * rhs,
        }
        // and relief after
        / relief;

        // And test
        let matches = worry % self.test == 0;

        if matches {
            Some((self.true_branch, worry))
        } else {
            Some((self.false_branch, worry))
        }
    }

    #[allow(dead_code)]
    pub fn go(&mut self, relief: i64) -> impl Iterator<Item = (MonkeyId, i64)> + '_ {
        std::iter::from_fn(move || self.inspect(relief))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Monkeys {
    // Monkeys are stored as a vector of (Monkey, inspections) pairs.
    monkeys: BTreeMap<MonkeyId, (Monkey, isize)>,
    // Least-common-multiple of the monkey tests
    lcm: i64,
}

impl FromStr for Monkeys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = MonkeyParser::parse(Rule::Monkeys, s)?.next().unwrap();

        let monkeys: BTreeMap<MonkeyId, (Monkey, isize)> = parsed
            .into_inner()
            .take_while(|p| p.as_rule() != Rule::EOI)
            .map(|m| {
                let monkey = Monkey::process(m);
                (monkey.id, (monkey, 0))
            })
            .collect();

        // Validate throws go to a real monkey, and calculate lcm
        let mut lcm = 1;
        for (id, (monkey, _)) in &monkeys {
            if lcm % monkey.test != 0 {
                lcm *= monkey.test;
            }
            if !monkeys.contains_key(&monkey.true_branch) {
                anyhow::bail!(
                    "Monkey {} has unknown true branch {}",
                    id.0,
                    monkey.true_branch.0
                );
            }
            if !monkeys.contains_key(&monkey.false_branch) {
                anyhow::bail!(
                    "Monkey {} has unknown false branch {}",
                    id.0,
                    monkey.false_branch.0
                );
            }
        }

        Ok(Monkeys { monkeys, lcm })
    }
}

impl Monkeys {
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.monkeys.len()
    }

    #[allow(dead_code)]
    pub fn tosses(&mut self, relief: i64) -> TossIter {
        let remaining = self.monkeys.keys().copied().collect();
        TossIter {
            monkeys: self,
            remaining,
            relief,
        }
    }

    pub fn round(&mut self, relief: i64) -> isize {
        self.tosses(relief).count() as isize
    }

    pub fn rounds(&mut self, relief: i64) -> RoundIter {
        RoundIter {
            monkeys: self,
            relief,
        }
    }

    pub fn two_maxes(&self) -> (isize, isize) {
        let (mut mx1, mut mx2) = (0, 0);
        for (_, &(_, insp)) in self.monkeys.iter() {
            if insp > mx1 {
                mx2 = mx1;
                mx1 = insp;
            } else if insp > mx2 {
                mx2 = insp;
            }
        }

        (mx1, mx2)
    }
}

pub struct TossIter<'a> {
    monkeys: &'a mut Monkeys,
    remaining: VecDeque<MonkeyId>,
    relief: i64,
}

impl<'a> Iterator for TossIter<'a> {
    // (from, to, worry)
    type Item = (MonkeyId, MonkeyId, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut from = *self.remaining.front()?;
        let (recv_id, worry) = loop {
            let (monkey, inspections) = &mut self.monkeys.monkeys.get_mut(&from).unwrap();
            match monkey.inspect(self.relief) {
                Some((to, worry)) => {
                    *inspections += 1;
                    break (to, worry);
                }
                None => {
                    self.remaining.pop_front();
                    from = *self.remaining.front()?;
                    continue;
                }
            }
        };

        let (recv, _) = &mut self.monkeys.monkeys.get_mut(&recv_id).unwrap();
        // Take the worry modulo the lcm, and throw it to the receiving monkey
        // We take the modulo here to "keep [the] worry levels manageable", as
        // hinted in the problem - otherwise, they can overflow
        recv.catch(worry % self.monkeys.lcm);
        Some((from, recv_id, worry))
    }
}

pub struct RoundIter<'a> {
    monkeys: &'a mut Monkeys,
    relief: i64,
}

impl<'a> Iterator for RoundIter<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.monkeys.round(self.relief))
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::problems::testfns::unindented;

    use super::*;

    const EXAMPLE: &str = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_monkeys() {
        let input = unindented(EXAMPLE).unwrap();
        let monkeys = Monkeys::from_str(&input).unwrap();
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_parser() {
        let basic_input = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        let parsed = match MonkeyParser::parse(Rule::Monkey, basic_input.trim()) {
            Ok(parsed) => parsed,
            Err(e) => {
                // println!("e1:\n{:#?}\n\n", e);
                // println!("e2:\n{}\n\n", e);
                panic!("{}", e);
            }
        };

        let parsed: Vec<_> = parsed.collect();
        // for pair in &parsed {
        //     println!("{:#?}", pair);
        // }

        let monkey = Monkey::process(parsed.into_iter().next().unwrap());
        assert_eq!(
            monkey,
            Monkey {
                id: MonkeyId(0),
                items: VecDeque::from(vec![79, 98]),
                operation: Operation::Mul,
                operand: Some(19),
                test: 23,
                true_branch: MonkeyId(2),
                false_branch: MonkeyId(3),
            }
        );
    }

    #[test]
    fn test_throw() {
        let input = unindented(EXAMPLE).unwrap();
        let mut monkeys = Monkeys::from_str(&input).unwrap();
        let tosses = monkeys.tosses(3).collect::<Vec<_>>();
        assert_eq!(tosses[0], (MonkeyId(0), MonkeyId(3), 500));
        assert_eq!(tosses[1], (MonkeyId(0), MonkeyId(3), 620));
        assert_eq!(tosses[2], (MonkeyId(1), MonkeyId(0), 20));
        assert_eq!(tosses.len(), 14);
    }

    #[test]
    fn test_part_one() {
        let day = Day11::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.part_one(), "10605");
    }

    #[test]
    fn test_part_two() {
        let day = Day11::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.part_two(), "2713310158");
    }
}
