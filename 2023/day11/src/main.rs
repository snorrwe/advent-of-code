use std::collections::HashSet;

use itertools::Itertools;

type Grid<T> = Vec<Vec<T>>;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(input.as_str(), 1_000_000));
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::new();
    let width = input.lines().next().unwrap().len();
    let mut cols = vec![true; width];
    for line in input.lines() {
        let mut empty = true;
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let s = match c {
                '.' => Space::Empty,
                '#' => {
                    empty = false;
                    cols[x] = false;
                    Space::Galaxy
                }
                _ => {
                    continue;
                }
            };
            row.push(s);
        }
        if empty {
            grid.push(row.clone());
            grid.push(row);
        } else {
            grid.push(row);
        }
    }

    let mut x_offset = 0;
    for (x, empty) in cols.into_iter().enumerate() {
        if empty {
            for row in grid.iter_mut() {
                row.insert(x + x_offset, Space::Empty);
            }
            x_offset += 1;
        }
    }
    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, s)| matches!(s, Space::Galaxy).then_some((x, y)))
        })
        .collect();

    let pairs: HashSet<_> = galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|(a, b)| [*a, *b])
        .map(|mut s| {
            s.sort();
            s
        })
        .collect();

    pairs.into_iter().map(|[a, b]| manhatten(a, b)).sum()
}

fn manhatten((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

fn part2(input: &str, scale: usize) -> usize {
    let mut grid = Grid::new();
    let width = input.lines().next().unwrap().len();
    let mut cols = vec![true; width];
    let mut empty_rows = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut empty = true;
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let s = match c {
                '.' => Space::Empty,
                '#' => {
                    empty = false;
                    cols[x] = false;
                    Space::Galaxy
                }
                _ => {
                    continue;
                }
            };
            row.push(s);
        }
        if empty {
            empty_rows.push(y);
        }
        grid.push(row);
    }
    let empty_rows = empty_rows;
    let empty_cols: Vec<_> = cols
        .into_iter()
        .enumerate()
        .filter(|(_, e)| *e)
        .map(|(x, _)| x)
        .collect();

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, s)| matches!(s, Space::Galaxy).then_some((x, y)))
        })
        .collect();

    let pairs: HashSet<_> = galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|(a, b)| [*a, *b])
        .map(|mut s| {
            s.sort();
            s
        })
        .collect();

    pairs
        .into_iter()
        .map(|[a, b]| {
            let (x1, y1) = a;
            let (x2, y2) = b;
            let mut x = (x1 as isize - x2 as isize).abs() as usize;
            let mut y = (y1 as isize - y2 as isize).abs() as usize;

            let mut xfromto = [x1, x2];
            let mut yfromto = [y1, y2];
            xfromto.sort();
            yfromto.sort();

            let y_offset = empty_rows
                .iter()
                .filter(|y| yfromto[0] <= **y && **y <= yfromto[1])
                .inspect(|_| y -= 1)
                .count();
            let x_offset = empty_cols
                .iter()
                .filter(|x| xfromto[0] <= **x && **x <= xfromto[1])
                .inspect(|_| x -= 1)
                .count();

            x + x_offset * scale + y + y_offset * scale
        })
        .sum()
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
enum Space {
    #[default]
    Empty,
    Galaxy,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 374);
    }

    #[test]
    fn test_p2_10() {
        let res = part2(INPUT, 10);
        assert_eq!(res, 1030);
    }

    #[test]
    fn test_p2_100() {
        let res = part2(INPUT, 100);
        assert_eq!(res, 8410);
    }
}
