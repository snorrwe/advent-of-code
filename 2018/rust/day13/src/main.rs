use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

mod point;
mod train;
mod turn;

use self::point::Point;
use self::train::Train;

pub type Map = HashMap<Point, Track>;
pub type Trains = Vec<Train>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Track {
    NS,
    EW,
    NE,
    NW,
    NESW,
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let (map, trains) = build_track(lines);

    let result = part1(map.clone(), trains.clone(), None);
    println!("Part1: {:?}", result);
    let result = part2(map, trains, None);
    println!("Part2: {:?}", result);
    Ok(())
}

fn part1(map: Map, mut trains: Trains, max_ticks: Option<usize>) -> Option<(usize, Point)> {
    for i in 0..max_ticks.unwrap_or(2000) {
        let collision = tick(&map, &mut trains, false);
        if let Some(collision) = collision {
            return Some((i + 1, collision));
        }
    }
    None
}

fn part2(map: Map, mut trains: Trains, max_ticks: Option<usize>) -> Option<(usize, Point)> {
    for i in 0..max_ticks.unwrap_or(1_000_000) {
        tick(&map, &mut trains, true);
        debug_assert!(
            trains.len() % 2 == 1,
            format!(
                "Length of trains must be an odd number, got: {}",
                trains.len()
            )
        );
        if trains.len() == 1 {
            return Some((i + 1, trains[0].point));
        } else if trains.is_empty() {
            println!("All the trains were destroyed at tick {}", i);
            return None;
        }
    }
    None
}

/// Returns the position of the collision if any
/// Updates the trains
fn tick(map: &Map, trains: &mut Trains, _remove_colliding: bool) -> Option<Point> {
    let mut points = trains
        .iter()
        .map(|t| (t.point, t.clone()))
        .collect::<BTreeMap<_, _>>();
    trains.sort_unstable_by_key(|train| train.point);
    let mut result = None;
    let mut collisions = BTreeSet::new();
    trains.iter().for_each(|train| {
        if collisions.contains(&train.point) {
            return;
        }
        points.remove(&train.point);
        let train = train.tick(&map);
        let colliding = points.contains_key(&train.point);
        if colliding {
            // Remove the existing train to destroy both
            // But let later trains occupy the position
            points.remove(&train.point);
            result = Some(train.point);
            collisions.insert(train.point);
        } else {
            points.insert(train.point, train.clone());
        }
    });
    *trains = points.values().map(|t| t.clone()).collect();
    result
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

        assert_eq!(i, 14);
        assert_eq!(result, Point::new(7, 3));
    }

    #[test]
    fn test_part2() {
        let input = [
            "/>-<\\  ", "|   |  ", "| /<+-\\", "| | | v", "\\>+</ |", "  |   ^", "  \\<->/",
        ];

        let (map, trains) = build_track(input.iter().map(|x| x.to_string()));

        let (i, result) = part2(map, trains, Some(8)).expect("Failed to find the collision");

        assert_eq!(i, 3);
        assert_eq!(result, Point::new(6, 4));
    }

    #[test]
    fn test_moves() {
        let input = ["|", "v", "|", "|", "|", "^", "|"];

        let (map, trains) = build_track(input.iter().map(|x| x.to_string()));

        let (i, _result) = part1(map, trains, Some(15)).expect("Failed to find the collision");

        assert_eq!(i, 2);
    }
}

