use std::str::Lines;

const INPUT: &str = include_str!("../res/day3.txt");

#[tracing::instrument]
pub fn part1() -> u64 {
    EnginePartIterator::new(INPUT).sum()
}

#[tracing::instrument]
pub fn part2() -> u64 {
    0
}

struct EnginePartIterator<'a> {
    line_iter: Lines<'a>,

    /// Line above.
    above: Option<&'a str>,
    /// Current line.
    current: &'a str,
    /// Line below.
    below: Option<&'a str>,

    current_symbol_idx: usize,
    current_symbol: Option<char>,

    found: [Option<u64>; 6],
    found_next: usize,
}

impl<'a> EnginePartIterator<'a> {
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
            found: [None; 6],
            found_next: 0,
        }
    }

    /// Returns false if end of input was reached / there is no more line.
    fn advance_to_next_line(&mut self) -> bool {
        // tracing::info!(below = self.below, "advance_to_next_line");
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

    fn advance_to_next_symbol(&mut self) {
        self.current_symbol = None;
        self.found = [None; 6];
        self.found_next = 0;

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
        // tracing::info!(symbol = ?self.current_symbol, at_index = self.current_symbol_idx, "found");

        let (above_a, above_b) = self.find_in_touching_line(self.above);
        let left = self.find_left();
        let right = self.find_right();
        let (below_a, below_b) = self.find_in_touching_line(self.below);

        self.found = [above_a, above_b, left, right, below_a, below_b];
        // tracing::info!(found = ?self.found, "advance_to_next_symbol...");

    }

    fn find_left(&self) -> Option<u64> {
        if self.current_symbol_idx == 0 {
            return None;
        }
        self.current[..self.current_symbol_idx]
            .chars()
            .rev()
            .next()
            .filter(char::is_ascii_digit)
            .map(|_| read_num(self.current, self.current_symbol_idx - 1))
    }

    fn find_right(&self) -> Option<u64> {
        if self.current_symbol_idx == self.current.len() - 1 {
            return None;
        }
        self.current[self.current_symbol_idx + 1..]
            .chars()
            .next()
            .filter(char::is_ascii_digit)
            .map(|_| read_num(self.current, self.current_symbol_idx + 1))
    }

    fn find_in_touching_line(&self, touching_line: Option<&str>) -> (Option<u64>, Option<u64>) {
        match touching_line {
            Some(touching_line) => {
                let left = if self.current_symbol_idx > 0 { self.current_symbol_idx - 1 } else { self.current_symbol_idx };
                let right = if self.current_symbol_idx < touching_line.len() - 1 { self.current_symbol_idx + 1 } else { self.current_symbol_idx };
                let mut chars_touching = touching_line[left..=right].chars();

                let l = chars_touching.next();
                let m = chars_touching.next();
                let r = chars_touching.next();
                // tracing::info!(?l, ?m, ?r, "touching");

                match m.filter(char::is_ascii_digit).map(|_m| read_num(touching_line, self.current_symbol_idx)) {
                    Some(only_possible_num) => (Some(only_possible_num), None),
                    None => {
                        let l = l.filter(char::is_ascii_digit).map(|_| read_num(touching_line, left));
                        let r = r.filter(char::is_ascii_digit).map(|_| read_num(touching_line, right));
                        (l, r)
                    },
                }
            },
            None => (None, None),
        }
    }
}

impl<'a> Iterator for EnginePartIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_part = None;

        while next_part.is_none() {
            if self.current_symbol.is_none() {
                self.advance_to_next_symbol();
                if self.current_symbol.is_none() {
                    return None;
                }
            }

            if self.found_next < 6 {
                next_part = self.found[self.found_next];
                self.found_next += 1;
            } else {
                self.current_symbol = None;
                if self.current_symbol_idx < self.current.len() {
                    self.current_symbol_idx += 1;
                }
            }
        }

        next_part
    }
}

fn read_num(input: &str, idx: usize) -> u64 {
    let min_idx = idx - input[..idx].chars().rev().enumerate().skip_while(|(_i, c)| c.is_ascii_digit()).next().map(|(i, _c)| i).unwrap_or(idx);
    let max_idx = idx + input[idx..].chars().enumerate().skip_while(|(_i, c)| c.is_ascii_digit()).next().map(|(i, _c)| i).unwrap_or(input.len() - idx);
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
    #[test]
    fn test_chars_above() {
        let above = "123456";
        let idx = 3;
        let mut chars_above = above[idx-1..=idx+1].chars();
        assert_eq!(chars_above.next(), Some('3'));
        assert_eq!(chars_above.next(), Some('4'));
        assert_eq!(chars_above.next(), Some('5'));
    }
}