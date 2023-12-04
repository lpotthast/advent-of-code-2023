const INPUT: &str = include_str!("../res/day4.txt");

#[tracing::instrument]
pub fn part1() -> u64 {
    INPUT
        .lines()
        .map(Card::init)
        .map(|(card, your_numbers)| {
            let mut winning_numbers = your_numbers.filter(|num| card.is_winning(*num));
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

#[tracing::instrument]
pub fn part2() -> u64 {
    const N: usize = 206;
    let mut copies = [1u32; N];

    INPUT.lines().map(Card::init).for_each(|(card, your_numbers)| {
        let idx = card.id as usize - 1;

        let copies_of_current = copies[idx];

        let num_winning_numbers = your_numbers.filter(|num| card.is_winning(*num)).count();

        for offset in 1..=num_winning_numbers {
            let i = usize::min(idx + offset, N - 1);
            copies[i] += copies_of_current; // 1 times copies_of_current !
        }
    });

    copies.iter().sum::<u32>() as u64
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning: [bool; 100],
}

impl Card {
    fn init<'a>(line: &'a str) -> (Self, impl Iterator<Item = u32> + 'a) {
        let (card, rest) = line.split_once(':').expect("at least one ':'");
        let id = card
            .trim_start_matches("Card")
            .trim_start()
            .parse::<u32>()
            .expect("card id to be number");

        let (winning_numbers, your_numbers) = rest
            .split_once('|')
            .map(|(s, e)| (s.trim(), e.trim()))
            .expect("at least one ':'");

        let mut winning = [false; 100];
        for num in winning_numbers
            .split_ascii_whitespace()
            .map(|it| it.parse::<u32>().expect("number"))
        {
            winning[num as usize] = true;
        }

        let card = Card { id, winning };

        (
            card,
            your_numbers
                .split_ascii_whitespace()
                .map(|it| it.parse::<u32>().expect("number")),
        )
    }

    fn is_winning(&self, num: u32) -> bool {
        self.winning[num as usize]
    }
}
