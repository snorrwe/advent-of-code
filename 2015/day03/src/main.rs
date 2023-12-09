use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert([x, y]);
    for c in input.chars() {
        match c {
            '>' => {
                x += 1;
            }
            '<' => {
                x -= 1;
            }
            'v' => {
                y -= 1;
            }
            '^' => {
                y += 1;
            }
            _ => {}
        }
        visited.insert([x, y]);
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    visited.insert([0; 2]);
    for (i, c) in input.chars().enumerate() {
        let d = match c {
            '>' => [1, 0],
            '<' => [-1, 0],
            'v' => [0, -1],
            '^' => [0, 1],
            _ => {
                continue;
            }
        };
        if i % 2 == 0 {
            sx += d[0];
            sy += d[1];
            visited.insert([sx, sy]);
        } else {
            rx += d[0];
            ry += d[1];
            visited.insert([rx, ry]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let res = part2("^v");

        assert_eq!(res, 3);
    }
}
