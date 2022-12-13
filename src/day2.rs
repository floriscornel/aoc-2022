use crate::read_lines;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn new(str: &str) -> Self {
        match str {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn anticipate(left: &RPS, result: &Result) -> Self {
        match (left, result) {
            (RPS::Rock, Result::Draw)
            | (RPS::Paper, Result::Lose)
            | (RPS::Scissors, Result::Win) => Self::Rock,
            (RPS::Rock, Result::Win)
            | (RPS::Paper, Result::Draw)
            | (RPS::Scissors, Result::Lose) => Self::Paper,
            (RPS::Scissors, Result::Draw)
            | (RPS::Rock, Result::Lose)
            | (RPS::Paper, Result::Win) => Self::Scissors,
        }
    }
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn play(left: RPS, right: RPS) -> Self {
        match (left, right) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                Self::Lose
            }
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Self::Draw
            }
            (RPS::Scissors, RPS::Rock) | (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) => {
                Self::Win
            }
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
        let (left, result) = (RPS::new(vec[0]), Result::new(vec[1]));
        let right = RPS::anticipate(&left, &result);

        points += match right {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
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
        let (left, right) = (RPS::new(vec[0]), RPS::new(vec[1]));

        points += match right {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
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
