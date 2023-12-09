use itertools::Itertools;
use smallvec::SmallVec;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut history: SmallVec<[i64; 32]> = read_history(line).collect();
            estimate_next(&mut history)
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut history: SmallVec<[i64; 32]> = read_history(line).collect();
            history.reverse();
            estimate_next(&mut history)
        })
        .sum()
}

fn estimate_next(history: &mut [i64]) -> i64 {
    let mut upper_bound = history.len();
    let mut slice = &mut history[0..upper_bound];

    while slice.iter().any(|it| *it != 0) {
        for (i, j) in (0..slice.len()).tuple_windows::<(_, _)>() {
            slice[i] = slice[j] - slice[i];
        }
        upper_bound = slice.len() - 1;
        slice = &mut history[0..upper_bound];
    }

    history[upper_bound - 1..history.len()].iter().sum::<i64>()
}

fn read_history(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split_ascii_whitespace()
        .map(|section| section.parse::<i64>().expect("number"))
}
