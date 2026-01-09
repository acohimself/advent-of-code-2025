use crate::read_input;
use std::{collections::HashMap, str::FromStr};

pub fn solve() {
    let input = read_input(8);

    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str, num_shortest: usize) -> usize {
    let points = parse(input);
    let distances: Vec<(usize, usize, f64)> = build_distances(&points);
    let mut circuits: Vec<usize> = (0..points.len()).collect();

    for (i, j, _) in distances.iter().take(num_shortest) {
        let ci = find_circuit_id(&mut circuits, *i);
        let cj = find_circuit_id(&mut circuits, *j);
        if ci != cj {
            circuits[cj] = ci
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..circuits.len() {
        let root = find_circuit_id(&mut circuits, i);
        *counts.entry(root).or_insert(0) += 1;
    }

    // Sort by count descending and take top 3
    let mut count_vec: Vec<_> = counts.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec.iter().take(3).map(|(_, count)| count).product()
}

fn part2(input: &str) -> i64 {
    let points = parse(input);
    let distances: Vec<(usize, usize, f64)> = build_distances(&points);
    let mut circuits: Vec<usize> = (0..points.len()).collect();
    let mut num_circuits = points.len();

    for (i, j, _) in distances {
        let ci = find_circuit_id(&mut circuits, i);
        let cj = find_circuit_id(&mut circuits, j);
        if ci != cj {
            circuits[cj] = ci;
            num_circuits -= 1;
            if num_circuits == 1 {
                return (points[i].x as i64) * (points[j].x as i64);
            }
        }
    }

    unreachable!();
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        Ok(Point {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            z: parts[2].parse()?,
        })
    }
}
impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn parse(s: &str) -> Vec<Point> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn build_distances(points: &[Point]) -> Vec<(usize, usize, f64)> {
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            distances.push((i, j, points[i].distance_to(&points[j])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    distances
}

fn find_circuit_id(circuits: &mut [usize], i: usize) -> usize {
    if circuits[i] != i {
        circuits[i] = find_circuit_id(circuits, circuits[i]);
    }
    circuits[i]
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_calc_distance() {
        let p1 = Point {
            x: 162,
            y: 817,
            z: 812,
        };
        let p2 = Point {
            x: 425,
            y: 690,
            z: 689,
        };
        assert_eq!(p1.distance_to(&p2), 316.90219311326956);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
