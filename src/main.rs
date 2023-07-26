mod problems;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use problems::solver;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    day: usize,

    #[clap(short, long, value_parser)]
    input: Option<PathBuf>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let path = args
        .input
        .unwrap_or_else(|| format!("inputs/day{:02}.txt", args.day).into());

    let input = std::fs::File::open(path).context("Opening file").unwrap();

    let solution = solver(args.day, input).unwrap();
    println!("Part one: {}", solution.part_one());
    println!("Part two: {}", solution.part_two());
}
