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
│  ├─ day1_part1  23.47 µs      │ 29.84 µs      │ 27.83 µs      │ 27.75 µs      │ 100     │ 100000
│  ├─ day1_part2  25.75 µs      │ 26.78 µs      │ 26.22 µs      │ 26.24 µs      │ 100     │ 100000
│  ├─ day2_part1  16.84 µs      │ 19.87 µs      │ 18.41 µs      │ 18.57 µs      │ 100     │ 100000
│  ├─ day2_part2  16.97 µs      │ 19.69 µs      │ 18.36 µs      │ 18.58 µs      │ 100     │ 100000
│  ├─ day3_part1  35.48 µs      │ 37.73 µs      │ 37.06 µs      │ 36.99 µs      │ 100     │ 100000
│  ├─ day3_part2  27.35 µs      │ 29.07 µs      │ 28.22 µs      │ 28.24 µs      │ 100     │ 100000
│  ├─ day4_part1  29.72 µs      │ 31.91 µs      │ 30.43 µs      │ 30.54 µs      │ 100     │ 100000
│  ├─ day4_part2  30.61 µs      │ 32.76 µs      │ 31.39 µs      │ 31.57 µs      │ 100     │ 100000
│  ├─ day5_part1  7.562 µs      │ 7.963 µs      │ 7.68 µs       │ 7.721 µs      │ 100     │ 100000
│  ├─ day5_part2  17.84 µs      │ 19.76 µs      │ 18.7 µs       │ 18.66 µs      │ 100     │ 100000
│  ├─ day6_part1  116.2 ns      │ 145.3 ns      │ 117.7 ns      │ 120.2 ns      │ 100     │ 100000
│  ├─ day6_part2  176.1 ns      │ 254.4 ns      │ 180.2 ns      │ 186.1 ns      │ 100     │ 100000
│  ├─ day7_part1  97.36 µs      │ 100.9 µs      │ 98.29 µs      │ 98.43 µs      │ 100     │ 100000
│  ├─ day7_part2  96.98 µs      │ 99.52 µs      │ 97.45 µs      │ 97.63 µs      │ 100     │ 100000
│  ├─ day8_part1  223.7 µs      │ 249.5 µs      │ 241.3 µs      │ 239.6 µs      │ 100     │ 100000
│  ╰─ day8_part2  504.6 µs      │ 525.3 µs      │ 514.8 µs      │ 514.3 µs      │ 100     │ 100000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  60.82 ns      │ 70.72 ns      │ 64.12 ns      │ 64.21 ns      │ 100     │ 100000
   ├─ day1_part2  119.9 ns      │ 124.5 ns      │ 120.1 ns      │ 120.4 ns      │ 100     │ 100000
   ├─ day2_part1  498.4 ns      │ 776.1 ns      │ 527.2 ns      │ 530.2 ns      │ 100     │ 100000
   ├─ day2_part2  490.2 ns      │ 661.4 ns      │ 520.6 ns      │ 522.8 ns      │ 100     │ 100000
   ├─ day3_part1  250.1 ns      │ 326.4 ns      │ 259 ns        │ 260.4 ns      │ 100     │ 100000
   ├─ day3_part2  215.6 ns      │ 255.7 ns      │ 220.9 ns      │ 223 ns        │ 100     │ 100000
   ├─ day4_part1  407 ns        │ 459.7 ns      │ 431.7 ns      │ 426.7 ns      │ 100     │ 100000
   ├─ day4_part2  431.5 ns      │ 619.8 ns      │ 460.3 ns      │ 462.4 ns      │ 100     │ 100000
   ├─ day5_part1  797.6 ns      │ 1.138 µs      │ 823.6 ns      │ 829.4 ns      │ 100     │ 100000
   ├─ day5_part2  1.388 µs      │ 1.444 µs      │ 1.412 µs      │ 1.41 µs       │ 100     │ 100000
   ├─ day6_part1  92.22 ns      │ 98.82 ns      │ 92.92 ns      │ 93.06 ns      │ 100     │ 100000
   ├─ day6_part2  124.4 ns      │ 167.6 ns      │ 125.9 ns      │ 129.2 ns      │ 100     │ 100000
   ├─ day7_part1  250.9 ns      │ 346.9 ns      │ 255.3 ns      │ 259 ns        │ 100     │ 100000
   ├─ day7_part2  250.2 ns      │ 388.8 ns      │ 254.5 ns      │ 262.3 ns      │ 100     │ 100000
   ├─ day8_part1  969 ns        │ 1.129 µs      │ 996.4 ns      │ 996.3 ns      │ 100     │ 100000
   ╰─ day8_part2  1.29 µs       │ 1.444 µs      │ 1.311 µs      │ 1.314 µs      │ 100     │ 100000
```

on a Ryzen 7950X
