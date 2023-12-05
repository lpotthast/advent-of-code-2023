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
benchmarks     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ day1_part1  22.78 µs      │ 25.15 µs      │ 24.4 µs       │ 24.24 µs      │ 100     │ 100000
├─ day1_part2  26.47 µs      │ 29.29 µs      │ 26.92 µs      │ 26.98 µs      │ 100     │ 100000
├─ day2_part1  18.93 µs      │ 21.56 µs      │ 19.21 µs      │ 19.51 µs      │ 100     │ 100000
├─ day2_part2  18.55 µs      │ 20.86 µs      │ 18.92 µs      │ 19.07 µs      │ 100     │ 100000
├─ day3_part1  34.67 µs      │ 36.88 µs      │ 35.3 µs       │ 35.34 µs      │ 100     │ 100000
├─ day3_part2  26.91 µs      │ 30.44 µs      │ 27.55 µs      │ 27.5 µs       │ 100     │ 100000
├─ day4_part1  28.51 µs      │ 30.18 µs      │ 29.13 µs      │ 29.16 µs      │ 100     │ 100000
├─ day4_part2  26.8 µs       │ 32.61 µs      │ 30.81 µs      │ 30.74 µs      │ 100     │ 100000
├─ day5_part1  7.727 µs      │ 8.125 µs      │ 7.962 µs      │ 7.959 µs      │ 100     │ 100000
╰─ day5_part2  22.63 µs      │ 28.35 µs      │ 24.48 µs      │ 24.4 µs       │ 100     │ 100000
```

on a Ryzen 7950X
