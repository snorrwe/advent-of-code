use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entry(&self.x).entry(&self.y).finish()
    }
}

impl IVec2 {
    pub const ZERO: IVec2 = IVec2 { x: 0, y: 0 };
    pub const ONE: IVec2 = IVec2 { x: 1, y: 1 };
    pub const X: IVec2 = IVec2 { x: 1, y: 0 };
    pub const Y: IVec2 = IVec2 { x: 0, y: 1 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// not really normalized, but reduces all dims to `[-1‥1]`
    pub fn as_direction(self) -> Self {
        let x = if self.x != 0 {
            self.x / self.x.abs()
        } else {
            0
        };
        let y = if self.y != 0 {
            self.y / self.y.abs()
        } else {
            0
        };
        Self::new(x, y)
    }

    pub fn len_sq(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    /// Chebyshev distance from self to the other
    pub fn chebyshev(self, rhs: IVec2) -> i32 {
        (self.x - rhs.x).abs().max((self.y - rhs.y).abs())
    }

    pub fn manhatten(self, rhs: IVec2) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    pub const fn splat(val: i32) -> Self {
        Self::new(val, val)
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl AddAssign for IVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl SubAssign for IVec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Index<usize> for IVec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            _ => &self.y,
        }
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl IndexMut<usize> for IVec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            _ => &mut self.y,
        }
    }
}

impl Neg for IVec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub fn read_input()->String {
    std::fs::read_to_string("input.txt").unwrap()
}
