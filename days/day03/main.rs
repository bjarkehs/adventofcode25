use adventofcode25::{example_path, input_path, read_lines};

fn main() {
    let example = example_path(3);
    println!("Example Part 1: {}", run(&example, 2));
    println!("Example Part 2: {}", run(&example, 12));
    let input = input_path(3);
    println!("Part 1: {}", run(&input, 2));
    println!("Part 2: {}", run(&input, 12));
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
