# Advent of Code - 2023

RUST based solutions for the [adventofcode - 2023](https://adventofcode.com/2023) problems,
focusing on high performance and low memory usage while trying to still be comprehensible.

Run all days using

    cargo run (--release)

Run benchmarks using

    cargo bench

## Performance

```shell
Timer precision: 100 ns
benchmarks        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                   │               │               │               │         │
│  ├─ day1_part1  25.63 µs      │ 34.52 µs      │ 29.5 µs       │ 29.53 µs      │ 100     │ 100000
│  ├─ day1_part2  26.92 µs      │ 28.31 µs      │ 27.26 µs      │ 27.29 µs      │ 100     │ 100000
│  ├─ day2_part1  19.18 µs      │ 30.32 µs      │ 19.67 µs      │ 19.95 µs      │ 100     │ 100000
│  ├─ day2_part2  19.23 µs      │ 20.67 µs      │ 19.91 µs      │ 19.97 µs      │ 100     │ 100000
│  ├─ day3_part1  31.25 µs      │ 36.62 µs      │ 34.57 µs      │ 34.72 µs      │ 100     │ 100000
│  ├─ day3_part2  23.55 µs      │ 26.14 µs      │ 23.79 µs      │ 23.83 µs      │ 100     │ 100000
│  ├─ day4_part1  27.46 µs      │ 30.4 µs       │ 29.37 µs      │ 29.36 µs      │ 100     │ 100000
│  ├─ day4_part2  24.21 µs      │ 34.71 µs      │ 25.67 µs      │ 26.97 µs      │ 100     │ 100000
│  ├─ day5_part1  6.974 µs      │ 9.688 µs      │ 7.32 µs       │ 7.361 µs      │ 100     │ 100000
│  ├─ day5_part2  17.57 µs      │ 19.72 µs      │ 18.24 µs      │ 18.13 µs      │ 100     │ 100000
│  ├─ day6_part1  120.4 ns      │ 126.6 ns      │ 121.9 ns      │ 121.3 ns      │ 100     │ 100000
│  ╰─ day6_part2  179.1 ns      │ 193.4 ns      │ 181.6 ns      │ 181.9 ns      │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  59.52 ns      │ 92.52 ns      │ 60.12 ns      │ 62.12 ns      │ 100     │ 100000
   ├─ day1_part2  118.4 ns      │ 185.7 ns      │ 124.9 ns      │ 126.7 ns      │ 100     │ 100000
   ├─ day2_part1  549.1 ns      │ 656 ns        │ 565.9 ns      │ 570.2 ns      │ 100     │ 100000
   ├─ day2_part2  580.3 ns      │ 609.6 ns      │ 600.2 ns      │ 600.8 ns      │ 100     │ 100000
   ├─ day3_part1  251.1 ns      │ 451.5 ns      │ 264.5 ns      │ 268.4 ns      │ 100     │ 100000
   ├─ day3_part2  211.4 ns      │ 303.6 ns      │ 220.5 ns      │ 222.4 ns      │ 100     │ 100000
   ├─ day4_part1  376.8 ns      │ 426.9 ns      │ 384.1 ns      │ 387.1 ns      │ 100     │ 100000
   ├─ day4_part2  390.7 ns      │ 555.4 ns      │ 426 ns        │ 420 ns        │ 100     │ 100000
   ├─ day5_part1  804.7 ns      │ 855.5 ns      │ 836.2 ns      │ 832.7 ns      │ 100     │ 100000
   ├─ day5_part2  1.413 µs      │ 1.52 µs       │ 1.44 µs       │ 1.438 µs      │ 100     │ 100000
   ├─ day6_part1  91.72 ns      │ 96.72 ns      │ 92.62 ns      │ 92.52 ns      │ 100     │ 100000
   ╰─ day6_part2  122.9 ns      │ 142.7 ns      │ 126.1 ns      │ 126.3 ns      │ 100     │ 100000
```

on a Ryzen 7950X
