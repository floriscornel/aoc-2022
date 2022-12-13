use crate::read_lines;
#[derive(Debug)]
struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    fn contains(&self, b: &Range) -> bool {
        (self.begin >= b.begin && self.end <= b.end) || (b.begin >= self.begin && b.end <= self.end)
    }

    fn overlaps(&self, b: &Range) -> bool {
        (self.begin <= b.begin && self.end >= b.begin)
            || (self.begin <= b.end && self.end >= b.end)
            || (b.begin <= self.begin && b.end >= self.begin)
            || (b.begin <= self.end && b.end >= self.end)
    }
}

#[allow(dead_code)]
fn solve(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();
    let mut a = None;
    let mut output = 0;
    for line in lines {
        for range in line.split(',') {
            let split = range.split('-').collect::<Vec<&str>>();
            if a.is_none() {
                a = Some(Range {
                    begin: split[0].parse::<i32>().unwrap(),
                    end: split[1].parse::<i32>().unwrap(),
                });
            } else {
                if a.unwrap().contains(&Range {
                    begin: split[0].parse::<i32>().unwrap(),
                    end: split[1].parse::<i32>().unwrap(),
                }) {
                    output += 1;
                }

                a = None;
            }
        }
    }
    output
}

#[allow(dead_code)]
fn solve2(filename: &str) -> i64 {
    let lines = read_lines(filename).unwrap().flatten();
    let mut a = None;
    let mut output = 0;
    for line in lines {
        for range in line.split(',') {
            let split = range.split('-').collect::<Vec<&str>>();
            if a.is_none() {
                a = Some(Range {
                    begin: split[0].parse::<i32>().unwrap(),
                    end: split[1].parse::<i32>().unwrap(),
                });
            } else {
                if a.unwrap().overlaps(&Range {
                    begin: split[0].parse::<i32>().unwrap(),
                    end: split[1].parse::<i32>().unwrap(),
                }) {
                    output += 1;
                }

                a = None;
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("4.test.in"), 2);
        assert_eq!(solve2("4.test.in"), 4);

        println!("Answer 4 pt.1: {}", solve("4.in"));
        println!("Answer 4 pt.2: {}", solve2("4.in"));
    }
}
