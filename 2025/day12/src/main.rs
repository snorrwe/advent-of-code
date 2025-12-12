use std::collections::HashMap;

use arrayvec::ArrayVec;
use utils::{Grid, IVec2};

#[derive(Default, Debug)]
struct Region {
    width: usize,
    height: usize,
    /// number of each shape
    shapes: ArrayVec<usize, 6>,
}

/// List of '#' points in the 3x3 grid
type Shape = ArrayVec<IVec2, 9>;

#[derive(Default, Debug)]
struct Input {
    shapes: ArrayVec<Shape, 6>,
    regions: Vec<Region>,
}

fn parse(input: &str) -> Input {
    let region_re = regex::Regex::new(r#"(\d+)x(\d+): (.*)$"#).unwrap();

    let mut out = Input::default();

    let mut lines = input.lines();
    for i in 0..=5 {
        let id = lines.next().unwrap().trim_end_matches(&[':', '\n']);
        assert_eq!(id.parse::<i32>().unwrap(), i);
        let mut shape: ArrayVec<IVec2, 9> = Default::default();

        for (y, l) in (&mut lines).take_while(|l| !l.is_empty()).enumerate() {
            for (x, c) in l.trim().bytes().enumerate() {
                if c == b'#' {
                    shape.push(IVec2::new(x as i32, y as i32));
                }
            }
        }

        out.shapes.push(shape);
    }

    for line in lines {
        let Some(cap) = region_re.captures(line) else {
            break;
        };

        let w = cap.get(1).unwrap().as_str().parse().unwrap();
        let h = cap.get(2).unwrap().as_str().parse().unwrap();

        let shapes = cap
            .get(3)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        out.regions.push(Region {
            width: w,
            height: h,
            shapes,
        });
    }

    out
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rotation {
    Identity,
    R90,
    R180,
    R270,
}

fn transformed_shape_point(
    mut p: IVec2,
    rotation: Rotation,
    flip_horizontal: bool,
    flip_vertical: bool,
    offset: IVec2,
) -> IVec2 {
    p -= IVec2::ONE; // transform origin to the center of the 3x3 board
    if flip_horizontal {
        p.x *= -1;
    }
    if flip_vertical {
        p.y *= -1;
    }
    let p = match rotation {
        Rotation::Identity => p,
        Rotation::R90 => IVec2::new(-p.y, p.x),
        Rotation::R180 => p * -1,
        Rotation::R270 => IVec2::new(p.y, -p.x),
    };
    p + offset + IVec2::ONE
}

fn fit_shapes(
    grid: &mut Grid<bool>,
    shapes: &HashMap<(usize, Rotation, bool, bool), Shape>,
    todo: &mut [usize],
) -> bool {
    let Some((idx, count)) = todo.iter_mut().enumerate().find(|(_, x)| **x != 0) else {
        // if no more shapes to do, then we found a fit
        return true;
    };

    *count -= 1;

    for rot in [
        Rotation::Identity,
        Rotation::R90,
        Rotation::R180,
        Rotation::R270,
    ] {
        for (flip_x, flip_y) in [(false, false), (true, false), (false, true), (true, true)] {
            let pts = &shapes[&(idx, rot, flip_x, flip_y)];
            for y in 0..=grid.height - 3 {
                'test: for x in 0..=grid.width - 3 {
                    let offset = IVec2::new(x as i32, y as i32);
                    // if shape doesn't fit continue the search
                    if pts.iter().any(|p| grid[*p + offset]) {
                        continue 'test;
                    }
                    for p in pts.iter() {
                        grid[*p + offset] = true;
                    }

                    if fit_shapes(grid, shapes, todo) {
                        return true;
                    }

                    // cleanup
                    for p in pts.iter() {
                        grid[*p + offset] = false;
                    }
                }
            }
        }
    }

    // cleanup
    todo[idx] += 1;

    false
}

fn part1(input: &Input) -> usize {
    let mut s = 0;

    let mut shapes = HashMap::new();
    for idx in 0..6 {
        for rot in [
            Rotation::Identity,
            Rotation::R90,
            Rotation::R180,
            Rotation::R270,
        ] {
            for (flip_x, flip_y) in [(false, false), (true, false), (false, true), (true, true)] {
                let mut pts = input.shapes[idx].clone();
                for p in pts.iter_mut() {
                    *p = transformed_shape_point(*p, rot, flip_x, flip_y, IVec2::ZERO);
                }
                shapes.insert((idx, rot, flip_x, flip_y), pts);
            }
        }
    }

    for region in input.regions.iter() {
        let mut total_area_needed = 0;
        for (idx, n) in region.shapes.iter().copied().enumerate() {
            total_area_needed += input.shapes[idx].len() * n;
        }
        if total_area_needed > region.width * region.height {
            continue;
        }

        let mut grid = Grid::new(region.width, region.height);
        grid.fill(false);
        let mut todo = region.shapes.clone();
        if fit_shapes(&mut grid, &shapes, &mut todo) {
            s += 1;
        }
    }
    s
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
