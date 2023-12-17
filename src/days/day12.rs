use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use smallvec::SmallVec;

pub fn part1(input: &str) -> u64 {
    parse_entries(input, 0).map(|entry| count_arrangements(entry)).sum()
}

pub fn part2(input: &str) -> u64 {
    parse_entries(input, 4).map(|entry| count_arrangements(entry)).sum()
}

fn count_arrangements(entry: Entry) -> u64 {
    let mut cache = HashMap::new();
    possible_arrangements_for_section(&entry, &mut cache, 0, 0)
}

fn possible_arrangements_for_section(
    entry: &Entry,
    cache: &mut HashMap<(usize, usize), u64>,
    spring_ix: usize,
    group_ix: usize,
) -> u64 {
    if let Some(cached_value) = cache.get(&(spring_ix, group_ix)) {
        return *cached_value;
    }

    // check if the current group can be satisfied from this position:
    let arrangements_when_consumed = entry.groups.get(group_ix).map_or(0, |group_len| {
        let group_len = *group_len as usize;

        // group is long enough to fit within remaining springs
        if (spring_ix + group_len) > entry.springs.len() {
            return 0;
        }

        // group does not contain Operational springs
        if (0..group_len).any(|p3os| entry.springs.get(spring_ix + pos) == Some(&Condition::Operational)) {
            return 0;
        }

        // item after group is not a Damaged spring
        if entry.springs.get(spring_ix + group_len) == Some(&Condition::Damaged) {
            return 0;
        }

        // if none of the above checks failed, we have a group which we can consume
        possible_arrangements_for_section(entry, cache, spring_ix + group_len + 1, group_ix + 1)
    });

    // also check if we can skip this position
    let arrangements_when_skipped = match entry.springs.get(spring_ix) {
        None => u64::from(group_ix >= entry.groups.len()),
        Some(Condition::Damaged) => 0,
        Some(_) => possible_arrangements_for_section(entry, cache, spring_ix + 1, group_ix),
    };

    let arrangements = arrangements_when_consumed + arrangements_when_skipped;
    cache.insert((spring_ix, group_ix), arrangements);
    arrangements
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        })
    }
}

#[derive(Debug)]
struct Entry {
    springs: SmallVec<[Condition; 32]>,
    groups: SmallVec<[u8; 8]>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Entry: ")?;
        for r in &self.springs {
            r.fmt(f)?;
        }
        f.write_char(' ')?;
        f.debug_list().entries(self.groups.iter()).finish()
    }
}

fn parse_entries(input: &str, repetitions: usize) -> impl Iterator<Item = Entry> + '_ {
    input.lines().map(move |line| {
        let (springs, groups) = line.split_once(' ').expect("one space");
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => panic!("Unexpected condition character"),
            })
            .collect::<SmallVec<[Condition; 32]>>();
        let groups = groups
            .split(',')
            .map(|num| num.parse::<u8>().expect("valid u8 number"))
            .collect::<SmallVec<[u8; 8]>>();

        match repetitions {
            0 => Entry { springs, groups },
            _ => {
                let repeated_springs = (0..=repetitions)
                    .enumerate()
                    .map(|(rep, _)| {
                        if rep == repetitions {
                            springs.iter().chain(&[]).copied()
                        } else {
                            springs.iter().chain(&[Condition::Unknown]).copied()
                        }
                    })
                    .flatten()
                    .collect();
                let repeated_groups = (0..=repetitions).map(|_| groups.iter().copied()).flatten().collect();
                Entry {
                    springs: repeated_springs,
                    groups: repeated_groups,
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use smallvec::SmallVec;
    use tracing_test::traced_test;

    use super::count_arrangements;
    use super::parse_entries;
    use super::Condition;
    use super::Entry;

    const D: Condition = Condition::Damaged;
    const O: Condition = Condition::Operational;
    const U: Condition = Condition::Unknown;

    #[test]
    fn test_parse_entries_without_repetition() {
        let parsed = parse_entries("###.## 3,2", 0).next().unwrap();
        assert_eq!(
            parsed.springs,
            [D, D, D, O, D, D].into_iter().collect::<SmallVec<[Condition; 32]>>()
        );
        assert_eq!(parsed.groups, [3, 2].into_iter().collect::<SmallVec<[u8; 8]>>());
    }

    #[test]
    fn test_parse_entries_with_repetition() {
        let parsed = parse_entries("###.## 3,2", 1).next().unwrap();
        assert_eq!(
            parsed.springs,
            [D, D, D, O, D, D, U, D, D, D, O, D, D]
                .into_iter()
                .collect::<SmallVec<[Condition; 32]>>()
        );
        assert_eq!(parsed.groups, [3, 2, 3, 2].into_iter().collect::<SmallVec<[u8; 8]>>());
    }

    #[traced_test]
    #[test]
    fn test_count_arrangements() {
        fn e(records: &[Condition], groups: &[u8]) -> Entry {
            Entry {
                springs: records.iter().copied().collect(),
                groups: groups.iter().copied().collect(),
            }
        }

        assert_eq!(count_arrangements(e(&[U, U, O, U, U], &[1, 1]).view()), 4); // ??.?? 1,1
        assert_eq!(
            count_arrangements(e(&[U, U, D, U, O, U, U, U, U, U], &[4, 2]).view()),
            4
        ); // ??#?.????? 4,2
        assert_eq!(
            count_arrangements(e(&[U, U, U, D, U, O, U, O, U, U], &[5, 1, 1]).view()),
            2
        ); // ???#?.?.?? 5,1,1
        assert_eq!(
            count_arrangements(e(&[U, U, U, U, U, U, D, D, U, U], &[2, 5]).view()),
            6
        ); // ??????##?? 2,5
        assert_eq!(
            count_arrangements(e(&[U, U, U, U, U, U, D, D, U, U, U, U, U, D, D], &[2, 5, 4]).view(),),
            6
        ); // ??????##?????## 2,5,4
        assert_eq!(
            count_arrangements(e(&[U, U, U, U, U, U, D, D, U, U, U, U, U, D, D], &[2, 5, 4]).view(),),
            6
        ); // ??????##?????## 2,5,4
        assert_eq!(
            count_arrangements(e(&[U, U, U, D, U, U, O, D, U, U], &[2, 1, 2]).view()),
            2
        ); // ???#??.#?? 2,1,2
        assert_eq!(
            count_arrangements(e(&[U, U, D, U, U, U, O, U, U, U, D, U, O, O, O, D, D], &[3, 1, 3, 2]).view(),),
            9
        ); // ??#???.???#?...## 3,1,3,2
        assert_eq!(
            count_arrangements(e(&[U, U, D, U, U, U, O, U, U, U, D, U], &[3, 1, 3]).view(),),
            9
        ); // ??#???.???#? 3,1,3
    }
}
