use rayon::prelude::*;

type Grid = Vec<u32>;

fn index(x: isize, y: isize, width: isize, height: isize) -> Option<usize> {
    if x >= width || y >= height || x < 0 || y < 0 {
        return None;
    }
    (y * width + x).try_into().ok()
}

/// none if not a low point
fn get_risk(grid: &[u32], x: isize, y: isize, width: isize, height: isize) -> Option<u32> {
    let h = grid[index(x, y, width, height).unwrap()];

    for dy in [-1, 1] {
        if let Some(i) = index(x, y + dy, width, height) {
            if grid[i] <= h {
                return None;
            }
        }
    }
    for dx in [-1, 1] {
        if let Some(i) = index(x + dx, y, width, height) {
            if grid[i] <= h {
                return None;
            }
        }
    }

    let risk = h + 1;
    Some(risk)
}

fn visit_basin(
    visited: &mut Vec<bool>,
    grid: &[u32],
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    mut h: u32,
    mut count: usize,
) -> usize {
    match index(x, y, width, height) {
        Some(i) => {
            if visited[i] || grid[i] < h || grid[i] == 9 {
                return count;
            }
            visited[i] = true;
            h = grid[i];
            count += 1;
        }
        None => return count,
    }

    for dy in [-1, 1] {
        let y = y + dy;
        count = visit_basin(visited, grid, x, y, width, height, h, count);
    }
    for dx in [-1, 1] {
        let x = x + dx;
        count = visit_basin(visited, grid, x, y, width, height, h, count);
    }
    count
}

fn main() {
    let mut grid = Grid::with_capacity(16000);
    let mut width = 0isize;
    let mut height = 0isize;

    let mut buffer = String::with_capacity(1024);
    let stdin = std::io::stdin();
    if let Ok(_size) = stdin.read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        assert!(!line.is_empty());

        for num in line.chars() {
            grid.push(num.to_digit(10).unwrap());
        }
        height += 1;
        width = grid.len() as isize;
    }
    buffer.clear();
    while let Ok(_size) = stdin.read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.is_empty() {
            break;
        }
        assert!(line.len() as isize == width);
        for num in line.chars() {
            grid.push(num.to_digit(10).unwrap());
        }
        height += 1;
        buffer.clear();
    }

    // part 1
    //
    let mut p1 = 0;
    let mut low_points = Vec::with_capacity(512);
    for y in 0..height {
        for x in 0..width {
            if let Some(risk) = get_risk(&grid, x, y, width, height) {
                p1 += risk;
                low_points.push([x, y]);
            }
        }
    }

    // part 2
    //
    let grid = grid.as_slice();
    let mut basins = low_points
        .par_iter()
        .map(|[x, y]| {
            let mut visited = vec![false; grid.len()];
            let h = grid[index(*x, *y, width, height).unwrap()];
            visit_basin(&mut visited, grid, *x, *y, width, height, h, 0)
        })
        .collect::<Vec<_>>();

    basins.par_sort();

    let p2: usize = basins[basins.len() - 3..].iter().product();

    println!("Part1: {} Part2: {}", p1, p2);
}
