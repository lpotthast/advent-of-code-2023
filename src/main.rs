use ::tracing::level_filters::LevelFilter;

use crate::util::timing::bench;

mod util;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    util::tracing::init(LevelFilter::INFO);

    warmup();
    run(day1::part1, 54_601);
    run(day1::part2, 54_078);
    run(day2::part1, 2_317);
    run(day2::part2, 74_804);
    run(day3::part1, 556_367); // day3_a.txt
    run(day3::part2, 89_471_771); // day3_a.txt
    //run(day3::part1, 526_404); // day3_b.txt
    //run(day3::part2, 84_399_773); // day3_b.txt
    run(day4::part1, 23_028);
    run(day4::part2, 9_236_992);
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
