use itertools::Itertools;

fn main() {
    let _input = include_str!("input/day_05.txt");

    println!("Part 1: {}", part1(_input));
    println!("Part 2: {}", part2(_input));
}

fn part2(input: &str) -> usize {
    let mut groups = input.split("\n\n");
    // Do seeds
    let seeds = groups
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .tuples()
        .collect::<Vec<(usize, usize)>>();

    let maps: Vec<Map> = groups
        .map(|group| group.split('\n').skip(1).collect::<Vec<_>>().into())
        .collect::<Vec<_>>();

    let mut min = std::usize::MAX;
    for (start, len) in seeds {
        for seed in start..start + len {
            let loc = maps
                .iter()
                .fold(seed, |current, map| map.next_location(current));
            if loc < min {
                min = loc;
            }
        }
    }
    min
}

fn part1(input: &str) -> usize {
    let mut groups = input.split("\n\n");
    // Do seeds
    let seeds: Vec<usize> = groups
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<Map> = groups
        .map(|group| group.split('\n').skip(1).collect::<Vec<_>>().into())
        .collect::<Vec<_>>();

    let locs = seeds.iter().map(|seed| {
        maps.iter()
            .fold(*seed, |current, map| map.next_location(current))
    });
    locs.min().unwrap()
}

#[derive(Debug, PartialEq)]
struct Map {
    rows: Vec<(usize, usize, usize)>,
}

impl From<Vec<&str>> for Map {
    fn from(s: Vec<&str>) -> Map {
        Map {
            rows: s
                .iter()
                .map(|line| line.split_whitespace())
                .map(|mut nums| {
                    (
                        nums.next().unwrap().parse().unwrap(),
                        nums.next().unwrap().parse().unwrap(),
                        nums.next().unwrap().parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

impl Map {
    fn next_location(&self, current: usize) -> usize {
        for row in self.rows.iter() {
            if row.1 <= current && current < row.1 + row.2 {
                return row.0 + current - row.1;
            }
        }
        current
    }
}

#[cfg(test)]
#[test]
fn test_new_map() {
    let input = "50 98 2
    52 50 48";
    let want = Map {
        rows: vec![(50, 98, 2), (52, 50, 48)],
    };
    assert_eq!(Map::from(input.split("\n").collect::<Vec<_>>()), want)
}

#[test]
fn test_next_location() {
    let input = "50 98 2
    52 50 48";
    let m = Map::from(input.split("\n").collect::<Vec<_>>());
    assert_eq!(m.next_location(0), 0);
    assert_eq!(m.next_location(49), 49);
    assert_eq!(m.next_location(50), 52);
    assert_eq!(m.next_location(51), 53);
    assert_eq!(m.next_location(97), 99);
    assert_eq!(m.next_location(98), 50);
    assert_eq!(m.next_location(99), 51);
}

#[test]
fn test_part1() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(part1(input), 35);
    assert_eq!(part2(input), 46);
}
