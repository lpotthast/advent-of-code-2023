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
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
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

struct Symbols<'a> {
    line_iter: Lines<'a>,

    /// Line above.
    above: Option<&'a str>,
    /// Current line.
    current: &'a str,
    /// Line below.
    below: Option<&'a str>,

    current_symbol_idx: usize,
    current_symbol: Option<char>,
}

impl<'a> Symbols<'a> {
    fn new(input: &'a str) -> Self {
        let mut line_iter = input.lines();
        let current = line_iter.next().expect("at least one line");
        let below = line_iter.next();
        Self {
            line_iter,
            above: None,
            current,
            below,
            current_symbol_idx: 0,
            current_symbol: None,
        }
    }

    /// Returns false if end of input was reached / there is no more line.
    fn advance_to_next_line(&mut self) -> bool {
        match self.below {
            Some(below) => {
                self.above = Some(self.current);
                self.current = below;
                self.below = self.line_iter.next();

                self.current_symbol_idx = 0;
                self.current_symbol = None;
                false
            }
            None => true,
        }
    }
}

impl<'a> Iterator for Symbols<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_symbol = None;
        if self.current_symbol_idx < self.current.len() {
            self.current_symbol_idx += 1;
        }

        while self.current_symbol.is_none() {
            let rest = &self.current[self.current_symbol_idx..];
            // tracing::info!(rest, "rest");
            // Search in the remainder of the current line.
            let sym = rest.chars().enumerate().find(|(_i, c)| match c {
                '0'..='9' | '.' => false,
                _ => true,
            });
            if let Some((i, c)) = sym {
                self.current_symbol_idx = self.current_symbol_idx + i;
                self.current_symbol = Some(c);
            } else {
                let end_reached = self.advance_to_next_line();
                if end_reached {
                    break;
                }
            }
        }

        match self.current_symbol {
            Some(symbol) => Some(Symbol {
                symbol,
                symbol_idx: self.current_symbol_idx,
                above: self.above,
                current: self.current,
                below: self.below,
            }),
            None => None,
        }
    }
}

struct EngineParts {
    pub(crate) parts: [Option<u64>; 6],
    next_part: usize,
}

impl EngineParts {
    fn new<'a>(
        symbol_idx: usize,
        above: Option<&'a str>,
        current: &'a str,
        below: Option<&'a str>,
    ) -> Self {
        let (above_a, above_b) = Self::find_in_touching_line(above, symbol_idx);
        let left = Self::find_left(current, symbol_idx);
        let right = Self::find_right(current, symbol_idx);
        let (below_a, below_b) = Self::find_in_touching_line(below, symbol_idx);

        Self {
            parts: [above_a, above_b, left, right, below_a, below_b],
            next_part: 0,
        }
    }

    fn find_left(current: &str, symbol_idx: usize) -> Option<u64> {
        if symbol_idx == 0 {
            return None;
        }
        current[..symbol_idx]
            .chars()
            .rev()
            .next()
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
                // tracing::info!(?l, ?m, ?r, "touching");

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

fn read_num(input: &str, idx: usize) -> u64 {
    let min_idx = idx
        - input[..idx]
            .chars()
            .rev()
            .enumerate()
            .skip_while(|(_i, c)| c.is_ascii_digit())
            .next()
            .map(|(i, _c)| i)
            .unwrap_or(idx);
    let max_idx = idx
        + input[idx..]
            .chars()
            .enumerate()
            .skip_while(|(_i, c)| c.is_ascii_digit())
            .next()
            .map(|(i, _c)| i)
            .unwrap_or(input.len() - idx);
    str::parse::<u64>(&input[min_idx..max_idx]).unwrap()
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
