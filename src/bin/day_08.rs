use std::{collections::HashMap, time::Instant};

use regex::Regex;

fn main() {
    let input = include_str!("input/day_08.txt");
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
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let map = lines.skip(1).collect::<Map>();

    find_steps(&map, instructions, "AAA", |n| n == "ZZZ")
}

fn find_steps<P>(map: &Map, instructions: &str, start: &str, end: P) -> usize
where
    P: Fn(&str) -> bool,
{
    let mut instructions = instructions.chars().cycle();
    let mut count = 0;
    let mut next = start;

    while !end(next) {
        next = match instructions.next().unwrap() {
            'L' => map.left(next),
            'R' => map.right(next),
            _ => unreachable!(),
        };
        count += 1;
    }
    count
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let map = lines.skip(1).collect::<Map>();

    let start: Vec<&str> = map
        .nodes
        .keys()
        .filter(|&n| n.chars().nth(2).unwrap() == 'A')
        .cloned()
        .collect();
    start
        .iter()
        .map(|&strat| {
            find_steps(&map, instructions, strat, |n| {
                n.chars().nth(2).unwrap() == 'Z'
            })
        })
        .reduce(num::integer::lcm)
        .unwrap()
}

struct Map<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> FromIterator<&'a str> for Map<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();
        let mut nodes = HashMap::new();
        for line in iter {
            let (_, [name, left, right]) = re.captures(line).unwrap().extract();
            nodes.insert(name, Node { left, right });
        }
        Map { nodes }
    }
}

impl Map<'_> {
    fn left(&self, next: &str) -> &str {
        self.nodes.get(next).unwrap().left
    }
    fn right(&self, next: &str) -> &str {
        self.nodes.get(next).unwrap().right
    }
}

#[cfg(test)]
#[test]
fn test_par1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part1(input), 2);

    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    // part2(input);
    assert_eq!(part2(input), 6);
}
