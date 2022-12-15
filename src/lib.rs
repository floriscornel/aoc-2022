use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let mut path: String = "inputs/".to_owned();
    path.push_str(filename);
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut path: String = "inputs/".to_owned();
    path.push_str(filename);
    read_to_string(path)
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
