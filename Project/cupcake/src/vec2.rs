use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn usize_x(&self) -> usize {
        self.x as usize
    }
    pub fn usize_y(&self) -> usize {
        self.y as usize
    }
    pub fn ZERO() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self > other {
            return Some(Ordering::Greater);
        }
        if self < other {
            return Some(Ordering::Less);
        }
        Some(Ordering::Equal)
    }

    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y
    }
    fn ge(&self, other: &Self) -> bool {
        self.x >= other.x && self.y >= other.y
    }
    fn lt(&self, other: &Self) -> bool {
        self.x < other.x && self.y < other.y
    }
    fn le(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y
    }
}
