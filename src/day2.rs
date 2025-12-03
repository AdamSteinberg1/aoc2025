#[derive(Default)]
pub struct Day2;

use std::iter::{empty, successors};

use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::{Either, Itertools};

fn count_digits(id: usize) -> usize {
    if id == 0 { 1 } else { id.ilog10() as usize + 1 }
}

fn first_factor(x: usize) -> usize {
    if x.is_multiple_of(2) {
        return 2;
    }

    (1..)
        .map(|m| 2 * m + 1)
        .take_while(|&n| n * n <= x)
        .find(|&n| x.is_multiple_of(n))
        .unwrap_or(x)
}

fn factors(x: usize) -> impl Iterator<Item = usize> {
    if x <= 1 {
        return Either::Right(empty());
    }

    Either::Left(
        successors(Some((x, first_factor(x))), |(remaining, factor)| {
            if *remaining == *factor {
                None
            } else {
                let next_remaining = *remaining / factor;
                Some((next_remaining, first_factor(next_remaining)))
            }
        })
        .map(|(_, factor)| factor)
        .dedup(),
    )
}

fn is_repetitions(id: usize, num_chunks: usize) -> bool {
    if num_chunks == 0 {
        return false;
    }

    let digit_count = count_digits(id);
    if !digit_count.is_multiple_of(num_chunks) {
        return false;
    }

    let chunk_size = digit_count / num_chunks;
    let divisor = 10_usize.pow(chunk_size as u32);

    successors(Some(id), |&current| {
        let next = current / divisor;
        (next > 0).then_some(next)
    })
    .map(|chunk| chunk % divisor)
    .all_equal()
}

fn is_any_repetitions(id: usize) -> bool {
    factors(count_digits(id)).any(|chunk| is_repetitions(id, chunk))
}

fn solve(input: &str, is_invalid: fn(usize) -> bool) -> Result<usize> {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').context("error parsing range")?;
            let start = start.trim().parse()?;
            let end = end.trim().parse()?;
            Ok(start..=end)
        })
        .flatten_ok()
        .filter_ok(|&id| is_invalid(id))
        .sum()
}

impl Solution for Day2 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        solve(input, |id| is_repetitions(id, 2))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        solve(input, is_any_repetitions)
    }
}
