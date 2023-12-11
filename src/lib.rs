use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Sub},
};

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

pub struct Map<P> {
    pub points: HashMap<Point<i32>, P>,
}

impl<'a, P, Err> FromIterator<&'a str> for Map<P>
where
    P: TryFrom<char, Error = Err>,
{
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut points = HashMap::new();
        for (y, line) in iter.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if let Ok(p) = P::try_from(c) {
                    points.insert(point, p);
                }
            }
        }
        Map::<P> { points }
    }
}

impl<P> Map<P> {
    pub fn get(&self, point: &Point<i32>) -> Option<&P> {
        self.points.get(point)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&Point<i32>, &P)> {
        self.points.iter()
    }
    pub fn keys(&self) -> impl Iterator<Item = &Point<i32>> {
        self.points.keys()
    }
    pub fn values(&self) -> impl Iterator<Item = &P> {
        self.points.values()
    }
}

impl<P> Display for Map<P>
where
    P: Into<char> + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.keys().map(|p| p.x).min().unwrap();
        let max_x = self.keys().map(|p| p.x).max().unwrap();
        let min_y = self.keys().map(|p| p.y).min().unwrap();
        let max_y = self.keys().map(|p| p.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = Point { x, y };
                let pt = self.get(&point);
                let c = if let Some(pt) = pt {
                    Into::<char>::into(*pt)
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_abs() {
    let p1 = Point::<i32> { x: 5, y: 6 };
    assert_eq!(p1.magnitude(), 11);
}
