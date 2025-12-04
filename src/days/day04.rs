use crate::{Grid, read_input};

pub fn solve() {
    let input = read_input(4);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let grid: Grid<char> = Grid::parse(input);
    let mut accessible_rolls = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if *grid.get(x, y).unwrap() != '@' {
                continue;
            }
            if count_adjacent_rolls(&grid, x, y) < 4 {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

fn part2(input: &str) -> i32 {
    let mut grid: Grid<char> = Grid::parse(input);
    let mut accessible_rolls = 0;
    loop {
        let mut removed = Vec::<(usize, usize)>::new();

        for x in 0..grid.width {
            for y in 0..grid.height {
                if *grid.get(x, y).unwrap() != '@' {
                    continue;
                }
                if count_adjacent_rolls(&grid, x, y) < 4 {
                    removed.push((x, y));
                }
            }
        }
        if removed.is_empty() {
            break;
        } else {
            accessible_rolls += removed.len() as i32;
            for (x, y) in removed {
                grid.set(x, y, 'x');
            }
        }
    }
    accessible_rolls
}

fn count_adjacent_rolls(grid: &Grid<char>, x: usize, y: usize) -> usize {
    grid.neighbors8(x, y).filter(|&&c| c == '@').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
