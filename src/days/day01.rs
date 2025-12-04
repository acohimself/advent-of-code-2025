use crate::read_input;

pub fn solve() {
    let input = read_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let rotations = input.lines();
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    for r in rotations {
        let value: i32 = r[1..].parse().unwrap();
        if r.starts_with("L") {
            dial -= value;
        } else {
            dial += value;
        }
        if dial % 100 == 0 {
            password += 1;
        }
    }
    password
}

fn part2(input: &str) -> i32 {
    let rotations = input.lines();
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    for r in rotations {
        let value: i32 = r[1..].parse().unwrap();
        password += value / 100; // full rotations

        // count times we rotate past 0
        if r.starts_with("L") {
            let was_at_zero = dial == 0;
            dial -= value % 100;
            if dial < 0 {
                if !was_at_zero {
                    password += 1;
                }
                dial += 100
            }
        } else {
            dial += value % 100;
            if dial > 99 {
                if dial != 100 {
                    password += 1;
                }
                dial -= 100;
            }
        }

        // count times we land on 0
        if dial == 0 {
            password += 1;
        }
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
