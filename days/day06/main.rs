use adventofcode25::{input_path, read_lines};

const DAY: u8 = 6;

fn main() {
    let input = parse_input(&input_path(DAY));
    solve_part1(&input);
    solve_part2(&input);
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("Unknown operator: {c}"),
        }
    }

    fn apply(self, values: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Add => values.sum(),
            Self::Multiply => values.product(),
        }
    }
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

fn find_problem_ranges(input: &Input) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = None;

    for col in 0..input.width {
        let is_separator = (0..input.height).all(|row| {
            input
                .grid
                .get(row)
                .and_then(|r| r.get(col))
                .is_none_or(|&c| c == ' ')
        });

        match (start, is_separator) {
            (None, false) => start = Some(col),
            (Some(s), true) => {
                ranges.push(s..col);
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        ranges.push(s..input.width);
    }
    ranges
}

fn solve_part1(input: &Input) -> u64 {
    let ranges = find_problem_ranges(input);
    let operator_row = input.height - 1;

    let result = ranges
        .iter()
        .map(|range| {
            let operator = Operator::from_char(input.grid[operator_row][range.start]);

            let values = (0..operator_row).map(|row| {
                let num_str: String = range
                    .clone()
                    .filter_map(|col| input.grid.get(row).and_then(|r| r.get(col)).copied())
                    .filter(|&c| c != ' ')
                    .collect();
                num_str.parse::<u64>().unwrap()
            });

            operator.apply(values)
        })
        .sum();

    println!("Part 1: {}", result);
    result
}

fn solve_part2(input: &Input) -> u64 {
    let ranges = find_problem_ranges(input);
    let operator_row = input.height - 1;

    let result = ranges
        .iter()
        .map(|range| {
            let operator = Operator::from_char(input.grid[operator_row][range.start]);

            let values = range.clone().rev().filter_map(|col| {
                let num_str: String = (0..operator_row)
                    .filter_map(|row| input.grid.get(row).and_then(|r| r.get(col)).copied())
                    .filter(|&c| c != ' ')
                    .collect();
                if num_str.is_empty() {
                    None
                } else {
                    Some(num_str.parse::<u64>().unwrap())
                }
            });

            operator.apply(values)
        })
        .sum();
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part1(&input), 4277556);
    }

    #[test]
    fn part1_real() {
        parse_input(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part2(&input), 3263827);
    }

    #[test]
    fn part2_real() {
        parse_input(&input_path(DAY));
    }
}
