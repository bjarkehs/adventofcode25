use adventofcode25::{input_path, read_lines};

const DAY: u8 = 3;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

fn solve_part1(input: &str) -> u64 {
    let result = run(input, 2);
    println!("Part 1: {}", result);
    result
}

fn solve_part2(input: &str) -> u64 {
    let result = run(input, 12);
    println!("Part 2: {}", result);
    result
}

fn run(input: &str, digit_count: usize) -> u64 {
    let mut sum = 0u64;
    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let digits: Vec<u8> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect();

            let found = find_highest_digits(&digits, digit_count);
            sum += found.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        }
    }
    sum
}

fn find_highest_digits(digits: &[u8], count: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(count);
    let mut current_index = 0;

    for i in 0..count {
        let remaining = count - i - 1;
        let end_index = digits.len() - remaining;
        let window = &digits[current_index..end_index];

        let (digit, idx) = highest_in_slice(window).unwrap();
        result.push(digit);
        current_index += idx + 1;
    }
    result
}

fn highest_in_slice(slice: &[u8]) -> Option<(u8, usize)> {
    let mut max_digit = 0u8;
    let mut max_idx = None;
    for (idx, &d) in slice.iter().enumerate() {
        if d > max_digit {
            max_digit = d;
            max_idx = Some(idx);
        }
    }
    max_idx.map(|idx| (max_digit, idx))
}

// --- O(n) stack-based solution ---

#[allow(dead_code)]
fn run_stack(input: &str, digit_count: usize) -> u64 {
    let mut sum = 0u64;
    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let digits: Vec<u8> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect();

            let found = find_highest_digits_stack(&digits, digit_count);
            sum += found.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        }
    }
    sum
}

/// O(n) solution using monotonic stack.
/// Instead of repeatedly finding max in windows, we greedily remove
/// (n - k) digits that would make the result smaller.
#[allow(dead_code)]
fn find_highest_digits_stack(digits: &[u8], count: usize) -> Vec<u8> {
    let to_remove = digits.len() - count;
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());
    let mut removed = 0;

    for &d in digits {
        while removed < to_remove && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            removed += 1;
        }
        stack.push(d);
    }

    stack.truncate(count);
    stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY)), 357);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 3121910778619);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
