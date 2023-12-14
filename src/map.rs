use std::{collections::HashMap, fmt::Display};

use crate::Point;

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl<P> FromIterator<(Point<i32>, P)> for Map<P> {
    fn from_iter<I: IntoIterator<Item = (Point<i32>, P)>>(iter: I) -> Self {
        Self {
            points: HashMap::from_iter(iter),
        }
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
