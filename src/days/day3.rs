use std::str::Lines;

pub fn part1(input: &str) -> u64 {
    Symbols::new(input)
        .map(|symbol| symbol.engine_parts().sum::<u64>())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    Symbols::new(input)
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

/*
// TODO: This would be an optimization...
pub fn part2_opt (input: &str) -> u64 {
    Symbols::new(input)
        .filter(|symbol| symbol.symbol == '*')
        .filter(|symbol| symbol.estimated_num_parts_will_be_greater_then(2))
        .map(|symbol| {
            let mut parts = symbol.engine_parts();
            let first = parts.next().expect("part 1");
            let second = parts.next().expect("part 2");
            first * second
        })
        .sum()
}
*/

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
        loop {
            if let Some((i, symbol)) = self.win.current[self.last_symbol_idx..]
                .chars()
                .enumerate()
                .find(|(_i, c)| !matches!(c, '0'..='9' | '.'))
            {
                self.last_symbol_idx += i;
                let symbol = Symbol {
                    symbol,
                    symbol_idx: self.last_symbol_idx,
                    above: self.win.above,
                    current: self.win.current,
                    below: self.win.below,
                };
                if self.last_symbol_idx < self.win.current.len() {
                    self.last_symbol_idx += 1; // would otherwise find same symbol again!
                }
                return Some(symbol);
            }
            self.last_symbol_idx = 0;
            let end_reached = self.win.advance();
            if end_reached {
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
    fn new<'a>(symbol_idx: usize, above: Option<&'a str>, current: &'a str, below: Option<&'a str>) -> Self {
        let (above_a, above_b) = match above {
            Some(above) => find_in_window(above, symbol_idx),
            None => (None, None),
        };
        let left = find_left(current, symbol_idx);
        let right = find_right(current, symbol_idx);
        let (below_a, below_b) = match below {
            Some(below) => find_in_window(below, symbol_idx),
            None => (None, None),
        };
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

/// Parse the full number to te left of `idx`.
fn find_left(line: &str, idx: usize) -> Option<u64> {
    if idx == 0 {
        return None;
    }
    line[..idx]
        .chars()
        .next_back()
        .filter(char::is_ascii_digit)
        .map(|_| parse_num(line, idx - 1))
}

/// Parse the full number to te right of `idx`.
fn find_right(line: &str, idx: usize) -> Option<u64> {
    if idx == line.len() - 1 {
        return None;
    }
    line[idx + 1..]
        .chars()
        .next()
        .filter(char::is_ascii_digit)
        .map(|_| parse_num(line, idx + 1))
}

/// Parse the full number(s) (potentially two) occurring around a three-character window
/// whose middle index is `idx`.
fn find_in_window(line: &str, idx: usize) -> (Option<u64>, Option<u64>) {
    let is_left_border = idx == 0;
    let is_right_border = idx == line.len() - 1;

    let left = if is_left_border { idx } else { idx - 1 };
    let right = if is_right_border { idx } else { idx + 1 };

    let mut chars_touching = line[left..=right].chars();
    let l = if is_left_border { None } else { chars_touching.next() };
    let m = chars_touching.next();
    let r = if is_right_border { None } else { chars_touching.next() };

    match m.filter(char::is_ascii_digit).map(|_m| parse_num(line, idx)) {
        Some(only_possible_num) => (Some(only_possible_num), None),
        None => {
            let l = l.filter(char::is_ascii_digit).map(|_| parse_num(line, left));
            let r = r.filter(char::is_ascii_digit).map(|_| parse_num(line, right));
            (l, r)
        }
    }
}

/// Parse the full number occupying at least `input[idx]`.
fn parse_num(input: &str, idx: usize) -> u64 {
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
    use super::parse_num;

    #[test]
    fn test_read_num() {
        assert_eq!(parse_num("123...", 0), 123);
        assert_eq!(parse_num("123...", 1), 123);
        assert_eq!(parse_num("123...", 2), 123);

        assert_eq!(parse_num("...123", 3), 123);
        assert_eq!(parse_num("...123", 4), 123);
        assert_eq!(parse_num("...123", 5), 123);

        assert_eq!(parse_num("...123...", 3), 123);
        assert_eq!(parse_num("...123...", 4), 123);
        assert_eq!(parse_num("...123...", 5), 123);

        assert_eq!(parse_num("...123.456..", 5), 123);
        assert_eq!(parse_num("...123.456..", 7), 456);
    }
}
