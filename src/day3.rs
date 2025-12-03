use crate::solution::Solution;
use anyhow::{Context, Result};
use std::ops::Range;

#[derive(Default)]
pub struct Day3;

impl Solution for Day3 {
    type Part1Output = u64;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        solve(input, 2)
    }

    type Part2Output = u64;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        solve(input, 12)
    }
}

fn solve(input: &str, num_batteries: usize) -> Result<u64> {
    input
        .lines()
        .map(parse_bank)
        .map(|bank| joltage(&bank?, num_batteries))
        .sum()
}

fn parse_bank(bank: &str) -> Result<Vec<u32>> {
    bank.chars()
        .map(|c| c.to_digit(10).context("found nonnumeric character"))
        .collect()
}

fn joltage(bank: &[u32], num_batteries: usize) -> Result<u64> {
    partial_joltage(bank, 0..bank.len() - num_batteries + 1)
}

fn partial_joltage(bank: &[u32], range: Range<usize>) -> Result<u64> {
    let start = range.start;
    let end = range.end;
    let (max_index, max_value) =
        max_with_index(&bank[range]).context("cannot find maximum of an empty slice")?;
    let max_value = max_value as u64;
    if end == bank.len() {
        return Ok(max_value);
    }
    let next_range = (start + max_index + 1)..(end + 1);
    let power = (bank.len() - end) as u32;
    let addend = max_value * 10u64.pow(power);
    partial_joltage(bank, next_range).map(|joltage| addend + joltage)
}

fn max_with_index(nums: &[u32]) -> Option<(usize, u32)> {
    nums.iter()
        .enumerate()
        .rev() //We reverse to get the first max in case of ties
        .max_by_key(|&(_, &value)| value)
        .map(|(i, max)| (i, *max))
}
