pub mod days;

use std::fs;
use std::path::Path;

const YEAR: u32 = 2024;

/// Download input for a given day from adventofcode.com
/// Requires AOC_SESSION environment variable to be set with your session cookie
fn download_input(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let session = std::env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable not set")?;

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
        Err(e) => panic!("Failed to download input for day {}: {}\n\nMake sure:\n1. AOC_SESSION environment variable is set\n2. The puzzle for day {} has been released", day, e, day),
    }
}

/// Parse a string into a vector of numbers, one per line
///
/// # Example
/// ```
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
