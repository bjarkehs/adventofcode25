use std::collections::HashMap;

use adventofcode25::{input_path, read_lines};

const DAY: u8 = 5;

fn main() {
    run_part1(&input_path(DAY));
    run_part2(&input_path(DAY));
}

fn run_part1(input: &str) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut available_ingredients: Vec<u64> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        let mut scanning_fresh = true;
        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                scanning_fresh = false;
                continue;
            }
            if scanning_fresh {
                let bounds: Vec<&str> = line.split('-').collect();
                let start: u64 = bounds[0].parse().unwrap();
                let end: u64 = bounds[1].parse().unwrap();
                ranges.push((start, end));
            } else {
                let ingredient: u64 = line.parse().unwrap();
                available_ingredients.push(ingredient);
            }
        }
    }
    println!(
        "Part 1: {}",
        count_fresh_ingredients(&ranges, &available_ingredients)
    );
}

fn count_fresh_ingredients(ranges: &Vec<(u64, u64)>, available_ingredients: &Vec<u64>) -> u32 {
    let mut count = 0;
    for &ingredient in available_ingredients {
        for &(start, end) in ranges {
            if ingredient >= start && ingredient <= end {
                count += 1;
                break;
            }
        }
    }
    count
}

fn run_part2(input: &str) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                break;
            }

            let bounds: Vec<&str> = line.split('-').collect();
            let start: u64 = bounds[0].parse().unwrap();
            let end: u64 = bounds[1].parse().unwrap();
            ranges.push((start, end));
        }
    }
    println!("Part 2: {}", count_all_fresh_ingredients(&ranges));
}

fn count_all_fresh_ingredients(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut points: Vec<(u64, i8)> = Vec::new();
    for &(start, end) in ranges {
        points.push((start, 1));
        points.push((end, -1));
    }

    points.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut in_range = 0i8;
    let mut range_start = 0u64;
    let mut count = 0u64;
    for (value, in_range_change) in points.iter() {
        in_range += *in_range_change;

        if in_range_change == &1 {
            if in_range == 1 {
                range_start = *value;
            }
            continue;
        }

        if in_range == 0 {
            count += value - range_start + 1;
        }
    }

    count
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
