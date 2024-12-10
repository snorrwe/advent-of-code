use std::collections::HashSet;

use utils::{Grid, IVec2};

type Input = Grid<u8>;

fn parse(input: String) -> Input {
    Grid::from_ascii_lines(&input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn trail_v1(pos: IVec2, visited: &mut HashSet<IVec2>, grid: &Input) -> usize {
    let mut total = 0;
    for d in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y] {
        let c = pos + d;
        if grid.contains_point(c) && !visited.contains(&c) && grid[c] == 1 + grid[pos] {
            visited.insert(c);
            if grid[c] == b'9' {
                total += 1
            } else {
                total += trail_v1(c, visited, grid)
            }
        }
    }
    total
}

fn part1(input: &Input) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();
    for start in input.rows().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_x, h)| h == &&b'0')
            .map(move |(x, _)| IVec2::new(x as i32, y as i32))
    }) {
        visited.clear();
        visited.insert(start);
        count += trail_v1(start, &mut visited, input);
    }
    count
}

fn trail_v2(pos: IVec2, grid: &Input) -> usize {
    let mut total = 0;
    for d in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y] {
        let c = pos + d;
        if grid.contains_point(c) && grid[c] == 1 + grid[pos] {
            if grid[c] == b'9' {
                total += 1
            } else {
                total += trail_v2(c, grid)
            }
        }
    }
    total
}

fn part2(input: &Input) -> usize {
    let mut count = 0;
    for start in input.rows().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_x, h)| h == &&b'0')
            .map(move |(x, _)| IVec2::new(x as i32, y as i32))
    }) {
        count += trail_v2(start, input);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 36);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 81);
    }
}
