use std::{collections::HashMap, fmt::Display, time::Instant};

use aoc::Point;

fn main() {
    let input = include_str!("input/day_14.txt");
    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let counts = roll_north(&map);
    let mut total = 0;
    for (p, c) in counts.counts.iter() {
        for y in 0..*c {
            total += map.dim.1 - (p.y as usize + y);
        }
    }
    total
}

fn part2(_input: &str) -> usize {
    0
    // let mut map = parse(input);
    // let mut cache: HashMap<Map, Map> = HashMap::new();
    // for i in 0..1_000_000_000 {
    //     if let Some(m) = cache.get(&map) {
    //         map = m.clone();
    //         break;
    //     } else {
    //         let next = cycle(&map);
    //         cache.insert(map.clone(), next.clone());
    //         map = next;
    //     }
    // }
    // map.map.points.values().filter(|r| **r == Rock::Round).count()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Rock {
    Round,
    Square,
}

impl TryFrom<char> for Rock {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'O' => Ok(Rock::Round),
            '#' => Ok(Rock::Square),
            _ => Err("Empty".to_string()),
        }
    }
}

impl From<Rock> for char {
    fn from(val: Rock) -> Self {
        match val {
            Rock::Round => 'O',
            Rock::Square => '#',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    map: aoc::Map<Rock>,
    dim: (usize, usize),
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Counts {
    counts: aoc::Map<usize>,
    dim: (usize, usize),
    squares: aoc::Map<Rock>,
}

fn parse(input: &str) -> Map {
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let map: aoc::Map<Rock> = input.lines().collect();
    Map { map, dim }
}

fn parse_counts(counts: Counts, dir: Point<i32>) -> Map {
    let mut map = Map {
        map: counts.squares.clone(),
        dim: counts.dim,
    };
    for (p, c) in counts.counts.iter() {
        let mut p = *p;
        for _ in 0..*c {
            map.map.points.insert(p, Rock::Round);
            p = p + dir;
        }
    }
    map
}

fn roll_north(map: &Map) -> Counts {
    roll(map, NORTH)
}
const NORTH: Point<i32> = Point { x: 0, y: -1 };
const SOUTH: Point<i32> = Point { x: 0, y: 1 };
const EAST: Point<i32> = Point { x: 1, y: 0 };
const WEST: Point<i32> = Point { x: -1, y: 0 };

fn roll(map: &Map, dir: Point<i32>) -> Counts {
    let mut counts: aoc::Map<usize> = aoc::Map {
        points: HashMap::new(),
    };
    map.map
        .iter()
        .filter(|(_, r)| **r == Rock::Round)
        .for_each(|(p, _)| {
            let mut pt = *p;
            while is_inbounds(pt, map.dim) {
                pt = pt + dir;
                if map.map.get(&pt).filter(|r| **r == Rock::Square).is_some() {
                    break;
                }
            }
            pt = pt - dir;
            *counts.points.entry(pt).or_insert(0) += 1;
        });
    Counts {
        counts,
        dim: map.dim,
        squares: map
            .map
            .clone()
            .iter()
            .filter_map(|(&p, &r)| {
                if r == Rock::Square {
                    Some((p, r))
                } else {
                    None
                }
            })
            .collect(),
    }
}

fn is_inbounds(pt: Point<i32>, dim: (usize, usize)) -> bool {
    pt.x >= 0 && pt.y >= 0 && pt.x < dim.0 as i32 && pt.y < dim.1 as i32
}

fn cycle(map: &Map) -> Map {
    let counts = roll_north(map);
    let mut map = parse_counts(counts, SOUTH);
    map = parse_counts(roll(&map, WEST), EAST);
    map = parse_counts(roll(&map, SOUTH), NORTH);
    parse_counts(roll(&map, EAST), WEST)
}

#[cfg(test)]
#[test]
fn test_nothing_day14() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut map = parse(input);
    assert_eq!(map.dim, (10, 10));
    assert_eq!(map.map.points.len(), 35);

    // println!("{:?}", roll_north(&map));
    assert_eq!(part1(input), 136);

    assert_eq!(roll(&map, NORTH), roll_north(&map));

    map = cycle(&map);
    let want: Map = parse(
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
    );
    assert_eq!(map, want);

    let start = Instant::now();
    for _ in 0..10_000 {
        map = cycle(&map);
    }
    let duration = start.elapsed();

    println!("{:?}", duration)
}

#[test]
fn test_part2() {
    let _input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    // assert_eq!(part2(input), 64);
}
