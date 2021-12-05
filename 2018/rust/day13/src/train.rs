use super::point::Point;
use super::turn::Turn;
use super::{Map, Track};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, Ord)]
pub struct Train {
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

