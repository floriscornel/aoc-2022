use std::collections::{HashSet, VecDeque};

use crate::read_file_to_string;

#[allow(dead_code)]
fn solve(filename: &str, part1: bool) -> usize {
    let line = read_file_to_string(filename).unwrap();
    let window_size = if part1 { 4 } else { 14 };

    let mut window = VecDeque::from_iter(line.chars().take(window_size));
    for (idx, char) in line.chars().enumerate().skip(window_size) {
        let unique = HashSet::<char>::from_iter(window.iter().cloned()).len() == window.len();
        if unique {
            return idx;
        }
        window.pop_front();
        window.push_back(char);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("6.test.in", true), 7);
        assert_eq!(solve("6.test.in", false), 19);

        println!("Answer 6 pt.1: {}", solve("6.in", true));
        println!("Answer 6 pt.2: {}", solve("6.in", false));
    }
}
