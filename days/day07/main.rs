use adventofcode25::{input_path, read_lines};
use std::collections::{HashMap, HashSet, VecDeque};

const DAY: u8 = 7;

type Pos = (usize, usize);

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

fn find_start(input: &Input) -> Pos {
    input.grid[0]
        .iter()
        .position(|&c| c == 'S')
        .map(|col| (0, col))
        .expect("No start position found")
}

fn solve_part1(input: &Input) -> u64 {
    let mut beam_deque: VecDeque<Pos> = VecDeque::new();
    beam_deque.push_back(find_start(input));

    let mut splits = 0u64;
    let mut visited: HashSet<Pos> = HashSet::new();

    while let Some((row, col)) = beam_deque.pop_front() {
        if !visited.insert((row, col)) {
            continue;
        }
        if row + 1 >= input.height {
            continue;
        }
        if input.grid[row + 1][col] == '^' {
            splits += 1;
            if col > 0 {
                beam_deque.push_back((row, col - 1));
            }
            if col + 1 < input.width {
                beam_deque.push_back((row, col + 1));
            }
        } else {
            beam_deque.push_back((row + 1, col));
        }
    }
    println!("Part 1: {splits}");
    splits
}

fn solve_part2(input: &Input) -> u64 {
    let (row, col) = find_start(input);
    let mut cache = HashMap::new();
    let timelines = count_timelines(input, row, col, &mut cache);
    println!("Part 2: {timelines}");
    timelines
}

fn count_timelines(input: &Input, row: usize, col: usize, cache: &mut HashMap<Pos, u64>) -> u64 {
    if let Some(&cached) = cache.get(&(row, col)) {
        return cached;
    }

    if row + 1 >= input.height {
        return 1;
    }

    let timelines = if input.grid[row + 1][col] == '^' {
        count_timelines(input, row, col - 1, cache) + count_timelines(input, row, col + 1, cache)
    } else {
        count_timelines(input, row + 1, col, cache)
    };

    cache.insert((row, col), timelines);
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
