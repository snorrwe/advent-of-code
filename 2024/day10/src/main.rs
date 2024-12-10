use std::collections::HashSet;

use utils::{Grid, IVec2};

type Input = Grid<u8>;

fn parse(input: String) -> Input {
    Grid::from_ascii_lines(&input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    let [p1, p2] = solve(&input);

    println!("{}\n{}", p1, p2);
}

fn trail_dfs(pos: IVec2, peaks: &mut HashSet<IVec2>, grid: &Input) -> usize {
    debug_assert!(b'0' <= grid[pos]);
    debug_assert!(grid[pos] < b'9');
    let needle = grid[pos] + 1;
    let mut total = 0;
    for d in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y] {
        let c = pos + d;
        if grid.contains_point(c) && grid[c] == needle {
            if needle == b'9' {
                peaks.insert(c);
                total += 1;
            } else {
                total += trail_dfs(c, peaks, grid);
            }
        }
    }
    total
}

fn solve(input: &Input) -> [usize; 2] {
    let mut peaks = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for start in input.rows().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_x, h)| h == &&b'0')
            .map(move |(x, _)| IVec2::new(x as i32, y as i32))
    }) {
        peaks.clear();
        part2 += trail_dfs(start, &mut peaks, input);
        part1 += peaks.len()
    }
    [part1, part2]
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
    fn test() {
        let inp = parse(INPUT.to_string());
        let [p1, p2] = solve(&inp);

        assert_eq!(p1, 36);
        assert_eq!(p2, 81);
    }
}
