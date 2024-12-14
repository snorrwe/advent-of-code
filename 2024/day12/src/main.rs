use std::collections::HashSet;

use utils::{Grid, IVec2};

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

fn corner_map(input: &Input) -> Grid<u8> {
    let mut corners = input.grid.like();

    for y in 0..input.grid.height {
        for x in 0..input.grid.width {
            let mut area = [None; 9];

            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    area[(dy + 1) as usize * 3 + (dx + 1) as usize] =
                        get_label(IVec2::new(x as i32 + dx, y as i32 + dy), &input.grid);
                }
            }

            let label = area[4];
            debug_assert_eq!(label, Some(*input.grid.get(x, y)));

            let mut c = 0;

            // convex corners
            //
            // TL
            if area[0] != label && area[1] != label && area[3] != label {
                c += 1;
            }
            // TR
            if area[2] != label && area[1] != label && area[5] != label {
                c += 1;
            }
            // BL
            if area[6] != label && area[7] != label && area[3] != label {
                c += 1;
            }
            // BR
            if area[8] != label && area[7] != label && area[5] != label {
                c += 1;
            }
            // concave corners
            // TL
            if area[0] == label && area[1] != label && area[3] == label {
                c += 1;
            }
            // TR
            if area[2] == label && area[1] != label && area[5] == label {
                c += 1;
            }
            // BL
            if area[6] == label && area[7] != label && area[3] == label {
                c += 1;
            }
            // BR
            if area[8] == label && area[7] != label && area[5] == label {
                c += 1;
            }

            corners.insert(x, y, c);
        }
    }

    corners
}

fn part2(input: &mut Input) -> u32 {
    // reset visited
    for y in 0..input.connections.height {
        let row = input.connections.row_mut(y);
        for c in row.iter_mut() {
            *c &= 0xF;
        }
    }

    let corners = corner_map(input);

    let mut todo = HashSet::new();

    todo.insert(IVec2::ZERO);

    let mut total = 0;
    while let Some(pos) = todo.iter().next().copied() {
        todo.remove(&pos);

        let (sides, area) = flood_v2(
            pos,
            &input.grid,
            &mut input.connections,
            &mut todo,
            &corners,
        );

        total += sides * area;
    }

    total
}

fn get_label(pos: IVec2, grid: &Grid<u8>) -> Option<u8> {
    grid.contains_point(pos).then(|| grid[pos])
}

/// return (sides, area)
fn flood_v2(
    pos: IVec2,
    grid: &Grid<u8>,
    connections: &mut Grid<u8>,
    todo: &mut HashSet<IVec2>,
    corners: &Grid<u8>,
) -> (u32, u32) {
    connections[pos] |= 1 << 5;
    todo.remove(&pos);

    let mut sides = corners[pos] as u32;

    let mut area = 1;
    for neighbor in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
        .into_iter()
        .map(|x| x + pos)
    {
        if grid.contains_point(neighbor) {
            if connections[neighbor] & (1 << 5) == 0 {
                if grid[neighbor] == grid[pos] {
                    let (p, a) = flood_v2(neighbor, grid, connections, todo, corners);
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
    fn test_corner_map() {
        const INPUT: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;
        let inp = parse(INPUT.to_string());

        let corners = corner_map(&inp);

        println!("{INPUT}\n{corners}");

        let mut ecorners = 0;

        for y in 0..inp.grid.height {
            for x in 0..inp.grid.width {
                if inp.grid.get(x, y) == &b'E' {
                    ecorners += corners.get(x, y);
                }
            }
        }

        assert_eq!(ecorners, 12);
    }
}
