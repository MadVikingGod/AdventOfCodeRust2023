// Time:        35     69     68     87
// Distance:   213   1168   1086   1248
fn main() {
    let _input = vec![(35, 213), (69, 1168), (68, 1086), (87, 1248)];
    let p1: u64 = _input.iter().map(|&(t, d)| count_solutions(t, d)).product();
    let p2: u64 = count_solutions(35696887, 213116810861248);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
#[test]
fn test_nothing_day06() {
    assert_eq!(count_solutions(7, 9), 4);
    assert_eq!(count_solutions(15, 40), 8);
    assert_eq!(count_solutions(30, 200), 9);
}

#[test]
fn test_part2() {
    assert_eq!(count_solutions(71530, 940200), 71503)
}

fn count_solutions(time: u64, distance: u64) -> u64 {
    let x = (0..distance).find(|x| x * (time - x) > distance).unwrap();
    time - 2 * x + 1
}

// is too low 20537781
