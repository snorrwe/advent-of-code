use std::collections::HashMap;

type Grid = HashMap<[i32; 2], Tile>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Occipied,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_str("L"),
            Tile::Occipied => f.write_str("#"),
        }
    }
}

fn parse(inp: &str) -> Grid {
    inp.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'L' => Some(([x as i32, y as i32], Tile::Empty)),
                '#' => Some(([x as i32, y as i32], Tile::Occipied)),
                _ => None,
            })
        })
        .collect()
}

/// return if state has changed
fn forward(grid_out: &mut Grid) -> bool {
    let grid_in = grid_out.clone();

    let mut mutated = false;
    for ([x, y], tile) in grid_in.iter() {
        match tile {
            Tile::Empty => {
                let mut has_occ = false;
                'outer: for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            if matches!(grid_in.get(&[x + dx, y + dy]), Some(Tile::Occipied)) {
                                has_occ = true;
                                break 'outer;
                            }
                        }
                    }
                }
                if !has_occ {
                    grid_out.insert([*x, *y], Tile::Occipied);
                    mutated = true;
                }
            }
            Tile::Occipied => {
                let mut count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            if matches!(grid_in.get(&[x + dx, y + dy]), Some(Tile::Occipied)) {
                                count += 1;
                            }
                        }
                    }
                }
                if count >= 4 {
                    grid_out.insert([*x, *y], Tile::Empty);
                    mutated = true;
                }
            }
        }
    }

    mutated
}

/// return if state has changed
fn forward2(grid_out: &mut Grid) -> bool {
    let grid_in = grid_out.clone();

    let mut mutated = false;
    for ([x, y], tile) in grid_in.iter() {
        match tile {
            Tile::Empty => {
                todo!()
            }
            Tile::Occipied => {
                todo!()
            }
        }
    }

    mutated
}

fn part1(grid: &mut Grid) -> usize {
    while forward(grid) {}
    grid.iter()
        .filter(|(_, t)| matches!(t, Tile::Occipied))
        .count()
}

fn part2(grid: &mut Grid) -> usize {
    while forward2(grid) {}
    grid.iter()
        .filter(|(_, t)| matches!(t, Tile::Occipied))
        .count()
}

fn main() {
    let mut input = String::new();

    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let mut grid = parse(input.as_str());
    let mut g1 = grid.clone();
    let res = part1(&mut g1);
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
