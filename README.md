# Advent of Code - Rust Template

A clean, efficient template for solving Advent of Code puzzles in Rust.

## Features

- Single binary with day selection via CLI
- Automatic input downloading from adventofcode.com
- Common utility functions for parsing
- Test-driven development support
- Automatic day generation script

## Quick Start

### 1. Use this template

Click "Use this template" on GitHub

### 2. Set up for your year

```bash
./setup.sh 2025
```

This will automatically update the project name and year in all necessary files.

### 3. Set up your session cookie (for automatic input downloads)

Get your session cookie from adventofcode.com:
1. Log into https://adventofcode.com
2. Open browser DevTools (F12)
3. Go to Application/Storage → Cookies
4. Copy the value of the `session` cookie

Set it as an environment variable:

**Bash/Zsh:**
```bash
export AOC_SESSION=your_session_cookie_here
```

**Fish:**
```fish
set -Ux AOC_SESSION your_session_cookie_here
```

### 4. Create a new day

```bash
./new_day.sh 2
```

This will:
- Create `src/days/day02.rs` with the template
- Update `src/days.rs` to include the module
- Update `src/main.rs` to add the day to the runner

### 5. Implement your solution

Edit `src/days/dayXX.rs`:
- Add example input to `EXAMPLE` constant
- Update test expectations
- Implement `part1()` and `part2()`

When you run your solution, the input will be automatically downloaded if not present!

### 6. Test and run

```bash
# Run tests
cargo test day01 -- --nocapture

# Run actual solution
cargo run 1
```

## Utility Functions

Available in `src/lib.rs`:

```rust
// Read input file as string (auto-downloads if missing)
let input = read_input(1);

// Parse lines into numbers
let numbers: Vec<i32> = parse_numbers(&input);

// Split by blank lines (for grouped inputs)
let groups = split_by_blank_lines(&input);
```

## Project Structure

```
.
├── Cargo.toml          # Project configuration
├── new_day.sh          # Script to generate new days
├── src/
│   ├── lib.rs          # Utility functions
│   ├── main.rs         # CLI runner
│   ├── days.rs         # Module declarations
│   └── days/
│       └── day01.rs    # Day 1 solution
└── input/
    └── day01.txt       # Puzzle input (gitignored)
```

