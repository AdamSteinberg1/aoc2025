#[derive(Default)]
pub struct Day5;

use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::ops::RangeInclusive;

fn merge_overlapping(
    ranges: impl Iterator<Item = RangeInclusive<usize>>,
) -> impl Iterator<Item = RangeInclusive<usize>> {
    ranges
        .sorted_unstable_by_key(|r| *r.start())
        .coalesce(|prev, curr| {
            if *curr.start() <= prev.end() + 1 {
                // Ranges overlap, merge them
                let &new_end = prev.end().max(curr.end());
                let merged = *prev.start()..=new_end;
                Ok(merged)
            } else {
                // No overlap, keep them separate
                Err((prev, curr))
            }
        })
}

fn contains<'a>(ranges: impl IntoIterator<Item = &'a RangeInclusive<usize>>, num: &usize) -> bool {
    ranges.into_iter().any(|range| range.contains(num))
}

fn parse_ranges(fresh_ranges: &str) -> impl Iterator<Item = Result<RangeInclusive<usize>>> + '_ {
    fresh_ranges.lines().map(|line| {
        let (start, end) = line
            .split_once('-')
            .context("unable to split line on '-'")?;
        let start = start.parse()?;
        let end = end.parse()?;
        Ok(start..=end)
    })
}

impl Solution for Day5 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let (fresh_ranges, available_ids) = input
            .split_once("\n\n")
            .context("unable to find empty line")?;
        let fresh_ranges: Vec<_> = parse_ranges(fresh_ranges)
            .process_results(|ranges| merge_overlapping(ranges).collect())?;

        let available_ids = available_ids.lines().map(|line| line.parse());
        let fresh_id_count = available_ids
            .filter_ok(|id| contains(&fresh_ranges, id))
            .process_results(|ids| ids.count())?;

        Ok(fresh_id_count)
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let (fresh_ranges, _) = input
            .split_once("\n\n")
            .context("unable to find empty line")?;
        parse_ranges(fresh_ranges).process_results(|ranges| {
            merge_overlapping(ranges)
                .map(|range| range.end() - range.start() + 1)
                .sum()
        })
    }
}
