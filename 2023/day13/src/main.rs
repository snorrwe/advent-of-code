use utils::Grid;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn is_symmetric_horizontal(grid: &Grid<u8>, col: usize) -> bool {
    debug_assert!(col > 0);
    let width = grid.width;
    let limit = (width - col).min(col);
    for line in grid.rows() {
        for i in 1..=limit {
            if line[col - i] != line[col + i - 1] {
                return false;
            }
        }
    }
    true
}

fn is_symmetric_vertical(grid: &Grid<u8>, row: usize) -> bool {
    debug_assert!(row > 0);
    let height = grid.height;
    let limit = (height - row).min(row);
    for i in 1..=limit {
        let lhs = grid.row(row - i);
        let rhs = grid.row(row + i - 1);
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
            let width = grid[0].len();
            let grid = Grid::from_data(grid.into_iter().flatten().copied().collect(), width);

            let mut sum = 0;
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
            let mut grid = Grid::from_data(grid.into_iter().flatten().copied().collect(), width);

            let mut ogx = 0;
            for i in 1..width {
                if is_symmetric_horizontal(&grid, i) {
                    ogx = i;
                    break;
                }
            }
            let mut ogy = 0;
            for i in 1..height {
                if is_symmetric_vertical(&grid, i) {
                    ogy = i;
                    break;
                }
            }
            for y in 0..height {
                for x in 0..width {
                    let item = *grid.get(x, y);
                    match item {
                        b'#' => grid.insert(x, y, b'.'),
                        b'.' => grid.insert(x, y, b'#'),
                        _ => unreachable!(),
                    }
                    let mut col = 0;
                    let mut row = 0;
                    for i in 1..x + 1 {
                        if is_symmetric_horizontal(&grid, i) {
                            col = i;
                        }
                    }
                    for i in 1..y + 1 {
                        if is_symmetric_vertical(&grid, i) {
                            row = i;
                        }
                    }
                    if (row != 0 && row != ogy) || (col != 0 && col != ogx) {
                        // new symmetry found
                        let mut sum = 0;
                        if row != ogy {
                            sum += row * 100;
                        }
                        if col != ogx {
                            sum += col;
                        }
                        return sum;
                    }
                    // restore the grid
                    grid.insert(x, y, item);
                }
            }
            unreachable!("Failed to find smudge for pattern:\n{pattern}\n{ogx} {ogy}");
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
