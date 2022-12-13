mod day1;
mod day2;
mod day3;
mod day4;

use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let mut path: String = "inputs/".to_owned();
    path.push_str(filename);
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
