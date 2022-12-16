use crate::read_lines;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn new(str: &str) -> Self {
        match str {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn anticipate(left: &Choice, result: &Result) -> Self {
        match (left, result) {
            (Choice::Rock, Result::Draw)
            | (Choice::Paper, Result::Lose)
            | (Choice::Scissors, Result::Win) => Self::Rock,
            (Choice::Rock, Result::Win)
            | (Choice::Paper, Result::Draw)
            | (Choice::Scissors, Result::Lose) => Self::Paper,
            (Choice::Scissors, Result::Draw)
            | (Choice::Rock, Result::Lose)
            | (Choice::Paper, Result::Win) => Self::Scissors,
        }
    }
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn play(left: Choice, right: Choice) -> Self {
        match (left, right) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Self::Lose,
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Self::Draw,
            (Choice::Scissors, Choice::Rock)
            | (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors) => Self::Win,
        }
    }

    fn new(str: &str) -> Self {
        match str {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid input"),
        }
    }
}

#[allow(dead_code)]
fn solve2(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let mut points = 0;
    for line in lines {
        let vec = line.split_whitespace().collect::<Vec<&str>>();
        let (left, result) = (Choice::new(vec[0]), Result::new(vec[1]));
        let right = Choice::anticipate(&left, &result);

        points += match right {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        points += match result {
            Result::Lose => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }
    points
}

#[allow(dead_code)]
fn solve(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();

    let mut points = 0;
    for line in lines {
        let vec = line.split_whitespace().collect::<Vec<&str>>();
        let (left, right) = (Choice::new(vec[0]), Choice::new(vec[1]));

        points += match right {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        points += match Result::play(left, right) {
            Result::Lose => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("2.test.in"), 15);
        assert_eq!(solve2("2.test.in"), 12);

        println!("Answer 2 pt.1: {}", solve("2.in"));
        println!("Answer 2 pt.2: {}", solve2("2.in"));
    }
}
