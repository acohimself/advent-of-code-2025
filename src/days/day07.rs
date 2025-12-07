use std::collections::HashMap;

use crate::read_input;

pub fn solve() {
    let input = read_input(7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut bm: BeamMap = parse(input);
    tachyon_beam(&mut bm.manifold, bm.start, bm.height);
    bm.manifold.into_iter().filter(|(_, x)| *x == 1).count()
}

fn part2(input: &str) -> usize {
    let mut bm: BeamMap = parse(input);
    quantum_tachyon_beam(&mut bm.manifold, bm.start, bm.height)
}

fn tachyon_beam(
    manifold: &mut HashMap<(usize, usize), usize>,
    start: (usize, usize),
    height: usize,
) {
    for y in start.1..height {
        match manifold.get(&(start.0, y)) {
            Some(0) => {
                manifold.insert((start.0, y), 1);
                if start.0 > 0 {
                    tachyon_beam(manifold, (start.0 - 1, y + 1), height);
                }
                tachyon_beam(manifold, (start.0 + 1, y + 1), height);
                return;
            }
            Some(_) => {
                return;
            }
            None => {}
        }
    }
}
fn quantum_tachyon_beam(
    manifold: &mut HashMap<(usize, usize), usize>,
    start: (usize, usize),
    height: usize,
) -> usize {
    for y in start.1..height {
        match manifold.get(&(start.0, y)) {
            Some(0) => {
                let worlds = quantum_tachyon_beam(manifold, (start.0 - 1, y + 1), height)
                    + quantum_tachyon_beam(manifold, (start.0 + 1, y + 1), height);
                manifold.insert((start.0, y), worlds);
                return worlds;
            }
            Some(x) => {
                return *x;
            }
            None => {}
        }
    }
    1
}
struct BeamMap {
    manifold: HashMap<(usize, usize), usize>,
    start: (usize, usize),
    height: usize,
}

fn parse(s: &str) -> BeamMap {
    let mut manifold = HashMap::new();
    let mut start = (0, 0);
    let mut height = 0;

    for (y, line) in s.lines().enumerate() {
        height += 1;

        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    manifold.insert((x, y), 0);
                }
                'S' => {
                    start = (x, y);
                }
                _ => {}
            }
        }
    }
    BeamMap {
        manifold,
        start,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
.......|.......
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
