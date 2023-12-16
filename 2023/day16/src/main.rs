use std::collections::HashSet;

use utils::{Grid, IVec2};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn count_energized(grid: &Grid<u8>, start: IVec2, start_dir: IVec2) -> usize {
    let mut beams = vec![(start, start_dir)];
    let mut touched = HashSet::new();

    while let Some((mut pos, mut dir)) = beams.pop() {
        assert_ne!(dir.x.abs(), dir.y.abs());
        loop {
            if !grid.contains_point(pos) {
                break;
            }
            if touched.contains(&(pos, dir)) {
                // entered a loop (this is why I include the direction of the light)
                break;
            }
            touched.insert((pos, dir));
            match grid[pos] {
                b'.' => {}
                b'|' => {
                    if dir.y == 0 {
                        beams.push((pos + IVec2::Y, IVec2::Y));
                        beams.push((pos - IVec2::Y, -IVec2::Y));
                        continue;
                    }
                }
                b'-' => {
                    if dir.x == 0 {
                        beams.push((pos + IVec2::X, IVec2::X));
                        beams.push((pos - IVec2::X, -IVec2::X));
                        continue;
                    }
                }
                b'\\' => dir = IVec2::new(dir.y, dir.x),
                b'/' => dir = -IVec2::new(dir.y, dir.x),
                _ => unreachable!(),
            }
            pos = pos + dir;
        }
    }

    let touched = touched.into_iter().map(|(p, _)| p).collect::<HashSet<_>>();

    touched.len()
}

fn part1(input: &str) -> usize {
    let Some(width) = input.lines().next().map(|l| l.len()) else {
        return 0;
    };
    let grid = Grid::from_data(input.lines().map(|l| l.bytes()).flatten().collect(), width);

    count_energized(&grid, IVec2::ZERO, IVec2::X)
}

fn part2(input: &str) -> usize {
    let Some(width) = input.lines().next().map(|l| l.len()) else {
        return 0;
    };
    let grid = Grid::from_data(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.bytes())
            .flatten()
            .collect(),
        width,
    );

    let height = grid.height;

    (0..grid.height)
        .flat_map(|y| {
            [
                (IVec2::new(0, y as i32), IVec2::X),
                (IVec2::new(width as i32 - 1, y as i32), -IVec2::X),
            ]
            .into_iter()
        })
        .chain((0..grid.width).flat_map(|x| {
            [
                (IVec2::new(x as i32, 0), IVec2::Y),
                (IVec2::new(x as i32, height as i32 - 1), -IVec2::Y),
            ]
            .into_iter()
        }))
        .map(|(pos, dir)| count_energized(&grid, pos, dir))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 46);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 51);
    }
}
