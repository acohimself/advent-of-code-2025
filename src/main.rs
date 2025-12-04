use advent_of_code_2025::days;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <day>");
        println!("Example: cargo run 1");
        return;
    }

    let day: u8 = args[1]
        .parse()
        .expect("Day must be a number between 1 and 25");

    if !(1..=25).contains(&day) {
        println!("Day must be between 1 and 25");
        return;
    }

    println!("Running Day {}", day);
    println!("---");

    let start = Instant::now();

    match day {
        1 => days::day01::solve(),
        // Add more days here as you implement them
        2 => days::day02::solve(),
        3 => days::day03::solve(),
        4 => days::day04::solve(),
        _ => println!("Day {} not yet implemented", day),
    }

    let elapsed = start.elapsed();
    println!("---");
    println!("Time: {:?}", elapsed);
}
