use anyhow::Result;

pub trait Solution {
    fn part1(&self, input: &str) -> Result<usize>;
    fn part2(&self, input: &str) -> Result<usize>;
}
