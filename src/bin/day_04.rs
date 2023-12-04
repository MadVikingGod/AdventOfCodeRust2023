use std::collections::HashSet;

fn main() {
    let _input = include_str!("input/day_04.txt");
    println!(
        "Result: {}",
        part1(&_input.split('\n').collect::<Vec<&str>>())
    );
    println!(
        "Result: {}",
        part2(&_input.split('\n').collect::<Vec<&str>>())
    );
}

fn part1(lines: &[&str]) -> i32 {
    let mut sum = 0;
    for &card in lines {
        sum += score_card(card);
    }
    sum
}

fn part2(lines: &[&str]) -> i32 {
    let mut counts = vec![1; lines.len()];
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i, count_matches(line) as usize))
        .for_each(|(i, matches)| {
            for j in i + 1..i + 1 + matches {
                counts[j] += counts[i];
            }
        });
    counts.iter().sum()
}

fn count_matches(card: &str) -> u32 {
    let card = card[card.find(':').unwrap()..].trim();
    let (winning_numbers, nums) = card.split_once('|').unwrap();
    let winning_numbers: HashSet<&str> = winning_numbers.split_whitespace().collect();
    nums.split_whitespace()
        .filter(|&num| winning_numbers.contains(num))
        .count() as u32
}

fn score_card(card: &str) -> i32 {
    let matches = count_matches(card);
    if matches > 0 {
        return i32::pow(2, matches - 1);
    };
    0
}

#[cfg(test)]

static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[test]
fn test_count_matches() {
    let matches: Vec<u32> = TEST_INPUT.lines().map(|line| count_matches(line)).collect();
    assert_eq!(matches, vec![4, 2, 2, 1, 0, 0])
}
#[test]
fn test_score_card() {
    let scores: Vec<i32> = TEST_INPUT.lines().map(|line| score_card(line)).collect();
    assert_eq!(scores, vec![8, 2, 2, 1, 0, 0])
}

#[test]
fn test_par2() {
    assert_eq!(part2(&TEST_INPUT.split('\n').collect::<Vec<&str>>()), 30);
}
