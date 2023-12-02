use ::tracing::level_filters::LevelFilter;

mod util;

mod day1;
mod day2;

fn main() {
    util::tracing::init(LevelFilter::INFO);

    day1::part1();
    day1::part2();

    day2::part1();
}
