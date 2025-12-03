#[derive(Default)]
pub struct Day1;

use crate::solution::Solution;
use anyhow::{Context, Error, Result, bail};
use itertools::Itertools;

enum Dir {
    Left,
    Right,
}

struct Dial {
    position: usize,
}

impl Dial {
    const SIZE: usize = 100;

    fn new(position: usize) -> Self {
        Self { position }
    }

    fn turn(&self, dir: &Dir, amount: usize) -> Self {
        let next = match *dir {
            Dir::Left => (self.position + Self::SIZE - (amount % Self::SIZE)) % Self::SIZE,
            Dir::Right => (self.position + amount) % Self::SIZE,
        };
        Self::new(next)
    }

    fn turn_and_count(&self, dir: &Dir, amount: usize) -> (Self, usize) {
        let next = self.turn(dir, amount);
        let full_turns = amount / Dial::SIZE;
        let crossed_zero = match dir {
            Dir::Left => next.position > self.position,
            Dir::Right => next.position < self.position,
        };
        let zero_crossings = if next.position == 0 || (self.position != 0 && crossed_zero) {
            1
        } else {
            0
        };
        let count = full_turns + zero_crossings;

        (next, count)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<(Dir, usize), Error>> {
    input.lines().map(|line| {
        let (direction, amount) = line.split_at_checked(1).context("error splitting line")?;
        let amount = amount.parse()?;
        let direction = match direction {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => bail!("invalid direction"),
        };
        Ok((direction, amount))
    })
}

impl Solution for Day1 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        parse_input(input)
            .scan(Dial::new(50), |dial, res| {
                Some(res.map(|(dir, amount)| {
                    *dial = dial.turn(&dir, amount);
                    dial.position
                }))
            })
            .filter_ok(|num| *num == 0)
            .process_results(|iter| iter.count())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        parse_input(input)
            .fold_ok((Dial::new(50), 0), |(dial, sum), (dir, amount)| {
                let (next, count) = dial.turn_and_count(&dir, amount);
                (next, sum + count)
            })
            .map(|(_, count)| count)
    }
}
