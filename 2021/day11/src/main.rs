type Grid = Vec<u8>;

fn index(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
    if x >= width || y >= height || x < 0 || y < 0 {
        return None;
    }
    Some((y * width + x) as usize)
}

fn update(
    t: u32,
    grid: &mut [u8],
    timestamp: &mut [u32],
    flash: &mut Vec<[i32; 2]>,
    width: i32,
    height: i32,
    p1: &mut usize,
) {
    flash.clear();
    for y in 0..height {
        for x in 0..width {
            let i = index(x, y, width, height).unwrap();
            grid[i] += 1;
            if grid[i] > 9 {
                timestamp[i] = t;
                flash.push([x, y]);
            }
        }
    }

    while let Some([x, y]) = flash.pop() {
        *p1 += 1;
        grid[index(x, y, width, height).unwrap()] = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let xp = x + dx;
                let yp = y + dy;

                if let Some(i) = index(xp, yp, width, height) {
                    if timestamp[i] < t {
                        grid[i] += 1;
                        if grid[i] == 10 {
                            timestamp[i] = t;
                            flash.push([xp, yp]);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let mut grid = Grid::with_capacity(width * height);

    let mut buffer = String::with_capacity(1024);
    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.len() == 0 {
            break;
        }
        for num in line.chars() {
            grid.push(num as u8 - '0' as u8);
        }
        buffer.clear();
    }

    assert!(grid.len() == width * height, "{}", grid.len());

    let mut timestamp = vec![0; grid.len()];
    let mut todo = Default::default();

    let mut p1 = 0;
    for t in 1..=100 {
        update(
            t,
            &mut grid,
            &mut timestamp,
            &mut todo,
            width as i32,
            height as i32,
            &mut p1,
        );
    }

    println!("P1: {}", p1);
}
