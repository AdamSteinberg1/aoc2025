use crate::solution::Solution;
use anyhow::{bail, Context, Result};
use num_traits::{One, Zero};
use std::ops::{Add, Mul};
use std::str::FromStr;

enum Op {
    Mul,
    Add,
}

impl Op {
    fn apply<T>(&self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }

    fn identity<T: One + Zero>(&self) -> T {
        match self {
            Op::Mul => T::one(),
            Op::Add => T::zero(),
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(op: &str) -> Result<Self> {
        Ok(match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => bail!("unexpected operation {}", op),
        })
    }
}

#[derive(Default)]
pub struct Day6;
impl Solution for Day6 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let mut lines = input.lines().rev();
        let operations = lines
            .next()
            .context("empty input")?
            .split_whitespace()
            .map(Op::from_str)
            .collect::<Result<Vec<_>>>()?;
        let mut accumulations = operations.iter().map(Op::identity).collect::<Vec<_>>();
        for line in lines {
            for (i, num) in line.split_whitespace().enumerate() {
                let num = num.parse()?;
                let acc = &mut accumulations[i];
                *acc = operations[i].apply(*acc, num);
            }
        }
        let total = accumulations.iter().sum();
        Ok(total)
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let row_length = input.find('\n').unwrap_or(input.len());
        let col_length = input.lines().count();
        let chars: Vec<_> = input
            .lines()
            .take(col_length - 1)
            .flat_map(str::chars)
            .collect();

        let columns =
            (0..row_length).map(|i| chars.iter().skip(i).step_by(row_length).collect::<String>());

        let operations = input
            .lines()
            .last()
            .context("empty input")?
            .split_whitespace()
            .map(Op::from_str)
            .collect::<Result<Vec<_>>>()?;
        let mut accumulations = operations.iter().map(Op::identity).collect::<Vec<_>>();

        let mut i = 0;
        for column in columns {
            let column = column.trim();
            if column.is_empty() {
                i += 1;
            } else {
                let num = column.parse()?;
                let acc = &mut accumulations[i];
                *acc = operations[i].apply(*acc, num);
            }
        }
        let total = accumulations.iter().sum();
        Ok(total)
    }
}
