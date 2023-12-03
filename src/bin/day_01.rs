fn main() {
    let input: Vec<&str> = include_str!("input/day_01.txt").lines().collect();
    println!("Part 1: {}", get_sum(&input));
    println!("Part 2: {}", get_sum2(&input));
}

fn get_number(line: &str) -> i32 {
    let nums = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as i32)
        .collect::<Vec<i32>>();
    nums[0] * 10 + nums.last().unwrap()
}

fn get_number2(line: &str) -> i32 {
    let nums = (0..line.len())
        .filter_map(|i| starts_with_number(line.get(i..)?))
        .collect::<Vec<i32>>();
    let first = nums[0];
    let last = nums[nums.len() - 1];
    first * 10 + last
}

static NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_digit(input: &str) -> Option<u32> {
    input.chars().take(1).next()?.to_digit(10)
}

fn starts_with_number(input: &str) -> Option<i32> {
    if let Some(n) = get_first_digit(input) {
        return Some(n as i32);
    };
    NUMS.iter()
        .enumerate()
        .filter_map(|(i, &n)| {
            let s = input.get(0..n.len())?;
            if s == n {
                Some(i as i32)
            } else {
                None
            }
        })
        .next()
}

fn get_sum(input: &[&str]) -> i32 {
    input.iter().map(|s| get_number(s)).sum()
}

fn get_sum2(input: &[&str]) -> i32 {
    input.iter().map(|s| get_number2(s)).sum()
}

#[cfg(test)]
#[test]
fn test_get_number() {
    let tests = vec![
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];
    for (input, expected) in tests {
        assert_eq!(get_number(input), expected);
    }
}

#[test]
fn test_get_sum() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    let lines = input.lines().collect::<Vec<&str>>();
    assert_eq!(get_sum(&lines), 142);
}

#[test]
fn test_get_number3() {
    let tests = vec![
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
    ];
    for (input, expected) in tests {
        assert_eq!(get_number2(input), expected);
    }
}
