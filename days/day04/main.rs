use adventofcode25::{example_path, input_path, read_lines};

fn main() {
    part2();
}

fn example_part1() {
    run_part1(example_path(4));
}

fn part1() {
    run_part1(input_path(4));
}

fn example_part2() {
    run_part2(example_path(4));
}

fn part2() {
    run_part2(input_path(4));
}

fn run_part1(input: String) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            let row = line
                .chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<u8>>();
            grid.push(row);
        }
    }

    let positions = find_positions(&grid, 3);
    let sum = positions.len();
    println!("Sum: {}", sum);
}

fn find_positions(grid: &Vec<Vec<u8>>, max_surrounding: u8) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value == 1 && check_position(grid, row_index, col_index, max_surrounding) {
                positions.push((row_index, col_index));
            }
        }
    }
    positions
}

fn check_position(
    grid: &Vec<Vec<u8>>,
    row_index: usize,
    col_index: usize,
    max_surrounding: u8,
) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut surrounding_count = 0;
    for (dr, dc) in directions.iter() {
        let new_row = row_index as isize + dr;
        let new_col = col_index as isize + dc;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            surrounding_count += grid[new_row as usize][new_col as usize];
            if surrounding_count > max_surrounding {
                return false;
            }
        }
    }
    true
}

fn run_part2(input: String) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines(&input) {
        for line in lines.map_while(Result::ok) {
            let row = line
                .chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<u8>>();
            grid.push(row);
        }
    }

    let mut find_more_positions = true;
    let mut sum = 0;
    while find_more_positions {
        let positions = find_positions(&grid, 3);
        for (row, col) in &positions {
            grid[*row][*col] = 0;
        }
        let found_positions = positions.len();
        println!("Found positions: {}", found_positions);
        sum += found_positions;
        if found_positions == 0 {
            find_more_positions = false;
        }
    }
    println!("Sum: {}", sum);
}
