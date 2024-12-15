use utils::{Grid, IVec2};

#[derive(Debug, Clone)]
struct Input {
    pub grid: Grid<u8>,
    pub rules: Vec<u8>,
}

fn parse(input: String) -> Input {
    let (grid, rules) = input.split_once("\n\n").unwrap();

    let grid = Grid::from_ascii_lines(grid).unwrap();
    let rules = rules.as_bytes().to_vec();

    Input { grid, rules }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn botpos(g: &Grid<u8>) -> IVec2 {
    for y in 0..g.height as i32 {
        for x in 0..g.width as i32 {
            let pos = IVec2::new(x, y);
            if g[pos] == b'@' {
                return pos;
            }
        }
    }

    unreachable!()
}

fn part1(input: &Input) -> usize {
    let mut grid = input.grid.clone();

    let mut botpos = botpos(&grid);

    for action in &input.rules {
        let vel = match action {
            b'^' => -IVec2::Y,
            b'v' => IVec2::Y,
            b'<' => -IVec2::X,
            b'>' => IVec2::X,
            _ => {
                continue;
            }
        };

        let mut success = true;
        let mut steps = 0;
        let mut scan = botpos + vel;
        while grid.contains_point(scan) {
            steps += 1;
            match grid[scan] {
                b'#' => {
                    success = false;
                    break;
                }
                b'.' => {
                    break;
                }
                _ => {}
            }
            scan += vel;
        }
        if success {
            // go from the end of the move backwards
            for s in (0..steps).rev() {
                let pos = botpos + vel * s;
                let next = botpos + vel * (s + 1);
                grid[next] = grid[pos];
            }
            grid[botpos] = b'.';
            botpos = botpos + vel;
        }
    }

    let mut res = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == &b'O' {
                res += 100 * y + x;
            }
        }
    }

    res
}

fn trystep(grid: &mut Grid<u8>, pos: IVec2, vel: IVec2) -> bool {
    match grid[pos] {
        b'.' => return true,
        b'#' => return false,
        b'@' => {
            if trystep(grid, pos + vel, vel) {
                grid[pos + vel] = b'@';
                grid[pos] = b'.';
                return true;
            }
        }
        b']' => return trystep(grid, pos - IVec2::X, vel),
        b'[' => {
            if vel.y == 0 {
                if vel.x < 0 {
                    if trystep(grid, pos - IVec2::X, vel) {
                        grid[pos + vel] = b'[';
                        grid[pos] = b']';
                        grid[pos - vel] = b'.';
                        return true;
                    }
                } else {
                    if trystep(grid, pos + IVec2::X * 2, vel) {
                        grid[pos] = b'.';
                        grid[pos + IVec2::X] = b'[';
                        grid[pos + IVec2::X * 2] = b']';
                        return true;
                    }
                }
            } else {
                if trystep(grid, pos + vel, vel) && trystep(grid, pos + vel + IVec2::X, vel) {
                    grid[pos + vel] = b'[';
                    grid[pos + vel + IVec2::X] = b']';
                    grid[pos] = b'.';
                    grid[pos + IVec2::X] = b'.';
                    return true;
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
    false
}

fn part2(input: &Input) -> usize {
    let mut grid = Grid::new(input.grid.width * 2, input.grid.height);
    grid.fill(b'.');
    let mut botpos = IVec2::default();
    for y in 0..input.grid.height {
        for x in 0..input.grid.width {
            let t = *input.grid.get(x, y);
            match t {
                b'#' => {
                    grid.insert(x * 2, y, t);
                    grid.insert(x * 2 + 1, y, t);
                }
                b'@' => {
                    botpos = IVec2::new(x as i32 * 2, y as i32);
                    grid.insert(x * 2, y, t);
                }
                b'O' => {
                    grid.insert(x * 2, y, b'[');
                    grid.insert(x * 2 + 1, y, b']');
                }
                _ => {}
            }
        }
    }

    for action in &input.rules {
        let vel = match action {
            b'^' => -IVec2::Y,
            b'v' => IVec2::Y,
            b'<' => -IVec2::X,
            b'>' => IVec2::X,
            _ => {
                continue;
            }
        };

        let rollback = grid.clone();
        if trystep(&mut grid, botpos, vel) {
            botpos = botpos + vel;
        } else {
            grid = rollback;
        }
    }

    if false {
        let debug = grid.as_char();
        println!("{debug}");
    }

    let mut res = 0;
    for y in 1..grid.height {
        for x in 2..grid.width {
            if grid.get(x, y) == &b'[' {
                res += 100 * y + x;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 10092);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 9021);
    }
}
