use crate::read_input;
use std::collections::HashMap;

pub fn solve() {
    let input = read_input(9);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let red_points = parse(input);
    red_points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| {
            red_points[i + 1..]
                .iter()
                .map(move |&b| rectangle_size(a, b))
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut red_points = parse(input);
    let (rows, min_y) = preprocess(&red_points);

    red_points.sort_unstable_by_key(|p| p.0);
    let max_h = rows.len();
    let mut best = 0;

    for (i, &a) in red_points.iter().enumerate() {
        for &b in red_points[i + 1..].iter().rev() {
            let width = b.0 - a.0 + 1;
            if width * max_h <= best {
                break;
            }
            let area = rectangle_size(a, b);
            if area > best && is_inside(a, b, &rows, min_y) {
                best = area;
            }
        }
    }

    best
}

fn parse(s: &str) -> Vec<(usize, usize)> {
    s.lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn rectangle_size(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let x = p1.0.abs_diff(p2.0) + 1;
    let y = p1.1.abs_diff(p2.1) + 1;
    x * y
}

fn preprocess(poly: &[(usize, usize)]) -> (Vec<Vec<(usize, usize)>>, usize) {
    let mut intersects: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];

        if x1 == x2 {
            let y_start = y1.min(y2);
            let y_end = y1.max(y2);
            for y in y_start..=y_end {
                intersects.entry(y).or_default().push(x1);
            }
        } else {
            intersects.entry(y1).or_default().push(x1);
            intersects.entry(y1).or_default().push(x2);
        }
    }

    let min_y = *intersects.keys().min().unwrap_or(&0);
    let max_y = *intersects.keys().max().unwrap_or(&0);
    let mut rows = vec![vec![]; max_y - min_y + 1];

    for (y, xs) in intersects {
        rows[y - min_y] = to_pairs(xs);
    }

    (rows, min_y)
}

fn to_pairs(mut xs: Vec<usize>) -> Vec<(usize, usize)> {
    if xs.len() < 2 {
        return vec![];
    }
    xs.sort_unstable();

    let first = xs[0];
    let last = *xs.last().unwrap();

    let mut result: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < xs.len() {
        let val = xs[i];
        let count = xs[i..].iter().take_while(|&&x| x == val).count();
        if count % 2 != 0 {
            result.push(val);
        }
        i += count;
    }

    // Make sure first and last are included
    if result.first() != Some(&first) {
        result.insert(0, first);
    }
    if result.last() != Some(&last) {
        result.push(last);
    }

    result
        .chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| (c[0], c[1]))
        .collect()
}

fn is_inside(
    p1: (usize, usize),
    p2: (usize, usize),
    rows: &[Vec<(usize, usize)>],
    min_y: usize,
) -> bool {
    let x1 = p1.0.min(p2.0);
    let x2 = p1.0.max(p2.0);
    let y1 = p1.1.min(p2.1);
    let y2 = p1.1.max(p2.1);

    for y in y1..=y2 {
        if y < min_y || y - min_y >= rows.len() {
            return false;
        }
        let ranges = &rows[y - min_y];
        if ranges.is_empty() || !ranges.iter().any(|&(sx, ex)| x1 >= sx && x2 <= ex) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
    }
}
