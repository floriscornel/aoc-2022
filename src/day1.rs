use crate::read_lines;

#[allow(dead_code)]
fn solve(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let (mut elves, mut idx) = (vec![0], 0);
    for line in lines {
        if let Ok(calories) = line.parse::<i64>() {
            elves[idx] += calories;
        } else {
            elves.push(0);
            idx += 1;
        }
    }
    elves.into_iter().max().unwrap()
}

#[allow(dead_code)]
fn solve2(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let (mut elves, mut idx) = (vec![0], 0);
    for line in lines {
        if let Ok(calories) = line.parse::<i64>() {
            elves[idx] += calories;
        } else {
            elves.push(0);
            idx += 1;
        }
    }
    elves.sort_unstable();
    elves.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("1.test.in"), 24000);
        assert_eq!(solve2("1.test.in"), 45000);

        println!("Answer 1 pt.1: {}", solve("1.in"));
        println!("Answer 1 pt.2: {}", solve2("1.in"));
    }
}
