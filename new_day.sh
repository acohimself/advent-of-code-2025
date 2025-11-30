#!/bin/bash

# Script to create a new Advent of Code day

if [ -z "$1" ]; then
    echo "Usage: ./new_day.sh <day_number>"
    echo "Example: ./new_day.sh 3"
    exit 1
fi

DAY=$1
DAY_PADDED=$(printf "%02d" $DAY)
DAY_FILE="src/days/day${DAY_PADDED}.rs"

# Check if day already exists
if [ -f "$DAY_FILE" ]; then
    echo "Error: Day $DAY already exists at $DAY_FILE"
    exit 1
fi

# Create the day file
cat > "$DAY_FILE" << 'EOF'
use crate::read_input;

pub fn solve() {
    let input = read_input(DAY_NUM);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
example input here";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
EOF

# Replace DAY_NUM with the actual day number
sed -i.bak "s/DAY_NUM/$DAY/g" "$DAY_FILE" && rm "${DAY_FILE}.bak"

echo "✓ Created $DAY_FILE"

# Update src/days.rs
if ! grep -q "pub mod day${DAY_PADDED};" src/days.rs; then
    echo "pub mod day${DAY_PADDED};" >> src/days.rs
    echo "✓ Added module to src/days.rs"
else
    echo "⚠ Module already exists in src/days.rs"
fi

# Update src/main.rs
if ! grep -q "$DAY => days::day${DAY_PADDED}::solve()" src/main.rs; then
    # Insert before the _ => default case
    sed -i.bak "s/        _ => /        $DAY => days::day${DAY_PADDED}::solve(),\\
        _ => /" src/main.rs && rm src/main.rs.bak
    echo "✓ Added day to src/main.rs"
else
    echo "⚠ Day already exists in src/main.rs"
fi

echo ""
echo "Day $DAY is ready!"
echo "Run with: cargo run $DAY"
echo "Test with: cargo test day${DAY_PADDED}"
