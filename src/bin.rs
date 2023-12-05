use lib::days::*;

const INPUT_D1: &str = include_str!("../res/day1.txt");
const INPUT_D2: &str = include_str!("../res/day2.txt");
const INPUT_D3: &str = include_str!("../res/day3_a.txt");
const INPUT_D4: &str = include_str!("../res/day4.txt");
const INPUT_D5: &str = include_str!("../res/day5.txt");

fn main() {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    run(day1::part1, INPUT_D1, 54_601);
    run(day1::part2, INPUT_D1, 54_078);
    run(day2::part1, INPUT_D2, 2_317);
    run(day2::part2, INPUT_D2, 74_804);
    run(day3::part1, INPUT_D3, 556_367);
    run(day3::part2, INPUT_D3, 89_471_771);
    run(day4::part1, INPUT_D4, 23_028);
    run(day4::part2, INPUT_D4, 9_236_992);
    run(day5::part1, INPUT_D5, Some(403_695_602));
    run(day5::part2, INPUT_D5, Some(219_529_182));
}

#[tracing::instrument(level = "INFO", skip_all, fields(name = std::any::type_name::<F>()))]
fn run<R: std::fmt::Debug + PartialEq, F: Fn(&str) -> R>(fun: F, input: &str, expect: R) {
    let result = fun(input);
    tracing::info!(?result);
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
