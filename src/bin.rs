use lib::days::*;

const INPUT_D1_PART1_TEST: &str = include_str!("../res/day1_part1_test.txt");
const INPUT_D1_PART2_TEST: &str = include_str!("../res/day1_part2_test.txt");
const INPUT_D2_TEST: &str = include_str!("../res/day2_test.txt");
const INPUT_D3_TEST: &str = include_str!("../res/day3_test.txt");
const INPUT_D4_TEST: &str = include_str!("../res/day4_test.txt");
const INPUT_D5_TEST: &str = include_str!("../res/day5_test.txt");
const INPUT_D6_TEST: &str = include_str!("../res/day6_test.txt");
const INPUT_D7_TEST: &str = include_str!("../res/day7_test.txt");
const INPUT_D7_TEST2: &str = include_str!("../res/day7_test2.txt");
const INPUT_D8_PART1_TEST: &str = include_str!("../res/day8_part1_test.txt");
const INPUT_D8_PART1_TEST2: &str = include_str!("../res/day8_part1_test2.txt");
const INPUT_D8_PART2_TEST: &str = include_str!("../res/day8_part2_test.txt");
const INPUT_D9_TEST: &str = include_str!("../res/day9_test.txt");
const INPUT_D10_PART1_TEST1: &str = include_str!("../res/day10_part1_test1.txt");
const INPUT_D10_PART1_TEST2: &str = include_str!("../res/day10_part1_test2.txt");
const INPUT_D10_PART2_TEST1: &str = include_str!("../res/day10_part2_test1.txt");
const INPUT_D10_PART2_TEST2: &str = include_str!("../res/day10_part2_test2.txt");
const INPUT_D10_PART2_TEST3: &str = include_str!("../res/day10_part2_test3.txt");
const INPUT_D11_TEST: &str = include_str!("../res/day11_test.txt");
const INPUT_D12_PART1_TEST1: &str = include_str!("../res/day12_part1_test1.txt");

const INPUT_D1: &str = include_str!("../res/day1.txt");
const INPUT_D2: &str = include_str!("../res/day2.txt");
const INPUT_D3: &str = include_str!("../res/day3.txt");
const INPUT_D4: &str = include_str!("../res/day4.txt");
const INPUT_D5: &str = include_str!("../res/day5.txt");
const INPUT_D6: &str = include_str!("../res/day6.txt");
const INPUT_D7: &str = include_str!("../res/day7.txt");
const INPUT_D8: &str = include_str!("../res/day8.txt");
const INPUT_D9: &str = include_str!("../res/day9.txt");
const INPUT_D10: &str = include_str!("../res/day10.txt");
const INPUT_D11: &str = include_str!("../res/day11.txt");
const INPUT_D12: &str = include_str!("../res/day12.txt");

