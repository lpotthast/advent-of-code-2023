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
benchmarks         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                    │               │               │               │         │
│  ├─ day01_part1  27.07 µs      │ 30.59 µs      │ 28.27 µs      │ 28.3 µs       │ 100     │ 10000
│  ├─ day01_part2  26.32 µs      │ 29.33 µs      │ 26.63 µs      │ 26.68 µs      │ 100     │ 10000
│  ├─ day02_part1  17.68 µs      │ 22.04 µs      │ 18.53 µs      │ 18.62 µs      │ 100     │ 10000
│  ├─ day02_part2  17.77 µs      │ 23.4 µs       │ 18.1 µs       │ 18.25 µs      │ 100     │ 10000
│  ├─ day03_part1  34.06 µs      │ 37.8 µs       │ 34.46 µs      │ 34.57 µs      │ 100     │ 10000
│  ├─ day03_part2  27.64 µs      │ 29.07 µs      │ 27.93 µs      │ 27.95 µs      │ 100     │ 10000
│  ├─ day04_part1  29.36 µs      │ 30.94 µs      │ 30.26 µs      │ 30.24 µs      │ 100     │ 10000
│  ├─ day04_part2  30.46 µs      │ 34.86 µs      │ 32.33 µs      │ 32.36 µs      │ 100     │ 10000
│  ├─ day05_part1  7.532 µs      │ 9.375 µs      │ 7.669 µs      │ 7.696 µs      │ 100     │ 10000
│  ├─ day05_part2  18 µs         │ 20.43 µs      │ 18.24 µs      │ 18.27 µs      │ 100     │ 10000
│  ├─ day06_part1  121.8 ns      │ 181.8 ns      │ 122.8 ns      │ 123.8 ns      │ 100     │ 10000
│  ├─ day06_part2  177.8 ns      │ 214.8 ns      │ 184.8 ns      │ 185.1 ns      │ 100     │ 10000
│  ├─ day07_part1  96.98 µs      │ 105.9 µs      │ 99.68 µs      │ 100 µs        │ 100     │ 10000
│  ├─ day07_part2  98.07 µs      │ 101.3 µs      │ 98.96 µs      │ 99.16 µs      │ 100     │ 10000
│  ├─ day08_part1  232.3 µs      │ 278.4 µs      │ 236.8 µs      │ 239.1 µs      │ 100     │ 5000
│  ├─ day08_part2  494.1 µs      │ 512.6 µs      │ 499 µs        │ 499.5 µs      │ 100     │ 5000
│  ├─ day09_part1  48.23 µs      │ 82.03 µs      │ 51.72 µs      │ 51.5 µs       │ 100     │ 10000
│  ├─ day09_part2  46.81 µs      │ 64.29 µs      │ 53.31 µs      │ 52.02 µs      │ 100     │ 10000
│  ├─ day10_part1  170.5 µs      │ 212.1 µs      │ 181.4 µs      │ 182.6 µs      │ 100     │ 1000
│  ├─ day10_part2  709.4 µs      │ 944.6 µs      │ 721.7 µs      │ 726.2 µs      │ 100     │ 1000
│  ├─ day11_part1  154.6 µs      │ 207.2 µs      │ 159.1 µs      │ 159.5 µs      │ 100     │ 10000
│  ╰─ day11_part2  156.9 µs      │ 199.4 µs      │ 159.4 µs      │ 160.4 µs      │ 100     │ 10000
╰─ test_input                    │               │               │               │         │
   ├─ day01_part1  58.82 ns      │ 120.8 ns      │ 59.82 ns      │ 75.11 ns      │ 100     │ 10000
   ├─ day01_part2  121.8 ns      │ 188.8 ns      │ 122.8 ns      │ 123.1 ns      │ 100     │ 10000
   ├─ day02_part1  515.8 ns      │ 564.8 ns      │ 522.3 ns      │ 523.5 ns      │ 100     │ 10000
   ├─ day02_part2  503.8 ns      │ 909.8 ns      │ 521.8 ns      │ 543 ns        │ 100     │ 10000
   ├─ day03_part1  253.8 ns      │ 319.8 ns      │ 264.8 ns      │ 264.5 ns      │ 100     │ 10000
   ├─ day03_part2  210.8 ns      │ 364.8 ns      │ 219.8 ns      │ 227.5 ns      │ 100     │ 10000
   ├─ day04_part1  391.8 ns      │ 794.8 ns      │ 413.8 ns      │ 414.3 ns      │ 100     │ 10000
   ├─ day04_part2  419.8 ns      │ 703.8 ns      │ 434.8 ns      │ 434.7 ns      │ 100     │ 10000
   ├─ day05_part1  802.8 ns      │ 934.8 ns      │ 817.8 ns      │ 829.3 ns      │ 100     │ 10000
   ├─ day05_part2  1.445 µs      │ 1.912 µs      │ 1.474 µs      │ 1.489 µs      │ 100     │ 10000
   ├─ day06_part1  92.82 ns      │ 165.8 ns      │ 92.82 ns      │ 94.6 ns       │ 100     │ 10000
   ├─ day06_part2  129.8 ns      │ 171.8 ns      │ 130.8 ns      │ 131.5 ns      │ 100     │ 10000
   ├─ day07_part1  239.8 ns      │ 410.8 ns      │ 244.8 ns      │ 251.4 ns      │ 100     │ 10000
   ├─ day07_part2  242.8 ns      │ 391.8 ns      │ 245.8 ns      │ 257.8 ns      │ 100     │ 10000
   ├─ day08_part1  827.8 ns      │ 1.047 µs      │ 837.8 ns      │ 841.5 ns      │ 100     │ 5000
   ├─ day08_part2  1.061 µs      │ 1.515 µs      │ 1.079 µs      │ 1.084 µs      │ 100     │ 5000
   ├─ day09_part1  144.8 ns      │ 200.8 ns      │ 145.8 ns      │ 147.4 ns      │ 100     │ 10000
   ├─ day09_part2  158.8 ns      │ 726.8 ns      │ 163.8 ns      │ 174.3 ns      │ 100     │ 10000
   ├─ day10_part1  269.8 ns      │ 959.8 ns      │ 279.8 ns      │ 285.5 ns      │ 100     │ 1000
   ├─ day10_part2  2.059 µs      │ 3.509 µs      │ 2.079 µs      │ 2.178 µs      │ 100     │ 1000
   ├─ day11_part1  558.8 ns      │ 638.8 ns      │ 570.8 ns      │ 574.2 ns      │ 100     │ 10000
   ╰─ day11_part2  561.8 ns      │ 1.05 µs       │ 567.8 ns      │ 579 ns        │ 100     │ 10000
```

on a Ryzen 7950X
