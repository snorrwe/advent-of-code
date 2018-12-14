use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::mem;
use std::ops::Add;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let (map, trains) = build_track(lines);

    let result = part1(map, trains, None);
    println!("Part1: {:?}", result);
    Ok(())
}

fn part1(map: Map, mut trains: Trains, max_ticks: Option<usize>) -> Option<(usize, Point)> {
    for i in 0..max_ticks.unwrap_or(2_000_000) {
        let collision = tick(&map, &mut trains);
        if let Some(collision) = collision {
            return Some((i, collision));
        }
    }
    None
}

/// Returns the position of the collision if any
/// Updates the trains
fn tick(map: &Map, trains: &mut Trains) -> Option<Point> {
    let mut points = trains.iter().map(|t| t.point).collect::<HashSet<_>>();
    trains.sort_unstable_by_key(|train| train.point);
    let mut result = None;
    *trains = trains
        .iter()
        .map(|train| {
            let prev = train.point;
            let train = train.tick(&map);
            if points.contains(&train.point) {
                result = Some(train.point);
            }
            points.remove(&prev);
            points.insert(train.point);
            train
        })
        .collect();
    result
}

type Map = HashMap<Point, Track>;
type Trains = Vec<Train>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Turn {
    Left = 0,
    _Straight = 1,
    Right = 2,
}

impl Turn {
    pub fn from_u8(n: u8) -> Option<Turn> {
        if n <= 2 {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { mem::transmute(*self) }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Track {
    NS,
    EW,
    NE,
    NW,
    NESW,
}

#[derive(Debug, Clone, Eq, Ord)]
struct Train {
    pub point: Point,
    pub facing: Point,
    pub turn: Turn,
}

impl PartialEq for Train {
    fn eq(&self, other: &Train) -> bool {
        self.point == other.point
    }
}

impl PartialOrd for Train {
    fn partial_cmp(&self, other: &Train) -> Option<Ordering> {
        Some(self.point.cmp(&other.point))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Right => Self::new(-self.y, self.x),
            Turn::Left => Self::new(self.y, -self.x),
            Turn::_Straight => self.clone(),
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Train {
    pub fn new(x: i32, y: i32, facing: Point) -> Train {
        Train {
            point: Point::new(x, y),
            facing: facing,
            turn: Turn::Left,
        }
    }

    pub fn tick(&self, map: &Map) -> Self {
        let mut result = self.clone();
        let next = self.point + self.facing;
        result.point = next;
        let node = map
            .get(&result.point)
            .expect(&format!("Point was not found on the map {:?}", result));
        match node {
            Track::NESW => {
                result.facing = result.facing.turn(result.turn);
                let mut x = result.turn.as_u8();
                x += 1;
                x %= 3;
                result.turn = Turn::from_u8(x).expect(&format!("Unexpected value for turn {}", x));
            }
            Track::NE => match result.facing {
                Point { x: -1, y: 0 } | Point { x: 1, y: 0 } => {
                    result.facing = result.facing.turn(Turn::Left)
                }
                Point { x: 0, y: -1 } | Point { x: 0, y: 1 } => {
                    result.facing = result.facing.turn(Turn::Right)
                }
                _ => unimplemented!(),
            },
            Track::NW => match result.facing {
                Point { x: 1, y: 0 } | Point { x: -1, y: 0 } => {
                    result.facing = result.facing.turn(Turn::Right)
                }
                Point { x: 0, y: 1 } | Point { x: 0, y: -1 } => {
                    result.facing = result.facing.turn(Turn::Left)
                }
                _ => unimplemented!(),
            },
            _ => {
                // Doesnt require turning
            }
        }
        result
    }
}

fn build_track<I>(lines: I) -> (Map, Trains)
where
    I: Iterator<Item = String>,
{
    let mut trains = Trains::new();
    let tracks = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    let track = match c {
                        '/' => Track::NE,
                        '\\' => Track::NW,
                        '|' => Track::NS,
                        '-' => Track::EW,
                        '+' => Track::NESW,
                        '>' => {
                            trains.push(Train::new(x, y, Point::new(1, 0)));
                            Track::EW
                        }
                        '<' => {
                            trains.push(Train::new(x, y, Point::new(-1, 0)));
                            Track::EW
                        }
                        '^' => {
                            trains.push(Train::new(x, y, Point::new(0, -1)));
                            Track::NS
                        }
                        'v' => {
                            trains.push(Train::new(x, y, Point::new(0, 1)));
                            Track::NS
                        }
                        ' ' => return None,
                        _ => {
                            println!("Unexpected character in input {}", c);
                            return None;
                        }
                    };
                    Some((Point::new(x, y), track))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    (tracks, trains)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ["/->\\", "^v<   |"];

        let expected = [
            (Point::new(0, 0), Track::NE),
            (Point::new(1, 0), Track::EW),
            (Point::new(2, 0), Track::EW),
            (Point::new(3, 0), Track::NW),
            (Point::new(0, 1), Track::NS),
            (Point::new(1, 1), Track::NS),
            (Point::new(2, 1), Track::EW),
            (Point::new(6, 1), Track::NS),
        ]
        .iter()
        .map(|x| *x)
        .collect::<Map>();

        let expected_trains = [
            Train::new(2, 0, Point::new(1, 0)),
            Train::new(0, 1, Point::new(0, -1)),
            Train::new(1, 1, Point::new(0, 1)),
            Train::new(2, 1, Point::new(-1, 0)),
        ]
        .iter()
        .map(|x| x.clone())
        .collect::<Trains>();

        let (map, trains) = build_track(input.iter().map(|x| x.to_string()));

        assert_eq!(map, expected);
        assert_eq!(trains, expected_trains);
    }

    #[test]
    fn test_part1() {
        let input = [
            "/->-\\        ",
            "|   |  /----\\",
            "| /-+--+-\\  |",
            "| | |  | v  |",
            "\\-+-/  \\-+--/",
            "  \\------/   ",
        ];

        let (map, trains) = build_track(input.iter().map(|x| x.to_string()));

        let (i, result) = part1(map, trains, Some(15)).expect("Failed to find the collision");

        assert_eq!(i, 13);
        assert_eq!(result, Point::new(7, 3));

        panic!();
    }

    #[test]
    fn test_moves() {
        let input = ["|", "v", "|", "|", "|", "^", "|"];

        let (map, trains) = build_track(input.iter().map(|x| x.to_string()));

        let (i, result) = part1(map, trains, Some(15)).expect("Failed to find the collision");

        assert_eq!(i, 1);
    }
}

