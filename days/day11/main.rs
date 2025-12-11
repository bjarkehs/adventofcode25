use std::collections::HashMap;

use adventofcode25::{input_path, read_lines};

const DAY: u8 = 11;

fn main() {
    let input = parse_input(&input_path(DAY));
    solve_part1(&input);
    solve_part2(&input);
}

struct Input {
    vertices: HashMap<String, Vec<String>>,
}

fn parse_input(input: &str) -> Input {
    read_lines(input)
        .expect("Failed to read input file")
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            let node = parts[0].trim().to_string();
            let edges = parts[1]
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            (node, edges)
        })
        .fold(
            Input {
                vertices: HashMap::new(),
            },
            |mut acc, (node, edges)| {
                acc.vertices.insert(node, edges);
                acc
            },
        )
}

fn solve_part1(input: &Input) -> u64 {
    let result = count_paths(input, vec!["you"], "out");
    println!("Part 1: {}", result);
    result
}

fn count_paths(input: &Input, current_path: Vec<&str>, end: &str) -> u64 {
    if current_path.last().unwrap() == &end {
        return 1;
    }

    let mut total_paths = 0u64;
    if let Some(neighbors) = input.vertices.get(*current_path.last().unwrap()) {
        for neighbor in neighbors {
            if !current_path.contains(&neighbor.as_str()) {
                let mut new_path = current_path.clone();
                new_path.push(neighbor);
                total_paths += count_paths(input, new_path, end);
            }
        }
    }
    total_paths
}

fn count_paths_with_dac_fft(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    end: &str,
    required: (bool, bool), // (has_dac, has_fft)
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let has_dac = required.0 || node == "dac";
    let has_fft = required.1 || node == "fft";

    if node == end {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    let key = (node.to_string(), has_dac, has_fft);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total = 0u64;
    if let Some(children) = graph.get(node) {
        for next in children {
            total += count_paths_with_dac_fft(graph, next, end, (has_dac, has_fft), memo);
        }
    }

    memo.insert(key, total);
    total
}

fn solve_part2(input: &Input) -> u64 {
    let mut memo = HashMap::new();
    let result = count_paths_with_dac_fft(&input.vertices, "svr", "out", (false, false), &mut memo);
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::{example_path, example_path_n};

    #[test]
    fn part1_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn part1_real() {
        let input = parse_input(&input_path(DAY));
        solve_part1(&input);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(&example_path_n(DAY, 2));
        assert_eq!(solve_part2(&input), 2);
    }

    #[test]
    fn part2_real() {
        let input = parse_input(&input_path(DAY));
        solve_part2(&input);
    }
}
