const INPUT: &str = include_str!("../res/day2.txt");

#[tracing::instrument]
pub fn part1() -> u64 {
    INPUT
        .lines()
        .map(Game::parse)
        .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
        .map(|game| game.id)
        .sum::<u64>()
}

#[tracing::instrument]
pub fn part2() -> u64 {
    INPUT
        .lines()
        .map(Game::parse)
        .map(|game| game.max_red * game.max_green * game.max_blue)
        .sum::<u64>()
}

#[derive(Debug)]
struct Game {
    id: u64,
    max_red: u64,
    max_green: u64,
    max_blue: u64,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, line) = line.split_once(':').expect("at least one ':'");
        let mut game = Self {
            id: game
                .split_ascii_whitespace()
                .nth(1)
                .expect("game id")
                .parse::<u64>()
                .expect("game id to be number"),
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };
        for draw in line.split(';') {
            for part in draw.split(',').map(str::trim) {
                let (num, color) = part.split_once(' ').expect("num color part");
                let num = num.parse::<u64>().expect("draw amount to be a number");
                match color.chars().next().expect("non empty color str") {
                    'r' => game.max_red = u64::max(game.max_red, num),
                    'g' => game.max_green = u64::max(game.max_green, num),
                    'b' => game.max_blue = u64::max(game.max_blue, num),
                    other => panic!("unexpected color: {other}"),
                }
            }
        }
        game
    }
}
