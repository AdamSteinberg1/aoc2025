mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod solution;

use crate::solution::Solution;
use anyhow::{Context, Result};
use std::{env, fmt, fs, time};

const SOLVERS: &[fn(&str) -> Result<()>] = &[
    solve::<day1::Day1>,
    solve::<day2::Day2>,
    solve::<day3::Day3>,
    solve::<day4::Day4>,
    solve::<day5::Day5>,
];

fn main() -> Result<()> {
    let day_num = env::args().nth(1);
    if let Some(day_num) = day_num {
        let day_num: usize = day_num
            .parse()
            .context("Day number must be a valid integer")?;
        run_day(day_num)?;
    } else {
        run_all_days()?;
    }
    Ok(())
}

fn solve<T: Solution>(input: &str) -> Result<()> {
    let solution = T::default();
    print!("Part 1: ");
    print_with_runtime(|| solution.part1(input))?;
    print!("Part 2: ");
    print_with_runtime(|| solution.part2(input))?;
    Ok(())
}

fn print_with_runtime<F, T>(part: F) -> Result<()>
where
    F: FnOnce() -> Result<T>,
    T: fmt::Display,
{
    let now = time::Instant::now();
    let result = part()?;
    let elapsed = now.elapsed();
    println!("{}\t{:.2?}", result, elapsed);
    Ok(())
}

fn run_day(num: usize) -> Result<()> {
    let solve = SOLVERS
        .get(num - 1)
        .with_context(|| format!("No solution for day {}", num))?;
    let path = format!("inputs/day{}.txt", num);
    let input = fs::read_to_string(path)?;
    println!("--- Day {} ---", num);
    solve(&input)?;
    println!();
    Ok(())
}

fn run_all_days() -> Result<()> {
    for day in 1..=SOLVERS.len() {
        run_day(day)?;
    }
    Ok(())
}
