use ::tracing::level_filters::LevelFilter;

use crate::util::timing::time;

mod util;

mod day1;
mod day2;

fn main() {
    util::tracing::init(LevelFilter::INFO);

    let (sum, _) = time(|| day1::part1());
    assert_eq!(sum, 54601);
    
    let (sum, _) = time(|| day1::part2());
    assert_eq!(sum, 54078);

    let (sum, _) = time(|| day2::part1());
    assert_eq!(sum, 2317);

    let (sum, _) = time(|| day2::part2());
    assert_eq!(sum, 74804);
}
