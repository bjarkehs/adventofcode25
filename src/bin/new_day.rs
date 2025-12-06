use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin new_day -- <day_number>");
        eprintln!("Example: cargo run --bin new_day -- 5");
        std::process::exit(1);
    }

    let day: u8 = match args[1].parse() {
        Ok(d) if d >= 1 && d <= 25 => d,
        _ => {
            eprintln!("Day must be a number between 1 and 25");
            std::process::exit(1);
        }
    };

    let day_dir = format!("days/day{:02}", day);
    let main_path = format!("{}/main.rs", day_dir);
    let input_path = format!("{}/input.txt", day_dir);
    let example_path = format!("{}/example.txt", day_dir);

    if Path::new(&day_dir).exists() {
        eprintln!("Day {} already exists at {}", day, day_dir);
        std::process::exit(1);
    }

    fs::create_dir_all(&day_dir).expect("Failed to create day directory");

    let template = format!(
        r#"use adventofcode25::{{input_path, read_lines}};

const DAY: u8 = {};

fn main() {{
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}}

fn solve_part1(input: &str) -> u64 {{
    if let Ok(lines) = read_lines(input) {{
        for line in lines.map_while(Result::ok) {{
            println!("{{line}}");
        }}
    }}
    let result = 0;
    println!("Part 1: {{}}", result);
    result
}}

fn solve_part2(input: &str) -> u64 {{
    if let Ok(lines) = read_lines(input) {{
        for _line in lines.map_while(Result::ok) {{
        }}
    }}
    let result = 0;
    println!("Part 2: {{}}", result);
    result
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {{
        assert_eq!(solve_part1(&example_path(DAY)), 0); // TODO: fill in expected value
    }}

    #[test]
    fn part1_real() {{
        solve_part1(&input_path(DAY));
    }}

    #[test]
    fn part2_example() {{
        assert_eq!(solve_part2(&example_path(DAY)), 0); // TODO: fill in expected value
    }}

    #[test]
    fn part2_real() {{
        solve_part2(&input_path(DAY));
    }}
}}
"#,
        day
    );

    fs::write(&main_path, template).expect("Failed to write main.rs");
    fs::write(&input_path, "").expect("Failed to write input.txt");
    fs::write(&example_path, "").expect("Failed to write example.txt");

    let cargo_entry = format!(
        r#"
[[bin]]
name = "day{:02}"
path = "days/day{:02}/main.rs"
"#,
        day, day
    );

    let mut cargo_file = OpenOptions::new()
        .append(true)
        .open("Cargo.toml")
        .expect("Failed to open Cargo.toml");

    cargo_file
        .write_all(cargo_entry.as_bytes())
        .expect("Failed to write to Cargo.toml");

    println!("Created day {} at {}", day, day_dir);
    println!("  - {}", main_path);
    println!("  - {}", input_path);
    println!("  - {}", example_path);
    println!("  - Added [[bin]] entry to Cargo.toml");
}
