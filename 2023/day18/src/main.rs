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

fn segment_intersect(from0: i32, to0: i32, from1: i32, to1: i32) -> Option<[i32; 2]> {
    debug_assert!(from0 <= to0);
    debug_assert!(from1 <= to1);
    if to0 < from1 || to1 < from0 {
        return None;
    }
    Some([from1.max(from0), to1.min(to0)])
}

fn split_segment_by_intersection(
    segment: (IVec2, IVec2),
    inter: [i32; 2],
    i: usize,
    contour: &mut Vec<(IVec2, IVec2)>,
) {
    if segment.1.x != inter[1] {
        let segment = (IVec2::new(inter[1] + 1, segment.1.y), segment.1);
        contour.insert(i, segment);
    }
    if segment.0.x != inter[0] {
        let segment = (segment.0, IVec2::new(inter[0] - 1, segment.0.y));
        contour.insert(i, segment);
    }
}

/// add the areas of recrangles building the shape
fn part2(input: &str) -> usize {
    let mut pos = IVec2::ZERO;

    let mut contour = Vec::new();
    let mut contour_size = 0;

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
        let n = i32::from_str_radix(n, 16).unwrap();

        let dir = match d {
            b'0' => IVec2::X,
            b'1' => IVec2::Y,
            b'2' => -IVec2::X,
            b'3' => -IVec2::Y,
            _ => unreachable!(),
        };

        let end = pos + dir * n;
        if dir.y == 0 {
            // contour only has the horizontal line segments
            contour.push((pos, end));
        }
        contour_size += n as usize;
        pos = end;
    }

    contour.iter_mut().for_each(|(a, b)| {
        if a.x > b.x {
            std::mem::swap(a, b);
        }
    });
    // sort contour from bottom to top (pop removes the topmost segment)
    contour.sort_by_key(|(from, _to)| -from.y);

    let mut total = 0;
    'main: while let Some(top_segment) = contour.pop() {
        // search for intersection
        //
        // remove the intersection
        // push the remaining segments back, in the appropriate positions so the vec remains sorted
        //
        // ???
        // profit
        if contour.is_empty() {
            total += (top_segment.1.x - top_segment.0.x) as usize + 1;
            break;
        }
        let mut i = contour.len() - 1;
        // segments are sorted by Y so the first match is the best one
        let inter = loop {
            let candidate = &contour[i];
            let inter = segment_intersect(
                top_segment.0.x,
                top_segment.1.x,
                candidate.0.x,
                candidate.1.x,
            );
            if inter.is_some() {
                break inter;
            }
            if i == 0 {
                // no segments intersecting to the bottom
                total += (top_segment.1.x - top_segment.0.x) as usize + 1;
                continue 'main;
            }
            i -= 1;
        };
        let inter = inter.unwrap();
        let bottom_segment = contour[i];

        debug_assert_ne!(top_segment.0.y, bottom_segment.0.y);

        // width is inclusive, height excludes the bottom
        let width = inter[1] - inter[0] + 1;
        debug_assert!(width >= 1);
        let height = bottom_segment.0.y - top_segment.0.y;
        debug_assert!(height >= 1);
        // the common area will be added, push the remaining segment parts back into the contour
        // split_segment_by_intersection(bottom_segment, inter, i, &mut contour);
        split_segment_by_intersection(top_segment, inter, contour.len(), &mut contour);

        total += (width as usize) * (height as usize);
    }

    total
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
