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
│  ├─ day1_part1  22.79 µs      │ 29.22 µs      │ 28.17 µs      │ 27.96 µs      │ 100     │ 100000
│  ├─ day1_part2  25.14 µs      │ 29.66 µs      │ 26.99 µs      │ 26.63 µs      │ 100     │ 100000
│  ├─ day2_part1  17.53 µs      │ 20.15 µs      │ 17.65 µs      │ 17.72 µs      │ 100     │ 100000
│  ├─ day2_part2  17.38 µs      │ 18 µs         │ 17.58 µs      │ 17.6 µs       │ 100     │ 100000
│  ├─ day3_part1  34.54 µs      │ 36.4 µs       │ 35.52 µs      │ 35.46 µs      │ 100     │ 100000
│  ├─ day3_part2  27.04 µs      │ 27.87 µs      │ 27.44 µs      │ 27.47 µs      │ 100     │ 100000
│  ├─ day4_part1  29.4 µs       │ 31.78 µs      │ 30.61 µs      │ 30.62 µs      │ 100     │ 100000
│  ├─ day4_part2  28.15 µs      │ 32 µs         │ 30.76 µs      │ 30.37 µs      │ 100     │ 100000
│  ├─ day5_part1  7.813 µs      │ 8.072 µs      │ 7.983 µs      │ 7.968 µs      │ 100     │ 100000
│  ╰─ day5_part2  18.15 µs      │ 24.97 µs      │ 18.94 µs      │ 19.04 µs      │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  8.72 ns       │ 9.62 ns       │ 9.52 ns       │ 9.371 ns      │ 100     │ 100000
   ├─ day1_part2  115.2 ns      │ 179 ns        │ 118.8 ns      │ 121.7 ns      │ 100     │ 100000
   ├─ day2_part1  514 ns        │ 623.2 ns      │ 547.1 ns      │ 550.8 ns      │ 100     │ 100000
   ├─ day2_part2  509.4 ns      │ 620 ns        │ 542.1 ns      │ 545.8 ns      │ 100     │ 100000
   ├─ day3_part1  261.6 ns      │ 389.2 ns      │ 263.3 ns      │ 264.9 ns      │ 100     │ 100000
   ├─ day3_part2  207.5 ns      │ 256.1 ns      │ 219.2 ns      │ 219.1 ns      │ 100     │ 100000
   ├─ day4_part1  381.9 ns      │ 452.4 ns      │ 409.4 ns      │ 409 ns        │ 100     │ 100000
   ├─ day4_part2  397.2 ns      │ 479.7 ns      │ 423.6 ns      │ 424.1 ns      │ 100     │ 100000
   ├─ day5_part1  801.1 ns      │ 866 ns        │ 815.6 ns      │ 817.3 ns      │ 100     │ 100000
   ╰─ day5_part2  1.448 µs      │ 1.649 µs      │ 1.47 µs       │ 1.474 µs      │ 100     │ 100000
```

on a Ryzen 7950X
