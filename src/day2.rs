#[derive(Default)]
pub struct Day2;

use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::{Either, Itertools};
use std::iter;

fn count_digits(id: usize) -> usize {
    id.checked_ilog10().unwrap_or(0) as usize + 1
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
        return Either::Right(iter::empty());
    }

    Either::Left(
        iter::successors(Some((x, first_factor(x))), |(remaining, factor)| {
            if remaining == factor {
                None
            } else {
                let next_remaining = remaining / factor;
                Some((next_remaining, first_factor(next_remaining)))
            }
        })
        .map(|(_, factor)| factor)
        .dedup(),
    )
}

fn is_repetition(id: usize, num_chunks: usize) -> bool {
    if num_chunks == 0 {
        return false;
    }

    let digit_count = count_digits(id);
    if !digit_count.is_multiple_of(num_chunks) {
        return false;
    }

    let chunk_size = (digit_count / num_chunks) as u32;
    let divisor = 10_usize.pow(chunk_size);

    iter::successors(Some(id), |&current| {
        let next = current / divisor;
        (next > 0).then_some(next)
    })
    .map(|chunk| chunk % divisor)
    .all_equal()
}

fn is_any_repetition(id: usize) -> bool {
    let digit_count = count_digits(id);
    factors(digit_count).any(|chunk| is_repetition(id, chunk))
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
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        solve(input, |id| is_repetition(id, 2))
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        solve(input, is_any_repetition)
    }
}
