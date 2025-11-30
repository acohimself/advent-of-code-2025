#!/bin/bash

# Script to set up the template for a specific year

if [ -z "$1" ]; then
    echo "Usage: ./setup.sh <year>"
    echo "Example: ./setup.sh 2025"
    exit 1
fi

YEAR=$1

echo "Setting up Advent of Code template for year $YEAR..."

# Update Cargo.toml
sed -i.bak "s/name = \"advent-of-code\"/name = \"advent-of-code-$YEAR\"/" Cargo.toml && rm Cargo.toml.bak
echo "✓ Updated Cargo.toml"

# Update src/main.rs
sed -i.bak "s/use advent_of_code::/use advent_of_code_$YEAR::/" src/main.rs && rm src/main.rs.bak
echo "✓ Updated src/main.rs"

# Update src/lib.rs
sed -i.bak "s/const YEAR: u32 = [0-9]*;/const YEAR: u32 = $YEAR;/" src/lib.rs && rm src/lib.rs.bak
echo "✓ Updated src/lib.rs"

echo ""
echo "Setup complete! Your template is now configured for Advent of Code $YEAR."
echo ""
echo "Next steps:"
echo "1. Set your session cookie:"
echo "   Bash/Zsh: export AOC_SESSION=your_session_cookie_here"
echo "   Fish:     set -Ux AOC_SESSION your_session_cookie_here"
echo "2. Create your first day: ./new_day.sh 1"
echo "3. Start coding!"
