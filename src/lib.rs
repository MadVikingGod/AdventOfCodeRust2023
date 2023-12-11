use std::ops::{Add, Sub};

mod map;

pub use map::Map;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: num::Signed,
{
    pub fn magnitude(&self) -> T {
        self.x.abs() + self.y.abs()
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Point<T> {
        Point { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Add for &Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, other: &Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T> Sub for &Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, other: &Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
#[test]
fn test_abs() {
    let p1 = Point::<i32> { x: 5, y: 6 };
    assert_eq!(p1.magnitude(), 11);
}
