use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Grid {
    seats: HashMap<[i32; 2], Tile>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Occupied = 1,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_str("L"),
            Tile::Occupied => f.write_str("#"),
        }
    }
}

fn parse(inp: &str) -> Grid {
    let mut w = 0;
    let mut h = 0;
    let seats = inp
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            h = h.max(y);
            w = w.max(line.len());
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'L' => Some(([x as i32, y as i32], Tile::Empty)),
                '#' => Some(([x as i32, y as i32], Tile::Occupied)),
                _ => None,
            })
        })
        .collect();

    Grid {
        seats,
        width: w + 1,
        height: h + 1,
    }
}

/// return if state has changed
fn forward(grid_out: &mut Grid) -> bool {
    let grid_in = grid_out.clone();

    let mut mutated = false;
    for ([x, y], tile) in grid_in.seats.iter() {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    if matches!(grid_in.seats.get(&[x + dx, y + dy]), Some(Tile::Occupied)) {
                        count += 1;
                    }
                }
            }
        }
        match tile {
            Tile::Empty => {
                if count == 0 {
                    grid_out.seats.insert([*x, *y], Tile::Occupied);
                    mutated = true;
                }
            }
            Tile::Occupied => {
                if count >= 4 {
                    grid_out.seats.insert([*x, *y], Tile::Empty);
                    mutated = true;
                }
            }
        }
    }

    mutated
}

/// start and velocity
fn get_in_direction([mut x, mut y]: [i32; 2], [vx, vy]: [i32; 2], grid: &Grid) -> Option<Tile> {
    while 0 <= x && x <= grid.width as i32 && 0 <= y && y <= grid.height as i32 {
        x += vx;
        y += vy;
        let res = grid.seats.get(&[x, y]);
        if res.is_some() {
            return res.cloned();
        }
    }
    None
}

fn count_occupied_all_directions([x, y]: [i32; 2], grid: &Grid) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                if let Some(Tile::Occupied) = get_in_direction([x, y], [dx, dy], grid) {
                    count += 1
                }
            }
        }
    }
    count
}

/// return if state has changed
fn forward2(grid_out: &mut Grid) -> bool {
    let grid_in = grid_out.clone();

    let mut mutated = false;
    for ([x, y], tile) in grid_in.seats.iter().map(|(k, v)| (*k, *v)) {
        let count = count_occupied_all_directions([x, y], &grid_in);
        match tile {
            Tile::Empty => {
                if count == 0 {
                    grid_out.seats.insert([x, y], Tile::Occupied);
                    mutated = true;
                }
            }
            Tile::Occupied => {
                if count >= 5 {
                    grid_out.seats.insert([x, y], Tile::Empty);
                    mutated = true;
                }
            }
        }
    }

    mutated
}

fn part1(grid: &mut Grid) -> usize {
    while forward(grid) {}
    grid.seats
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Occupied))
        .count()
}

fn part2(grid: &mut Grid) -> usize {
    while forward2(grid) {}
    grid.seats
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Occupied))
        .count()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let mut grid = parse(input.as_str());
    let mut g1 = grid.clone();
    let res = part1(&mut g1);
    println!("{}", res);
    let res = part2(&mut grid);
    println!("{}", res);
}

#[test]
fn test_p1() {
    let mut grid = parse(
        r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#,
    );

    let res = part1(&mut grid);
    assert_eq!(res, 37)
}

#[test]
fn test_p2() {
    let mut grid = parse(
        r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#,
    );

    let res = part2(&mut grid);
    assert_eq!(res, 26)
}
