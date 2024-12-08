use std::collections::{HashMap, HashSet};

use utils::{Grid, IVec2};

struct Input {
    grid: Grid<u8>,
    freq_map: HashMap<u8, Vec<IVec2>>,
}

fn parse(input: String) -> Input {
    let grid = Grid::from_ascii_lines(&input).unwrap();
    let mut freq_map: HashMap<u8, Vec<IVec2>> = HashMap::default();

    for y in 0..grid.height {
        let row = grid.row(y);
        for (x, f) in row.iter().enumerate() {
            if f != &b'.' {
                freq_map
                    .entry(*f)
                    .or_default()
                    .push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    Input { grid, freq_map }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut candidates = HashSet::new();

    for (f, nodes) in input.freq_map.iter() {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                let d = b - a;
                let c1 = b + d;
                let c2 = a - d;

                if input.grid.contains_point(c1) && input.grid[c1] != *f {
                    candidates.insert(c1);
                }
                if input.grid.contains_point(c2) && input.grid[c2] != *f {
                    candidates.insert(c2);
                }
            }
        }
    }

    candidates.len()
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == b {
        return a;
    }
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }

    a
}

fn part2(input: &Input) -> usize {
    let mut candidates = HashSet::new();

    for nodes in input.freq_map.values() {
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            for j in i + 1..nodes.len() {
                let b = nodes[j];

                let d = b - a;

                let g = gcd(d.x, d.y);

                let d = d / g;
                for (base, delta) in [(a, -d), (b, d)] {
                    let mut x = 0;
                    loop {
                        let c = base + delta * x;

                        if !input.grid.contains_point(c) {
                            break;
                        }
                        candidates.insert(c);
                        x += 1
                    }
                }
            }
        }
    }

    candidates.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 14);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 34);
    }
}
