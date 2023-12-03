use std::cmp::max;

fn main() {
    let input = include_str!("input/day_02.txt");
    let games = input.lines().map(Game::new).collect::<Vec<Game>>();
    let bag = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };
    let part1 = games
        .iter()
        .filter(|g| g.is_possible(&bag))
        .map(|g| g.number)
        .sum::<u32>();
    println!("Part 1: {}", part1);

    let part2 = games.iter().map(|g| g.power()).sum::<u64>();
    println!("Part 2: {}", part2);
}

#[derive(PartialEq, Debug)]
struct Game {
    number: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn new(line: &str) -> Self {
        let (game, pulls) = line.split_once(": ").unwrap();
        let number = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let pulls = pulls.split("; ").map(Pull::new).collect();
        Self { number, pulls }
    }
    fn is_possible(&self, c: &Pull) -> bool {
        self.max().red <= c.red && self.max().green <= c.green && self.max().blue <= c.blue
    }
    fn max(&self) -> Pull {
        self.pulls.iter().fold(
            Pull {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, pull| Pull {
                red: max(acc.red, pull.red),
                green: max(acc.green, pull.green),
                blue: max(acc.blue, pull.blue),
            },
        )
    }
    fn power(&self) -> u64 {
        self.max().red as u64 * self.max().green as u64 * self.max().blue as u64
    }
}
#[derive(PartialEq, Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}
impl Pull {
    fn new(pull: &str) -> Self {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for token in pull.split(", ") {
            if let Some(r) = token.strip_suffix(" red") {
                red = r.parse().unwrap()
            }
            if let Some(r) = token.strip_suffix(" green") {
                green = r.parse().unwrap()
            }
            if let Some(r) = token.strip_suffix(" blue") {
                blue = r.parse().unwrap()
            }
        }
        Self { red, green, blue }
    }
}

#[cfg(test)]
#[test]
fn test_new_pull() {
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    assert_eq!(
        Pull {
            red: 4,
            green: 0,
            blue: 3
        },
        Pull::new("3 blue, 4 red")
    );
    assert_eq!(
        Pull {
            red: 1,
            green: 2,
            blue: 6
        },
        Pull::new("1 red, 2 green, 6 blue")
    );
    assert_eq!(
        Pull {
            red: 0,
            green: 2,
            blue: 0
        },
        Pull::new("2 green")
    );
}
#[test]
fn test_new_game() {
    //Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let want = Game {
        number: 2,
        pulls: vec![
            Pull {
                red: 0,
                green: 2,
                blue: 1,
            },
            Pull {
                red: 1,
                green: 3,
                blue: 4,
            },
            Pull {
                red: 0,
                green: 1,
                blue: 1,
            },
        ],
    };
    assert_eq!(
        want,
        Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
    )
}

#[test]
fn test_is_possible() {
    let games: Vec<Game> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .lines()
        .map(Game::new)
        .collect();
    let bag = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };
    assert_eq!(
        8,
        games
            .iter()
            .filter(|g| g.is_possible(&bag))
            .map(|g| g.number)
            .sum::<u32>()
    );
}
#[test]
fn test_power() {
    let games: Vec<Game> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .lines()
        .map(Game::new)
        .collect();

    assert_eq!(2286, games.iter().map(|g| g.power()).sum::<u64>());
}
