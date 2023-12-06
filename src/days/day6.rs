pub fn part1(input: &str) -> i64 {
    parse_games(input)
        .map(|g| solve(g.time as u32, g.distance as i64, 1))
        .map(|(a, b)| b - a + 1)
        .fold(None, |acc, next| Some(acc.unwrap_or(1) * next))
        .expect("at least one game")
}

pub fn part2(input: &str) -> i64 {
    let g = parse_long_game(input);
    let (a, b) = solve(g.time as u32, g.distance as i64, 1);
    b - a + 1
}

fn solve(t_run: u32, dist_record: i64, v: u8) -> (i64, i64) {
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

    let a: i64 = -1;
    let b: i64 = i64::from(v as u32 * t_run);
    let c: i64 = -dist_record;

    let b_squared = b.pow(2);
    let term = b_squared - 4 * a * c;
    let sq = f64::sqrt(term as f64);

    let a_doubled = 2 * a;

    let x_1 = (-b as f64 + sq) / a_doubled as f64;
    let x_2 = (-b as f64 - sq) / a_doubled as f64;

    let (x_1, x_2) = (x_1.floor() as i64 + 1, x_2.ceil() as i64 - 1);

    (x_1, x_2)
}

#[derive(Debug)]
struct Game {
    time: u64,
    distance: u64,
}

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
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
    times.zip(distances).map(|(time, distance)| Game { time, distance })
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
        .parse::<u64>().expect("number");
    let distance = lines
        .next()
        .expect("second line")
        .trim_start_matches("Distance:")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>().expect("number");
    Game { time, distance }
}
