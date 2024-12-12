use std::collections::HashSet;

use utils::{Grid, IVec2};

struct Input {
    grid: Grid<u8>,
    connections: Grid<u8>,
}

fn parse(input: String) -> Input {
    let grid = Grid::from_ascii_lines(&input).unwrap();
    let connections = connections(&grid);
    Input { grid, connections }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

/// return (perimeter, area)
fn flood(
    pos: IVec2,
    grid: &Grid<u8>,
    connections: &Grid<u8>,
    visited: &mut HashSet<IVec2>,
    todo: &mut HashSet<IVec2>,
) -> (u32, u32) {
    visited.insert(pos);
    todo.remove(&pos);
    let mut perimeter = connections[pos].count_ones();
    let mut area = 1;
    for n in [-IVec2::Y, -IVec2::X, IVec2::Y, IVec2::X]
        .into_iter()
        .map(|x| x + pos)
    {
        if grid.contains_point(n) && !visited.contains(&n) {
            if grid[n] == grid[pos] {
                let (p, a) = flood(n, grid, connections, visited, todo);
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

fn part1(input: &Input) -> u32 {
    let mut visited = HashSet::new();
    let mut todo = HashSet::new();

    todo.insert(IVec2::ZERO);

    let mut total = 0;
    while let Some(pos) = todo.iter().next().copied() {
        todo.remove(&pos);

        let (p, a) = flood(
            pos,
            &input.grid,
            &input.connections,
            &mut visited,
            &mut todo,
        );

        total += p * a;
    }

    total
}

fn part2(input: &Input) -> i32 {
    todo!()
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
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 140);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
