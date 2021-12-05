use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn dist(&self, other: &Point) -> u32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let x = x.abs() as u32;
        let y = y.abs() as u32;
        x + y
    }

    /// [
    ///     top,
    ///     left,
    ///     right,
    ///     bottom
    /// ]
    pub fn neighbours(&self) -> [Point; 4] {
        let x = self.x;
        let y = self.y;
        [
            Point::new(x, y - 1),
            Point::new(x - 1, y),
            Point::new(x + 1, y),
            Point::new(x, y + 1),
        ]
    }

    pub fn left() -> Self {
        Point::new(-1, 0)
    }

    pub fn right() -> Self {
        Point::new(1, 0)
    }

    pub fn top() -> Self {
        Point::new(0, -1)
    }

    pub fn bottom() -> Self {
        Point::new(0, 1)
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        let lhs = Point::new(0, 0);
        let rhs = Point::new(0, 1);

        assert!(lhs < rhs);

        let lhs = Point::new(0, 0);
        let rhs = Point::new(1, 0);

        assert!(lhs < rhs);

        let lhs = Point::new(0, 0);
        let rhs = Point::new(1, 1);

        assert!(lhs < rhs);
    }
}

