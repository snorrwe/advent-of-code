use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn manhatten(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn squared_dist(&self, other: &Self) -> i32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        x * x + y * y
    }

    pub fn len(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;

        (x * x + y * y).sqrt()
    }

    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Div<i32> for Point {
    type Output = Point;
    fn div(self, other: i32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, other: f32) -> Point {
        Point {
            x: (self.x as f32 * other).round() as i32,
            y: (self.y as f32 * other).round() as i32,
        }
    }
}
/// Compute AB segment and C point distance squared
fn sq_dist_segment_point(a: Point, b: Point, c: Point) -> i32 {
    let ab = b - a;
    let ac = c - a;
    let bc = c - b;
    let e = ac.dot(&ab);
    if e < 0 {
        return ac.dot(&ac);
    };
    let f = ab.dot(&ab);
    if e >= f {
        return bc.dot(&bc);
    }
    ac.dot(&ac) - e * e / f
}

fn part1(map: &GameMap) -> (Point, usize) {
    map.asteriods
        .iter()
        .map(|a| {
            let visible = map
                .asteriods
                .iter()
                .filter(|b| {
                    for c in map.asteriods.iter().filter(|c| *a != **c && b != c) {
                        if sq_dist_segment_point(*a, **b, *c) == 0 {
                            return false;
                        }
                    }
                    true
                })
                .count();
            (*a, visible - 1)
        })
        .max_by_key(|(_, i)| *i)
        .unwrap()
}

fn visible(map: &GameMap, a: &Point, out: &mut Vec<(usize, Point)>) {
    out.clear();
    out.extend(map.asteriods.iter().cloned().enumerate().filter(|(_, b)| {
        for c in map.asteriods.iter().filter(|c| *a != **c && b != *c) {
            if sq_dist_segment_point(*a, *b, *c) == 0 {
                return false;
            }
        }
        true
    }));
}

fn part2(pos: Point, mut map: GameMap, n: usize) -> Point {
    let mut vis = Vec::with_capacity(map.asteriods.len());
    visible(&map, &pos, &mut vis);
    let first = vis
        .iter()
        .filter(|(_, c)| c.x == pos.x && pos.y > c.y)
        .min_by_key(|(_, c)| pos.squared_dist(c))
        .unwrap();
    map.asteriods.remove(first.0);
    let mut line = (pos, first.1);
    let mut count = 1;
    while !map.asteriods.is_empty() {
        visible(&map, &pos, &mut vis);
        let (i, c, _o) = vis
            .iter()
            .filter(|(_, c)| *c != line.0) // TODO: just remove at the beginning
            .filter_map(|(i, c)| {
                let (a, b) = line;
                // is C clockwise from AB?
                let orient = (a.x - c.x) * -(b.y - c.y) + (a.y - c.y) * (b.x - c.x);
                // and is the angle acute?
                if orient < 0 && (b - a).dot(&(*c - a)) > 0 {
                    Some((i, c, orient))
                } else {
                    None
                }
            })
            .min_by(|x, y| {
                let (a, b) = line;
                let ab = b - a;
                let len = ab.len();
                let x = *x.1 - a;
                let y = *y.1 - a;
                let mut t1 = x.dot(&ab) as f32;
                let mut t2 = y.dot(&ab) as f32;
                t1 /= len * x.len();
                t2 /= len * y.len();
                t1.acos()
                    .partial_cmp(&t2.acos())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();
        count += 1;
        line = (pos, *c);
        if count == n {
            return *c;
        }
        map.asteriods.remove(*i);
    }
    unreachable!("wtf {:?}", line);
}

#[derive(Debug, Clone)]
struct GameMap {
    asteriods: Vec<Point>,
}

fn parse(input: &str) -> GameMap {
    let points = input
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            let mut asteriods = Vec::with_capacity(line.len());
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteriods.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            asteriods
        })
        .flatten()
        .collect();

    GameMap { asteriods: points }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = parse(&input);

    let p1 = part1(&map);
    println!("{:?}", p1);

    let p2 = part2(p1.0, map, 200);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_simple() {
        let input = ".#..#\n.....\n#####\n....#\n...##\n";

        let map = parse(input);

        let res = part1(&map);

        println!("{:?}", res);

        assert_eq!(res.0, Point { x: 3, y: 4 });
        assert_eq!(res.1, 8);
    }

    #[test]
    fn part1_simple_2() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";

        let map = parse(input);

        let res = part1(&map);

        println!("{:?}", res);

        assert_eq!(res.0, Point { x: 5, y: 8 });
        assert_eq!(res.1, 33);
    }

    #[test]
    fn part2_simple() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";

        let map = parse(input);

        let res = part2(Point { x: 11, y: 13 }, map, 200);

        println!("{:?}", res);

        assert_eq!(res, Point { x: 8, y: 2 });
    }
}
