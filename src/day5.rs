use crate::read_lines;

fn solve(filename: &str) -> String {
    let lines = read_lines(filename).unwrap().flatten();
    let (mut stacks, mut n) = (Vec::<Vec<char>>::new(), 0);

    let mut second_part = false;
    for line in lines {
        if !second_part {
            let bytes = line.as_bytes();
            if stacks.is_empty() {
                n = bytes.len() / 4 + 1;
                for _i in 0..n {
                    stacks.push(vec![]);
                }
            }
            if bytes[1] == b'1' {
                second_part = true;
            } else {
                for i in 0..n {
                    let char = bytes[i * 4 + 1];
                    if char != b' ' {
                        stacks[i].push(char as char);
                    }
                }
            }
        } else {
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts.len() >= 5 {
                let (count, from, to) = (
                    parts[1].parse::<usize>().unwrap(),
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[5].parse::<usize>().unwrap() - 1,
                );
                println!("{} {} {}", count, from, to);

                let nc = count.min(stacks[from].len());
                for _i in 0..nc {
                    let char = stacks[from].pop().unwrap();
                    stacks[to].push(char);
                }
            }
        }
        println!("stacks: {:?}", stacks);
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("5.test.in"), "CMZ".to_string());
        // assert_eq!(solve2("5.test.in"), "CMZ".to_string());

        // println!("Answer 5 pt.1: {}", solve("5.in"));
        // println!("Answer 5 pt.2: {}", solve2("5.in"));
    }
}
