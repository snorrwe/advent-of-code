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

    let mut todo = HashSet::new();

    todo.insert(IVec2::ZERO);

    let mut total = 0;
    while let Some(pos) = todo.iter().next().copied() {
        todo.remove(&pos);

        let (sides, area) = flood_v2(pos, &input.grid, &mut input.connections, &mut todo);

        total += sides * area;
    }

    total
}

fn get_label(pos: IVec2, grid: &Grid<u8>) -> Option<u8> {
    grid.contains_point(pos).then(|| grid[pos])
}

fn check_concave(
    con: u8,
    mask: u8,
    pos: IVec2,
    grid: &Grid<u8>,
    label: u8,
    lhs: IVec2,
    rhs: IVec2,
) -> u32 {
    if con & mask != 0 {
        let a = get_label(pos + lhs, grid);
        let b = get_label(pos + rhs, grid);

        if a == Some(label) && b == Some(label) {
            return 2;
        } else if a == Some(label) {
            return 1;
        } else if b == Some(label) {
            return 1;
        }
    }
    0
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

    let label = grid[pos];
    let con = connections[pos] & 0xF;

    match (con.count_ones(), con) {
        (0, _) => {}
        // convex edges
        (3, _) => {
            sides = 2;
        }
        (4, _) => {
            sides = 4;
        }
        (2, NW | SW | NE | SE) => {
            sides = 1;
        }
        _ => {}
    }
    match con.count_ones() {
        1 | 2 => {
            // check concave edges
            sides += check_concave(
                con,
                E,
                pos,
                grid,
                label,
                IVec2::new(1, -1),
                IVec2::new(1, 1),
            );
            sides += check_concave(
                con,
                W,
                pos,
                grid,
                label,
                IVec2::new(-1, -1),
                IVec2::new(-1, 1),
            );
            sides += check_concave(
                con,
                N,
                pos,
                grid,
                label,
                IVec2::new(-1, -1),
                IVec2::new(1, -1),
            );
            sides += check_concave(
                con,
                S,
                pos,
                grid,
                label,
                IVec2::new(-1, 1),
                IVec2::new(1, 1),
            );
        }
        _ => {}
    }

    let mut area = 1;
    for neighbor in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
        .into_iter()
        .map(|x| x + pos)
    {
        if grid.contains_point(neighbor) {
            if connections[neighbor] & (1 << 5) == 0 {
                if grid[neighbor] == grid[pos] {
                    let (p, a) = flood_v2(neighbor, grid, connections, todo);
                    sides += p;
                    area += a;
                } else {
                    todo.insert(neighbor);
                }
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
    fn test_p2_basic() {
        println!("{INPUT}");
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 80);
    }

    #[test]
    fn test_p2_eshape() {
        const INPUT: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;

        println!("{INPUT}");
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 236);
    }

    #[test]
    fn test_p2_squares() {
        const INPUT: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        println!("{INPUT}");

        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 368);
    }

    #[test]
    fn test_p2_single_square() {
        let mut inp = parse(
            r#"AAAAAA
AAAAAA
AAAAAA
AAAAAA
AAAAAA
AAAAAA"#
                .to_string(),
        );
        let res = part2(&mut inp);

        assert_eq!(res, 36 * 4);
    }

    #[test]
    fn test_p2_single_rect() {
        let mut inp = parse(r#"AAAAAA"#.to_string());
        let res = part2(&mut inp);
        assert_eq!(res, 6 * 4);
    }

    #[test]
    fn test_p2_single_tile() {
        let mut inp = parse(r#"A"#.to_string());
        let res = part2(&mut inp);
        assert_eq!(res, 1 * 4);
    }

    #[test]
    fn test_flood_v2_s_shape() {
        let mut inp = parse(INPUT.to_string());
        let pos = IVec2::new(2, 1);
        println!("{INPUT}\n{pos:?}\n");
        let mut todo = Default::default();
        let (sides, area) = flood_v2(pos, &inp.grid, &mut inp.connections, &mut todo);

        let mut visited = inp.connections.like();

        for y in 0..inp.connections.height {
            for x in 0..inp.connections.width {
                if inp.connections.get(x, y) & (1 << 5) != 0 {
                    visited.insert(x, y, 1u8);
                }
            }
        }

        dbg!(visited);

        assert_eq!(sides, 8);
        assert_eq!(area, 4);
    }
}
