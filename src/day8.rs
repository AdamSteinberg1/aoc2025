use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn coords(&self) -> impl Iterator<Item = usize> {
        [self.x, self.y, self.z].into_iter()
    }
    fn squared_distance(&self, other: &Self) -> usize {
        self.coords()
            .zip(other.coords())
            .map(|(a, b)| {
                let d = a.abs_diff(b);
                d * d
            })
            .sum()
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [x, y, z] = s
            .split(',')
            .map(str::parse)
            .collect_array()
            .with_context(|| format!("failed to parse point {}", s))?;
        Ok(Self {
            x: x?,
            y: y?,
            z: z?,
        })
    }
}

fn parse_points(input: &str) -> Result<Vec<Point>> {
    input.lines().map(str::parse).collect()
}

fn sorted_pairs(points: &[Point]) -> impl Iterator<Item = ((usize, &Point), (usize, &Point))> {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_cached_key(|((_, p1), (_, p2))| p1.squared_distance(p2))
}

#[derive(Default)]
pub struct Day8;
impl Solution for Day8 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let num_connections = 1000; // 10 for the example input

        let points = parse_points(input)?;

        let mut uf = UnionFind::new(points.len());
        sorted_pairs(&points)
            .take(num_connections)
            .for_each(|((id1, _), (id2, _))| {
                uf.union(id1, id2);
            });

        // Count component sizes
        let product = uf
            .into_labeling()
            .into_iter()
            .counts()
            .values()
            .k_largest(3)
            .product();

        Ok(product)
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let points = parse_points(input)?;

        let mut uf = UnionFind::new(points.len());
        sorted_pairs(&points)
            .filter_map(|((id1, p1), (id2, p2))| uf.union(id1, id2).then_some((p1, p2)))
            .nth(points.len() - 2)
            .map(|(p1, p2)| p1.x * p2.x)
            .ok_or_else(|| anyhow::anyhow!("failed to connect all points"))
    }
}
