use crate::solution::Solution;
use anyhow::{Context, Result};
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

fn count_all_paths(start: Node, target: Node, graph: &HashMap<Node, Vec<Node>>) -> usize {
    //depth first search
    if start == target {
        return 1;
    }
    graph
        .get(&start)
        .into_iter()
        .flatten()
        .map(|&neighbor| count_all_paths(neighbor, target, graph))
        .sum()
}

fn count_paths_with_dac_fft(
    state: (Node, bool, bool),
    target: Node,
    graph: &HashMap<Node, Vec<Node>>,
    cache: &mut HashMap<(Node, bool, bool), usize>,
) -> usize {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    let (current, visited_dac, visited_fft) = state;
    let visited_dac = visited_dac || current == *b"dac";
    let visited_fft = visited_fft || current == *b"fft";

    let result = if current == target {
        if visited_dac && visited_fft { 1 } else { 0 }
    } else {
        graph
            .get(&current)
            .into_iter()
            .flatten()
            .map(|&neighbor| {
                let next_state = (neighbor, visited_dac, visited_fft);
                count_paths_with_dac_fft(next_state, target, graph, cache)
            })
            .sum()
    };

    cache.insert(state, result);
    result
}

fn count_valid_paths(start: Node, target: Node, graph: &HashMap<Node, Vec<Node>>) -> usize {
    // a path is valid if it visits 'dac' and 'fft'
    let mut cache = HashMap::new();
    let initial_state = (start, false, false);
    count_paths_with_dac_fft(initial_state, target, graph, &mut cache)
}

#[derive(Default)]
pub struct Day11;
impl Solution for Day11 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let graph = parse_graph(input)?;
        Ok(count_all_paths(*b"you", *b"out", &graph))
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let graph = parse_graph(input)?;
        Ok(count_valid_paths(*b"svr", *b"out", &graph))
    }
}
