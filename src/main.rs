use ::tracing::level_filters::LevelFilter;

mod tracing;

mod day1;

fn main() {
    tracing::init(LevelFilter::INFO);

    day1::part1();
    day1::part2();
}
