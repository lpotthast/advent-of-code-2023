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
│  ├─ day1_part1  26.12 µs      │ 29.65 µs      │ 28.22 µs      │ 28.22 µs      │ 100     │ 100000
│  ├─ day1_part2  25.84 µs      │ 27.48 µs      │ 26.48 µs      │ 26.48 µs      │ 100     │ 100000
│  ├─ day2_part1  17.77 µs      │ 19.96 µs      │ 18.2 µs       │ 18.35 µs      │ 100     │ 100000
│  ├─ day2_part2  17.66 µs      │ 18.66 µs      │ 18.09 µs      │ 18.1 µs       │ 100     │ 100000
│  ├─ day3_part1  34.01 µs      │ 36.38 µs      │ 35.09 µs      │ 35.09 µs      │ 100     │ 100000
│  ├─ day3_part2  26.9 µs       │ 30.54 µs      │ 27.3 µs       │ 27.35 µs      │ 100     │ 100000
│  ├─ day4_part1  26 µs         │ 27.17 µs      │ 26.48 µs      │ 26.53 µs      │ 100     │ 100000
│  ├─ day4_part2  29.18 µs      │ 33.03 µs      │ 30.81 µs      │ 30.84 µs      │ 100     │ 100000
│  ├─ day5_part1  7.505 µs      │ 8.443 µs      │ 7.919 µs      │ 7.885 µs      │ 100     │ 100000
│  ├─ day5_part2  19.12 µs      │ 19.92 µs      │ 19.75 µs      │ 19.62 µs      │ 100     │ 100000
│  ├─ day6_part1  119.3 ns      │ 172.9 ns      │ 121.2 ns      │ 122.6 ns      │ 100     │ 100000
│  ├─ day6_part2  198 ns        │ 215.3 ns      │ 206.9 ns      │ 205.6 ns      │ 100     │ 100000
│  ├─ day7_part1  97.49 µs      │ 101.1 µs      │ 98.7 µs       │ 98.62 µs      │ 100     │ 100000
│  ╰─ day7_part2  98.21 µs      │ 100.2 µs      │ 98.95 µs      │ 99 µs         │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  58.62 ns      │ 74.82 ns      │ 59.42 ns      │ 60.14 ns      │ 100     │ 100000
   ├─ day1_part2  118.3 ns      │ 179.7 ns      │ 122.1 ns      │ 125.4 ns      │ 100     │ 100000
   ├─ day2_part1  519.8 ns      │ 727.3 ns      │ 544.3 ns      │ 549.5 ns      │ 100     │ 100000
   ├─ day2_part2  517.2 ns      │ 788.9 ns      │ 541 ns        │ 547.9 ns      │ 100     │ 100000
   ├─ day3_part1  251.8 ns      │ 334.4 ns      │ 257.8 ns      │ 261.4 ns      │ 100     │ 100000
   ├─ day3_part2  208.8 ns      │ 230.5 ns      │ 220.3 ns      │ 219.4 ns      │ 100     │ 100000
   ├─ day4_part1  410.6 ns      │ 562.1 ns      │ 433.6 ns      │ 437.7 ns      │ 100     │ 100000
   ├─ day4_part2  434.8 ns      │ 829.1 ns      │ 458.1 ns      │ 518.6 ns      │ 100     │ 100000
   ├─ day5_part1  771.7 ns      │ 831.7 ns      │ 805.6 ns      │ 802.6 ns      │ 100     │ 100000
   ├─ day5_part2  1.449 µs      │ 1.751 µs      │ 1.481 µs      │ 1.488 µs      │ 100     │ 100000
   ├─ day6_part1  90.52 ns      │ 138.3 ns      │ 91.92 ns      │ 95.21 ns      │ 100     │ 100000
   ├─ day6_part2  126.9 ns      │ 158.2 ns      │ 133.2 ns      │ 133.5 ns      │ 100     │ 100000
   ├─ day7_part1  257.4 ns      │ 279.7 ns      │ 263.4 ns      │ 264.6 ns      │ 100     │ 100000
   ╰─ day7_part2  256.1 ns      │ 391.3 ns      │ 263.8 ns      │ 267 ns        │ 100     │ 100000
```

on a Ryzen 7950X
