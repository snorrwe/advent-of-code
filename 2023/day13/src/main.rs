fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn is_symmetric_horizontal(grid: &[&[u8]], col: usize) -> bool {
    let width = grid[0].len();
    let limit = (width - col).min(col);
    for line in grid {
        for i in 1..=limit {
            if line[col - i] != line[col + i - 1] {
                return false;
            }
        }
    }
    true
}

fn is_symmetric_vertical(grid: &[&[u8]], row: usize) -> bool {
    let height = grid.len();
    let limit = (height - row).min(row);
    for i in 1..=limit {
        let lhs = grid[row - i];
        let rhs = grid[row + i - 1];
        if lhs != rhs {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let grid = pattern.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
            let height = grid.len();
            if height == 0 {
                return 0;
            }
            let mut sum = 0;
            let width = grid[0].len();
            for i in 1..width {
                if is_symmetric_horizontal(&grid, i) {
                    sum += i;
                    break;
                }
            }
            for i in 1..height {
                if is_symmetric_vertical(&grid, i) {
                    sum += 100 * i;
                    break;
                }
            }
            sum
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 405);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 42);
    }
}
