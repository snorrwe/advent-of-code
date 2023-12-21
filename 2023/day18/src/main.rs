#![feature(slice_group_by)]

use std::collections::HashSet;

use utils::{Grid, IVec2};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> usize {
    let mut pos = IVec2::ZERO;

    let mut visited = HashSet::new();
    visited.insert(pos);

    let mut min = pos;
    let mut max = pos;

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let Some(dir) = split.next() else {
            continue;
        };
        let Some(n) = split.next() else {
            continue;
        };
        let n: usize = n.parse().unwrap();

        let dir = match dir {
            "U" => -IVec2::Y,
            "D" => IVec2::Y,
            "R" => IVec2::X,
            "L" => -IVec2::X,
            _ => unreachable!(),
        };

        for _ in 0..n {
            pos += dir;
            max = max.max(pos);
            min = min.min(pos);
            visited.insert(pos);
        }
    }

    let s = (max + IVec2::ONE) - min;
    // move min to zero
    let max = max - min;

    const FILL: u8 = 255;

    let mut grid = Grid::new(s.x as usize, s.y as usize);
    for pos in visited {
        grid[pos - min] = FILL;
    }
    grid.save_as_image("grid_contour.png");

    let min = IVec2::ZERO;
    for y in min.y..=max.y {
        let row = grid.row_mut(y as usize);
        let mut is_inside = false;
        let mut groups = row.group_by_mut(|a, b| a == b);

        // 1st chunk is special
        if let Some(chunk) = groups.next() {
            if chunk[0] != 0 {
                is_inside = true;
            }
        }

        for chunk in groups {
            if chunk[0] != 0 {
                if chunk.len() % 2 == 1 {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                chunk.fill(FILL);
            }
        }
    }

    grid.save_as_image("grid_filled.png");

    grid.rows()
        .flat_map(|r| r.iter())
        .filter(|x| **x != 0)
        .count()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 62);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 42);
    }
}
