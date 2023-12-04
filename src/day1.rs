use smallvec::SmallVec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("../res/day1.txt");

pub fn part1() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let first = DigitIterator::new(line, false).next().expect("first");
            let last = DigitIterator::new(line, false).next_back().expect("last");
            AsciiDigit::into_u64([first, last])
        })
        .sum::<u64>()
}

pub fn part2() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let first = DigitIterator::new(line, true).next().expect("first");
            let last = DigitIterator::new(line, true).next_back().expect("last");
            AsciiDigit::into_u64([first, last])
        })
        .sum::<u64>()
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum AsciiDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl AsciiDigit {
    fn into_u64<const N: usize>(digits: [Self; N]) -> u64 {
        match N {
            1 => u64::from(digits[0].into_u8()),
            2 => u64::from(10 * digits[0].into_u8() + digits[1].into_u8()),
            _ => {
                let bytes = digits
                    .into_iter()
                    .map(Self::into_ascii_byte)
                    .collect::<SmallVec<[u8; 16]>>();
                let s = std::str::from_utf8(&bytes).expect("not UTF-8");
                s.parse::<u64>().expect("not a number")
            }
        }
    }

    const fn into_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }

    const fn into_ascii_byte(self) -> u8 {
        match self {
            Self::Zero => b'0',
            Self::One => b'1',
            Self::Two => b'2',
            Self::Three => b'3',
            Self::Four => b'4',
            Self::Five => b'5',
            Self::Six => b'6',
            Self::Seven => b'7',
            Self::Eight => b'8',
            Self::Nine => b'9',
        }
    }

    const fn en_str_repr(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
        }
    }
}

impl TryFrom<char> for AsciiDigit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            other => Err(format!("Cannot convert '{other}' to AsciiDigit.")),
        }
    }
}

struct DigitIterator<'a> {
    left: &'a str,
    search_str_repr: bool,
}

impl<'a> DigitIterator<'a> {
    const fn new(string: &'a str, search_str_repr: bool) -> Self {
        Self {
            left: string,
            search_str_repr,
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
                let digit = AsciiDigit::try_from(char).expect("ascii digit");
                self.skip(1);
                return Some(digit);
            }
        }
        if self.search_str_repr {
            for digit in AsciiDigit::iter() {
                if self.left.starts_with(digit.en_str_repr()) {
                    self.skip(digit.en_str_repr().len());
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
                let digit = AsciiDigit::try_from(char).expect("ascii digit");
                self.skip_back(1);
                return Some(digit);
            }
        }
        if self.search_str_repr {
            for digit in AsciiDigit::iter() {
                if self.left.ends_with(digit.en_str_repr()) {
                    self.skip_back(digit.en_str_repr().len());
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
