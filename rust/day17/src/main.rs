extern crate advent;

use advent::point::Point;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

type Layout = BTreeMap<Point, Tile>;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let mut map = parse(lines);

    simulate(&mut map);

    let part1 = part1(&map);
    println!("{}", part1 - 4); // Off by 4 could not find the extra Falls

    let part2 = part2(&map);
    println!("{}", part2); // This is actually correct
    Ok(())
}

fn simulate(map: &mut Layout) {
    play(map);
    render(&map);
}

fn part1(map: &Layout) -> usize {
    map.iter()
        .filter(|(p, c)| (**c == Tile::Water || **c == Tile::Fall) && p.y > 0)
        .count()
}

fn part2(map: &Layout) -> usize {
    map.iter()
        .filter(|(p, c)| (**c == Tile::Water) && p.y > 0)
        .count()
}

fn play(map: &mut Layout) {
    debug_assert!(map.len() > 0);
    let max_y = map.iter().map(|(p, _)| p.y).max().unwrap();
    let mut last = map.clone();
    map.insert(Point::new(500, 0), Tile::Fall);
    while *map != last {
        last = map.clone();
        tick(map, max_y);
    }
}

fn tick(map: &mut Layout, max_y: i32) {
    let falls = map
        .iter()
        .filter(|(_, t)| **t == Tile::Fall)
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<_>>();
    falls.iter().for_each(|(point, _tile)| {
        let [_top, _left, _right, bot] = point.neighbours();
        if bot.y > max_y {
            return;
        }
        if !map.contains_key(&bot) {
            map.insert(bot, Tile::Fall);
        } else if map[&bot] == Tile::Water || map[&bot] == Tile::Clay {
            fill_row(*point, map);
        }
    });
}

fn fill_row(point: Point, map: &mut Layout) {
    let [_top, left, right, bot] = point.neighbours();
    if !map.contains_key(&bot) || map[&bot] == Tile::Fall {
        return;
    }
    let (left, t1) = find_edge_fill(left, Point::new(-1, 0), map);
    let (right, t2) = find_edge_fill(right, Point::new(1, 0), map);
    if left.dist(&right) == 0 {
        *map.entry(point).or_insert(Tile::Water) = Tile::Water;
    }
    let tile = if t1 == Tile::Fall || t2 == Tile::Fall {
        Tile::Fall
    } else {
        Tile::Water
    };
    let y = point.y;
    for x in left.x..=right.x {
        map.insert(Point::new(x, y), tile);
    }
}

