pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(Card::init)
        .map(|(card, our_numbers)| {
            let mut winning_numbers = our_numbers.filter(|num| card.is_winning(*num));
            let mut r = 0;
            if let Some(_first) = winning_numbers.next() {
                r += 1;
            }
            for _num in winning_numbers {
                r *= 2;
            }
            r
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    const N: usize = 206;
    let mut copies = [1u32; N];

    input.lines().map(Card::init).for_each(|(card, our_numbers)| {
        let idx = card.id as usize - 1;
        let copies_of_current_card = copies[idx];

        for offset in 1..=our_numbers.filter(|num| card.is_winning(*num)).count() {
            copies[usize::min(idx + offset, N - 1)] += copies_of_current_card;
        }
    });

    u64::from(copies.into_iter().sum::<u32>())
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_bitmask: u128,
}

impl Card {
    fn init(line: &str) -> (Self, impl Iterator<Item = u8> + '_) {
        let (card, rest) = line.split_once(':').expect("at least one ':'");
        let id = card
            .trim_start_matches("Card")
            .trim_start()
            .parse::<u32>()
            .expect("card id to be number");

        let (winning_numbers, our_numbers) = rest
            .split_once('|')
            .map(|(w, y)| (parse_numbers(w), parse_numbers(y)))
            .expect("at least one ':'");

        let winning_bitmask: u128 = winning_numbers.fold(0u128, |bitmask, num| bitmask | (1 << num));

        (Self { id, winning_bitmask }, our_numbers)
    }

    fn is_winning(&self, num: u8) -> bool {
        debug_assert!((0..=127).contains(&num));
        self.winning_bitmask & (1 << num) != 0
    }
}

fn parse_numbers(line: &str) -> impl Iterator<Item = u8> + '_ {
    line.split_ascii_whitespace().map(|num| match num.len() {
        1 => num.as_bytes()[0] - b'0',
        2 => 10 * (num.as_bytes()[0] - b'0') + (num.as_bytes()[1] - b'0'),
        _ => panic!("Only 2-digit numbers are supported! Received: {num}"),
    })
}

#[cfg(test)]
mod test {
    use super::parse_numbers;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(vec![1, 2, 42, 99], parse_numbers(" 1  2 42   99  ").collect::<Vec<_>>())
    }
}
