mod day1;
mod solution;

use crate::solution::Solution;
use anyhow::{Context, Result};
use std::env;
use std::fs;

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

fn run_solution(num: usize, solution: Box<dyn Solution>) -> Result<()> {
    let path = format!("inputs/day{}.txt", num);
    let input = fs::read_to_string(path)?;
    println!("--- Day {} ---", num);
    println!("Part 1: {}", solution.part1(&input)?);
    println!("Part 2: {}", solution.part2(&input)?);
    println!();
    Ok(())
}

fn run_day(num: usize) -> Result<()> {
    let day = get_solution(num).context("No solution for that day")?;
    run_solution(num, day)?;
    Ok(())
}

fn run_all_days() -> Result<()> {
    let solutions = (1..).map_while(|num| get_solution(num).map(|solution| (num, solution)));
    for (num, solution) in solutions {
        run_solution(num, solution)?;
    }
    Ok(())
}

fn get_solution(day: usize) -> Option<Box<dyn Solution>> {
    Some(match day {
        1 => Box::new(day1::Day1),
        _ => return None,
    })
}
