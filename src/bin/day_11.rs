use std::{collections::HashSet, time::Instant};

use aoc::{Map, Point};
use itertools::Itertools;

fn main() {
    let input = include_str!("input/day_11.txt");
    let start = Instant::now();
    let p1 = part1(input.lines().collect());
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);

    let start = Instant::now();
    let p2 = part2(input.lines().collect(), 1000000);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p2, duration);
}

fn part1(input: Img) -> i32 {
    let max_x = input.keys().map(|p| p.x).max().unwrap();
    let max_y = input.keys().map(|p| p.y).max().unwrap();
    let xs: HashSet<i32> = input.keys().map(|p| p.x).collect();
    let ys: HashSet<i32> = input.keys().map(|p| p.y).collect();
    let no_x: Vec<i32> = (0..=max_x).filter(|x| !xs.contains(x)).collect();
    let no_y: Vec<i32> = (0..=max_y).filter(|y| !ys.contains(y)).collect();
    let xlate = Translation { x: no_x, y: no_y };

    input
        .keys()
        .map(|p| xlate.expand(p))
        .collect::<Vec<_>>()
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1 - p2)
        .map(|p| p.magnitude())
        .sum()
}

fn part2(input: Img, scale: i64) -> i64 {
    let max_x = input.keys().map(|p| p.x).max().unwrap();
    let max_y = input.keys().map(|p| p.y).max().unwrap();
    let xs: HashSet<i32> = input.keys().map(|p| p.x).collect();
    let ys: HashSet<i32> = input.keys().map(|p| p.y).collect();
    let no_x: Vec<i32> = (0..=max_x).filter(|x| !xs.contains(x)).collect();
    let no_y: Vec<i32> = (0..=max_y).filter(|y| !ys.contains(y)).collect();
    let xlate = Translation { x: no_x, y: no_y };

    input
        .keys()
        .map(|p| xlate.expand_scale(p, scale))
        .collect::<Vec<_>>()
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1 - p2)
        .map(|p| p.magnitude())
        .sum()

    //702152906986 is too high
}

struct Translation {
    x: Vec<i32>,
    y: Vec<i32>,
}

impl Translation {
    fn expand(&self, p: &Point<i32>) -> Point<i32> {
        let x = p.x + self.x.iter().filter(|x| p.x > **x).count() as i32;
        let y = p.y + self.y.iter().filter(|y| p.y > **y).count() as i32;
        Point { x, y }
    }
    fn expand_scale(&self, p: &Point<i32>, scale: i64) -> Point<i64> {
        let x = p.x as i64 + self.x.iter().filter(|x| p.x > **x).count() as i64 * (scale - 1);
        let y = p.y as i64 + self.y.iter().filter(|y| p.y > **y).count() as i64 * (scale - 1);
        Point { x, y }
    }
}

type Img = Map<Galaxy>;

enum Galaxy {
    Galaxy,
}

impl TryFrom<char> for Galaxy {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Galaxy::Galaxy),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
#[test]
fn test_nothing_day11() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(part1(input.lines().collect()), 374);
    assert_eq!(part2(input.lines().collect(), 10), 1030);
    assert_eq!(part2(input.lines().collect(), 100), 8410);
}
