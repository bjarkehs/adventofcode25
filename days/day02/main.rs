use adventofcode25::{input_path, read_lines};

const DAY: u8 = 2;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

fn solve_part1(input: &str) -> u64 {
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
    println!("Part 1: {}", sum);
    sum
}

fn solve_part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines(input) {
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
    println!("Part 2: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY)), 1227775554);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 4174379265);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
