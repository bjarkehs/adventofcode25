use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn input_path(day: u8) -> String {
    format!("days/day{:02}/input.txt", day)
}

pub fn example_path(day: u8) -> String {
    format!("days/day{:02}/example.txt", day)
}

pub fn example_path_n(day: u8, n: u8) -> String {
    format!("days/day{:02}/example_{}.txt", day, n)
}
