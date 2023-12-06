pub fn part1(input: &str) -> u32 {
    parse_individual_games(input)
        .map(|g| solve(g.duration, g.distance_record, 1))
        .map(WinningDuration::num_options_to_win)
        .fold(None, |acc, next| Some(acc.unwrap_or(1) * next))
        .expect("at least one game")
}

pub fn part2(input: &str) -> u32 {
    let g = parse_long_game(input);
    solve(g.duration, g.distance_record, 1).num_options_to_win()
}

#[derive(Debug, Clone, Copy)]
struct WinningDuration {
    min_press: u32,
    max_press: u32,
}

impl WinningDuration {
    const fn num_options_to_win(self) -> u32 {
        self.max_press - self.min_press + 1
    }
}

#[allow(clippy::cast_possible_truncation)]
fn solve(t_run: u64, dist_record: u64, v: u8) -> WinningDuration {
    // visualization: https://www.geogebra.org/calculator/zebs3ca6
    //
    // dist(t_press) = v * t_press * (t_run - t_press)           || remove parenthesis
    // dist(t_press) = v * t_press * t_run - t_press^2           || reorder to get squared part in front
    // dist(t_press) = -t_press^2 + v * t_run * t_press          || note: `v` and `t_run` are constants, v being speed
    //
    // Subtracting the previous record shifts the parabola down
    // so that its two zero-points are the min and max value we have to press to beat the record, so...
    // rec(t_press) = dist_record
    // opt(t_press) = dist(t_press) - rec(t_press)
    // dist_shifted(t_press) = -t_press^2 + v * t_run * t_press - dist_record
    //
    // solving with: x_1/2 = (-b +/- sqrt(b^2 - 4ac)) / (2a)
    // with:
    //   a = -1
    //   b = v * t_run
    //   c = -dist_record

    let b = u64::from(v) * t_run;
    let term = b.pow(2) - 4 * dist_record;
    let sqrt = f64::sqrt(term as f64);

    let x_1 = (-(b as f64) + sqrt) / -2.0;
    let x_2 = (-(b as f64) - sqrt) / -2.0;

    assert!(x_1.is_sign_positive());
    assert!(x_2.is_sign_positive());

    WinningDuration {
        min_press: x_1.floor() as u32 + 1,
        max_press: x_2.ceil() as u32 - 1,
    }
}

#[derive(Debug)]
struct Game {
    duration: u64,
    distance_record: u64,
}

fn parse_individual_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("first line")
        .trim_start_matches("Time:")
        .split_ascii_whitespace()
        .map(|it| it.parse::<u64>().expect("number"));
    let distances = lines
        .next()
        .expect("second line")
        .trim_start_matches("Distance:")
        .split_ascii_whitespace()
        .map(|it| it.parse::<u64>().expect("number"));
    times.zip(distances).map(|(time, distance)| Game { duration: time, distance_record: distance })
}

fn parse_long_game(input: &str) -> Game {
    let mut lines = input.lines();
    let time = lines
        .next()
        .expect("first line")
        .trim_start_matches("Time:")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .expect("number");
    let distance = lines
        .next()
        .expect("second line")
        .trim_start_matches("Distance:")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .expect("number");
    Game { duration: time, distance_record: distance }
}
