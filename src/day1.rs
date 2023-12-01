use smallvec::SmallVec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("../res/day1.txt");

#[tracing::instrument]
pub fn part1() {
    let start = std::time::Instant::now();
    let sum: u64 = INPUT
        .lines()
        .map(|line| {
            tracing::debug!(line, "parsing");
            let first = DigitIterator::new(line, false).next().unwrap();
            let last = DigitIterator::new(line, false).next_back().unwrap();
            AsciiDigit::parse([first, last])
        })
        .sum();
    let end = std::time::Instant::now();
    let took = format!("{} μs", (end - start).as_micros());
    tracing::info!(took, sum, "result");
    assert_eq!(sum, 54601);
}

#[tracing::instrument]
pub fn part2() {
    let start = std::time::Instant::now();
    let sum: u64 = INPUT
        .lines()
        .map(|line| {
            tracing::debug!(line, "parsing");
            let first = DigitIterator::new(line, true).next().unwrap();
            let last = DigitIterator::new(line, true).next_back().unwrap();
            AsciiDigit::parse([first, last])
        })
        .sum();
    let end = std::time::Instant::now();
    let took = format!("{} μs", (end - start).as_micros());
    tracing::info!(took, sum, "result");
    assert_eq!(sum, 54078);
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum AsciiDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl AsciiDigit {
    fn parse<const N: usize>(digits: [AsciiDigit; N]) -> u64 {
        let bytes: SmallVec<[u8; 16]> =
            SmallVec::from_iter(digits.into_iter().map(|d| d.into_ascii_byte()));
        let s = std::str::from_utf8(&bytes).expect("not UTF-8");
        s.parse::<u64>().expect("not a number")
    }

    fn into_ascii_byte(self) -> u8 {
        match self {
            AsciiDigit::Zero => b'0',
            AsciiDigit::One => b'1',
            AsciiDigit::Two => b'2',
            AsciiDigit::Three => b'3',
            AsciiDigit::Four => b'4',
            AsciiDigit::Five => b'5',
            AsciiDigit::Six => b'6',
            AsciiDigit::Seven => b'7',
            AsciiDigit::Eight => b'8',
            AsciiDigit::Nine => b'9',
        }
    }

    fn into_str(self) -> &'static str {
        match self {
            AsciiDigit::Zero => "zero",
            AsciiDigit::One => "one",
            AsciiDigit::Two => "two",
            AsciiDigit::Three => "three",
            AsciiDigit::Four => "four",
            AsciiDigit::Five => "five",
            AsciiDigit::Six => "six",
            AsciiDigit::Seven => "seven",
            AsciiDigit::Eight => "eight",
            AsciiDigit::Nine => "nine",
        }
    }
}

impl TryFrom<char> for AsciiDigit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(AsciiDigit::Zero),
            '1' => Ok(AsciiDigit::One),
            '2' => Ok(AsciiDigit::Two),
            '3' => Ok(AsciiDigit::Three),
            '4' => Ok(AsciiDigit::Four),
            '5' => Ok(AsciiDigit::Five),
            '6' => Ok(AsciiDigit::Six),
            '7' => Ok(AsciiDigit::Seven),
            '8' => Ok(AsciiDigit::Eight),
            '9' => Ok(AsciiDigit::Nine),
            other => Err(format!("Cannot convert '{other}' to AsciiDigit.")),
        }
    }
}

struct DigitIterator<'a> {
    left: &'a str,
    search_str_representation: bool,
}

impl<'a> DigitIterator<'a> {
    pub fn new(string: &'a str, search_str_representation: bool) -> Self {
        Self {
            left: string,
            search_str_representation,
        }
    }

    fn skip(&mut self, num: usize) {
        self.left = &self.left[num..];
    }

    fn skip_back(&mut self, num: usize) {
        self.left = &self.left[..self.left.len() - num];
    }

    fn advance(&mut self) -> Option<AsciiDigit> {
        if let Some(char) = self.left.chars().next() {
            if char.is_ascii_digit() {
                let digit = AsciiDigit::try_from(char).unwrap();
                self.skip(1);
                return Some(digit);
            }
        }
        if self.search_str_representation {
            for digit in AsciiDigit::iter() {
                if self.left.starts_with(digit.into_str()) {
                    self.skip(digit.into_str().len());
                    return Some(digit);
                }
            }
        }
        self.skip(1);
        None
    }

    fn advance_back(&mut self) -> Option<AsciiDigit> {
        if let Some(char) = self.left.chars().next_back() {
            if char.is_ascii_digit() {
                let digit = AsciiDigit::try_from(char).unwrap();
                self.skip_back(1);
                return Some(digit);
            }
        }
        if self.search_str_representation {
            for digit in AsciiDigit::iter() {
                if self.left.ends_with(digit.into_str()) {
                    self.skip_back(digit.into_str().len());
                    return Some(digit);
                }
            }
        }
        self.skip_back(1);
        None
    }
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = AsciiDigit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Option<AsciiDigit> = None;
        while next.is_none() && !self.left.is_empty() {
            next = self.advance();
        }
        next
    }
}

impl<'a> DoubleEndedIterator for DigitIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut next: Option<AsciiDigit> = None;
        while next.is_none() && !self.left.is_empty() {
            next = self.advance_back();
        }
        next
    }
}
