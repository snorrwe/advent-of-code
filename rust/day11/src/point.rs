use std::ops::{Add, Div, Mul, Sub};

pub enum Direction {
    Clockwise,
    CounterClock,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
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

    pub fn rotated_90(&self, direction: Direction) -> Point {
        let x = self.x;
        let y = self.y;
        match direction {
            Direction::CounterClock => Point { x: y, y: -x },
            Direction::Clockwise => Point { x: -y, y: x },
        }
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
