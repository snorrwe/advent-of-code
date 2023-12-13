fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn is_symmetric_horizontal(grid: &[&[u8]], col: usize) -> bool {
    count_horizontal_error(grid, col) == 0
}

fn count_horizontal_error(grid: &[&[u8]], col: usize) -> usize {
    let width = grid[0].len();
    let limit = (width - col).min(col);
    let mut err = 0;
    for line in grid {
        for i in 1..=limit {
            if line[col - i] != line[col + i - 1] {
                err += 1;
            }
        }
    }
    err
}

fn is_symmetric_vertical(grid: &[&[u8]], row: usize) -> bool {
    count_vertical_error(grid, row) == 0
}

fn count_vertical_error(grid: &[&[u8]], row: usize) -> usize {
    let height = grid.len();
    let limit = (height - row).min(row);
    let mut err = 0;
    for i in 1..=limit {
        let lhs = grid[row - i];
        let rhs = grid[row + i - 1];
        for x in 0..lhs.len() {
            if lhs[x] != rhs[x] {
                err += 1;
            }
        }
    }
    err
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

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let grid = pattern.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
            let height = grid.len();
            if height == 0 {
                return 0;
            }
            let width = grid[0].len();
            let mut minx = width;
            for x in 1..width {
                let err = count_horizontal_error(&grid, x);
                if err <= 1 {
                    minx = x;
                    break;
                }
            }
            let mut miny = height;
            for y in 1..height {
                let err = count_vertical_error(&grid, y);
                if err <= 1 {
                    miny = y;
                    break;
                }
            }
            miny * 100 // + minx
        })
        .sum()
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

        assert_eq!(res, 400);
    }
}
