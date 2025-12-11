use crate::solution::Solution;
use anyhow::{Context, Result};
use memoize::memoize;
use std::collections::HashMap;

type Node = [u8; 3];

fn parse_graph(input: &str) -> Result<HashMap<Node, Vec<Node>>> {
    input
        .lines()
        .map(|line| {
            let (node, neighbors) = line
                .split_once(':')
                .with_context(|| format!("missing ':' from line: '{}'", line))?;
            let node = node
                .as_bytes()
                .try_into()
                .with_context(|| format!("'{}' is not exactly 3 bytes", node))?;
            let neighbors = neighbors
                .split_whitespace()
                .map(|s| {
                    s.as_bytes()
                        .try_into()
                        .with_context(|| format!("'{}' is not exactly 3 bytes", s))
                })
                .collect::<Result<_>>()?;
            Ok((node, neighbors))
        })
        .collect()
}

#[memoize(Ignore: graph)]
fn count_paths(start: Node, target: Node, graph: &HashMap<Node, Vec<Node>>) -> usize {
    //depth first search
    if start == target {
        return 1;
    }
    graph
        .get(&start)
        .into_iter()
        .flatten()
        .map(|&neighbor| count_paths(neighbor, target, graph))
        .sum()
}

#[derive(Default)]
pub struct Day11;
impl Solution for Day11 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let graph = parse_graph(input)?;
        Ok(count_paths(*b"you", *b"out", &graph))
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let graph = parse_graph(input)?;

        // svr → dac → fft → out
        let svr_to_dac = count_paths(*b"svr", *b"dac", &graph);
        let dac_to_fft = count_paths(*b"dac", *b"fft", &graph);
        let fft_to_out = count_paths(*b"fft", *b"out", &graph);
        let paths_via_dac_first = svr_to_dac * dac_to_fft * fft_to_out;

        // svr → fft → dac → out
        let svr_to_fft = count_paths(*b"svr", *b"fft", &graph);
        let fft_to_dac = count_paths(*b"fft", *b"dac", &graph);
        let dac_to_out = count_paths(*b"dac", *b"out", &graph);
        let paths_via_fft_first = svr_to_fft * fft_to_dac * dac_to_out;

        let total_paths = paths_via_dac_first + paths_via_fft_first;
        Ok(total_paths)
    }
}
