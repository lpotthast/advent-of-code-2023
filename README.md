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
│  ├─ day1_part1  26.2 µs       │ 28.47 µs      │ 27.08 µs      │ 27.12 µs      │ 100     │ 100000
│  ├─ day1_part2  25.99 µs      │ 26.87 µs      │ 26.37 µs      │ 26.33 µs      │ 100     │ 100000
│  ├─ day2_part1  16.79 µs      │ 19.95 µs      │ 17.62 µs      │ 17.88 µs      │ 100     │ 100000
│  ├─ day2_part2  17.41 µs      │ 19.23 µs      │ 17.83 µs      │ 17.89 µs      │ 100     │ 100000
│  ├─ day3_part1  34.99 µs      │ 44.54 µs      │ 37.45 µs      │ 37.49 µs      │ 100     │ 100000
│  ├─ day3_part2  28.79 µs      │ 32.56 µs      │ 29.26 µs      │ 29.44 µs      │ 100     │ 100000
│  ├─ day4_part1  26.85 µs      │ 31.26 µs      │ 30.1 µs       │ 30.06 µs      │ 100     │ 100000
│  ├─ day4_part2  30.29 µs      │ 33.2 µs       │ 32.62 µs      │ 32.6 µs       │ 100     │ 100000
│  ├─ day5_part1  7.644 µs      │ 8.086 µs      │ 8.001 µs      │ 7.947 µs      │ 100     │ 100000
│  ├─ day5_part2  18.45 µs      │ 19.35 µs      │ 18.65 µs      │ 18.74 µs      │ 100     │ 100000
│  ├─ day6_part1  118.2 ns      │ 163.3 ns      │ 120 ns        │ 122.3 ns      │ 100     │ 100000
│  ├─ day6_part2  175.2 ns      │ 196.2 ns      │ 183.1 ns      │ 183.8 ns      │ 100     │ 100000
│  ├─ day7_part1  96.75 µs      │ 100.1 µs      │ 97.66 µs      │ 97.83 µs      │ 100     │ 100000
│  ├─ day7_part2  97.13 µs      │ 99.94 µs      │ 97.94 µs      │ 97.99 µs      │ 100     │ 100000
│  ├─ day8_part1  217.2 µs      │ 232.4 µs      │ 223 µs        │ 223.2 µs      │ 100     │ 100000
│  ╰─ day8_part2  507.9 µs      │ 521.1 µs      │ 513.2 µs      │ 513.1 µs      │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  59.32 ns      │ 73.42 ns      │ 59.92 ns      │ 61.26 ns      │ 100     │ 100000
   ├─ day1_part2  118.5 ns      │ 127.1 ns      │ 120.1 ns      │ 120.7 ns      │ 100     │ 100000
   ├─ day2_part1  484.7 ns      │ 574.1 ns      │ 492.4 ns      │ 497.4 ns      │ 100     │ 100000
   ├─ day2_part2  480.7 ns      │ 732.8 ns      │ 509 ns        │ 512 ns        │ 100     │ 100000
   ├─ day3_part1  329.3 ns      │ 509.4 ns      │ 340.4 ns      │ 346 ns        │ 100     │ 100000
   ├─ day3_part2  279.6 ns      │ 317.5 ns      │ 289.8 ns      │ 291.2 ns      │ 100     │ 100000
   ├─ day4_part1  404.6 ns      │ 676.6 ns      │ 419.3 ns      │ 429 ns        │ 100     │ 100000
   ├─ day4_part2  431.1 ns      │ 591.1 ns      │ 452.9 ns      │ 454.8 ns      │ 100     │ 100000
   ├─ day5_part1  780.3 ns      │ 883.3 ns      │ 805.9 ns      │ 808.3 ns      │ 100     │ 100000
   ├─ day5_part2  1.414 µs      │ 1.635 µs      │ 1.442 µs      │ 1.446 µs      │ 100     │ 100000
   ├─ day6_part1  91.32 ns      │ 116.4 ns      │ 92.02 ns      │ 93.22 ns      │ 100     │ 100000
   ├─ day6_part2  125.1 ns      │ 188.1 ns      │ 127.9 ns      │ 130.8 ns      │ 100     │ 100000
   ├─ day7_part1  257.2 ns      │ 298.2 ns      │ 259.6 ns      │ 260.3 ns      │ 100     │ 100000
   ├─ day7_part2  250.7 ns      │ 318.5 ns      │ 256.5 ns      │ 258.7 ns      │ 100     │ 100000
   ├─ day8_part1  991 ns        │ 1.243 µs      │ 1.011 µs      │ 1.014 µs      │ 100     │ 100000
   ╰─ day8_part2  1.224 µs      │ 2.123 µs      │ 1.244 µs      │ 1.315 µs      │ 100     │ 100000
```

on a Ryzen 7950X
