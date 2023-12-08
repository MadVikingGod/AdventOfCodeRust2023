use std::{collections::{HashMap, HashSet}, time::Instant};

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
    let mut instructions = lines.next().unwrap().chars().cycle();
    let map = lines.skip(1).collect::<Map>();

    let mut count = 0;
    let mut next = "AAA";
    while next != "ZZZ" {
        next = match instructions.next().unwrap() {
            'L' => map.left(next),
            'R' => map.right(next),
            _ => unreachable!(),
        };
        count += 1;   
    };
    count
}
fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    // let mut instructions = lines.next().unwrap().chars().cycle();
    let instructions = lines.next().unwrap();
    let map = lines.skip(1).collect::<Map>();
    let loops = map.nodes.keys().filter(|&n| n.chars().nth(2).unwrap() == 'A').cloned().map(|start| find_loop(instructions, &map, start)).collect::<Vec<_>>();

    println!("{:?}", loops);

    // find all stopping points
    // find the LCM of all stopping points

    // let mut count = 0;
    // let mut next: Vec<&str> = map.nodes.keys().filter(|&n| n.chars().nth(2).unwrap() == 'A').cloned().collect();
    // while !next.iter().all(|n| n.chars().nth(2).unwrap() == 'Z') {
    //     next = match instructions.next().unwrap() {
    //         'L' => next.iter().map(|n| map.left(n)).collect(),
    //         'R' => next.iter().map(|n| map.right(n)).collect(),
    //         _ => unreachable!(),
    //     };
    //     count += 1;   
    // };
    // count
    0
}

fn find_loop(instructsion: &str, map: &Map, start: &str) -> (usize, usize) {
    let mut instructions = instructsion.chars().enumerate().cycle();
    let mut count = 0;
    let mut seen: HashMap<(usize, char, &str), usize> = HashMap::new();
    let mut next = start;
    for (i, c) in instructions {
        
        if let Some(seen_count) = seen.get(&(i,c,next)) {
            return (*seen_count, count - seen_count);
        }
        seen.insert((i, c, next), count);

        next =  match c {
            'L' => map.left(next),
            'R' => map.right(next),
            _ => unreachable!(),
        };
        count += 1;
    }
    unreachable!()
}


struct Map<'a> {
    nodes: HashMap<&'a str, Node<'a>>
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> FromIterator<&'a str> for Map<'a> {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
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
    
        assert_eq!(part2(input), 6);
    }