# Advent of Code

I decided AoC would be a good chance to learn some Rust, so this project
is me working through the puzzles and picking up the language as I go.

## Day 1

This solution reads the input from inputs/day01-part01.txt and parses
each rotation. L means rotate left (negative), R means rotate right
(positive). The dial has 100 positions so all movement is modulo 100.

Part 1:
Track the dial position starting from 50. Count how many times the dial
lands exactly on 0.

Part 2:
Count how many times the dial passes through 0 during rotation. Large
moves may cross zero multiple times. For each move:
• full wraps: abs(delta) / 100
• leftover: check if prev + leftover crosses below 0 or above 99

You can run everything with:
`cargo run`

## Day 2

This solution reads ranges from inputs/day02.txt and searches for
invalid product IDs. An ID is invalid if it is made from a digit sequence
repeated multiple times.

For now the implementation uses straightforward string checks inside each
range. It works but is not the most efficient approach and I plan to
revisit this later to explore a cleaner method.
