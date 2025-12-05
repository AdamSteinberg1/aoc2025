use crate::solution::Solution;
use anyhow::{anyhow, Result};
use itertools::iproduct;

struct Grid {
    contents: Vec<bool>,
    row_length: usize,
    col_length: usize,
}

impl Grid {
    fn new(input: &str) -> Result<Self> {
        let row_length = input.find(['\n', '\r']).unwrap_or(input.len());
        let col_length = input.lines().count();
        let contents = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '@' => Ok(true),
                '.' => Ok(false),
                _ => Err(anyhow!("invalid char '{}'", c)),
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            contents,
            row_length,
            col_length,
        })
    }

    fn is_occupied(&self, i: usize, j: usize) -> bool {
        if i >= self.col_length || j >= self.row_length {
            return false;
        }
        self.contents
            .get(i * self.row_length + j)
            .copied()
            .unwrap_or_default()
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut bool> {
        if i >= self.col_length || j >= self.row_length {
            return None;
        }
        self.contents.get_mut(i * self.row_length + j)
    }

    fn get_neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        offsets.into_iter().filter_map(move |(i_offset, j_offset)| {
            Some((
                i.checked_add_signed(i_offset)?,
                j.checked_add_signed(j_offset)?,
            ))
        })
    }

    fn is_accessible(&self, i: usize, j: usize) -> bool {
        if !self.is_occupied(i, j) {
            return false;
        }
        let count = self
            .get_neighbors(i, j)
            .filter(|neighbor| self.is_occupied(neighbor.0, neighbor.1))
            .count();
        count < 4
    }

    fn get_accessible(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(0..self.col_length, 0..self.row_length).filter(|&(i, j)| self.is_accessible(i, j))
    }

    fn without(mut self, disallowed: impl Iterator<Item = (usize, usize)>) -> Self {
        for (i, j) in disallowed {
            if let Some(entry) = self.get_mut(i, j) {
                *entry = false
            }
        }
        self
    }

    fn count_removable(self) -> usize {
        let accessible_locations: Vec<_> = self.get_accessible().collect();
        if accessible_locations.is_empty() {
            return 0;
        }
        let count = accessible_locations.len();
        let next_grid = self.without(accessible_locations.into_iter());
        count + next_grid.count_removable()
    }
}

#[derive(Default)]
pub struct Day4;

impl Solution for Day4 {
    type Part1Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let grid = Grid::new(input)?;
        let count = grid.get_accessible().count();
        Ok(count)
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let grid = Grid::new(input)?;
        Ok(grid.count_removable())
    }
}
