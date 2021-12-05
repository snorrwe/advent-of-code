use super::turn::Turn;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, Copy)]
pub struct Point {
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

