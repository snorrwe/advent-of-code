extern crate advent;

use advent::point::Point;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

type Map = BTreeMap<Point, Tile>;

trait Adjacent {
    type Output;

    fn adjacent(&self) -> [Self::Output; 8];
}

impl Adjacent for Point {
    type Output = Self;

    /// returns
    /// [
    /// top left
    /// top
    /// top right
    /// right
    /// bottom right
    /// bottom
    /// bottom left
    /// left
    /// ]
    fn adjacent(&self) -> [Self::Output; 8] {
        let [top, left, right, bot] = self.neighbours();
        [
            left + Point::top(),
            top,
            top + Point::right(),
            right,
            right + Point::bottom(),
            bot,
            left + Point::bottom(),
            left,
        ]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let map = parse(lines);

    let part1 = part1(map.clone());
    println!("{}", part1);

    let part2 = part2(map);
    println!("{}", part2);

    Ok(())
}

fn part2(mut map: Map) -> usize {
    simulate(1_000_000_000, &mut map)
}

fn part1(mut map: Map) -> usize {
    simulate(10, &mut map)
}

fn value(map: &Map) -> usize {
    let [trees, yards] = map.values().fold([0, 0], |mut result, tile| {
        match tile {
            Tile::Trees => result[0] += 1,
            Tile::Lumberyard => result[1] += 1,
            _ => {}
        }
        result
    });

    trees * yards
}

/// Returns the value of the map after the specified number of ticks
fn simulate(ticks: usize, map: &mut Map) -> usize {
    let mut seen = HashMap::new();
    let mut elapsed = 0;
    for t in 0..ticks {
        elapsed = t;
        seen.entry(map.clone()).or_insert(vec![]).push(t);
        *map = tick(map);
        if seen.contains_key(&map) && seen[&map].len() >= 2 {
            break;
        }
    }
    let result = if elapsed < ticks - 1 {
        let repeating = seen
            .iter()
            .filter(|(_, v)| v.len() >= 2)
            .map(|(k, v)| v.iter().map(move |i| (i, value(&k))))
            .flatten()
            .collect::<HashMap<_, _>>();
        let start = repeating.iter().map(|(k, _)| *k).min().expect("??");
        let left = ticks - elapsed;
        let left = left % repeating.len();
        let left = left + start - 1;
        repeating[&left]
    } else {
        value(&map)
    };
    result
}

fn tick(map: &Map) -> Map {
    let mut result = map.clone();
    map.iter().for_each(|(point, tile)| {
        let count = count(map, point);
        *result.get_mut(&point).unwrap() = match tile {
            Tile::Open => {
                if count[1] >= 3 {
                    Tile::Trees
                } else {
                    Tile::Open
                }
            }
            Tile::Trees => {
                if count[2] >= 3 {
                    Tile::Lumberyard
                } else {
                    Tile::Trees
                }
            }
            Tile::Lumberyard => {
                if count[1] >= 1 && count[2] >= 1 {
                    Tile::Lumberyard
                } else {
                    Tile::Open
                }
            }
        };
    });
    result
}

/// return count of [open, trees, lumberyards] adjacent to `point`
fn count(map: &Map, point: &Point) -> [usize; 3] {
    let mut count = [0; 3];
    point.adjacent().iter().for_each(|point| {
        if let Some(tile) = map.get(point) {
            match tile {
                Tile::Open => count[0] += 1,
                Tile::Trees => count[1] += 1,
                Tile::Lumberyard => count[2] += 1,
            }
        }
    });
    count
}

fn parse<I>(lines: I) -> Map
where
    I: Iterator<Item = String>,
{
    lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, chr)| {
                    let tile = match chr {
                        '.' => Tile::Open,
                        '#' => Tile::Lumberyard,
                        '|' => Tile::Trees,
                        _ => unimplemented!(),
                    };
                    (Point::new(x as i32, y as i32), tile)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = [".#", "..", ".|"];

        let expected = [
            (Point::new(0, 0), Tile::Open),
            (Point::new(1, 0), Tile::Lumberyard),
            (Point::new(0, 1), Tile::Open),
            (Point::new(1, 1), Tile::Open),
            (Point::new(0, 2), Tile::Open),
            (Point::new(1, 2), Tile::Trees),
        ]
        .iter()
        .map(|(p, t)| (*p, *t))
        .collect::<Map>();

        let map = parse(input.iter().map(|x| x.to_string()));

        assert_eq!(map, expected);
    }

    #[test]
    fn test_single_tick() {
        let input = [
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ];
        let map = parse(input.iter().map(|x| x.to_string()));

        let expected = [
            ".......##.",
            "......|###",
            ".|..|...#.",
            "..|#||...#",
            "..##||.|#|",
            "...#||||..",
            "||...|||..",
            "|||||.||.|",
            "||||||||||",
            "....||..|.",
        ];
        let expected = parse(expected.iter().map(|x| x.to_string()));

        let map = tick(&map);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_part1() {
        let input = [
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ];
        let map = parse(input.iter().map(|x| x.to_string()));

        let result = part1(map);

        assert_eq!(result, 1147);
    }
}

