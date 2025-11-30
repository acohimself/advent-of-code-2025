use crate::read_input;

pub fn solve() {
    let input = read_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
example input here";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
