use adventofcode25::{input_path, read_lines};

const DAY: u8 = 12;

fn main() {
    let input = parse_input(&input_path(DAY));
    solve_part1(&input);
    solve_part2(&input);
}

type Coordinate = (u64, u64);

struct Present {
    coords: Vec<Coordinate>,
}

struct Region {
    size: (u64, u64),
    shape_quantities: [u64; 6],
}

struct Input {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> Input {
    let mut presents = Vec::new();
    let mut regions = Vec::new();

    let mut is_scanning_presents = true;
    let mut current_coords: Vec<Coordinate> = Vec::new();
    let mut current_y = 0u64;
    let lines = read_lines(input).expect("Failed to read input file");
    for line in lines.map_while(Result::ok) {
        if line.contains("x") {
            is_scanning_presents = false;
        }

        if is_scanning_presents {
            if line.contains(':') {
                continue;
            }

            if line.is_empty() {
                presents.push(Present {
                    coords: current_coords.clone(),
                });
                current_coords.clear();
                current_y = 0;
                continue;
            }

            let positions = line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((x as u64, current_y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coordinate>>();

            current_coords.extend(positions);
            current_y += 1;
            continue;
        } else {
            let parts = line.split(':').collect::<Vec<&str>>();
            let size_parts = parts[0]
                .trim()
                .split('x')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let size = (size_parts[0], size_parts[1]);
            let shape_quantities = parts[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            regions.push(Region {
                size,
                shape_quantities: [
                    shape_quantities[0],
                    shape_quantities[1],
                    shape_quantities[2],
                    shape_quantities[3],
                    shape_quantities[4],
                    shape_quantities[5],
                ],
            });
        }
    }

    Input { presents, regions }
}

fn solve_part1(input: &Input) -> u64 {
    let mut result = 0;
    for region in &input.regions {}
    println!("Part 1: {}", result);
    result
}

fn fast_pruning(region: &Region, presents: &Vec<Present>) -> Option<bool> {
    let presents_area = presents
        .iter()
        .enumerate()
        .map(|(index, p)| region.shape_quantities[index] * p.coords.len() as u64)
        .sum::<u64>();
    if region.size.0 * region.size.1 < presents_area {
        return Some(false);
    }

    let available_blocks = region.size.0 / 3 * region.size.1 / 3;
    let required_blocks = presents
        .iter()
        .enumerate()
        .map(|(index, _)| region.shape_quantities[index])
        .sum::<u64>();

    if required_blocks <= available_blocks {
        return Some(true);
    }

    return None;
}

fn solve_part2(input: &Input) -> u64 {
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
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn part1_real() {
        let input = parse_input(&input_path(DAY));
        solve_part1(&input);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(&example_path(DAY));
        assert_eq!(solve_part2(&input), 0); // TODO: fill in expected value
    }

    #[test]
    fn part2_real() {
        let input = parse_input(&input_path(DAY));
        solve_part2(&input);
    }
}
