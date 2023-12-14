use std::{cmp::min, time::Instant};

fn main() {
    let input = include_str!("input/day_13.txt");
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
    let map: Vec<Map> = input.split("\n\n").map(parse).collect();

    map.iter()
        .map(find_index)
        .map(|i| match i {
            Index::H(i) => i,
            Index::V(i) => i * 100,
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let map: Vec<Map> = input.split("\n\n").map(parse).collect();

    map.iter()
        .map(find_differences)
        .map(|i| match i {
            Index::H(i) => i,
            Index::V(i) => i * 100,
        })
        .sum()
}

fn find_index(map: &Map) -> Index {
    for i in 1..map.len() {
        let (top, bottom) = split_vertically(map, i);
        let top = flip_vertically(top);
        if eq(&top, &bottom) {
            return Index::V(i);
        }
    }
    for i in 1..map[0].len() {
        let (left, right) = split_horizontally(map, i);
        let left = flip_horizontally(left);
        if eq(&left, &right) {
            return Index::H(i);
        }
    }
    panic!("No index found");
}

fn find_differences(map: &Map) -> Index {
    for i in 1..map.len() {
        let (top, bottom) = split_vertically(map, i);
        let top = flip_vertically(top);
        if differences(&top, &bottom) == 1 {
            return Index::V(i);
        }
    }
    for i in 1..map[0].len() {
        let (left, right) = split_horizontally(map, i);
        let left = flip_horizontally(left);
        if differences(&left, &right) == 1 {
            return Index::H(i);
        }
    }
    panic!("No index found");
}

#[derive(Debug, PartialEq, Eq)]
enum Index {
    H(usize),
    V(usize),
}

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn split_vertically(map: &Map, idx: usize) -> (Map, Map) {
    let (top, bottom) = map.split_at(idx);
    (top.to_vec(), bottom.to_vec())
}

fn split_horizontally(map: &Map, idx: usize) -> (Map, Map) {
    map.iter()
        .map(|row| row.split_at(idx))
        .map(|(l, r)| (l.to_vec(), r.to_vec()))
        .unzip()
}

fn flip_vertically(map: Map) -> Map {
    map.iter().rev().cloned().collect()
}
fn flip_horizontally(map: Map) -> Map {
    map.iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

fn eq(map1: &Map, map2: &Map) -> bool {
    let min_y = min(map1.len(), map2.len());
    let min_x = min(map1[0].len(), map2[0].len());

    map1.iter()
        .take(min_y)
        .zip(map2.iter().take(min_y))
        .all(|(row1, row2)| row1[0..min_x] == row2[0..min_x])
}

fn differences(map1: &Map, map2: &Map) -> usize {
    let min_y = min(map1.len(), map2.len());
    let min_x = min(map1[0].len(), map2[0].len());

    map1.iter()
        .take(min_y)
        .zip(map2.iter().take(min_y))
        .map(|(row1, row2)| {
            row1[0..min_x]
                .iter()
                .zip(row2[0..min_x].iter())
                .filter(|(c1, c2)| c1 != c2)
                .count()
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_nothing_day13() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    let input = parse(input);

    let (left, right) = split_horizontally(&input, 5);
    let left = flip_horizontally(left);
    assert!(eq(&left, &right));
    assert_eq!(find_index(&input), Index::H(5));

    let (left, right) = split_vertically(&input, 3);
    let left = flip_vertically(left);
    assert_eq!(differences(&left, &right), 1);

    let input = parse(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    );

    let (left, right) = split_vertically(&input, 4);
    let left = flip_vertically(left);
    assert!(eq(&left, &right));
    assert_eq!(find_index(&input), Index::V(4));
}

#[test]
fn test_flip() {
    let input = parse(
        "...
.##
.#.",
    );

    let horiz = parse(
        "...
##.
.#.",
    );

    let vert = parse(
        ".#.
.##
...",
    );

    assert_eq!(flip_horizontally(input.clone()), horiz);
    assert_eq!(flip_vertically(input.clone()), vert);
}

#[test]
fn test_part1() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    assert_eq!(part1(input), 405);
    assert_eq!(part2(input), 400);
}
