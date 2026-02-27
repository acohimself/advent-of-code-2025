use crate::{Grid, read_input};

pub fn solve() {
    let input = read_input(6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let (numbers, operations): (Grid<u64>, Vec<Op>) = parse(input);
    let mut answer;
    let mut grand_total = 0;

    for (i, operation) in operations.iter().enumerate() {
        match operation {
            Op::Add => {
                answer = 0;
                for j in 0..numbers.height {
                    answer += numbers.get(i, j).unwrap();
                }
            }
            Op::Mul => {
                answer = 1;
                for j in 0..numbers.height {
                    answer *= numbers.get(i, j).unwrap();
                }
            }
        }

        grand_total += answer;
    }
    grand_total
}

fn neutral(op: &Option<Op>) -> u64 {
    match op {
        Some(Op::Mul) => 1,
        _ => 0,
    }
}

fn part2(input: &str) -> u64 {
    let g: Grid<char> = Grid::parse(input).transpose();

    let mut transposed = String::with_capacity(g.width * (g.height + 1));

    for y in 0..g.height {
        for x in 0..(g.width - 1) {
            transposed.push(*g.get(x, y).unwrap());
        }
        transposed.push('\n');
    }
    let ops: String = g.col(g.width - 1).cloned().collect();
    let mut operations = ops.split_whitespace().map(|s| match s {
        "+" => Op::Add,
        "*" => Op::Mul,
        _ => panic!("bad op"),
    });

    let mut current_op = operations.next();
    let mut answer = neutral(&current_op);

    let mut grand_total = 0;

    for line in transposed.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            grand_total += answer;
            current_op = operations.next();
            answer = neutral(&current_op)
        } else {
            match current_op {
                Some(Op::Add) => {
                    answer += trimmed.parse::<u64>().unwrap();
                }
                Some(Op::Mul) => {
                    answer *= trimmed.parse::<u64>().unwrap();
                }
                _ => panic!("bad op"),
            }
        }
    }
    answer + grand_total
}

fn parse(s: &str) -> (Grid<u64>, Vec<Op>) {
    let (ns, ops): (&str, &str) = s.trim().rsplit_once("\n").unwrap();
    let operations = ops.split_whitespace().map(|s| match s {
        "+" => Op::Add,
        "*" => Op::Mul,
        _ => panic!("bad op"),
    });
    (Grid::parse_u64s(ns), operations.collect())
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
