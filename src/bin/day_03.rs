use std::{cmp::min, collections::HashMap};

use regex::Regex;

use aoc::Point;

static test_input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let _input = include_str!("input/day_03.txt");

    let lines = _input.split("\n").collect::<Vec<&str>>();

    println!("Result: {}", part1(&lines));
    println!("Result: {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    lines
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            let before = if i == 0 { None } else { lines.get(i - 1) };
            let after = lines.get(i + 1);
            (line, before, after)
        })
        .fold(0, |acc, (line, before, after)| {
            let numbers = re.find_iter(line).collect::<Vec<_>>();
            acc + numbers
                .iter()
                .map(|m| {
                    let val = m.as_str().parse::<i32>().unwrap();
                    let start = if m.start() == 0 { 0 } else { m.start() - 1 };
                    if find_symbol(line, start, m.end() + 1) {
                        return val;
                    };
                    if let Some(b) = before {
                        if find_symbol(b, start, m.end() + 1) {
                            return val;
                        }
                    }
                    if let Some(a) = after {
                        if find_symbol(a, start, m.end() + 1) {
                            return val;
                        }
                    }

                    0
                })
                .sum::<i32>()
        })
}

fn part2(lines: &Vec<&str>) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut found: HashMap<Point, i32> = HashMap::new();
    lines
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            let before = if i == 0 { None } else { lines.get(i - 1) };
            let after = lines.get(i + 1);
            (i, line, before, after)
        })
        .fold(0, |acc, (i, line, before, after)| {
            let numbers = re.find_iter(line).collect::<Vec<_>>();
            acc + numbers
            .iter()
            .filter_map(|m| {
                let val = m.as_str().parse::<i32>().unwrap();
                let start = if m.start() == 0 { 0 } else { m.start() - 1 };
                let end = m.end() + 1;
                if let Some(x) = find_star(line, start, end) {
                    let point = Point {
                        x: x,
                        y: i,
                    };
                    if let Some(v) = found.get(&point) {
                        return Some(*v * val);
                    }
                    found.insert(point, val);
                    return None;
                };
                if let Some(b) = before {
                    if let Some(x) = find_star(b, start, end) {
                        let point = Point {
                            x: x,
                            y: i-1,
                        };
                        if let Some(v) = found.get(&point) {
                            return Some(*v * val);
                        }
                        found.insert(point, val);
                        return None;
                    }
                }
                if let Some(a) = after {
                    if let Some(x) = find_star(a, start, end) {
                        let point = Point {
                            x: x,
                            y: i+1,
                        };
                        if let Some(v) = found.get(&point) {
                            return Some(*v * val);
                        }
                        found.insert(point, val);
                        return None;
                    }
                }
                None
            })
            .sum::<i32>()
        })
}

fn find_symbol(line: &str, start: usize, end: usize) -> bool {
    let mut index = start;
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
    let mut index = start;
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
#[test]
fn test_find_symbol() {
    let line = "...*......";
    assert_eq!(find_symbol(line, 0, 3), false);
    assert_eq!(find_symbol(line, 0, 4), true);
    assert_eq!(find_symbol(line, 3, 4), true);
    assert_eq!(find_symbol(line, 3, 5), true);
    assert_eq!(find_symbol(line, 5, 12), false);

    let line = ".........#";
    assert_eq!(find_symbol(line, 0, 3), false);
    assert_eq!(find_symbol(line, 5, 12), true);
    assert_eq!(find_symbol(line, 5, 10), true);
}
