use anyhow::Result;
use std::fmt::Display;

pub trait Solution {
    type Part1Output: Display;
    type Part2Output: Display;

    fn part1(&self, input: &str) -> Result<Self::Part1Output>;
    fn part2(&self, input: &str) -> Result<Self::Part2Output>;
}
