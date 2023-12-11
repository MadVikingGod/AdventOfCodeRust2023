use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = include_str!("input/day_09.txt");
    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);

    let start = Instant::now();
    let p2 = part2(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p2, duration);
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let end = numbers.len() - 1;
            let mut ders: Vec<Vec<i64>> = Vec::new();
            let mut current = numbers;
            for _ in 0..end {
                ders.push(current.clone());
                current = derivative(&current);
            }

            ders.iter()
                .rev()
                .fold(0, |acc, current| acc + current.last().unwrap())
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let end = numbers.len() - 1;
            let mut ders: Vec<Vec<i64>> = Vec::new();
            let mut current = numbers;
            for _ in 0..end {
                ders.push(current.clone());
                current = derivative(&current);
            }

            ders.iter()
                .rev()
                .fold(0, |acc, current| current.first().unwrap() - acc)
        })
        .sum()
}

fn derivative(input: &[i64]) -> Vec<i64> {
    input.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

#[cfg(test)]
#[test]
fn test_nothing_day09() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(part1(input), 114);
    assert_eq!(part2(input), 2)
}
