use std::{io::Read, str::FromStr};

use super::{solutions::parse_lines, Solver};

use pest::{iterators::Pair, Parser};

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "problems/day11.pest"]
pub struct MonkeyParser;

// mod ast {
//     use super::{Operation, Rule};
//     use pest::Span;

//     fn span_into_str(span: Span) -> &str {
//         span.as_str()
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::number))]
//     pub struct number {
//         #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
//         pub value: i64,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::monkeyId))]
//     pub struct monkeyId {
//         pub value: number,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::startingLine))]
//     pub struct startingLine {
//         pub items: Vec<number>,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::operationLine))]
//     pub struct operationLine<'a> {
//         #[pest_ast(outer(with(span_into_str)))]
//         operation: &'a str,
//         operand: number,
//     }
// }

pub struct Day11(Vec<i64>);

impl Day11 {
    pub fn max(&self) -> i64 {
        self.0.iter().copied().max().unwrap_or_default()
    }

    pub fn sum(&self) -> i64 {
        self.0.iter().copied().sum::<i64>()
    }
}

impl Solver for Day11 {
    fn from_input(input: impl Read) -> anyhow::Result<Self> {
        let items = parse_lines::<i64>(input)?;

        Ok(Day11(items))
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

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Monkey {
    id: i64,
    starting_items: Vec<i64>,
    operation: Operation,
    operand: i64,
    test: i64,
    true_branch: i64,
    false_branch: i64,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = MonkeyParser::parse(Rule::Monkey, s)?.next().unwrap();
        process(pair)
    }
}

fn to_number(pair: Pair<Rule>) -> anyhow::Result<i64> {
    pair.as_str().parse::<i64>().map_err(Into::into)
}

fn process(pair: Pair<Rule>) -> anyhow::Result<Monkey> {
    if pair.as_rule() != Rule::Monkey {
        anyhow::bail!("Expected Monkey, got {:?}", pair.as_rule());
    }

    let mut lines = pair.into_inner();

    let id_line = lines
        .next()
        .ok_or(anyhow::format_err!("Expected MonkeyId"))?;
    if id_line.as_rule() != Rule::monkeyId {
        anyhow::bail!("Expected MonkeyId, got {:?}", id_line.as_rule());
    }

    let id = to_number(id_line.into_inner().next().unwrap())?;

    let starting_line = lines
        .next()
        .ok_or(anyhow::format_err!("Expected MonkeyId"))?;

    if starting_line.as_rule() != Rule::startingLine {
        anyhow::bail!("Expected StartingItems, got {:?}", starting_line.as_rule());
    }

    let starting_items = starting_line
        .into_inner()
        .map(to_number)
        .collect::<anyhow::Result<Vec<_>>>()?;

    let operation_line = lines
        .next()
        .ok_or(anyhow::format_err!("Expected OperationLine"))?;

    if operation_line.as_rule() != Rule::operationLine {
        anyhow::bail!("Expected OperationLine, got {:?}", operation_line.as_rule());
    }

    let mut op_elements = operation_line.into_inner();
    let operation = op_elements
        .next()
        .ok_or(anyhow::format_err!("Expected Operation"))?
        .as_str()
        .parse::<Operation>()?;

    let op_string = op_elements
        .next()
        .ok_or(anyhow::format_err!("Expected Operation"))?;
    let operand = to_number(op_string)?;

    Ok(Monkey {
        id,
        starting_items,
        operation,
        operand,
        test: 0,
        true_branch: 0,
        false_branch: 0,
    })
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
    If false: throw to monkey 1
    ";

    #[test]
    fn test_part_one() {
        let day = Day11::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.max(), 9);
    }

    #[test]
    fn test_parser() {
        let basic_input = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19";
        let parsed = match MonkeyParser::parse(Rule::Monkey, basic_input.trim()) {
            Ok(parsed) => parsed,
            Err(e) => {
                // println!("e1:\n{:#?}\n\n", e);
                // println!("e2:\n{}\n\n", e);
                panic!("{}", e);
            }
        };

        let parsed: Vec<_> = parsed.collect();
        for pair in &parsed {
            println!("{:#?}", pair);
        }

        let monkey = process(parsed.into_iter().next().unwrap()).unwrap();
        assert_eq!(
            monkey,
            Monkey {
                id: 0,
                starting_items: vec![79, 98],
                operation: Operation::Mul,
                operand: 19,
                test: 0,
                true_branch: 0,
                false_branch: 0,
            }
        );
        // let input = unindented(EXAMPLE).unwrap();
        // let items = MonkeyParser::parse(Rule::expression, &input)
        //     .expect("Should pass")
        //     .next()
        //     .unwrap();
    }

    #[test]
    fn test_part_two() {
        let day = Day11::from_input(unindented(EXAMPLE).unwrap().as_bytes()).unwrap();
        assert_eq!(day.sum(), 14);
    }
}
