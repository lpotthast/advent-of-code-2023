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
│  ├─ day01_part1  27.63 µs      │ 30.23 µs      │ 28.77 µs      │ 28.84 µs      │ 100     │ 10000
│  ├─ day01_part2  25.28 µs      │ 28.58 µs      │ 27.51 µs      │ 27.48 µs      │ 100     │ 10000
│  ├─ day02_part1  17.8 µs       │ 21.6 µs       │ 18.1 µs       │ 18.23 µs      │ 100     │ 10000
│  ├─ day02_part2  17.94 µs      │ 20.34 µs      │ 18.51 µs      │ 18.6 µs       │ 100     │ 10000
│  ├─ day03_part1  34.73 µs      │ 39.31 µs      │ 35.23 µs      │ 35.31 µs      │ 100     │ 10000
│  ├─ day03_part2  23.72 µs      │ 41.95 µs      │ 24.06 µs      │ 25.66 µs      │ 100     │ 10000
│  ├─ day04_part1  26.08 µs      │ 46.37 µs      │ 30.41 µs      │ 30.98 µs      │ 100     │ 10000
│  ├─ day04_part2  29.08 µs      │ 32.12 µs      │ 30.4 µs       │ 30.35 µs      │ 100     │ 10000
│  ├─ day05_part1  6.694 µs      │ 8.494 µs      │ 7.005 µs      │ 6.993 µs      │ 100     │ 10000
│  ├─ day05_part2  17.37 µs      │ 31.91 µs      │ 18.8 µs       │ 19.43 µs      │ 100     │ 10000
│  ├─ day06_part1  121.8 ns      │ 181.8 ns      │ 122.8 ns      │ 123.9 ns      │ 100     │ 10000
│  ├─ day06_part2  183.8 ns      │ 299.8 ns      │ 189.8 ns      │ 193.9 ns      │ 100     │ 10000
│  ├─ day07_part1  100.5 µs      │ 120.8 µs      │ 101.4 µs      │ 101.8 µs      │ 100     │ 10000
│  ├─ day07_part2  100.9 µs      │ 112.2 µs      │ 102.3 µs      │ 102.7 µs      │ 100     │ 10000
│  ├─ day08_part1  226.2 µs      │ 260.7 µs      │ 234.7 µs      │ 236.3 µs      │ 100     │ 5000
│  ├─ day08_part2  514.4 µs      │ 564.7 µs      │ 519.9 µs      │ 521.1 µs      │ 100     │ 5000
│  ├─ day09_part1  40.4 µs       │ 43.94 µs      │ 42.62 µs      │ 42.46 µs      │ 100     │ 10000
│  ├─ day09_part2  38.01 µs      │ 67.05 µs      │ 44.92 µs      │ 46.78 µs      │ 100     │ 10000
│  ├─ day10_part1  177.2 µs      │ 226.4 µs      │ 181.8 µs      │ 183.1 µs      │ 100     │ 1000
│  ╰─ day10_part2  706.2 µs      │ 901.3 µs      │ 727.7 µs      │ 744.9 µs      │ 100     │ 1000
╰─ test_input                    │               │               │               │         │
   ├─ day01_part1  60.82 ns      │ 214.8 ns      │ 61.82 ns      │ 63.34 ns      │ 100     │ 10000
   ├─ day01_part2  126.8 ns      │ 218.8 ns      │ 137.8 ns      │ 139 ns        │ 100     │ 10000
   ├─ day02_part1  522.8 ns      │ 572.8 ns      │ 527.8 ns      │ 529.5 ns      │ 100     │ 10000
   ├─ day02_part2  511.8 ns      │ 1.002 µs      │ 516.8 ns      │ 571.1 ns      │ 100     │ 10000
   ├─ day03_part1  245.8 ns      │ 531.8 ns      │ 251.8 ns      │ 260.6 ns      │ 100     │ 10000
   ├─ day03_part2  210.8 ns      │ 547.8 ns      │ 213.8 ns      │ 240.8 ns      │ 100     │ 10000
   ├─ day04_part1  420.8 ns      │ 1.131 µs      │ 780.8 ns      │ 656.1 ns      │ 100     │ 10000
   ├─ day04_part2  447.8 ns      │ 783.8 ns      │ 451.3 ns      │ 482.8 ns      │ 100     │ 10000
   ├─ day05_part1  769.8 ns      │ 1.013 µs      │ 796.8 ns      │ 801.6 ns      │ 100     │ 10000
   ├─ day05_part2  1.381 µs      │ 1.812 µs      │ 1.401 µs      │ 1.414 µs      │ 100     │ 10000
   ├─ day06_part1  94.82 ns      │ 136.8 ns      │ 95.82 ns      │ 96.18 ns      │ 100     │ 10000
   ├─ day06_part2  127.8 ns      │ 247.8 ns      │ 132.8 ns      │ 166 ns        │ 100     │ 10000
   ├─ day07_part1  273.8 ns      │ 332.8 ns      │ 279.8 ns      │ 281.8 ns      │ 100     │ 10000
   ├─ day07_part2  271.8 ns      │ 400.8 ns      │ 276.3 ns      │ 279.7 ns      │ 100     │ 10000
   ├─ day08_part1  801.8 ns      │ 1.267 µs      │ 821.8 ns      │ 849.1 ns      │ 100     │ 5000
   ├─ day08_part2  1.045 µs      │ 1.905 µs      │ 1.091 µs      │ 1.188 µs      │ 100     │ 5000
   ├─ day09_part1  143.8 ns      │ 213.8 ns      │ 144.8 ns      │ 148.1 ns      │ 100     │ 10000
   ├─ day09_part2  164.8 ns      │ 247.8 ns      │ 166.8 ns      │ 175.2 ns      │ 100     │ 10000
   ├─ day10_part1  289.8 ns      │ 849.8 ns      │ 304.8 ns      │ 323.8 ns      │ 100     │ 1000
   ╰─ day10_part2  2.199 µs      │ 3.659 µs      │ 2.219 µs      │ 2.266 µs      │ 100     │ 1000
```

on a Ryzen 7950X
