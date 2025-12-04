# Advent of Code 2025 🎄

Ho ho ho! This is my attempt at solving [Advent of Code 2025](https://adventofcode.com/2025) puzzles using **Rust** 🦀

## Why Rust?

Because fighting the borrow checker is more fun than fighting elves.

## Running Solutions

```bash
cargo run --bin day01                                 # Run both parts with real input
cargo test --bin day01 -- --nocapture                 # Run all tests for a day
cargo test --bin day01 part1_example -- --nocapture   # Run specific test
```

Each day has 4 tests for flexible execution:
- `part1_example` / `part2_example` - Run with example input
- `part1_real` / `part2_real` - Run with real input

In VSCode with rust-analyzer, click "Run" or "Debug" above any test function.

## Creating a New Day

```bash
cargo run --bin new_day -- 5
```

This creates `days/day05/` with `main.rs`, `input.txt`, and `example.txt`.

## Progress

| Day | Stars |
| --- | ----- |
| 01  | ⭐⭐  |
| 02  | ⭐⭐  |
| 03  | ⭐⭐  |
| 04  | ⭐⭐  |
| ... | 💤    |

Legend: ⭐ = solved, 🎅 = in progress, 💤 = not started

## License

Free as in "free cookies for Santa"
