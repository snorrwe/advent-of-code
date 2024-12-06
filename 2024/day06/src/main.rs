use std::collections::HashSet;

use utils::{Grid, IVec2};

struct Input {
    grid: Grid<u8>,
    x: usize,
    y: usize,
}

const GUARD: u8 = b'^';
const EMPTY: u8 = b'.';
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
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let grid = &input.grid;
    let x = input.x as i32;
    let y = input.y as i32;
    let mut pos = IVec2::new(x, y);
    let mut vel = -IVec2::Y;
    let mut visited = HashSet::new();
    visited.insert(pos);
    loop {
        let peak = pos + vel;
        if peak.x < 0
            || peak.y < 0
            || grid.width <= peak.x as usize
            || grid.height <= peak.y as usize
        {
            break;
        }
        if grid[peak] == OBS {
            vel = vel.rotate_cw();
        } else {
            pos = pos + vel;
            visited.insert(pos);
        }
    }
    visited.len()
}

fn part2(input: &Input) -> i32 {
    todo!()
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
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 41);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
