use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

