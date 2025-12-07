use adventofcode25::{input_path, read_lines};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DAY: u8 = 7;

fn main() {
    let input = parse_input(&input_path(DAY));
    solve_part1(&input);
    solve_part2(&input);
}

struct Input {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn parse_input(path: &str) -> Input {
    let lines = read_lines(path).expect("Failed to read input");
    let grid: Vec<Vec<char>> = lines
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let width = grid.iter().map(Vec::len).max().unwrap_or(0);
    let height = grid.len();

    Input {
        grid,
        width,
        height,
    }
}

fn solve_part1(input: &Input) -> u64 {
    let mut beam_deque: VecDeque<(usize, usize)> = VecDeque::new();
    let initial_beam = input.grid[0]
        .iter()
        .enumerate()
        .filter_map(|(col, &c)| if c == 'S' { Some((0, col)) } else { None })
        .collect::<Vec<(usize, usize)>>()
        .first()
        .unwrap()
        .clone();
    beam_deque.push_back(initial_beam);
    let mut splits = 0u64;
    let mut beam_positions: HashSet<(usize, usize)> = HashSet::new();
    while !beam_deque.is_empty() {
        let (row, column) = beam_deque.pop_front().unwrap();
        if beam_positions.contains(&(row, column)) {
            continue;
        }
        beam_positions.insert((row, column));
        if row + 1 >= input.height {
            continue;
        }
        let below = input.grid[row + 1][column];
        if below == '^' {
            // split beam
            splits += 1;
            // left beam
            if column > 0 {
                beam_deque.push_back((row, column - 1));
            }
            // right beam
            if column + 1 < input.width {
                beam_deque.push_back((row, column + 1));
            }
            continue;
        } else {
            // continue downwards
            beam_deque.push_back((row + 1, column));
        }
    }
    println!("Part 1: {}", splits);
    splits
}

fn solve_part2(input: &Input) -> u64 {
    let initial_beam = input.grid[0]
        .iter()
        .enumerate()
        .filter_map(|(col, &c)| if c == 'S' { Some((0, col)) } else { None })
        .collect::<Vec<(usize, usize)>>()
        .first()
        .unwrap()
        .clone();
    let mut cache = HashMap::new();
    let timelines = solve_timeline(input, initial_beam.0, initial_beam.1, &mut cache);
    println!("Part 2: {}", timelines);
    timelines
}

fn solve_timeline(
    input: &Input,
    row: usize,
    column: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(row, column)) {
        return cached;
    }

    if row + 1 >= input.height {
        return 1;
    }

    let below = input.grid[row + 1][column];

    let mut timelines = 0u64;
    if below == '^' {
        timelines += solve_timeline(input, row, column - 1, cache);
        timelines += solve_timeline(input, row, column + 1, cache);
    } else {
        timelines = solve_timeline(input, row + 1, column, cache);
    }
    cache.insert((row, column), timelines);
    timelines
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part1(&input), 21);
    }

    #[test]
    fn part1_real() {
        let input = parse_input(&input_path(DAY));
        solve_part1(&input);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part2(&input), 40);
    }

    #[test]
    fn part2_real() {
        let input = parse_input(&input_path(DAY));
        solve_part2(&input);
    }
}
