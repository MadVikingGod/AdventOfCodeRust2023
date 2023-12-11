use aoc::{Map, Point};
use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("input/day_10.txt");
    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);

    let start = Instant::now();
    let p2 = part2(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p2, duration);
}

fn part1(input: &str) -> usize {
    let mut map: Map<Pipe> = input.lines().collect();
    let (start, kind) = find_and_replace_start(&mut map);

    let mut prev = match kind {
        Pipe::Vertical => (start + NORTH, start + SOUTH),
        Pipe::Horizontal => (start + EAST, start + WEST),
        Pipe::NorthEast => (start + NORTH, start + EAST),
        Pipe::NorthWest => (start + NORTH, start + WEST),
        Pipe::SouthEast => (start + SOUTH, start + EAST),
        Pipe::SouthWest => (start + SOUTH, start + WEST),
        _ => unimplemented!(),
    };

    let mut current = (start, start);
    let mut count = 0;
    let mut seen: HashMap<Point<i32>, usize> = HashMap::new();
    loop {
        let next = (
            next(&map, &prev.0, &current.0),
            next(&map, &prev.1, &current.1),
        );
        if seen.contains_key(&next.0) || seen.contains_key(&next.1) {
            break;
        }
        seen.insert(next.0, count);
        seen.insert(next.1, count);
        prev = current;
        current = next;
        count += 1;
    }
    count
}

fn part2(input: &str) -> usize {
    let mut map: Map<Pipe> = input.lines().collect();
    let (start, kind) = find_and_replace_start(&mut map);

    let map = get_loop(&map, start, kind);
    println!("{}", map);

    0
}

// struct Map {
//     pipes: HashMap<Point<i32>, Pipe>,
// }

// impl<'a> FromIterator<&'a str> for Map {
//     fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
//         let mut pipes = HashMap::new();
//         for (y, line) in iter.into_iter().enumerate() {
//             for (x, c) in line.chars().enumerate() {
//                 let point = Point {
//                     x: x as i32,
//                     y: y as i32,
//                 };
//                 if let Ok(pipe) = Pipe::try_from(c) {
//                     pipes.insert(point, pipe);
//                 }
//             }
//         }
//         Map { pipes }
//     }
// }

fn find_start(map: &Map<Pipe>) -> Point<i32> {
    *map.iter()
        .find(|(_, pipe)| **pipe == Pipe::Start)
        .unwrap()
        .0
}

// TODO: fill out other directions
fn find_start_kind(map: &Map<Pipe>, p: Point<i32>) -> Pipe {
    let north = map.get(&(p + NORTH));
    let south = map.get(&(p + SOUTH));
    let east = map.get(&(p + EAST));
    let west = map.get(&(p + WEST));

    match (north, south, east, west) {
        (_, Some(Pipe::Vertical), Some(Pipe::Horizontal), _) => Pipe::SouthEast,
        (_, Some(Pipe::Vertical), Some(Pipe::NorthWest), _) => Pipe::SouthEast,
        (Some(Pipe::SouthEast), Some(Pipe::NorthEast), _, _) => Pipe::Vertical,
        _ => unimplemented!(),
    }
}

fn find_and_replace_start(map: &mut Map<Pipe>) -> (Point<i32>, Pipe) {
    let start = find_start(map);
    let kind = find_start_kind(map, start);
    map.points.entry(start).and_modify(|p| *p = kind);
    (start, kind)
}

fn next(map: &Map<Pipe>, prev: &Point<i32>, current: &Point<i32>) -> Point<i32> {
    let pipe = map.get(current).unwrap();
    let direction = *current - *prev;
    match (pipe, direction) {
        (Pipe::Vertical, NORTH) => *current + NORTH,
        (Pipe::Vertical, SOUTH) => *current + SOUTH,
        (Pipe::Horizontal, EAST) => *current + EAST,
        (Pipe::Horizontal, WEST) => *current + WEST,
        (Pipe::NorthEast, SOUTH) => *current + EAST,
        (Pipe::NorthEast, WEST) => *current + NORTH,
        (Pipe::NorthWest, SOUTH) => *current + WEST,
        (Pipe::NorthWest, EAST) => *current + NORTH,
        (Pipe::SouthEast, NORTH) => *current + EAST,
        (Pipe::SouthEast, WEST) => *current + SOUTH,
        (Pipe::SouthWest, NORTH) => *current + WEST,
        (Pipe::SouthWest, EAST) => *current + SOUTH,
        _ => unimplemented!(),
    }
}

fn get_loop(map: &Map<Pipe>, start: Point<i32>, kind: Pipe) -> Map<Pipe> {
    let mut prev = match kind {
        Pipe::Vertical => (start + NORTH, start + SOUTH),
        Pipe::Horizontal => (start + EAST, start + WEST),
        Pipe::NorthEast => (start + NORTH, start + EAST),
        Pipe::NorthWest => (start + NORTH, start + WEST),
        Pipe::SouthEast => (start + SOUTH, start + EAST),
        Pipe::SouthWest => (start + SOUTH, start + WEST),
        _ => unimplemented!(),
    };
    let mut current = (start, start);

    let mut pipes: HashMap<Point<i32>, Pipe> = HashMap::new();
    loop {
        pipes.insert(current.0, *map.get(&current.0).unwrap());
        pipes.insert(current.1, *map.get(&current.1).unwrap());

        let next = (
            next(map, &prev.0, &current.0),
            next(map, &prev.1, &current.1),
        );
        if pipes.contains_key(&next.0) || pipes.contains_key(&next.1) {
            pipes.insert(next.0, *map.get(&next.0).unwrap());
            pipes.insert(next.1, *map.get(&next.1).unwrap());
            break;
        }
        prev = current;
        current = next;
    }
    Map { points: pipes }
}

const NORTH: Point<i32> = Point { x: 0, y: -1 };
const SOUTH: Point<i32> = Point { x: 0, y: 1 };
const EAST: Point<i32> = Point { x: 1, y: 0 };
const WEST: Point<i32> = Point { x: -1, y: 0 };

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            'S' => Ok(Pipe::Start),
            _ => Err(()),
        }
    }
}

impl From<Pipe> for char {
    fn from(p: Pipe) -> char {
        match p {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthWest => '7',
            Pipe::SouthEast => 'F',
            Pipe::Start => 'S',
        }
    }
}

#[cfg(test)]
#[test]
fn test_direction() {
    let p1 = Point { x: 5, y: 6 };
    assert_eq!(p1 - Point { x: 5, y: 7 }, NORTH);
}

#[test]
fn test_part1() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(part1(input), 4);
}
#[test]
fn test_part1_complex() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part1(input), 8);
}

#[test]
fn test_part2() {
    let _input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    // assert_eq!(part2(input), 4);
}
