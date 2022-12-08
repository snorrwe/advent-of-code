type Grid = Vec<Vec<u8>>;
type Vizible = Vec<Vec<bool>>;

fn parse(input: &str) -> Grid {
    let mut result = Grid::with_capacity(128);
    for line in input.lines() {
        let line = line.as_bytes();
        result.push(line.iter().copied().collect());
    }
    result
}

fn part1(grid: &Grid) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut viz: Vizible = (0..rows)
        .map(|_| (0..columns).map(|_| false).collect())
        .collect();

    // top down scan
    for x in 1..columns - 1 {
        'scany: for y in 1..rows - 1 {
            for y0 in 0..y {
                if grid[y0][x] >= grid[y][x] {
                    continue 'scany;
                }
            }
            viz[y][x] = true;
        }
    }
    // bottom up scan
    for x in 1..columns - 1 {
        'scany: for y in 1..rows - 1 {
            for y0 in y + 1..rows {
                if grid[y0][x] >= grid[y][x] {
                    continue 'scany;
                }
            }
            viz[y][x] = true;
        }
    }
    // right to left scan
    for y in 1..rows - 1 {
        'scanx: for x in 1..columns - 1 {
            for x0 in x + 1..columns {
                if grid[y][x0] >= grid[y][x] {
                    continue 'scanx;
                }
            }
            viz[y][x] = true;
        }
    }
    // left to right scan
    for y in 1..rows - 1 {
        'scanx: for x in 1..columns - 1 {
            for x0 in 0..x {
                if grid[y][x0] >= grid[y][x] {
                    continue 'scanx;
                }
            }
            viz[y][x] = true;
        }
    }
    let mut result = rows * 2 + columns * 2 - 4; // all visible in first and last row - dupes
    result += viz
        .into_iter()
        .flat_map(|row| row.into_iter())
        .map(|x| x as usize)
        .sum::<usize>();

    result
}

fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse(&inp);
    let p1 = part1(&grid);
    println!("part1: {p1}");
}

#[test]
fn part1_test() {
    let grid = parse(
        r#"30373
25512
65332
33549
35390"#,
    );

    let res = part1(&grid);

    assert_eq!(21, res);
}
