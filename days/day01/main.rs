use adventofcode25::{input_path, read_lines};
use regex::Regex;

const DAY: u8 = 1;
const INITIAL_VALUE: i32 = 50;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

fn solve_part1(input: &str) -> i32 {
    let mut value = INITIAL_VALUE;
    let regex = Regex::new(r"([LR])(\d+)").unwrap();
    let mut count = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            for cap in regex.captures_iter(&line) {
                let direction = &cap[1];
                let distance: i32 = cap[2].parse().unwrap();

                match direction {
                    "L" => {
                        value = (value - distance).rem_euclid(100);
                    }
                    "R" => {
                        value = (value + distance).rem_euclid(100);
                    }
                    _ => panic!("Unknown direction"),
                }
                if value == 0 {
                    count += 1;
                }
            }
        }
    }
    println!("Part 1: {}", count);
    count
}

fn solve_part2(input: &str) -> i32 {
    let mut value = INITIAL_VALUE;
    let regex = Regex::new(r"([LR])(\d+)").unwrap();
    let mut count = 0;
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            for cap in regex.captures_iter(&line) {
                let direction = &cap[1];
                let distance: i32 = cap[2].parse().unwrap();
                count += match direction {
                    "R" => (value + distance) / 100,
                    "L" => {
                        if value == 0 {
                            distance / 100
                        } else if distance >= value {
                            (distance - value) / 100 + 1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown direction"),
                };

                value = match direction {
                    "R" => (value + distance).rem_euclid(100),
                    "L" => (value - distance).rem_euclid(100),
                    _ => unreachable!(),
                };
            }
        }
    }
    println!("Part 2: {}", count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY)), 3);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 6);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
