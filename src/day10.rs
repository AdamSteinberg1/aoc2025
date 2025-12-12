use Entry::Vacant;
use crate::solution::Solution;
use anyhow::{bail, Context, Result};
use good_lp::Solution as LpSolution;
use good_lp::{default_solver, variable, Expression, ProblemVariables, SolverModel};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
// to avoid colliding with Solution trait

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct LightState(u16);

impl FromStr for LightState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.chars()
            .rev()
            .try_fold(0u16, |acc, c| {
                let new_bit = match c {
                    '#' => 1,
                    '.' => 0,
                    _ => bail!("invalid character: {}", c),
                };
                Ok((acc >> 1) | (new_bit << 15))
            })
            .map(Self)
    }
}

impl LightState {
    fn press_button(&self, button: &[usize]) -> Self {
        let Self(inner) = self;
        let modified = button
            .iter()
            .fold(*inner, |acc, &num| acc ^ (1 << (15 - num)));
        Self(modified)
    }
}

fn parse_line(line: &str) -> Result<(LightState, Vec<Vec<usize>>, Vec<u32>)> {
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

fn count_button_presses(target: LightState, buttons: &[Vec<usize>]) -> Option<usize> {
    //breadth first search
    let start = LightState::default(); //lights are all off
    let mut queue = VecDeque::from([start]);
    let mut distances = HashMap::from([(start, 0usize)]);

    while let Some(state) = queue.pop_front() {
        if state == target {
            return distances.get(&target).copied();
        }
        let current_dist = distances[&state];
        for button in buttons {
            let next_state = state.press_button(button);
            if let Vacant(entry) = distances.entry(next_state) {
                entry.insert(current_dist + 1);
                queue.push_back(next_state);
            }
        }
    }
    None
}

fn solve_machine(buttons: &[Vec<usize>], joltages: &[u32]) -> Result<usize> {
    let mut vars = ProblemVariables::new();

    // Create variables for each button
    // These variable represent the amount of times that button needs to be pressed
    let button_vars = vars.add_vector(variable().integer().min(0), buttons.len());

    // for each counter the final joltage equals the sum of all the button presses that affect it
    let constraints = joltages.iter().enumerate().map(|(joltage_idx, &joltage)| {
        let expr: Expression = button_vars
            .iter()
            .zip(buttons)
            .filter(|(_, button)| button.contains(&joltage_idx))
            .map(|(&var, _)| var)
            .sum();
        expr.eq(joltage)
    });

    // objective: sum of all button presses
    let objective: Expression = button_vars.iter().copied().sum();

    let mut problem = vars
        .minimise(&objective) // find minimum number of button presses
        .using(default_solver)
        .with_all(constraints);
    problem.set_parameter("log", "0");

    let solution = problem.solve()?;
    Ok(solution.eval(objective) as usize)
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
