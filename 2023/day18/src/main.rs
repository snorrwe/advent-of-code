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

    const FILL_CONTOUR: u8 = 255;
    const FILL_INSIDE: u8 = 128;

    let mut grid = Grid::new(s.x as usize, s.y as usize);
    // fill contour
    let mut start = IVec2::ZERO;
    let n = visited.len() as i32;
    for pos in visited {
        grid[pos - min] = FILL_CONTOUR;
        start += pos - min;
    }
    grid.save_as_image("grid_contour.png");

    let mut q = Vec::new();
    // pray that the average point is inside
    q.push(start / n);

    // flood fill
    while let Some(p) = q.pop() {
        grid[p] = FILL_INSIDE;
        for d in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y] {
            let p = p + d;
            if grid[p] == 0 {
                q.push(p);
            }
        }
    }

    grid.save_as_image("grid_filled.png");

    grid.rows()
        .flat_map(|r| r.iter())
        .filter(|x| **x != 0)
        .count()
}

fn part2(input: &str) -> usize {
    let mut pos = IVec2::ZERO;

    let mut contour = HashSet::new();
    contour.insert(pos);

    let mut min = pos;
    let mut max = pos;

    for line in input.lines() {
        let split = line.split_ascii_whitespace();
        let Some(hex) = split.skip(2).next() else {
            continue;
        };
        let hex = hex.as_bytes();
        let digits = &hex[2..8];
        let n = &digits[0..5];
        let d = digits[5];
        let n = std::str::from_utf8(n).unwrap();
        let n = usize::from_str_radix(n, 16).unwrap();

        let dir = match d {
            b'0' => IVec2::X,
            b'1' => IVec2::Y,
            b'2' => -IVec2::X,
            b'3' => -IVec2::Y,
            _ => unreachable!(),
        };

        for _ in 0..n {
            pos += dir;
            max = max.max(pos);
            min = min.min(pos);
            contour.insert(pos);
        }
    }

    dbg!(min, max, max - min, contour.len());

    todo!();

    let s = (max + IVec2::ONE) - min;

    const FILL_CONTOUR: u8 = 255;
    const FILL_INSIDE: u8 = 128;

    let mut grid = Grid::new(s.x as usize, s.y as usize);
    // fill contour
    let mut start = IVec2::ZERO;
    let n = contour.len() as i32;
    for pos in contour {
        grid[pos - min] = FILL_CONTOUR;
        start += pos - min;
    }
    grid.save_as_image("grid_contour.png");

    let mut q = Vec::new();
    // pray that the average point is inside
    q.push(start / n);

    // flood fill
    while let Some(p) = q.pop() {
        grid[p] = FILL_INSIDE;
        for d in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y] {
            let p = p + d;
            if grid[p] == 0 {
                q.push(p);
            }
        }
    }

    grid.save_as_image("grid_filled.png");

    grid.rows()
        .flat_map(|r| r.iter())
        .filter(|x| **x != 0)
        .count()
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

        assert_eq!(res, 952408144115);
    }
}
