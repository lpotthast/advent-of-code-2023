use ::tracing::level_filters::LevelFilter;

use crate::util::timing::bench;

mod util;

mod day1;
mod day2;

fn main() {
    util::tracing::init(LevelFilter::INFO);

    run("day1::part1", day1::part1, 54601);
    run("day1::part2", day1::part2, 54078);
    run("day2::part1", day2::part1, 2317);
    run("day2::part2", day2::part2, 74804);
}

#[tracing::instrument(level = "INFO", skip(fun, expected))]
fn run<R: std::fmt::Debug + PartialEq>(name: &'static str, fun: impl Fn() -> R, expected: R) {
    let result = fun();
    let avg = bench(fun);
    tracing::info!(avg = format!("{} μs", avg.as_micros()), ?result);
    assert_eq!(result, expected);
}
