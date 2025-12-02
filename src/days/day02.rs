use crate::read_input;

pub fn solve() {
    let input = read_input(2);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let ranges = input.split(",");
    ranges
        .flat_map(|r| {
            let (start, end) = r.split_once("-").unwrap();
            find_matches(
                start.trim().parse().unwrap(),
                end.trim().parse().unwrap(),
                is_double_block,
            )
        })
        .sum()
}

fn find_matches<F>(start: u64, end: u64, pred: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    (start..=end).filter(|&n| pred(n)).collect()
}

fn is_double_block(v: u64) -> bool {
    let s = v.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (a, b) = s.split_at(len / 2);
    a == b
}

fn is_repeated_block(v: u64) -> bool {
    let s = v.to_string();
    if s.len() == 1 {
        return false;
    }
    let half_len = s.len() / 2;

    for i in 1..=half_len {
        if s.trim_start_matches(&s[0..i]).is_empty() {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> u64 {
    let ranges = input.split(",");
    ranges
        .flat_map(|r| {
            let (start, end) = r.split_once("-").unwrap();
            find_matches(
                start.trim().parse().unwrap(),
                end.trim().parse().unwrap(),
                is_repeated_block,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
    #[test]
    fn test_is_repeated_block() {
        assert!(is_repeated_block(888));
    }
}
