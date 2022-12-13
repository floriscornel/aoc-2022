use std::collections::{HashMap, HashSet};

use crate::read_lines;

#[allow(dead_code)]
fn solve(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let u8_to_val = |char: u8| match char {
        b'a'..=b'z' => char - b'a' + 1,
        b'A'..=b'Z' => char - b'A' + 27,
        _ => panic!("Unexpected char"),
    } as i64;

    let mut sum = 0;
    for line in lines {
        let (left, right) = line.as_bytes().split_at(line.len() / 2);

        let mut set = HashSet::new();
        for char in left {
            set.insert(char);
        }
        for char in right {
            if set.remove(char) {
                sum += u8_to_val(*char);
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn solve2(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let u8_to_val = |char: u8| match char {
        b'a'..=b'z' => char - b'a' + 1,
        b'A'..=b'Z' => char - b'A' + 27,
        _ => panic!("Unexpected char"),
    } as i64;

    let mut sum = 0;
    let mut map = HashMap::new();
    for (idx, line) in lines.into_iter().enumerate() {
        if idx % 3 == 0 {
            map = HashMap::new();
        }
        let set = HashSet::<char>::from_iter(line.chars());
        for char in set {
            map.entry(char).and_modify(|c| *c += 1).or_insert(1);
        }
        if idx % 3 == 2 {
            for (key, val) in &map {
                if *val == 3 {
                    sum += u8_to_val(*key as u8);
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("3.test.in"), 157);
        assert_eq!(solve2("3.test.in"), 70);

        println!("Answer 3 pt.1: {}", solve("3.in"));
        println!("Answer 3 pt.2: {}", solve2("3.in"));
    }
}
