use std::time::Instant;

use regex::Regex;

fn main() {
    let input = include_str!("input/day_12.txt");
    println!("Hello, world!");

    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);
}

fn part1(input: &str) -> usize {
    input.lines().map(count).sum()
}

fn expand(input: &str) -> Vec<String> {
    let mut current: Vec<String> = vec![input.to_string()];
    let mut next: Vec<String> = Vec::new();

    for c in input.chars() {
        if c == '?' {
            for line in current.iter() {
                next.push(line.replacen('?', ".", 1));
                next.push(line.replacen('?', "#", 1));
            }
            (current, next) = (next, current);
            next.clear();
        }
    }
    current
}

fn check(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    let counts = parts
        .next()
        .unwrap()
        .split('.')
        .filter(|&part| !part.is_empty())
        .map(|part| part.len())
        .collect::<Vec<_>>();
    let vals = parts
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    counts == vals
}

fn count(input: &str) -> usize {
    expand(input).iter().filter(|&line| check(line)).count()
}

#[cfg(test)]
#[test]
fn test_check() {
    let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    for line in input.lines() {
        assert!(check(line))
    }
}

#[test]
fn test_expand() {
    let input = "?###???????? 3,2,1";
    let out = expand(input);

    assert_eq!(out.len(), 512);
    assert!(out.iter().all(|line| !line.contains("?")));
}

#[test]
fn test_count() {
    let test_cases = vec![
        ("???.### 1,1,3", 1),
        (".??..??...?##. 1,1,3", 4),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("?###???????? 3,2,1", 10),
    ];

    for (input, want) in test_cases {
        assert_eq!(count(input), want)
    }
}
#[test]
fn test_part1() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(part1(input), 21)
}

#[test]
fn test_regex() {
    let input = "?###????????";
    let regx = Regex::new(r"[\?\.]+[\?#]{3}[\?\.]+[\?#]{2}[\?\.]+[\?#]{1}[\?\.]+").unwrap();

    let _count = regx.find_iter(input).count();
    // assert_eq!(count, 10)
    //This doesn't work
}
