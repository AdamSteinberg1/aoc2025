use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::ops::RangeInclusive;

fn count_digits(id: usize) -> usize {
    id.checked_ilog10().map(|log| log + 1).unwrap_or(1) as usize
}

fn repeated_numbers(
    range: RangeInclusive<usize>,
    repetition_count: usize,
) -> impl Iterator<Item = usize> {
    let &start = range.start();
    let &end = range.end();

    // Calculate the number of digits in the repeated chunk
    let chunk_digit_count = count_digits(end) / repetition_count;

    // Calculate the factor we will multiply our base number by
    let pow10_total = 10usize.pow((chunk_digit_count * repetition_count) as u32);
    let pow10_section = 10usize.pow(chunk_digit_count as u32);
    let factor = (pow10_total - 1) / (pow10_section - 1);

    // The base number must have chunk_digit_count digits
    let min_base_from_digits = 10usize.pow((chunk_digit_count - 1) as u32);
    let max_base_from_digits = pow10_section - 1;

    // The base number must also produce values within our range
    let min_k_from_range = start.div_ceil(factor);
    let max_k_from_range = end / factor;

    // Combine both constraints
    let min_base = min_base_from_digits.max(min_k_from_range);
    let max_base = max_base_from_digits.min(max_k_from_range);

    (min_base..=max_base)
        .map(move |base_num| base_num * factor)
        .skip_while(move |&n| n < start)
        .take_while(move |&n| n <= end)
}

fn invalid_numbers(range: RangeInclusive<usize>) -> impl Iterator<Item = usize> {
    let max_digit_count = count_digits(*range.end());
    (2..=max_digit_count)
        .flat_map(move |num| repeated_numbers(range.clone(), num))
        .sorted()
        .dedup()
}

fn solve<F, I>(input: &str, invalid_number_generator: F) -> Result<usize>
where
    F: Fn(RangeInclusive<usize>) -> I,
    I: Iterator<Item = usize>,
{
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').context("error parsing range")?;
            let start = start.trim().parse()?;
            let end = end.trim().parse()?;
            let sum: usize = invalid_number_generator(start..=end).sum();
            Ok(sum)
        })
        .sum()
}

#[derive(Default)]
pub struct Day2;
impl Solution for Day2 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        solve(input, |range| repeated_numbers(range, 2))
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        solve(input, invalid_numbers)
    }
}
