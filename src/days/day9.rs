use itertools::Itertools;
use smallvec::SmallVec;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut hist = SmallVec::<[i64; 32]>::from_iter(read_history(line));

            let mut next_len = hist.len() - 1;
            let mut slice = &mut hist[0..=next_len];

            while slice.iter().find(|it| **it != 0).is_some() {
                for (i, j) in (0..slice.len()).tuple_windows::<(_, _)>() {
                    slice[i] = slice[j] - slice[i];
                }
                next_len = slice.len() - 2;
                slice = &mut hist[0..=next_len];
            }

            hist[next_len - 1..hist.len()].iter().sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    0
}

struct History {}

fn read_history(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split_ascii_whitespace()
        .map(|section| section.parse::<i64>().unwrap())
}
