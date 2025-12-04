use adventofcode25::{example_path, input_path, read_lines};
use regex::Regex;

fn main() {
    part2();
}

fn example_part1() {
    run_part1(example_path(1), 50);
}

fn part1() {
    run_part1(input_path(1), 50);
}

fn example_part2() {
    run_part2(example_path(1), 50);
}

fn part2() {
    run_part2(input_path(1), 50);
}

fn run_part1(input: String, inital_value: i32) {
    let mut value = inital_value;
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
    println!("Count: {}", count);
}

fn run_part2(input: String, inital_value: i32) {
    let mut value = inital_value;
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
    println!("Count: {}", count);
}
