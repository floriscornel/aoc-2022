use crate::read_lines;

fn build_stack(lines: &mut Vec<String>) -> Vec<Vec<char>> {
    let n = lines.last().unwrap().len() / 4 + 1;
    let mut stacks = vec![vec![]; n];
    while let Some(line) = lines.pop() {
        let bytes = line.as_bytes();
        if bytes[1] == b'1' {
            lines.pop();
            stacks.iter_mut().for_each(|s| s.reverse());
            break;
        }
        for i in 0..n {
            let char = bytes[i * 4 + 1];
            if char != b' ' {
                stacks[i].push(char as char);
            }
        }
    }
    stacks
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let parts = line.split(' ').collect::<Vec<&str>>();
    (
        parts[1].parse::<usize>().unwrap(),
        parts[3].parse::<usize>().unwrap() - 1,
        parts[5].parse::<usize>().unwrap() - 1,
    )
}

#[allow(dead_code)]
fn solve(filename: &str, part1: bool) -> String {
    let mut lines = read_lines(filename)
        .unwrap()
        .flatten()
        .collect::<Vec<String>>();
    lines.reverse();
    let mut stacks = build_stack(&mut lines);
    while let Some(line) = lines.pop() {
        let (count, from, to) = parse_move(&line);
        let from_len = stacks[from].len();
        let from_count = count.min(from_len);
        if part1 {
            for _i in 0..from_count {
                let char = stacks[from].pop().unwrap();
                stacks[to].push(char);
            }
        } else {
            let payload = stacks[from]
                .drain(from_len - from_count..)
                .collect::<Vec<char>>();
            stacks[to].extend(payload);
        }
    }
    stacks
        .into_iter()
        .map(|s| s.last().unwrap_or(&' ').to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("5.test.in", true), "CMZ".to_string());
        assert_eq!(solve("5.test.in", false), "MCD".to_string());

        println!("Answer 5 pt.1: {}", solve("5.in", true));
        println!("Answer 5 pt.2: {}", solve("5.in", false));
    }
}
