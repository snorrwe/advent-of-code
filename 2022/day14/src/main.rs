use std::collections::HashSet;
use utils::IVec2;

#[derive(Default)]
struct Cave {
    rocks: HashSet<IVec2>,
    bottom: i32,
}

struct LineIterator {
    current: IVec2,
    end: IVec2,
}

impl LineIterator {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for LineIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.end - self.current;
        if d == IVec2::ZERO {
            return None;
        }
        let res = self.current;
        self.current = self.current + d.as_direction();
        Some(res)
    }
}

fn parse_vec(s: &str) -> IVec2 {
    let mut s = s.split(',');
    let x = s.next().expect("next").parse().expect("parse");
    let y = s.next().expect("next").parse().expect("parse");
    IVec2 { x, y }
}

fn parse(input: &str) -> Cave {
    let mut result = Cave::default();
    for line in input.lines() {
        let a = line.split(" -> ");
        let b = line.split(" -> ");
        for (a, b) in a.zip(b.skip(1)) {
            let a = parse_vec(a);
            let b = parse_vec(b);
            result.rocks.extend(LineIterator::new(a, b));
            result.rocks.insert(b);
            result.bottom = result.bottom.max(a.y);
            result.bottom = result.bottom.max(b.y);
        }
    }
    result
}

fn part1(cave: &Cave) -> usize {
    let bottom = cave.bottom;
    let mut cave = cave.rocks.clone();
    let mut sand_count = 0;
    'p1: loop {
        let mut sand = IVec2::new(500, 1);
        'sand: loop {
            if sand.y > bottom {
                break 'p1 sand_count;
            }
            for d in [IVec2::Y, IVec2::new(-1, 1), IVec2::new(1, 1)] {
                let s = sand + d;
                if !cave.contains(&s) {
                    sand = s;
                    continue 'sand;
                }
            }
            cave.insert(sand);
            sand_count += 1;
            break;
        }
    }
}

fn part2(cave: &Cave) -> usize {
    let bottom = cave.bottom + 2;
    let mut cave = cave.rocks.clone();
    let mut sand_count = 0;
    let contains = |p: IVec2, cave: &HashSet<IVec2>| cave.contains(&p) || p.y == bottom;
    loop {
        let mut sand = IVec2::new(500, 0);
        if cave.contains(&sand) {
            return sand_count;
        }
        'sand: loop {
            for d in [IVec2::Y, IVec2::new(-1, 1), IVec2::new(1, 1)] {
                let s = sand + d;
                if !contains(s, &cave) {
                    sand = s;
                    continue 'sand;
                }
            }
            cave.insert(sand);
            sand_count += 1;
            break;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cave = parse(&input);
    let res = part1(&cave);
    println!("part1: {res}");
    let res = part2(&cave);
    println!("part2: {res}");
}

#[test]
fn test_part1() {
    let grid = parse(
        r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
    );

    let res = part1(&grid);
    assert_eq!(24, res);
}

#[test]
fn test_part2() {
    let grid = parse(
        r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
    );

    let res = part2(&grid);
    assert_eq!(93, res);
}
