use crate::{parse_numbers, read_input};
use std::ops::RangeInclusive;

pub fn solve() {
    let input = read_input(5);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let (ranges, ids): (Vec<RangeInclusive<u64>>, Vec<u64>) = {
        let (rs, ids) = input.split_once("\n\n").unwrap();
        (
            rs.lines()
                .map(|s| {
                    let (start, end) = s.split_once('-').unwrap();
                    RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
                })
                .collect(),
            parse_numbers(ids),
        )
    };
    let mut fresh = 0;
    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                fresh += 1;
                break;
            }
        }
    }
    fresh
}
fn part2(input: &str) -> u64 {
    let mut ranges = Vec::new();
    let (rs, _) = input.split_once("\n\n").unwrap();
    for line in rs.lines() {
        let (start, end) = line.split_once('-').unwrap();
        ranges.push(RangeElem::Open(start.parse().unwrap()));
        ranges.push(RangeElem::Close(end.parse().unwrap()));
    }

    ranges.sort_by(|a, b| {
        let av = match a {
            RangeElem::Open(v) | RangeElem::Close(v) => v,
        };
        let bv = match b {
            RangeElem::Open(v) | RangeElem::Close(v) => v,
        };

        av.cmp(bv).then_with(|| match (a, b) {
            (RangeElem::Open(_), RangeElem::Close(_)) => std::cmp::Ordering::Less,
            (RangeElem::Close(_), RangeElem::Open(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        })
    });

    let mut opens = 0;
    let mut trimmed = Vec::new();
    for range in ranges {
        match range {
            RangeElem::Open(v) => {
                if 0 == opens {
                    trimmed.push(v);
                };
                opens += 1;
            }
            RangeElem::Close(v) => {
                if 1 == opens {
                    trimmed.push(v);
                };
                opens -= 1;
            }
        }
    }

    trimmed
        .chunks_exact(2)
        .map(|pair| pair[1] - pair[0] + 1)
        .sum()
}

#[derive(Debug)]
enum RangeElem {
    Open(u64),
    Close(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