fn main() {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    run(day1::part1, INPUT_D1_PART1_TEST, 142);
    run(day1::part2, INPUT_D1_PART2_TEST, 281);
    run(day1::part1, INPUT_D1, 54_601);
    run(day1::part2, INPUT_D1, 54_078);

    run(day2::part1, INPUT_D2_TEST, 8);
    run(day2::part2, INPUT_D2_TEST, 2_286);
    run(day2::part1, INPUT_D2, 2_317);
    run(day2::part2, INPUT_D2, 74_804);

    run(day3::part1, INPUT_D3_TEST, 4_361);
    run(day3::part2, INPUT_D3_TEST, 467_835);
    run(day3::part1, INPUT_D3, 556_367);
    run(day3::part2, INPUT_D3, 89_471_771);

    run(day4::part1, INPUT_D4_TEST, 13);
    run(day4::part2, INPUT_D4_TEST, 230);
    run(day4::part1, INPUT_D4, 23_028);
    run(day4::part2, INPUT_D4, 9_236_992);

    run(day5::part1, INPUT_D5_TEST, Some(35));
    run(day5::part2, INPUT_D5_TEST, Some(46));
    run(day5::part1, INPUT_D5, Some(403_695_602));
    run(day5::part2, INPUT_D5, Some(219_529_182));

    run(day6::part1, INPUT_D6_TEST, 288);
    run(day6::part2, INPUT_D6_TEST, 71_503);
    run(day6::part1, INPUT_D6, 211_904);
    run(day6::part2, INPUT_D6, 43_364_472);

    run(day7::part1, INPUT_D7_TEST, 6_440);
    run(day7::part1, INPUT_D7_TEST2, 6_592);
    run(day7::part2, INPUT_D7_TEST, 5_905);
    run(day7::part2, INPUT_D7_TEST2, 6_839);
    run(day7::part1, INPUT_D7, 253_205_868);
    run(day7::part2, INPUT_D7, 253_907_829);

    run(day8::part1, INPUT_D8_PART1_TEST, 2);
    run(day8::part1, INPUT_D8_PART1_TEST2, 6);
    run(day8::part1, INPUT_D8, 18_157);
    run(day8::part2, INPUT_D8_PART2_TEST, 6);
    run(day8::part2, INPUT_D8, 14_299_763_833_181);

    run(day9::part1, INPUT_D9_TEST, 114);
    run(day9::part1, INPUT_D9, 1_877_825_184);
    run(day9::part2, INPUT_D9_TEST, 2);
    run(day9::part2, INPUT_D9, 1_108);

    run(day10::part1, INPUT_D10_PART1_TEST1, 4);
    run(day10::part1, INPUT_D10_PART1_TEST2, 8);
    run(day10::part1, INPUT_D10, 6_875);
    run(day10::part2, INPUT_D10_PART1_TEST1, 1);
    run(day10::part2, INPUT_D10_PART1_TEST2, 1);
    run(day10::part2, INPUT_D10_PART2_TEST1, 4);
    run(day10::part2, INPUT_D10_PART2_TEST2, 8);
    run(day10::part2, INPUT_D10_PART2_TEST3, 10);
    run(day10::part2, INPUT_D10, 471);

    run(day11::part1, INPUT_D11_TEST, 374);
    run(day11::part1, INPUT_D11, 9_627_977);
    run(day11::part2_expand_9, INPUT_D11_TEST, 1_030);
    run(day11::part2_expand_99, INPUT_D11_TEST, 8_410);
    run(day11::part2_expand_999_999, INPUT_D11_TEST, 82_000_210);
    run(day11::part2_expand_999_999, INPUT_D11, 644_248_339_497);

    run(day12::part1, INPUT_D12_PART1_TEST1, 21);
    run(day12::part1, INPUT_D12, 7_195);
    run(day12::part2, INPUT_D12_PART1_TEST1, 525152);
    run(day12::part2, INPUT_D12, 33992866292225);
}

#[tracing::instrument(level = "INFO", skip_all, fields(name = std::any::type_name::<F>()))]
fn run<R: std::fmt::Debug + PartialEq, F: Fn(&str) -> R>(fun: F, input: &str, expect: R) {
    let start = std::time::Instant::now();
    let result = fun(input);
    let end = std::time::Instant::now();
    let took = (end - start).as_micros();
    tracing::info!(?result, took = format!("{took} μs"));
    assert_eq!(result, expect);
}

pub fn init_tracing(level: tracing::level_filters::LevelFilter) {
    use tracing_subscriber::filter::Targets;
    use tracing_subscriber::fmt::format::{Format, Pretty};
    use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{Layer, Registry};

    fn build_log_filter(default_log_level: tracing::level_filters::LevelFilter) -> Targets {
        Targets::new().with_default(default_log_level)
    }

    fn build_tracing_subscriber_fmt_layer() -> tracing_subscriber::fmt::Layer<Registry, Pretty, Format<Pretty>> {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_thread_names(false)
            .with_thread_ids(false)
    }

    let fmt_layer_filtered = build_tracing_subscriber_fmt_layer().with_filter(build_log_filter(level));

    Registry::default().with(fmt_layer_filtered).init();
}
