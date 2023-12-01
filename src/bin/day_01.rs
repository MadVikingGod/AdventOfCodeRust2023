fn main() {
    let input: Vec<&str> = include_str!("input/day_01.txt").lines().collect();
    println!("Part 1: {}", get_sum(&input));
    println!("Part 2: {}", get_sum2(&input));
}

fn get_number(input: &str) -> i32 {
    let first = input.chars().filter(|c| c.is_numeric() ).take(1).map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()[0];
    let last = input.chars().rev().filter(|c| c.is_numeric() ).take(1).map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()[0];
    return first*10+last;
}

fn get_number2(input: &str) -> i32 {
    let nums = input.char_indices().filter_map(|(i, _)| get_num(input, i)).collect::<Vec<i32>>();
    let first = nums[0];
    let last = nums[nums.len()-1];
    return first*10+last;
}

fn get_num(input: &str, index: usize) -> Option<i32> {
    if let Some(c) = input.get(index..index+1) {
        if let Ok(n) = c.parse() {
            return Some(n);
        }
    };
    // one, two, six
    if let Some(s) = input.get(index..index+3) {
        if s == "one" {
            return Some(1);
        } else if s == "two" {
            return Some(2);
        } else if s == "six" {
            return Some(6);
        }
    };
    // four, five, nine
    if let Some(s) = input.get(index..index+4) {
        if s == "four" {
            return Some(4);
        } else if s == "five" {
            return Some(5);
        } else if s == "nine" {
            return Some(9);
        }
    };
    // three, seven, eight
    if let Some(s) = input.get(index..index+5) {
        if s == "three" {
            return Some(3);
        } else if s == "seven" {
            return Some(7);
        } else if s == "eight" {
            return Some(8);
        }
    };
    None
}

fn get_sum(input: &Vec<&str>) -> i32 {
    input.iter().map(|s| get_number(s)).sum()
}

fn get_sum2(input: &Vec<&str>) -> i32 {
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
    let input ="1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(get_sum(input.lines().collect()), 142);
}

#[test]
fn test_get_number2() {
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