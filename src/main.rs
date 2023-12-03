use ::tracing::level_filters::LevelFilter;

use crate::util::timing::bench;

mod util;

mod day1;
mod day2;
mod day3;

fn main() {
    util::tracing::init(LevelFilter::INFO);

    warmup();
    run(day1::part1, 54_601);
    run(day1::part2, 54_078);
    run(day2::part1, 2_317);
    run(day2::part2, 74_804);
    run(day3::part1, 556_367);
    run(day3::part2, 89_471_771);
}

fn warmup() {
    let avg = bench(day1::part1);
    tracing::info!(avg = format!("{} μs", avg.as_micros()), "warmup");
}

#[tracing::instrument(level = "INFO", skip_all, fields(name = std::any::type_name::<F>()))]
fn run<R: std::fmt::Debug + PartialEq, F: Fn() -> R>(fun: F, expected: R) {
    let result = fun();
    let avg = bench(fun);
    tracing::info!(avg = format!("{} μs", avg.as_micros()), ?result);
    assert_eq!(result, expected);
}
