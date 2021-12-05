#![feature(test)]
extern crate test;

mod point;
use point::*;

/// Test AB segment and C point intersection
fn on_segment(a: Point, b: Point, c: Point) -> bool {
    let ab = b - a;
    let ac = c - a;
    let e = ac.dot(&ab);
    if e < 0 {
        return false;
    };
    let f = ab.dot(&ab);
    e < f
}

fn part1(asteriods: &mut [Point]) -> (Point, usize) {
    asteriods
        .iter()
        .cloned()
        .map(|a| {
            let visible = asteriods
                .iter()
                .cloned()
                .filter(|b| {
                    let minx = a.x.min(b.x);
                    let miny = a.y.min(b.y);
                    let maxx = a.x.max(b.x);
                    let maxy = a.y.max(b.y);

                    for c in asteriods
                        .iter()
                        .cloned()
                        .filter(|c| minx <= c.x && miny <= c.y && c.x <= maxx && c.y <= maxy)
                        .filter(|c| a != *c && b != c)
                    {
                        if on_segment(a, *b, c) {
                            return false;
                        }
                    }
                    true
                })
                .count();
            (a, visible - 1)
        })
        .max_by_key(|(_, i)| *i)
        .unwrap()
}

fn part2(pos: Point, mut asteriods: Vec<Point>, n: usize) -> Point {
    let first = asteriods
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, c)| c.x == pos.x && pos.y > c.y)
        .min_by_key(|(_, c)| pos.squared_dist(c))
        .unwrap();
    asteriods.swap(0, first.0);
    let mut line = (pos, first.1);
    let mut count = 1;
    loop {
        asteriods[count..].sort_unstable_by_key(|x| {
            // sort by the angle from line
            let (a, b) = line;
            let ab = b - a;
            let len = ab.len();
            let x = *x - a;
            let mut t1 = x.dot(&ab) as f32;
            t1 /= len * x.len();
            (t1.acos() * 100_000.0) as i32
        });
        let (i, c) = asteriods[count..]
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, c)| {
                let (a, b) = line;
                // is C clockwise from AB?
                let orient = (a.x - c.x) * -(b.y - c.y) + (a.y - c.y) * (b.x - c.x);
                // and is the angle acute?
                orient < 0
            })
            .expect("wtf");
        asteriods.swap(count, i + count);
        count += 1;
        line = (pos, c);
        if count == n {
            return c;
        }
    }
}

fn parse(input: &str) -> Vec<Point> {
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

    points
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = parse(&input);

    let p1 = part1(&mut map);
    println!("{:?}", p1);

    let p2 = part2(p1.0, map, 200);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_simple() {
        let input = ".#..#\n.....\n#####\n....#\n...##\n";

        let mut map = parse(input);

        let res = part1(&mut map);

        println!("{:?}", res);

        assert_eq!(res.0, Point { x: 3, y: 4 });
        assert_eq!(res.1, 8);
    }

    #[test]
    fn part1_simple_2() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";

        let mut map = parse(input);

        let res = part1(&mut map);

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

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let mut map = parse(&input);
        let p = part1(&mut map);
        b.iter(|| part2(p.0, map.clone(), 200));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let mut map = parse(&input);
        b.iter(|| part1(&mut map));
    }
}
