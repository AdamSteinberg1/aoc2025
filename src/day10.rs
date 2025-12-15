use crate::solution::Solution;
use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct BitSequence(u16);

impl FromStr for BitSequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.chars()
            .map(|c| {
                Ok(match c {
                    '#' => true,
                    '.' => false,
                    _ => bail!("invalid character: {}", c),
                })
            })
            .process_results(|iter| iter.collect())
    }
}

impl FromIterator<bool> for BitSequence {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        let value = iter
            .into_iter()
            .take(16)
            .enumerate()
            .fold(0u16, |acc, (i, b)| acc | ((b as u16) << (15 - i)));
        Self(value)
    }
}

impl BitSequence {
    fn toggle_bits(&self, indices: &[usize]) -> Self {
        let Self(inner) = self;
        let toggled = indices.iter().fold(*inner, |acc, &i| acc ^ (1 << (15 - i)));
        Self(toggled)
    }
}

fn parse_line(line: &str) -> Result<(BitSequence, Vec<Vec<usize>>, Vec<usize>)> {
    let (lights, buttons, joltages) = line
        .split_once(']')
        .and_then(|(lights, rest)| {
            let lights = lights.strip_prefix('[')?;
            let (buttons, joltages) = rest.split_once('{')?;
            let joltages = joltages.strip_suffix('}')?;
            Some((lights, buttons, joltages))
        })
        .context("unable to parse line")?;

    let lights = lights.parse()?;
    let buttons = buttons
        .split_whitespace()
        .map(parse_button)
        .collect::<Result<_>>()?;
    let joltages = joltages
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    Ok((lights, buttons, joltages))
}

fn parse_button(button: &str) -> Result<Vec<usize>> {
    let button = button
        .strip_prefix('(')
        .and_then(|button| button.strip_suffix(')'))
        .context("unable to parse button")?;
    let nums = button
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    Ok(nums)
}

fn count_button_presses(target: BitSequence, buttons: &[Vec<usize>]) -> Option<usize> {
    //breadth-first search
    let start = BitSequence::default(); //lights are all off
    let mut queue = VecDeque::from([start]);
    let mut distances = HashMap::from([(start, 0usize)]);

    while let Some(state) = queue.pop_front() {
        if state == target {
            return distances.get(&target).copied();
        }
        let current_dist = distances[&state];
        for button in buttons {
            let next_state = state.toggle_bits(button);
            if let Vacant(entry) = distances.entry(next_state) {
                entry.insert(current_dist + 1);
                queue.push_back(next_state);
            }
        }
    }
    None
}

// compute costs for all button combinations, grouped by parity pattern
// returns: parity_pattern -> list of (counts, cost) pairs
fn button_combination_costs(
    buttons: &[Vec<usize>],
) -> HashMap<BitSequence, Vec<(Vec<usize>, usize)>> {
    let num_counters = buttons.iter().flatten().max().copied().unwrap_or_default() + 1;

    // generate all button combinations with their parity patterns and costs
    (0..buttons.len())
        .powerset()
        .map(|button_indices| {
            let cost = button_indices.len();
            let counts: Vec<usize> = (0..num_counters)
                .map(|i| {
                    button_indices
                        .iter()
                        .filter(|&&btn_idx| buttons[btn_idx].contains(&i))
                        .count()
                })
                .collect();
            let parity_pattern = counts
                .iter()
                .map(|&count| !count.is_multiple_of(2))
                .collect();
            (parity_pattern, (counts, cost))
        })
        .into_group_map()
}

fn solve_recursive(
    goal: &[usize],
    costs: &HashMap<BitSequence, Vec<(Vec<usize>, usize)>>,
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    // base case: all zeros
    if goal.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&cached) = cache.get(goal) {
        return cached;
    }

    // get parity pattern of current goal
    let parity_pattern: BitSequence = goal.iter().map(|x| !x.is_multiple_of(2)).collect();

    // try all combinations that match this parity
    let answer = costs
        .get(&parity_pattern)
        .into_iter()
        .flatten()
        .filter(|(counts, _)| counts.iter().zip(goal).all(|(&p, &g)| p <= g))
        .map(|(counts, cost)| {
            // calculate new goal: (goal - count) / 2
            let new_goal: Vec<_> = counts
                .iter()
                .zip(goal)
                .map(|(&c, &g)| (g - c) / 2)
                .collect();

            let recursive_cost = solve_recursive(&new_goal, costs, cache);
            cost.saturating_add(recursive_cost.saturating_mul(2))
        })
        .min()
        .unwrap_or(usize::MAX);

    cache.insert(goal.to_owned(), answer);
    answer
}

fn solve_machine(buttons: &[Vec<usize>], joltages: &[usize]) -> Result<usize> {
    let costs = button_combination_costs(buttons);
    let mut cache = HashMap::new();
    let answer = solve_recursive(joltages, &costs, &mut cache);
    ensure!(
        answer < usize::MAX,
        "No solution found for joltages {:?}",
        joltages
    );
    Ok(answer)
}

#[derive(Default)]
pub struct Day10;
impl Solution for Day10 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        input
            .lines()
            .map(|line| {
                let (light_state, buttons, _) = parse_line(line)?;
                count_button_presses(light_state, &buttons).context("unable to find solution")
            })
            .sum()
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        input
            .lines()
            .map(|line| {
                let (_, buttons, joltages) = parse_line(line)?;
                solve_machine(&buttons, &joltages)
            })
            .sum()
    }
}
