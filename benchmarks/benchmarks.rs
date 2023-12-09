fn main() {
    // Run registered benchmarks.
    divan::main();
}

use lib::days::*;

const SAMPLE_COUNT: u32 = 100;
const SAMPLE_SIZE: u32 = 100;

mod test_input {
    use super::*;
    use divan::bench;
    use divan::black_box;

    const INPUT_D1_PART1_TEST: &str = include_str!("../res/day1_part1_test.txt");
    const INPUT_D1_PART2_TEST: &str = include_str!("../res/day1_part2_test.txt");
    const INPUT_D2_TEST: &str = include_str!("../res/day2_test.txt");
    const INPUT_D3_TEST: &str = include_str!("../res/day3_test.txt");
    const INPUT_D4_TEST: &str = include_str!("../res/day4_test.txt");
    const INPUT_D5_TEST: &str = include_str!("../res/day5_test.txt");
    const INPUT_D6_TEST: &str = include_str!("../res/day6_test.txt");
    const INPUT_D7_TEST: &str = include_str!("../res/day7_test.txt");
    const INPUT_D8_PART1_TEST: &str = include_str!("../res/day8_part1_test.txt");
    const INPUT_D8_PART2_TEST: &str = include_str!("../res/day8_part2_test.txt");
    const INPUT_D9_TEST: &str = include_str!("../res/day9_test.txt");

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day1_part1() -> u64 {
        day1::part1(black_box(INPUT_D1_PART1_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day1_part2() -> u64 {
        day1::part2(black_box(INPUT_D1_PART2_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day2_part1() -> u64 {
        day2::part1(black_box(INPUT_D2_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day2_part2() -> u64 {
        day2::part2(black_box(INPUT_D2_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day3_part1() -> u64 {
        day3::part1(black_box(INPUT_D3_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day3_part2() -> u64 {
        day3::part2(black_box(INPUT_D3_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day4_part1() -> u64 {
        day4::part1(black_box(INPUT_D4_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day4_part2() -> u64 {
        day4::part2(black_box(INPUT_D4_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day5_part1() -> Option<i64> {
        day5::part1(black_box(INPUT_D5_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day5_part2() -> Option<i64> {
        day5::part2(black_box(INPUT_D5_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day6_part1() -> u32 {
        day6::part1(black_box(INPUT_D6_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day6_part2() -> u32 {
        day6::part2(black_box(INPUT_D6_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day7_part1() -> u64 {
        day7::part1(black_box(INPUT_D7_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day7_part2() -> u64 {
        day7::part2(black_box(INPUT_D7_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day8_part1() -> u64 {
        day8::part1(black_box(INPUT_D8_PART1_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day8_part2() -> u64 {
        day8::part2(black_box(INPUT_D8_PART2_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day9_part1() -> i64 {
        day9::part1(black_box(INPUT_D9_TEST))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day9_part2() -> i64 {
        day9::part2(black_box(INPUT_D9_TEST))
    }
}

mod real_input {
    use super::*;
    use divan::bench;
    use divan::black_box;

    const INPUT_D1: &str = include_str!("../res/day1.txt");
    const INPUT_D2: &str = include_str!("../res/day2.txt");
    const INPUT_D3: &str = include_str!("../res/day3.txt");
    const INPUT_D4: &str = include_str!("../res/day4.txt");
    const INPUT_D5: &str = include_str!("../res/day5.txt");
    const INPUT_D6: &str = include_str!("../res/day6.txt");
    const INPUT_D7: &str = include_str!("../res/day7.txt");
    const INPUT_D8: &str = include_str!("../res/day8.txt");
    const INPUT_D9: &str = include_str!("../res/day9.txt");

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day1_part1() -> u64 {
        day1::part1(black_box(INPUT_D1))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day1_part2() -> u64 {
        day1::part2(black_box(INPUT_D1))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day2_part1() -> u64 {
        day2::part1(black_box(INPUT_D2))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day2_part2() -> u64 {
        day2::part2(black_box(INPUT_D2))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day3_part1() -> u64 {
        day3::part1(black_box(INPUT_D3))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day3_part2() -> u64 {
        day3::part2(black_box(INPUT_D3))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day4_part1() -> u64 {
        day4::part1(black_box(INPUT_D4))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day4_part2() -> u64 {
        day4::part2(black_box(INPUT_D4))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day5_part1() -> Option<i64> {
        day5::part1(black_box(INPUT_D5))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day5_part2() -> Option<i64> {
        day5::part2(black_box(INPUT_D5))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day6_part1() -> u32 {
        day6::part1(black_box(INPUT_D6))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day6_part2() -> u32 {
        day6::part2(black_box(INPUT_D6))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day7_part1() -> u64 {
        day7::part1(black_box(INPUT_D7))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day7_part2() -> u64 {
        day7::part2(black_box(INPUT_D7))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE / 2)]
    fn day8_part1() -> u64 {
        day8::part1(black_box(INPUT_D8))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE / 2)]
    fn day8_part2() -> u64 {
        day8::part2(black_box(INPUT_D8))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day9_part1() -> i64 {
        day9::part1(black_box(INPUT_D9))
    }

    #[bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
    fn day9_part2() -> i64 {
        day9::part2(black_box(INPUT_D9))
    }
}
