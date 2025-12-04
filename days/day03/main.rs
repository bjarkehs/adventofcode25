use adventofcode25::{example_path, input_path, read_lines};

fn main() {
    part2();
}

fn example_part1() {
    run_part1(example_path(3));
}

fn part1() {
    run_part1(input_path(3));
}

fn example_part2() {
    run_part2(example_path(3));
}

fn part2() {
    run_part2(input_path(3));
}

fn run_part1(input: String) {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            let numbers = line.chars().map(|c| c.to_digit(10).unwrap() as u64);
            let mut highest_digit = 0;
            let mut second_highest_digit = 0;
            for (index, number) in numbers.enumerate() {
                if number > highest_digit && index != line.len() - 1 {
                    second_highest_digit = 0;
                    highest_digit = number;
                } else if number > second_highest_digit {
                    second_highest_digit = number;
                }
            }

            let number_as_string = format!("{}{}", highest_digit, second_highest_digit);
            let number: u64 = number_as_string.parse().unwrap();
            sum += number;
        }
    }
    println!("Sum: {}", sum);
}

fn run_part2(input: String) {
    let mut sum = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            let mut current_index = 0;
            let numbers = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();
            let mut found_digits = [0u8; 12];
            for digit_index in 0..12 {
                let end_index = line.len() - (11 - digit_index);
                let (highest_digit, highest_digit_index) = highest_digit(
                    numbers
                        .clone()
                        .iter()
                        .skip(current_index)
                        .take(end_index - current_index)
                        .map(ToOwned::to_owned),
                )
                .unwrap();
                found_digits[digit_index] = highest_digit;
                current_index += highest_digit_index + 1;
            }

            sum += found_digits
                .iter()
                .fold(0u64, |acc, &digit| acc * 10 + digit as u64);
        }
    }
    println!("Sum: {}", sum);
}

fn highest_digit<T>(numbers: T) -> Option<(u8, usize)>
where
    T: IntoIterator<Item = u8>,
{
    let mut highest_digit = 0u8;
    let mut highest_index = -1;
    for (index, number) in numbers.into_iter().enumerate() {
        if number > highest_digit {
            highest_digit = number;
            highest_index = index as isize;
        }
    }
    if highest_index == -1 {
        return None;
    }
    Some((highest_digit, highest_index as usize))
}
