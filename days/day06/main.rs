use adventofcode25::{input_path, read_lines};

const DAY: u8 = 6;

fn main() {
    run_part1(&input_path(DAY));
    run_part2(&input_path(DAY));
}

enum Operator {
    Add,
    Multiply,
}

struct Problem {
    values: Vec<u64>,
    operator: Operator,
}

fn run_part1(input: &str) {
    let mut rows: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let mut columns: Vec<String> = Vec::new();
            for value in line.split_whitespace() {
                columns.push(value.to_string());
            }
            rows.push(columns);
        }
    }

    let problem_count = rows[0].len();
    let operator_index = rows.len() - 1;
    let mut problems: Vec<Problem> = Vec::with_capacity(problem_count);
    for col in 0..problem_count {
        let mut values: Vec<u64> = Vec::new();
        for row in 0..operator_index {
            let value: u64 = rows[row][col].parse().unwrap();
            values.push(value);
        }
        let operator = rows[operator_index][col]
            .chars()
            .next()
            .map(map_char_to_operator)
            .unwrap();
        problems.push(Problem { values, operator });
    }
    let sum_of_results: u64 = problems.iter().map(solve_problem).sum();
    println!("Part 1: {}", sum_of_results);
}

fn solve_problem(problem: &Problem) -> u64 {
    match problem.operator {
        Operator::Add => problem.values.iter().sum(),
        Operator::Multiply => problem.values.iter().product(),
    }
}

fn map_char_to_operator(c: char) -> Operator {
    match c {
        '+' => Operator::Add,
        '*' => Operator::Multiply,
        _ => panic!("Unknown operator"),
    }
}

fn run_part2(input: &str) {
    let mut problems: Vec<Problem> = Vec::new();
    let mut column_widths: Vec<u8> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        let all_lines: Vec<String> = lines.map_while(Result::ok).collect();
        let mut current_index: usize = 0;
        column_widths.push(1);
        for char in all_lines[all_lines.len() - 1].chars().skip(1) {
            if char != ' ' {
                current_index += 1;
                column_widths.push(1);
                column_widths[current_index - 1] -= 1;
            } else {
                column_widths[current_index] += 1;
            }
        }

        let mut index_offset = 0;
        let amount_of_values = all_lines.len() - 1;
        for &width in column_widths.iter() {
            let mut value_lists: Vec<Vec<u64>> = vec![Vec::new(); width as usize];
            for line in all_lines.iter().take(amount_of_values) {
                let value_str = &line[index_offset..index_offset + width as usize]
                    .chars()
                    .rev();
                for (char_index, c) in value_str.clone().into_iter().enumerate() {
                    if c != ' ' {
                        let value = c.to_digit(10).unwrap() as u64;
                        value_lists[char_index].push(value);
                    }
                }
            }

            let values = value_lists
                .into_iter()
                .map(|digits| digits.into_iter().fold(0u64, |acc, d| acc * 10 + d as u64))
                .collect::<Vec<u64>>();

            let operator = all_lines[all_lines.len() - 1]
                .chars()
                .nth(index_offset)
                .map(map_char_to_operator)
                .unwrap();

            index_offset += width as usize + 1;

            problems.push(Problem { values, operator });
        }
    }

    let sum_of_results: u64 = problems.iter().map(solve_problem).sum();
    println!("Part 2: {}", sum_of_results);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        run_part1(&example_path(DAY));
    }

    #[test]
    fn part1_real() {
        run_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        run_part2(&example_path(DAY));
    }

    #[test]
    fn part2_real() {
        run_part2(&input_path(DAY));
    }
}
