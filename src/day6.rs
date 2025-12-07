use crate::solution::Solution;
use anyhow::Result;

#[derive(Default)]
pub struct Day6;
impl Solution for Day6 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        // let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let mut lines = input.lines().rev();
        let operations = lines.next().unwrap().split_whitespace().collect::<Vec<_>>();
        let mut accumulations = operations
            .iter()
            .map(|&op| match op {
                "*" => 1,
                "+" => 0,
                _ => panic!(),
            })
            .collect::<Vec<usize>>();
        for line in lines {
            for (i, num) in line.split_whitespace().enumerate() {
                let num = num.parse::<usize>()?;
                match operations[i] {
                    "*" => accumulations[i] *= num,
                    "+" => accumulations[i] += num,
                    _ => panic!("unknown operation"),
                }
            }
        }
        let total = accumulations.iter().sum::<usize>();
        Ok(total)
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        // let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
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
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let mut accumulations = operations
            .iter()
            .map(|&op| match op {
                "*" => 1,
                "+" => 0,
                _ => panic!(),
            })
            .collect::<Vec<usize>>();

        let mut i = 0;
        for column in columns {
            let column = column.trim();
            if column.is_empty() {
                i += 1;
            } else {
                let num = column.parse::<usize>()?;
                match operations[i] {
                    "*" => accumulations[i] *= num,
                    "+" => accumulations[i] += num,
                    _ => panic!("unknown operation"),
                }
            }
        }
        let total = accumulations.iter().sum::<usize>();
        Ok(total)
    }
}
