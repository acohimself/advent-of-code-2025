use crate::read_input;

pub fn solve() {
    let input = read_input(3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input.lines().map(|b| max_joltage_of_size(b, 2)).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(|b| max_joltage_of_size(b, 12)).sum()
}

fn max_joltage_of_size(s: &str, size: usize) -> u64 {
    fn first_max(s: &str) -> Option<(usize, char)> {
        s.chars().enumerate().fold(None, |acc, (i, c)| match acc {
            None => Some((i, c)),
            Some((_, max_c)) if c > max_c => Some((i, c)),
            _ => acc,
        })
    }

    let mut result = 0u64;
    let mut start = 0;

    for picks_remaining in (0..size).rev() {
        let last = s.len() - picks_remaining;
        let (relative_index, c) = first_max(&s[start..last]).unwrap();
        start += relative_index + 1;
        result = result * 10 + c.to_digit(10).unwrap() as u64
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}
