use anyhow::Result;
use std::fmt::Display;

pub trait Solution: Default {
    type Part1Output: Display;
    fn part1(&self, input: &str) -> Result<Self::Part1Output>;

    type Part2Output: Display;
    fn part2(&self, input: &str) -> Result<Self::Part2Output>;
}