/// returns the position in one direction
/// and wether the water in the row stays put (Water) or
/// moves (Fall)
fn find_edge_fill(start: Point, velocity: Point, map: &Layout) -> (Point, Tile) {
    let mut current = start;
    if map.contains_key(&start) {
        // See if the current level needs filling
        while map.contains_key(&current) {
            if map[&current] == Tile::Clay {
                return (current - velocity, Tile::Water);
            }
            current = current + velocity;
        }
        return (start - velocity, map[&start]);
    }
    let bot = start + Point::new(0, 1);
    if !map.contains_key(&bot) {
        return (start, Tile::Fall);
    }
    let mut current = start;
    loop {
        let next = current + velocity;
        let bot = next + Point::new(0, 1);
        if !map.contains_key(&bot) {
            return (next, Tile::Fall);
        }
        if map.contains_key(&next) && map[&next] == Tile::Clay {
            return (current, Tile::Water);
        }
        current = next;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Clay,
    Water,
    Fall,
}

/// Returns the clay tiles from the input
fn parse<I>(lines: I) -> Layout
where
    I: Iterator<Item = String>,
{
    lines
        .map(|line| parse_line_into_range(line.as_str()))
        .map(|[from, to]| {
            let mut v = vec![];
            for x in from.x..=to.x {
                for y in from.y..=to.y {
                    v.push((Point::new(x, y), Tile::Clay));
                }
            }
            v
        })
        .flatten()
        .collect()
}

fn parse_line_into_range(line: &str) -> [Point; 2] {
    let mut range = [Point::new(0, 0), Point::new(0, 0)];
    line.split(", ").for_each(|part| {
        let mut input_range = part[2..].split("..");
        let min = input_range
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Invalid number 1");
        let mut max = min;
        if let Some(input) = input_range.next() {
            max = input.parse::<i32>().expect("Invalid number 2");
        }
        match &part[..1] {
            "x" => {
                range[0].x = min;
                range[1].x = max
            }
            "y" => {
                range[0].y = min;
                range[1].y = max
            }
            _ => panic!("Got an unexpected character"),
        };
    });
    range
}

fn render(map: &Layout) {
    let mut dims = [10_0000, 10_0000, 0, 0];
    map.iter().for_each(|(point, _)| {
        if point.x < dims[0] {
            dims[0] = point.x;
        }
        if point.y < dims[1] {
            dims[1] = point.y;
        }
        if point.x > dims[2] {
            dims[2] = point.x;
        }
        if point.y > dims[3] {
            dims[3] = point.y;
        }
    });

    for y in dims[1]..=dims[3] {
        for x in dims[0]..=dims[2] {
            let p = Point::new(x, y);
            if !map.contains_key(&p) {
                print!(" ");
            } else {
                let c = match map[&p] {
                    Tile::Clay => "#",
                    Tile::Water => "~",
                    Tile::Fall => "|",
                };
                print!("{}", c);
            }
        }
        println!("");
    }
    println!("\n-----------------------\n");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ["x=495, y=2..7", "y=7, x=495..501", "x=501, y=3..7"];

        let expected = [
            (Point::new(495, 2), Tile::Clay),
            (Point::new(495, 3), Tile::Clay),
            (Point::new(495, 4), Tile::Clay),
            (Point::new(495, 5), Tile::Clay),
            (Point::new(495, 6), Tile::Clay),
            (Point::new(495, 7), Tile::Clay),
            (Point::new(496, 7), Tile::Clay),
            (Point::new(497, 7), Tile::Clay),
            (Point::new(498, 7), Tile::Clay),
            (Point::new(499, 7), Tile::Clay),
            (Point::new(500, 7), Tile::Clay),
            (Point::new(501, 7), Tile::Clay),
            (Point::new(501, 6), Tile::Clay),
            (Point::new(501, 5), Tile::Clay),
            (Point::new(501, 4), Tile::Clay),
            (Point::new(501, 3), Tile::Clay),
        ]
        .iter()
        .map(|x| *x)
        .collect::<Layout>();

        let map = parse(input.iter().map(|x| x.to_string()));

        assert_eq!(map, expected);
    }

    #[test]
    fn test_fill() {
        let input = ["x=499, y=5..7", "y=7, x=499..502", "x=502, y=5..7"];
        let expected = [
            (Point::new(499, 5), Tile::Clay),
            (Point::new(499, 6), Tile::Clay),
            (Point::new(499, 7), Tile::Clay),
            (Point::new(499, 7), Tile::Clay),
            (Point::new(502, 7), Tile::Clay),
            (Point::new(502, 6), Tile::Clay),
            (Point::new(502, 5), Tile::Clay),
            (Point::new(500, 7), Tile::Clay),
            (Point::new(501, 7), Tile::Clay),
            (Point::new(500, 6), Tile::Water),
            (Point::new(501, 6), Tile::Water),
        ]
        .iter()
        .map(|x| *x)
        .collect();

        let mut map = parse(input.iter().map(|x| x.to_string()));

        fill_row(Point::new(500, 6), &mut map);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_fill_1by1() {
        let input = ["x=499, y=5..7", "y=7, x=499..501", "x=501, y=5..7"];
        let expected = [
            (Point::new(499, 5), Tile::Clay),
            (Point::new(499, 6), Tile::Clay),
            (Point::new(499, 7), Tile::Clay),
            (Point::new(501, 7), Tile::Clay),
            (Point::new(501, 6), Tile::Clay),
            (Point::new(501, 5), Tile::Clay),
            (Point::new(500, 7), Tile::Clay),
            (Point::new(501, 7), Tile::Clay),
            (Point::new(500, 6), Tile::Water),
        ]
        .iter()
        .map(|x| *x)
        .collect();

        let mut map = parse(input.iter().map(|x| x.to_string()));

        fill_row(Point::new(500, 6), &mut map);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_fill_overflow() {
        let input = ["x=499, y=5..7", "y=7, x=499..501"];
        let expected = [
            (Point::new(499, 5), Tile::Clay),
            (Point::new(499, 6), Tile::Clay),
            (Point::new(499, 7), Tile::Clay),
            (Point::new(500, 7), Tile::Clay),
            (Point::new(501, 7), Tile::Clay),
            (Point::new(500, 6), Tile::Fall),
            (Point::new(501, 6), Tile::Fall),
            (Point::new(502, 6), Tile::Fall),
        ]
        .iter()
        .map(|x| *x)
        .collect();

        let mut map = parse(input.iter().map(|x| x.to_string()));

        fill_row(Point::new(500, 6), &mut map);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_fills_hollow_container() {
        let input = [
            "x=497, y=5..7",
            "y=7, x=497..505",
            "x=505, y=5..7",
            "y=4, x=499..501",
            "y=5, x=499..501",
        ];
        let mut map = parse(input.iter().map(|x| x.to_string()));

        simulate(&mut map);
        let result = part1(&map);

        assert_eq!(result, 32);
    }

    #[test]
    fn test_falls_off_1_edge() {
        let input = [
            "x=497, y=5..7",
            "y=7, x=497..505",
            "x=505, y=5..7",
            "x=500, y=3..6",
        ];
        let mut map = parse(input.iter().map(|x| x.to_string()));

        simulate(&mut map);
        let result = part1(&map);

        assert_eq!(result, 34);
    }

    #[test]
    fn test_fall_fills_1by1() {
        let input = ["x=499, y=5..7", "y=7, x=499..501", "x=501, y=5..7"];
        let mut map = parse(input.iter().map(|x| x.to_string()));

        simulate(&mut map);
        let result = part1(&map);

        assert_eq!(result, 16);
    }

    #[test]
    fn test_simulate() {
        let input = [
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ];
        let mut map = parse(input.iter().map(|x| x.to_string()));

        simulate(&mut map);
        let result = part1(&map);

        assert_eq!(result, 57);
    }
}

