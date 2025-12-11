use adventofcode25::{input_path, read_lines};

const DAY: u8 = 11;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

fn solve_part1(input: &str) -> u64 {
    let lines = read_lines(input).expect("Failed to read input file");
    for line in lines.map_while(Result::ok) {
        println!("{line}");
    }
    let result = 0;
    println!("Part 1: {}", result);
    result
}

fn solve_part2(input: &str) -> u64 {
    let lines = read_lines(input).expect("Failed to read input file");
    for _line in lines.map_while(Result::ok) {
    }
    let result = 0;
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY)), 0); // TODO: fill in expected value
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 0); // TODO: fill in expected value
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
