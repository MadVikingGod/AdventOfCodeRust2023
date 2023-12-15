use std::{fmt::Display, time::Instant};

fn main() {
    let input = include_str!("input/day_15.txt");
    let h: u64 = input.split(',').map(hash).sum();
    println!("Result: {}", h);

    let start = Instant::now();
    let p2 = part2(input);
    let duration = start.elapsed();
    println!("Result: {}\t\t {:?}", p2, duration);
}

fn part2(input: &str) -> u64 {
    let mut hm = HM { boxes: Vec::new() };
    hm.boxes.resize(256, Vec::new());
    for instruction in input.split(',') {
        if let Some(id) = instruction.strip_suffix('-') {
            hm.remove(id.to_string());
        } else {
            let id = &instruction[..instruction.len() - 2];
            let value = instruction[instruction.len() - 1..].parse().unwrap();
            hm.insert(id, value);
        }
    }
    hm.boxes
        .iter()
        .enumerate()
        .flat_map(|(bn, b)| {
            b.iter()
                .enumerate()
                .map(move |(sn, b)| (bn as u64 + 1) * (sn as u64 + 1) * b.value)
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Box<'a> {
    id: &'a str,
    value: u64,
}

struct HM<'a> {
    boxes: Vec<Vec<Box<'a>>>,
}

impl Display for HM<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, bucket) in self.boxes.iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }
            write!(f, "Box {}: ", i)?;
            for b in bucket {
                write!(f, "[{} {}] ", b.id, b.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> HM<'a> {
    fn insert(&mut self, id: &'a str, value: u64) {
        let index = hash(id) as usize;
        let bucket = &mut self.boxes[index];

        if let Some(idx) =
            bucket
                .iter()
                .enumerate()
                .find_map(|(i, b)| if b.id == id { Some(i) } else { None })
        {
            bucket[idx].value = value;
        } else {
            let b = Box { id, value };
            bucket.push(b);
        };
    }
    fn remove(&mut self, id: String) {
        let index = hash(id.as_str()) as usize;
        let bucket = &mut self.boxes[index];
        for i in 0..bucket.len() {
            if bucket[i].id == id {
                bucket.remove(i);
                return;
            }
        }
    }
}

fn hash(input: &str) -> u64 {
    let mut val = 0;
    for c in input.chars() {
        val = (val + c as u64) * 17 % 256;
    }
    val
}

#[cfg(test)]
#[test]
fn test_nothing_day15() {
    let test_cases = vec![
        ("HASH", 52),
        ("rn=1", 30),
        ("cm-", 253),
        ("qp=3", 97),
        ("cm=2", 47),
        ("qp-", 14),
        ("pc=4", 180),
        ("ot=9", 9),
        ("ab=5", 197),
        ("pc-", 48),
        ("pc=6", 214),
        ("ot=7", 231),
    ];
    for tc in test_cases {
        assert_eq!(hash(tc.0), tc.1)
    }
    println!("{}", hash("qp"))
}

#[test]
fn test_part2_day15() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part2(input), 145);
}
