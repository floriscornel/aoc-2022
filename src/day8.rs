use crate::read_lines;

fn is_visible((i, j): (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    let (mut l, mut r, mut t, mut b) = (true, true, true, true);
    for k in 0..grid.len() {
        if k < j && grid[i][k] >= grid[i][j] {
            l = false;
        }
        if k < i && grid[k][j] >= grid[i][j] {
            t = false;
        }
        if k > j && grid[i][k] >= grid[i][j] {
            r = false;
        }
        if k > i && grid[k][j] >= grid[i][j] {
            b = false;
        }
    }
    l || r || t || b
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn steps((i, j): (i32, i32), grid: &Vec<Vec<u8>>, direction: Direction, height: u8) -> usize {
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[i as usize].len() as i32 {
        return 0;
    }
    if grid[i as usize][j as usize] >= height {
        return 1;
    }
    1 + match direction {
        Direction::Up => steps((i - 1, j), grid, direction, height),
        Direction::Down => steps((i + 1, j), grid, direction, height),
        Direction::Left => steps((i, j - 1), grid, direction, height),
        Direction::Right => steps((i, j + 1), grid, direction, height),
    }
}

fn scenic_score((i, j): (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    let height = grid[i][j];
    let (i, j) = (i as i32, j as i32);
    steps((i - 1, j), grid, Direction::Up, height)
        * steps((i + 1, j), grid, Direction::Down, height)
        * steps((i, j - 1), grid, Direction::Left, height)
        * steps((i, j + 1), grid, Direction::Right, height)
}

#[allow(dead_code)]
fn solve(filename: &str, part1: bool) -> usize {
    let grid: Vec<Vec<u8>> = read_lines(filename)
        .unwrap()
        .flatten()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| b - b'0')
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if part1 && is_visible((i, j), &grid) {
                answer += 1;
            } else if !part1 {
                answer = answer.max(scenic_score((i, j), &grid));
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("8.test.in", true), 21);
        assert_eq!(solve("8.test.in", false), 8);

        println!("Answer 8 pt.1: {}", solve("8.in", true));
        println!("Answer 8 pt.2: {}", solve("8.in", false));
    }
}
