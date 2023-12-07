use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("input/day_07.txt");
    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);

    let start = Instant::now();
    let p2 = part2(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p2, duration);
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = Hand::from(parts.next().unwrap());
            let score = parts.next().unwrap().parse().unwrap();
            (hand, score)
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, score))| (i as u64 + 1) * *score as u64)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = Hand::from_part2(parts.next().unwrap());
            let score = parts.next().unwrap().parse().unwrap();
            (hand, score)
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, score))| (i as u64 + 1) * *score as u64)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: Vec<u8>,
    hand_kind: HandKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn kind(hand: &str, use_wild: bool) -> HandKind {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for card in hand.chars() {
        *freq.entry(card).or_insert(0) += 1;
    }
    let wild = *freq.get(&'J').unwrap_or(&0);

    let mut five = false;
    let mut four = false;
    let mut three = false;
    let mut pairs = 0;

    for (c, freq) in freq.iter() {
        if use_wild && c == &'J' {
            continue;
        }
        match freq {
            5 => five = true,
            4 => four = true,
            3 => three = true,
            2 => pairs += 1,
            1 => (),
            _ => panic!("Invalid input"),
        };
    }
    if use_wild {
        match wild {
            5 => five = true,
            4 => five = true,
            3 => match pairs {
                1 => five = true,
                0 => four = true,
                _ => panic!("Invalid input"),
            },
            2 => match (three, pairs) {
                (true, _) => five = true,
                (_, 1) => four = true,
                (_, 0) => three = true,
                _ => panic!("Invalid input"),
            },
            1 => match (four, three, pairs) {
                (true, _, _) => five = true,
                (false, true, _) => four = true,
                (_, _, 2) => {
                    pairs -= 1;
                    three = true;
                }
                (_, _, 1) => {
                    pairs -= 1;
                    three = true;
                }
                (_, _, 0) => pairs = 1,
                _ => panic!("Invalid input"),
            },
            0 => (),
            _ => panic!("Invalid input"),
        }
    };

    match (five, four, three, pairs) {
        (true, _, _, _) => HandKind::FiveOfAKind,
        (_, true, _, _) => HandKind::FourOfAKind,
        (_, _, true, 1) => HandKind::FullHouse,
        (_, _, true, _) => HandKind::ThreeOfAKind,
        (_, _, _, 2) => HandKind::TwoPair,
        (_, _, _, 1) => HandKind::OnePair,
        (_, _, _, 0) => HandKind::HighCard,
        _ => panic!("Invalid input"),
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Hand {
        Hand {
            cards: s
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect(),
            hand_kind: kind(s, false),
        }
    }
}

impl Hand {
    fn from_part2(s: &str) -> Hand {
        Hand {
            cards: s
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect(),
            // TODO: fix kind for part2
            hand_kind: kind(s, true),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if self.hand_kind != other.hand_kind {
                return self.hand_kind.partial_cmp(&other.hand_kind);
            } else if a > b {
                return Some(std::cmp::Ordering::Greater);
            } else if a < b {
                return Some(std::cmp::Ordering::Less);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(input), 6440);
}

#[test]
fn test_hand_cmp() {
    assert!(Hand::from("32T3K").hand_kind == HandKind::OnePair);
    assert!(Hand::from("T55T5").hand_kind == HandKind::FullHouse);
    assert!(Hand::from("KKKK7").hand_kind == HandKind::FourOfAKind);
    assert!(Hand::from("TTTTT").hand_kind == HandKind::FiveOfAKind);

    assert!(Hand::from("32T3K") < Hand::from("T55T5"));
    assert!(Hand::from("T55T5") < Hand::from("KKKK7"));
    assert!(Hand::from("KKKK7") < Hand::from("TTTTT"));
    assert!(Hand::from("TTTTT") < Hand::from("AAAAA"));
    assert!(Hand::from("33AAA") < Hand::from("KKAAA"));

    assert!(Hand::from_part2("J345A").hand_kind == HandKind::OnePair);
    assert!(Hand::from_part2("T55J5").hand_kind == HandKind::FourOfAKind);
}

//249845595 is too high

#[test]
fn test_better_example() {
    let input = include_str!("input/day_07_test.txt");
    assert_eq!(part1(input), 6592);

    assert_eq!(part2(input), 6839);
}
