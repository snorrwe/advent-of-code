type Grid = Vec<u32>;

fn index(x: isize, y: isize, width: isize, height: isize) -> Option<usize> {
    if x >= width || y >= height || x < 0 || y < 0 {
        return None;
    }
    (y * width + x).try_into().ok()
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
    for y in 0..height {
        'row: for x in 0..width {
            let h = grid[index(x, y, width, height).unwrap()];

            for dy in [-1, 1] {
                if let Some(i) = index(x, y + dy, width, height) {
                    if grid[i] <= h {
                        continue 'row;
                    }
                }
            }
            for dx in [-1, 1] {
                if let Some(i) = index(x + dx, y, width, height) {
                    if grid[i] <= h {
                        continue 'row;
                    }
                }
            }

            let risk = h + 1;
            p1 += risk;
        }
    }

    println!("Part1: {}", p1);
}
