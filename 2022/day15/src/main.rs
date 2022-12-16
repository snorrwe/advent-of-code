use std::collections::HashSet;
use utils::IVec2;

#[derive(Debug)]
struct Sensor {
    pos: IVec2,
    radius: i32,
}

impl Sensor {
    pub fn contains(&self, point: IVec2) -> bool {
        self.pos.manhatten(point) <= self.radius
    }
}

#[derive(Default, Debug)]
struct Map {
    sensors: Vec<Sensor>,
    beacons: HashSet<IVec2>,
    min: i32,
    max: i32,
}

fn parse(input: &str) -> Map {
    let regex = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    let mut result = Map::default();
    result.min = std::i32::MAX;
    result.max = std::i32::MIN;
    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let fromx = caps.get(1).unwrap().as_str().parse().unwrap();
        let fromy = caps.get(2).unwrap().as_str().parse().unwrap();
        let tox = caps.get(3).unwrap().as_str().parse().unwrap();
        let toy = caps.get(4).unwrap().as_str().parse().unwrap();

        let pos = IVec2::new(fromx, fromy);
        let beacon = IVec2::new(tox, toy);
        let radius = beacon.manhatten(pos);

        result.min = result.min.min(fromx - radius);
        result.max = result.max.max(fromx + radius);

        result.sensors.push(Sensor { pos, radius });
        result.beacons.insert(beacon);
    }

    result
}

fn part1(map: &Map, y: i32) -> usize {
    let mut count = 0;
    'x: for x in map.min..=map.max {
        let pos = IVec2::new(x, y);
        if map.beacons.contains(&pos) {
            continue;
        }
        for s in map.sensors.iter() {
            if s.contains(pos) && pos != s.pos {
                count += 1;
                continue 'x;
            }
        }
    }
    count
}

fn part2(map: &Map, max: i32) -> usize {
    for s in map.sensors.iter() {
        'a: for x in -s.radius - 1..=s.radius + 1 {
            let y = s.radius + 1 - x.abs();
            let x = s.pos.x + x;
            let y = s.pos.y + y;

            if x < 0 || x > max || y < 0 || y > max {
                continue;
            }

            let pos = IVec2::new(x, y);
            for s in map.sensors.iter() {
                if s.contains(pos) {
                    continue 'a;
                }
            }
            return (pos.x as usize * 4000000) + pos.y as usize;
        }
    }
    unreachable!()
}

fn main() {
    let input = utils::read_input();
    let map = parse(&input);

    let res = part1(&map, 2000000);
    println!("part1: {res}");
    let res = part2(&map, 4000000);
    println!("part2: {res}");
}

#[test]
fn part1_test() {
    let m = parse(
        r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
    );

    let res = part1(&m, 10);
    assert_eq!(26, res);
    let res = part1(&m, 11);
    assert_eq!(27, res);
}

#[test]
fn part2_test() {
    let m = parse(
        r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
    );

    let res = part2(&m, 20);
    assert_eq!(56000011, res);
}
