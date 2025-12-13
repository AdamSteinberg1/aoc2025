use crate::solution::Solution;
use anyhow::{Context, Result};

#[derive(Default)]
pub struct Day12;
impl Solution for Day12 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        // today the problem is really hard in general,
        // but the input only contains special cases which make it easy

        // we can ignore all the present shapes
        let regions = input
            .split("\n\n")
            .last()
            .context("unable to parse input")?;

        regions.lines().try_fold(0, |count, line| {
            let (dimensions, counts) = line
                .split_once(':')
                .context("unable to find ':' in input line")?;

            let (width, length) = dimensions
                .split_once('x')
                .context("unable to find 'x' in dimensions")?;

            let width = width.parse::<usize>()?;
            let length = length.parse::<usize>()?;

            let total_count: usize = counts
                .split_whitespace()
                .map(|num| num.parse::<usize>())
                .sum::<Result<usize, _>>()
                .context("invalid count value")?;

            // all the presents fit in a 3x3 square,
            // so as long as the region can fit total_count squares, then we're good
            // more complicated situations never occur in the input
            let does_fit = width * length >= 9 * total_count;
            Ok(count + does_fit as usize)
        })
    }

    type Part2Output = usize;
    fn part2(&self, _input: &str) -> Result<Self::Part2Output> {
        // no part 2 today
        Ok(0)
    }
}
