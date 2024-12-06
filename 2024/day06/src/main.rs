use std::collections::HashSet;

use utils::{Grid, IVec2};

struct Input {
    grid: Grid<u8>,
    x: usize,
    y: usize,
}

const GUARD: u8 = b'^';
const OBS: u8 = b'#';

fn parse(input: String) -> Input {
    let mut x = 0;
    let mut y = 0;
    let grid = {
        let lines: &str = &input;
        let width = lines
            .lines()
            .next()
            .map(|l| l.len())
            .expect("No lines found");
        let data = lines
            .lines()
            .filter(|l| l.len() == width)
            .enumerate()
            .inspect(|(yp, l)| {
                if let Some((xp, _)) = l.bytes().enumerate().find(|(_x, b)| b == &GUARD) {
                    x = xp;
                    y = *yp;
                }
            })
            .flat_map(|(_y, l)| l.bytes())
            .collect();
        Grid::from_data(data, width)
    };
    Input { grid, x, y }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = parse(input);

    let (p1, p2) = solve(&mut input);
    println!("{}", p1);
    println!("{}", p2);
}

fn solve(input: &mut Input) -> (usize, usize) {
    let grid = &mut input.grid;

    let x = input.x as i32;
    let y = input.y as i32;
    let starting_pos = IVec2::new(x, y);
    let mut pos = starting_pos;
    let mut vel = -IVec2::Y;
    let mut visited = HashSet::new();
    visited.insert(pos);
    loop {
        let peek = pos + vel;
        if !grid.contains_point(peek) {
            break;
        }
        if grid[peek] == OBS {
            vel = vel.rotate_cw();
        } else {
            pos = peek;
            visited.insert(pos);
        }
    }
    // p2
    let mut obs = HashSet::new();
    for p in visited.iter() {
        let candidate = *p;
        if grid[candidate] != OBS {
            let tile = std::mem::replace(&mut grid[candidate], OBS);
            if check_loop(starting_pos, -IVec2::Y, grid) {
                obs.insert(candidate);
            }
            grid[candidate] = tile;
        }
    }

    (visited.len(), obs.len())
}

fn check_loop(mut pos: IVec2, mut vel: IVec2, grid: &Grid<u8>) -> bool {
    let mut visited = HashSet::new();
    loop {
        let peek = pos + vel;
        if !grid.contains_point(peek) {
            return false;
        }
        if grid[peek] == OBS {
            vel = vel.rotate_cw();
        } else {
            pos = peek;
            if !visited.insert((pos, vel)) {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn test() {
        let mut inp = parse(INPUT.to_string());
        let (p1, p2) = solve(&mut inp);

        assert_eq!(p1, 41);
        assert_eq!(p2, 6);
    }
}
