use std::str::Lines;

const INPUT: &str = include_str!("../res/day3.txt");

#[tracing::instrument]
pub fn part1() -> u64 {
    Symbols::new(INPUT)
        .map(|symbol| symbol.engine_parts().sum::<u64>())
        .sum()
}

#[tracing::instrument]
pub fn part2() -> u64 {
    Symbols::new(INPUT)
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| symbol.engine_parts())
        .filter(|parts| parts.len() == 2)
        .map(|mut parts| {
            let first = parts.next().expect("part 1");
            let second = parts.next().expect("part 2");
            first * second
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Symbol<'a> {
    symbol: char,
    symbol_idx: usize,

    above: Option<&'a str>,
    current: &'a str,
    below: Option<&'a str>,
}

impl<'a> Symbol<'a> {
    fn engine_parts(&self) -> EngineParts {
        EngineParts::new(self.symbol_idx, self.above, self.current, self.below)
    }
}

struct SlidingWindow<'a> {
    above: Option<&'a str>,
    current: &'a str,
    below: Option<&'a str>,
    line_iter: Lines<'a>,
}

impl<'a> SlidingWindow<'a> {
    fn new(input: &'a str) -> Self {
        let mut line_iter = input.lines();
        Self {
            above: None,
            current: line_iter.next().expect("at least one line"),
            below: line_iter.next(),
            line_iter,
        }
    }

    /// Returns true if end of input is already reached.
    fn advance(&mut self) -> bool {
        if let Some(below) = self.below {
            self.above = Some(self.current);
            self.current = below;
            self.below = self.line_iter.next();
            false
        } else {
            true
        }
    }
}

struct Symbols<'a> {
    win: SlidingWindow<'a>,
    last_symbol_idx: usize,
}

impl<'a> Symbols<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            win: SlidingWindow::new(input),
            last_symbol_idx: 0,
        }
    }
}

impl<'a> Iterator for Symbols<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_symbol_idx < self.win.current.len() {
            self.last_symbol_idx += 1;
        }
        loop {
            if let Some((i, symbol)) = self.win.current[self.last_symbol_idx..]
                .chars()
                .enumerate()
                .find(|(_i, c)| !matches!(c, '0'..='9' | '.'))
            {
                self.last_symbol_idx += i;
                return Some(Symbol {
                    symbol,
                    symbol_idx: self.last_symbol_idx,
                    above: self.win.above,
                    current: self.win.current,
                    below: self.win.below,
                });
            }
            self.last_symbol_idx = 0;
            if self.win.advance() {
                return None;
            }
        }
    }
}

struct EngineParts {
    parts: [Option<u64>; 6],
    next_part: usize,
}

impl EngineParts {
    fn new<'a>(
        symbol_idx: usize,
        above: Option<&'a str>,
        current: &'a str,
        below: Option<&'a str>,
    ) -> Self {
        let (above_a, above_b) = find_in_touching_line(above, symbol_idx);
        let left = find_left(current, symbol_idx);
        let right = find_right(current, symbol_idx);
        let (below_a, below_b) = find_in_touching_line(below, symbol_idx);
        Self {
            parts: [above_a, above_b, left, right, below_a, below_b],
            next_part: 0,
        }
    }
}

impl Iterator for EngineParts {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_part = None;
        while next_part.is_none() {
            if self.next_part < 6 {
                next_part = self.parts[self.next_part];
                self.next_part += 1;
            } else {
                break;
            }
        }
        next_part
    }
}

impl ExactSizeIterator for EngineParts {
    fn len(&self) -> usize {
        self.parts.iter().filter(|it| it.is_some()).count()
    }
}

fn find_left(current: &str, symbol_idx: usize) -> Option<u64> {
    if symbol_idx == 0 {
        return None;
    }
    current[..symbol_idx]
        .chars()
        .next_back()
        .filter(char::is_ascii_digit)
        .map(|_| read_num(current, symbol_idx - 1))
}

fn find_right(current: &str, symbol_idx: usize) -> Option<u64> {
    if symbol_idx == current.len() - 1 {
        return None;
    }
    current[symbol_idx + 1..]
        .chars()
        .next()
        .filter(char::is_ascii_digit)
        .map(|_| read_num(current, symbol_idx + 1))
}

fn find_in_touching_line(
    touching_line: Option<&str>,
    symbol_idx: usize,
) -> (Option<u64>, Option<u64>) {
    match touching_line {
        Some(touching_line) => {
            let left = if symbol_idx > 0 {
                symbol_idx - 1
            } else {
                symbol_idx
            };
            let right = if symbol_idx < touching_line.len() - 1 {
                symbol_idx + 1
            } else {
                symbol_idx
            };
            let mut chars_touching = touching_line[left..=right].chars();

            let l = chars_touching.next();
            let m = chars_touching.next();
            let r = chars_touching.next();

            match m
                .filter(char::is_ascii_digit)
                .map(|_m| read_num(touching_line, symbol_idx))
            {
                Some(only_possible_num) => (Some(only_possible_num), None),
                None => {
                    let l = l
                        .filter(char::is_ascii_digit)
                        .map(|_| read_num(touching_line, left));
                    let r = r
                        .filter(char::is_ascii_digit)
                        .map(|_| read_num(touching_line, right));
                    (l, r)
                }
            }
        }
        None => (None, None),
    }
}

fn read_num(input: &str, idx: usize) -> u64 {
    let min_idx = idx
        - input[..idx]
            .chars()
            .rev()
            .enumerate()
            .find(|(_i, c)| !c.is_ascii_digit())
            .map_or(idx, |(i, _c)| i);
    let max_idx = idx
        + input[idx..]
            .chars()
            .enumerate()
            .find(|(_i, c)| !c.is_ascii_digit())
            .map_or_else(|| input.len() - idx, |(i, _c)| i);
    str::parse::<u64>(&input[min_idx..max_idx]).expect("number")
}

#[cfg(test)]
mod test {
    use crate::day3::read_num;

    #[test]
    fn test_read_num() {
        assert_eq!(read_num("123...", 0), 123);
        assert_eq!(read_num("123...", 1), 123);
        assert_eq!(read_num("123...", 2), 123);

        assert_eq!(read_num("...123", 3), 123);
        assert_eq!(read_num("...123", 4), 123);
        assert_eq!(read_num("...123", 5), 123);

        assert_eq!(read_num("...123...", 3), 123);
        assert_eq!(read_num("...123...", 4), 123);
        assert_eq!(read_num("...123...", 5), 123);

        assert_eq!(read_num("...123.456..", 5), 123);
        assert_eq!(read_num("...123.456..", 7), 456);
    }
}
