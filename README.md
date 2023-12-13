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
│  ├─ day01_part1  22.17 µs      │ 30.14 µs      │ 23.14 µs      │ 23.56 µs      │ 100     │ 10000
│  ├─ day01_part2  24.26 µs      │ 28.47 µs      │ 26.27 µs      │ 25.88 µs      │ 100     │ 10000
│  ├─ day02_part1  17.93 µs      │ 22.09 µs      │ 19.1 µs       │ 19.12 µs      │ 100     │ 10000
│  ├─ day02_part2  18.61 µs      │ 28.77 µs      │ 20.07 µs      │ 20.33 µs      │ 100     │ 10000
│  ├─ day03_part1  27.46 µs      │ 28.52 µs      │ 27.88 µs      │ 27.89 µs      │ 100     │ 10000
│  ├─ day03_part2  23.63 µs      │ 29.92 µs      │ 28.38 µs      │ 27.99 µs      │ 100     │ 10000
│  ├─ day04_part1  24.11 µs      │ 32.93 µs      │ 29.28 µs      │ 29.03 µs      │ 100     │ 10000
│  ├─ day04_part2  27.72 µs      │ 38.78 µs      │ 28.77 µs      │ 29.18 µs      │ 100     │ 10000
│  ├─ day05_part1  7.664 µs      │ 9.625 µs      │ 8.006 µs      │ 8.058 µs      │ 100     │ 10000
│  ├─ day05_part2  18.28 µs      │ 21.61 µs      │ 19.09 µs      │ 19.24 µs      │ 100     │ 10000
│  ├─ day06_part1  120.8 ns      │ 175.8 ns      │ 121.8 ns      │ 122.6 ns      │ 100     │ 10000
│  ├─ day06_part2  182.8 ns      │ 236.8 ns      │ 187.8 ns      │ 189.1 ns      │ 100     │ 10000
│  ├─ day07_part1  100.8 µs      │ 105.3 µs      │ 101.8 µs      │ 102 µs        │ 100     │ 10000
│  ├─ day07_part2  101.2 µs      │ 110.3 µs      │ 102.4 µs      │ 102.7 µs      │ 100     │ 10000
│  ├─ day08_part1  225.1 µs      │ 250.9 µs      │ 229.7 µs      │ 231.7 µs      │ 100     │ 5000
│  ├─ day08_part2  487.7 µs      │ 518.1 µs      │ 495.4 µs      │ 496.1 µs      │ 100     │ 5000
│  ├─ day09_part1  40.68 µs      │ 53.93 µs      │ 45.5 µs       │ 45.06 µs      │ 100     │ 10000
│  ├─ day09_part2  39.98 µs      │ 50.03 µs      │ 45.55 µs      │ 45.9 µs       │ 100     │ 10000
│  ├─ day10_part1  178.4 µs      │ 203.6 µs      │ 185.3 µs      │ 185 µs        │ 100     │ 1000
│  ├─ day10_part2  710.1 µs      │ 732.4 µs      │ 717.9 µs      │ 717.3 µs      │ 100     │ 1000
│  ├─ day11_part1  238.5 µs      │ 241.8 µs      │ 239.1 µs      │ 239.2 µs      │ 100     │ 10000
│  ╰─ day11_part2  234 µs        │ 242.6 µs      │ 234.9 µs      │ 235.7 µs      │ 100     │ 10000
╰─ test_input                    │               │               │               │         │
   ├─ day01_part1  58.82 ns      │ 102.8 ns      │ 59.82 ns      │ 60.41 ns      │ 100     │ 10000
   ├─ day01_part2  117.8 ns      │ 236.8 ns      │ 118.8 ns      │ 121.2 ns      │ 100     │ 10000
   ├─ day02_part1  507.8 ns      │ 626.8 ns      │ 520.8 ns      │ 525 ns        │ 100     │ 10000
   ├─ day02_part2  484.8 ns      │ 1.262 µs      │ 489.8 ns      │ 535.1 ns      │ 100     │ 10000
   ├─ day03_part1  244.8 ns      │ 345.8 ns      │ 247.8 ns      │ 254.9 ns      │ 100     │ 10000
   ├─ day03_part2  207.8 ns      │ 532.8 ns      │ 213.8 ns      │ 218.3 ns      │ 100     │ 10000
   ├─ day04_part1  392.8 ns      │ 436.8 ns      │ 395.8 ns      │ 396.6 ns      │ 100     │ 10000
   ├─ day04_part2  409.8 ns      │ 569.8 ns      │ 411.8 ns      │ 414.7 ns      │ 100     │ 10000
   ├─ day05_part1  771.8 ns      │ 1.014 µs      │ 805.8 ns      │ 801.2 ns      │ 100     │ 10000
   ├─ day05_part2  1.373 µs      │ 1.799 µs      │ 1.416 µs      │ 1.426 µs      │ 100     │ 10000
   ├─ day06_part1  91.82 ns      │ 252.8 ns      │ 92.82 ns      │ 109.8 ns      │ 100     │ 10000
   ├─ day06_part2  125.8 ns      │ 170.8 ns      │ 128.8 ns      │ 129.1 ns      │ 100     │ 10000
   ├─ day07_part1  266.8 ns      │ 402.8 ns      │ 269.8 ns      │ 276.5 ns      │ 100     │ 10000
   ├─ day07_part2  265.8 ns      │ 442.8 ns      │ 270.8 ns      │ 296.1 ns      │ 100     │ 10000
   ├─ day08_part1  785.8 ns      │ 1.011 µs      │ 805.8 ns      │ 814 ns        │ 100     │ 5000
   ├─ day08_part2  1.011 µs      │ 1.845 µs      │ 1.041 µs      │ 1.104 µs      │ 100     │ 5000
   ├─ day09_part1  151.8 ns      │ 256.8 ns      │ 154.8 ns      │ 157.9 ns      │ 100     │ 10000
   ├─ day09_part2  156.8 ns      │ 223.8 ns      │ 160.8 ns      │ 164.9 ns      │ 100     │ 10000
   ├─ day10_part1  279.8 ns      │ 719.8 ns      │ 289.8 ns      │ 297.6 ns      │ 100     │ 1000
   ├─ day10_part2  2.179 µs      │ 3.569 µs      │ 2.189 µs      │ 2.205 µs      │ 100     │ 1000
   ├─ day11_part1  571.8 ns      │ 887.8 ns      │ 614.8 ns      │ 606.9 ns      │ 100     │ 10000
   ╰─ day11_part2  571.8 ns      │ 690.8 ns      │ 579.8 ns      │ 583.8 ns      │ 100     │ 10000
```

on a Ryzen 7950X
