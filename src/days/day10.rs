use std::{convert::Infallible, str::FromStr};

use crate::read_input;

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, microlp, variable,
};

pub fn solve() {
    let input = read_input(10);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let machines = parse(input);

    let mut fewest_button_presses = 0;
    for machine in &machines {
        let button_masks: Vec<u32> = machine
            .buttons
            .iter()
            .map(|inner| {
                inner.iter().fold(0u32, |acc, &i| {
                    acc | (1 << (machine.diagram.length - 1 - i))
                })
            })
            .collect();
        // Check all subsets and get min. This could be optimized by searching first in small
        // subsets and then breaking when finding a solution
        if let Some(min) = (0u32..1 << button_masks.len())
            .filter(|&mask| {
                button_masks
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, v)| v)
                    .fold(0, |acc, x| acc ^ x)
                    == machine.diagram.value
            })
            .map(|mask| mask.count_ones() as usize)
            .min()
        {
            fewest_button_presses += min;
        }
    }
    fewest_button_presses
}

#[derive(Debug)]
struct LightDiagram {
    value: u32,
    length: usize,
}

impl FromStr for LightDiagram {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = &s[1..s.len() - 1];
        let length = trimmed.len();
        let value = trimmed
            .chars()
            .fold(0u32, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 });

        Ok(LightDiagram { value, length })
    }
}

struct Machine {
    diagram: LightDiagram,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let last_word_index = words.len() - 1;

        let diagram = LightDiagram::from_str(words[0]).unwrap();

        let buttons: Vec<Vec<usize>> = words[1..last_word_index]
            .iter()
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .filter_map(|x| x.trim().parse().ok())
                    .collect()
            })
            .collect();

        let joltage_str = &words[last_word_index][1..words[last_word_index].len() - 1];
        let joltage = joltage_str
            .split(',')
            .map(|j| j.parse::<usize>().unwrap())
            .collect();

        machines.push(Machine {
            diagram,
            buttons,
            joltage,
        });
    }
    machines
}

fn part2(input: &str) -> i32 {
    let machines = parse(input);
    let mut total = 0;

    for machine in &machines {
        let mut problem = ProblemVariables::new();

        // One integer variable per button
        let vars: Vec<Variable> = machine
            .buttons
            .iter()
            .map(|_| problem.add(variable().integer().min(0)))
            .collect();

        // Minimize total button presses
        let objective: Expression = vars.iter().copied().sum();
        let mut model = problem.minimise(objective).using(microlp);

        // For each light index, sum the presses of buttons that affect it == joltage[i]
        for (i, &jolt) in machine.joltage.iter().enumerate() {
            let expr: Expression = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, lights)| lights.contains(&i))
                .map(|(b, _)| vars[b])
                .sum();
            model = model.with(constraint!(expr == jolt as f64));
        }

        if let Ok(solution) = model.solve() {
            total += vars.iter().map(|v| solution.value(*v) as i32).sum::<i32>();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
    }
}
