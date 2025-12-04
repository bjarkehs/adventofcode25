use adventofcode25::{example_path, input_path, read_lines};

fn main() {
    part2();
}

fn example_part1() {
    run_part1(example_path(2));
}

fn part1() {
    run_part1(input_path(2));
}

fn example_part2() {
    run_part2(example_path(2));
}

fn part2() {
    run_part2(input_path(2));
}

fn run_part1(input: String) {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            for range in line.split(',') {
                let bounds: Vec<&str> = range.split('-').collect();
                let start: u64 = bounds[0].parse().unwrap();
                let end: u64 = bounds[1].parse().unwrap();
                for number in start..=end {
                    let number_as_string = number.to_string();
                    let first_half = &number_as_string[..number_as_string.len() / 2];
                    let second_half = &number_as_string[number_as_string.len() / 2..];
                    if first_half == second_half {
                        sum += number;
                    }
                }
            }
        }
    }
    println!("Sum: {}", sum);
}

fn run_part2(input: String) {
    let mut sum = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            for range in line.split(',') {
                let bounds: Vec<&str> = range.split('-').collect();
                let start: u64 = bounds[0].parse().unwrap();
                let end: u64 = bounds[1].parse().unwrap();
                for number in start..=end {
                    let number_as_string = number.to_string();
                    let middle_index = number_as_string.len() / 2;
                    for i in 1..=middle_index {
                        if (number_as_string.len() % i) != 0 {
                            // Not divisible
                            continue;
                        }
                        let possible_slices = number_as_string.len() / i;
                        let current_string = &number_as_string[..i];
                        let mut is_match = true;
                        for j in 1..possible_slices {
                            let checked_string = &number_as_string[j * i..j * i + i];
                            if checked_string != current_string {
                                is_match = false;
                                break;
                            }
                        }

                        if is_match {
                            sum += number;
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("Sum: {}", sum);
}
