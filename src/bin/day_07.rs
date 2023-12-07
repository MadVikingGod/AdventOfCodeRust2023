use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("input/day_07.txt");
    let start = Instant::now();
    let p1 = part1(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p1, duration);
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
    hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
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

fn kind(hand: &str) -> HandKind {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for card in hand.chars() {
        *freq.entry(card).or_insert(0) += 1;
    }

    let mut five = false;
    let mut four = false;
    let mut three = false;
    let mut pairs = 0;

    for freq in freq.values() {
        if freq == &5 {
            five = true;
        } else if freq == &4 {
            four = true;
        } else if freq == &3 {
            three = true;
        } else if freq == &2 {
            pairs += 1;
        }
    }
    if five {
        HandKind::FiveOfAKind
    } else if four {
        HandKind::FourOfAKind
    } else if three && pairs == 1 {
        HandKind::FullHouse
    } else if three {
        HandKind::ThreeOfAKind
    } else if pairs == 2 {
        HandKind::TwoPair
    } else if pairs == 1 {
        HandKind::OnePair
    } else {
        HandKind::HighCard
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Hand {
        Hand {
            cards: s
                .chars()
                .map(|c| match c {
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    'A' => 1,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect(),
            hand_kind: kind(s),
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

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            match card {
                13 => write!(f, "K"),
                12 => write!(f, "Q"),
                11 => write!(f, "J"),
                10 => write!(f, "T"),
                1 => write!(f, "A"),
                _ => write!(f, "{}", card),
            }?;
        }
        Ok(())
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
    assert!(Hand::from("TTTTT") > Hand::from("AAAAA"));
    assert!(Hand::from("33AAA") < Hand::from("KKAAA"));
}

//249845595 is too high