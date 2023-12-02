use crate::util::timing::time;

const INPUT: &str = include_str!("../res/day2.txt");

#[tracing::instrument]
pub fn part1() {
    let (sum, _) = time(|| {
        let assume_max_red = 12;
        let assume_max_green = 13;
        let assume_max_blue = 14;

        INPUT
            .lines()
            .map(|line| Game::parse(line))
            .filter(|game| {
                game.max_red <= Some(assume_max_red)
                    && game.max_green <= Some(assume_max_green)
                    && game.max_blue <= Some(assume_max_blue)
            })
            .map(|game| game.id)
            .sum::<u32>()
    });
    assert_eq!(sum, 2317);
}

#[tracing::instrument]
pub fn part2() {
    todo!()
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
    max_red: Option<u8>,
    max_green: Option<u8>,
    max_blue: Option<u8>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let mut draws = Vec::new();

        let (game, line) = line.split_once(':').expect("at least one ':'");
        let game_id = game
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .expect("game id")
            .parse::<u32>()
            .expect("game id to be number");

        let draws_raw = line.split(';');
        for draw_raw in draws_raw {
            let mut draw = Draw {
                red: 0,
                green: 0,
                blue: 0,
            };

            let parts = draw_raw.trim().split(',').map(|it| it.trim());
            for part in parts {
                let (num, color) = part.split_once(' ').expect("num color part");
                let num = num.parse::<u8>().expect("draw amount to be a number");
                match color {
                    "red" => draw.red = num,
                    "green" => draw.green = num,
                    "blue" => draw.blue = num,
                    other => panic!("unexpected color: {other}"),
                }
            }

            draws.push(draw);
        }

        Self {
            id: game_id,
            max_red: draws.iter().map(|it| it.red).max(),
            max_green: draws.iter().map(|it| it.green).max(),
            max_blue: draws.iter().map(|it| it.blue).max(),
            draws,
        }
    }
}

#[derive(Debug)]
struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}
