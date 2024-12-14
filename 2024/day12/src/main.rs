use std::collections::HashSet;

use utils::{Grid, IVec2};

const N: u8 = 1 << 0;
const W: u8 = 1 << 1;
const S: u8 = 1 << 2;
const E: u8 = 1 << 3;

const NW: u8 = N | W;
const SW: u8 = S | W;
const NE: u8 = N | E;
const SE: u8 = S | E;

const WE: u8 = W | E;
const NS: u8 = N | S;

struct Input {
    grid: Grid<u8>,
    /// bits: visited,NWSE connection (=fence)
    connections: Grid<u8>,
}

fn parse(input: String) -> Input {
    let grid = Grid::from_ascii_lines(&input).unwrap();
    let connections = connections(&grid);
    Input { grid, connections }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = parse(input);

    println!("{}", part1(&mut input));
    println!("{}", part2(&mut input));
}

/// return (perimeter, area)
fn flood_v1(
    pos: IVec2,
    grid: &Grid<u8>,
    connections: &mut Grid<u8>,
    todo: &mut HashSet<IVec2>,
) -> (u32, u32) {
    connections[pos] |= 1 << 5;
    todo.remove(&pos);
    let mut perimeter = connections[pos].count_ones() - 1;
    let mut area = 1;
    for n in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
        .into_iter()
        .map(|x| x + pos)
    {
        if grid.contains_point(n) && connections[n] & (1 << 5) == 0 {
            if grid[n] == grid[pos] {
                let (p, a) = flood_v1(n, grid, connections, todo);
                perimeter += p;
                area += a;
            } else {
                todo.insert(n);
            }
        }
    }
    (perimeter, area)
}

/// return a grid where each cell is a bitmask
/// bits: NWSE connection (=fence)
fn connections(input: &Grid<u8>) -> Grid<u8> {
    let mut connections = input.like();
    connections.fill(0x0Fu8);

    for (y, row) in input.rows().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);

            for (i, n) in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
                .into_iter()
                .map(|n| n + pos)
                .enumerate()
            {
                if input.contains_point(n) && input[n] == *c {
                    connections[pos] &= !(1u8 << i)
                }
            }
        }
    }
    connections
}

fn part1(input: &mut Input) -> u32 {
    let mut todo = HashSet::new();

    todo.insert(IVec2::ZERO);

    let mut total = 0;
    while let Some(pos) = todo.iter().next().copied() {
        todo.remove(&pos);

        let (p, a) = flood_v1(pos, &input.grid, &mut input.connections, &mut todo);

        total += p * a;
    }

    total
}

fn part2(input: &mut Input) -> u32 {
    // reset visited
    for y in 0..input.connections.height {
        let row = input.connections.row_mut(y);
        for c in row.iter_mut() {
            *c &= 0xF;
        }
    }

    dbg!(&input.connections);
    let mut todo = HashSet::new();

    todo.insert(IVec2::ZERO);

    let mut total = 0;
    while let Some(pos) = todo.iter().next().copied() {
        todo.remove(&pos);

        let (sides, area) = flood_v2(pos, &input.grid, &mut input.connections, &mut todo);

        total += sides.max(4) * area;
    }

    total
}

/// return (sides, area)
fn flood_v2(
    pos: IVec2,
    grid: &Grid<u8>,
    connections: &mut Grid<u8>,
    todo: &mut HashSet<IVec2>,
) -> (u32, u32) {
    connections[pos] |= 1 << 5;
    todo.remove(&pos);

    let mut sides = 0;
    match connections[pos] & 0xF {
        3 | 9 | 6 | 12 => {
            sides = 1;
        }
        7 | 13 | 11 | 14 => {
            sides = 2;
        }
        0xf => {
            return (4, 1);
        }
        _ => {}
    }

    let mut area = 1;
    for n in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
        .into_iter()
        .map(|x| x + pos)
    {
        if grid.contains_point(n) && connections[n] & (1 << 5) == 0 {
            if grid[n] == grid[pos] {
                let (p, a) = flood_v2(n, grid, connections, todo);
                sides += p;
                area += a;
            } else {
                todo.insert(n);
            }
        }
    }
    (sides, area)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"AAAA
BBCD
BBCC
EEEC
"#;

    #[test]
    fn test_p1() {
        let mut inp = parse(INPUT.to_string());
        let res = part1(&mut inp);

        assert_eq!(res, 140);
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 80);
    }

    #[test]
    fn test_p2_eshape() {
        let mut inp = parse(
            r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#
            .to_string(),
        );
        let res = part2(&mut inp);

        assert_eq!(res, 236);
    }

    #[test]
    fn test_p2_squares() {
        let mut inp = parse(
            r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#
                .to_string(),
        );
        let res = part2(&mut inp);

        assert_eq!(res, 368);
    }
}
