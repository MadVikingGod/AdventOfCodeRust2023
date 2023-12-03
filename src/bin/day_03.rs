use std::{cmp::min, collections::HashMap};

use regex::Regex;

use aoc::Point;

fn main() {
    let _input = include_str!("input/day_03.txt");

    let lines = _input.split('\n').collect::<Vec<&str>>();

    println!("Result: {}", part1(&lines));
    println!("Result: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let lines = [""]
        .iter()
        .chain(lines.iter())
        .chain([""].iter())
        .collect::<Vec<_>>();
    lines.windows(3).fold(0, |acc, window| {
        let (before, line, after) = (window[0], window[1], window[2]);
        let numbers = re.find_iter(line).collect::<Vec<_>>();
        acc + numbers
            .iter()
            .map(|m| {
                let val = m.as_str().parse::<i32>().unwrap();
                let start = if m.start() == 0 { 0 } else { m.start() - 1 };
                let end = m.end() + 1;
                if find_symbol(line, start, end)
                    || find_symbol(before, start, end)
                    || find_symbol(after, start, end)
                {
                    return val;
                };
                0
            })
            .sum::<i32>()
    })
}

fn part2(lines: &[&str]) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut found: HashMap<Point, i32> = HashMap::new();
    let lines = [""]
        .iter()
        .chain(lines.iter())
        .chain([""].iter())
        .collect::<Vec<_>>();

    lines.windows(3).enumerate().fold(0, |acc, (i, window)| {
        let (before, line, after) = (window[0], window[1], window[2]);
        let numbers = re.find_iter(line).collect::<Vec<_>>();
        acc + numbers
            .iter()
            .filter_map(|m| {
                let val = m.as_str().parse::<i32>().unwrap();
                let start = if m.start() == 0 { 0 } else { m.start() - 1 };
                let end = m.end() + 1;
                if let Some(x) = find_star(line, start, end) {
                    let point = Point { x, y: i };
                    if let Some(v) = found.get(&point) {
                        return Some(*v * val);
                    }
                    found.insert(point, val);
                    return None;
                };
                if let Some(x) = find_star(before, start, end) {
                    let point = Point { x, y: i - 1 };
                    if let Some(v) = found.get(&point) {
                        return Some(*v * val);
                    }
                    found.insert(point, val);
                    return None;
                }
                if let Some(x) = find_star(after, start, end) {
                    let point = Point { x, y: i + 1 };
                    if let Some(v) = found.get(&point) {
                        return Some(*v * val);
                    }
                    found.insert(point, val);
                    return None;
                }
                None
            })
            .sum::<i32>()
    })
}

fn find_symbol(line: &str, start: usize, end: usize) -> bool {
    let index = start;
    let stop = min(end, line.len());
    for i in index..stop {
        let c = line.chars().nth(i).unwrap();
        if c != '.' && !c.is_numeric() {
            return true;
        }
    }
    false
}

fn find_star(line: &str, start: usize, end: usize) -> Option<usize> {
    let index = start;
    let stop = min(end, line.len());
    for i in index..stop {
        let c = line.chars().nth(i).unwrap();
        if c == '*' {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_find_symbol() {
    let line = "...*......";
    assert!(!find_symbol(line, 0, 3));
    assert!(find_symbol(line, 0, 4));
    assert!(find_symbol(line, 3, 4));
    assert!(find_symbol(line, 3, 5));
    assert!(!find_symbol(line, 5, 12));

    let line = ".........#";
    assert!(!find_symbol(line, 0, 3));
    assert!(find_symbol(line, 5, 12));
    assert!(find_symbol(line, 5, 10));
}

#[test]
fn test_part1() {
    let lines = TEST_INPUT.split('\n').collect::<Vec<&str>>();
    assert_eq!(part1(&lines), 4361);
}
