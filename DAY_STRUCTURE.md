# Day Structure

Each day follows a consistent structure for solving Advent of Code puzzles.

## File Layout

```
days/dayXX/
  main.rs      # Solution code
  input.txt    # Puzzle input (gitignored)
  example.txt  # Example input (gitignored)
```

## Code Structure

### Function Naming

- `solve_part1(input: &str) -> T` - Solves part 1
- `solve_part2(input: &str) -> T` - Solves part 2

### Return Values

Functions must both print AND return their result:

```rust
fn solve_part1(input: &str) -> u64 {
    // ... compute result
    println!("Part 1: {}", result);
    result
}
```

### Tests

Example tests should assert against expected values:

```rust
#[test]
fn part1_example() {
    assert_eq!(solve_part1(&example_path(DAY)), expected_value);
}

#[test]
fn part1_real() {
    solve_part1(&input_path(DAY));
}
```

## Creating a New Day

Use the scaffolding tool:

```bash
cargo run --bin new_day -- <day_number>
```

This creates the directory structure and registers the binary in `Cargo.toml`.
