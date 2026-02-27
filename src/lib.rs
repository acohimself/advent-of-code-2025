pub mod days;

use std::fs;
use std::path::Path;

const YEAR: u32 = 2025;

/// Download input for a given day from adventofcode.com
/// Requires AOC_SESSION environment variable to be set with your session cookie
fn download_input(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let session =
        std::env::var("AOC_SESSION").map_err(|_| "AOC_SESSION environment variable not set")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download input: HTTP {}", response.status()).into());
    }

    let input = response.text()?;

    // Create input directory if it doesn't exist
    fs::create_dir_all("input")?;

    // Save to file
    let path = format!("input/day{:02}.txt", day);
    fs::write(&path, &input)?;

    println!("✓ Downloaded input for day {} to {}", day, path);

    Ok(input)
}

/// Read input file for a given day and return as a single string
/// If the file doesn't exist, attempts to download it from adventofcode.com
pub fn read_input(day: u8) -> String {
    let path = format!("input/day{:02}.txt", day);

    // If file exists, read it
    if Path::new(&path).exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));
    }

    // Otherwise, try to download it
    println!("Input file not found, attempting to download...");
    match download_input(day) {
        Ok(input) => input,
        Err(e) => panic!(
            "Failed to download input for day {}: {}\n\nMake sure:\n1. AOC_SESSION environment variable is set\n2. The puzzle for day {} has been released",
            day, e, day
        ),
    }
}

/// Parse a string into a vector of numbers, one per line
///
/// # Example
/// ```
/// # use advent_of_code_2025::parse_numbers;
/// let input = "1\n2\n3";
/// let numbers: Vec<i32> = parse_numbers(input);
/// assert_eq!(numbers, vec![1, 2, 3]);
/// ```
pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Split a string by blank lines (double newlines) into groups
///
/// Useful for puzzles where input is separated into groups
pub fn split_by_blank_lines(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone)]
struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl Grid<char> {
    /// Parse a grid of chars from a multiline string.
    /// Assumes all lines have the same length.
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map_or(0, |l| l.chars().count());

        assert!(width > 0, "grid must not be empty");

        let mut data = Vec::with_capacity(width * height);
        for line in lines {
            let len = line.chars().count();
            assert_eq!(len, width, "all lines must have the same length");
            data.extend(line.chars());
        }

        Grid {
            width,
            height,
            data,
        }
    }
}
impl Grid<u64> {
    fn parse_u64s(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map_or(0, |l| l.split_whitespace().count());

        assert!(width > 0, "grid must not be empty");

        let mut data = Vec::with_capacity(width * height);
        for line in lines {
            let len = line.split_whitespace().count();
            assert_eq!(len, width, "all lines must have the same length");
            data.extend(line.split_whitespace().map(|s| s.parse::<u64>().unwrap()));
        }

        Grid {
            width,
            height,
            data,
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn transpose(&self) -> Grid<T> {
        let mut data = Vec::with_capacity(self.width * self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                data.push(self.data[self.index(x, y)]);
            }
        }

        Grid {
            width: self.height,
            height: self.width,
            data,
        }
    }
}

impl<T> Grid<T> {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.in_bounds(x, y) {
            Some(&self.data[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn col(&self, x: usize) -> impl Iterator<Item = &T> {
        (0..self.height).map(move |y| &self.data[y * self.width + x])
    }

    /// Return (x, y) indices of all 8 neighbors inside the grid.
    fn neighbors8_indices(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        OFFSETS.into_iter().filter_map(move |(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
    }

    /// Directly iterate over neighbor values.
    fn neighbors8(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.neighbors8_indices(x, y)
            .map(move |(nx, ny)| &self.data[self.index(nx, ny)])
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            self.data[idx] = value;
        }
    }
}
