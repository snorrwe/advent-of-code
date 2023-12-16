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

pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

pub fn walk_square(from: IVec2, to: IVec2) -> impl Iterator<Item = IVec2> {
    debug_assert!(from.x <= to.x);
    debug_assert!(from.y <= to.y);

    let fx = from.x;
    let tx = to.x;
    (from.y..=to.y).flat_map(move |y| (fx..=tx).map(move |x| IVec2::new(x, y)))
}

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Eq for Grid<T> where T: Eq {}

impl<T> std::hash::Hash for Grid<T>
where
    T: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.data.hash(state);
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.data == other.data
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.width, self.height)?;
        for row in self.rows() {
            for i in row {
                write!(f, "{i:?}\t")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.width, self.height)?;
        for row in self.rows() {
            for i in row {
                write!(f, "{i}\t")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_data(data: Vec<T>, width: usize) -> Self {
        assert_eq!(data.len() % width, 0);
        let height = data.len() / width;
        Self {
            data,
            width,
            height,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, item: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x] = item;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[y * self.width + x]
    }

    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.height);
        &self.data[y * self.width..(y + 1) * self.width]
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        assert!(y < self.height);
        &mut self.data[y * self.width..(y + 1) * self.width]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|y| self.row(y))
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
    }

    pub fn col(&self, x: usize) -> impl Iterator<Item = &T> {
        assert!(x < self.width);
        (0..self.height).map(move |y| &self.row(y)[x])
    }
}

impl<T> Index<IVec2> for Grid<T> {
    type Output = T;
    fn index(&self, index: IVec2) -> &Self::Output {
        assert!(index.x >= 0);
        assert!(index.y >= 0);
        &self.row(index.y as usize)[index.x as usize]
    }
}

impl<T> IndexMut<IVec2> for Grid<T> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        assert!(index.x >= 0);
        assert!(index.y >= 0);
        &mut self.row_mut(index.y as usize)[index.x as usize]
    }
}
