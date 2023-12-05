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
│  ├─ day1_part1  27.15 µs      │ 32.62 µs      │ 28.06 µs      │ 28.18 µs      │ 100     │ 100000
│  ├─ day1_part2  25.28 µs      │ 32.31 µs      │ 26.86 µs      │ 26.91 µs      │ 100     │ 100000
│  ├─ day2_part1  17.42 µs      │ 23.03 µs      │ 17.63 µs      │ 17.81 µs      │ 100     │ 100000
│  ├─ day2_part2  17.15 µs      │ 18.86 µs      │ 17.4 µs       │ 17.42 µs      │ 100     │ 100000
│  ├─ day3_part1  34.39 µs      │ 39.72 µs      │ 35.12 µs      │ 35.22 µs      │ 100     │ 100000
│  ├─ day3_part2  27.16 µs      │ 28.94 µs      │ 27.58 µs      │ 27.66 µs      │ 100     │ 100000
│  ├─ day4_part1  27.11 µs      │ 30.79 µs      │ 28.94 µs      │ 28.99 µs      │ 100     │ 100000
│  ├─ day4_part2  28.08 µs      │ 39.43 µs      │ 28.53 µs      │ 28.86 µs      │ 100     │ 100000
│  ├─ day5_part1  7.616 µs      │ 8.081 µs      │ 7.784 µs      │ 7.79 µs       │ 100     │ 100000
│  ╰─ day5_part2  18.17 µs      │ 22.44 µs      │ 18.95 µs      │ 19.08 µs      │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  59.22 ns      │ 81.12 ns      │ 62.72 ns      │ 63.21 ns      │ 100     │ 100000
   ├─ day1_part2  121.5 ns      │ 177.2 ns      │ 123.5 ns      │ 125.4 ns      │ 100     │ 100000
   ├─ day2_part1  506.1 ns      │ 617.1 ns      │ 516.6 ns      │ 530.9 ns      │ 100     │ 100000
   ├─ day2_part2  504.7 ns      │ 638.2 ns      │ 533.6 ns      │ 541.3 ns      │ 100     │ 100000
   ├─ day3_part1  250 ns        │ 299 ns        │ 261 ns        │ 261 ns        │ 100     │ 100000
   ├─ day3_part2  207 ns        │ 237.9 ns      │ 215.5 ns      │ 217.1 ns      │ 100     │ 100000
   ├─ day4_part1  373.9 ns      │ 608.2 ns      │ 390.9 ns      │ 395.5 ns      │ 100     │ 100000
   ├─ day4_part2  386.1 ns      │ 593.7 ns      │ 399.6 ns      │ 404.2 ns      │ 100     │ 100000
   ├─ day5_part1  816.4 ns      │ 969.5 ns      │ 825.1 ns      │ 828.7 ns      │ 100     │ 100000
   ╰─ day5_part2  1.428 µs      │ 1.709 µs      │ 1.44 µs       │ 1.447 µs      │ 100     │ 100000
```

on a Ryzen 7950X
