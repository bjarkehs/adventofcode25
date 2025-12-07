use adventofcode25::{input_path, read_lines};

const DAY: u8 = 5;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut parsing_ranges = true;

    let lines = read_lines(input).expect("Failed to read input file");
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }
        if parsing_ranges {
            let (start, end) = line.split_once('-').expect("Invalid range format");
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }
    (ranges, ingredients)
}

fn solve_part1(input: &str) -> u32 {
    let (mut ranges, ingredients) = parse_input(input);
    ranges.sort_unstable();

    let result = count_fresh_ingredients(&ranges, &ingredients);
    println!("Part 1: {result}");
    result
}

fn count_fresh_ingredients(sorted_ranges: &[(u64, u64)], ingredients: &[u64]) -> u32 {
    ingredients
        .iter()
        .filter(|&&id| is_fresh(sorted_ranges, id))
        .count() as u32
}

fn is_fresh(sorted_ranges: &[(u64, u64)], id: u64) -> bool {
    sorted_ranges
        .iter()
        .any(|&(start, end)| start <= id && id <= end)
}

fn solve_part2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let result = count_all_fresh_ids(&ranges);
    println!("Part 2: {result}");
    result
}

fn count_all_fresh_ids(ranges: &[(u64, u64)]) -> u64 {
    let mut events: Vec<(u64, i8)> = Vec::with_capacity(ranges.len() * 2);
    for &(start, end) in ranges {
        events.push((start, 1));
        events.push((end + 1, -1));
    }
    events.sort_unstable();

    let mut depth = 0i32;
    let mut range_start = 0u64;
    let mut count = 0u64;

    for (pos, delta) in events {
        let prev_depth = depth;
        depth += delta as i32;

        if prev_depth == 0 && depth > 0 {
            range_start = pos;
        } else if prev_depth > 0 && depth == 0 {
            count += pos - range_start;
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
        assert_eq!(solve_part1(&example_path(DAY)), 3);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 14);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
