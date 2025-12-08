use crate::solution::Solution;
use anyhow::{bail, Result};

fn solve(input: &str) -> Result<(u64, Vec<u64>)> {
    let width = input.find('\n').unwrap_or(input.len());
    let mut beams = vec![0u64; width];
    let mut count = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    count += (beams[i] > 0) as u64;
                    beams[i + 1] += beams[i];
                    beams[i - 1] += beams[i];
                    beams[i] = 0;
                }
                '.' => {}
                'S' => beams[i] = 1,
                _ => bail!("unexpected char {}", c),
            }
        }
    }
    Ok((count, beams))
}

#[derive(Default)]
pub struct Day7;
impl Solution for Day7 {
    type Part1Output = u64;

    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        solve(input).map(|(count, _)| count)
    }

    type Part2Output = u64;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        solve(input).map(|(_, beams)| beams.into_iter().sum())
    }
}
