use crate::util::timing::time;

const INPUT: &str = include_str!("../res/day2.txt");

#[tracing::instrument]
pub fn part1() {
    let (sum, _) = time(|| {
        INPUT
            .lines()
            .map(|line| Game::parse(line))
            .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
            .map(|game| game.id)
            .sum::<u32>()
    });
    assert_eq!(sum, 2317);
}

#[tracing::instrument]
pub fn part2() {
    let (sum, _) = time(|| {
        INPUT
            .lines()
            .map(|line| Game::parse(line))
            .map(|game| game.max_red as u32 * game.max_green as u32 * game.max_blue as u32)
            .sum::<u32>()
    });
    assert_eq!(sum, 74804);
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u8,
    max_green: u8,
    max_blue: u8,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, line) = line.split_once(':').expect("at least one ':'");
        let id = game
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .expect("game id")
            .parse::<u32>()
            .expect("game id to be number");
        let mut game = Self {
            id,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };
        for draw_raw in line.split(';') {
            for part in draw_raw.trim().split(',').map(|it| it.trim()) {
                let (num, color) = part.split_once(' ').expect("num color part");
                let num = num.parse::<u8>().expect("draw amount to be a number");
                match color {
                    "red" => game.max_red = u8::max(game.max_red, num),
                    "green" => game.max_green = u8::max(game.max_green, num),
                    "blue" => game.max_blue = u8::max(game.max_blue, num),
                    other => panic!("unexpected color: {other}"),
                }
            }
        }
        game
    }
}
