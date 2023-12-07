use std::cmp::Ordering;

pub fn part1(input: &str) -> u64 {
    solve(input, false)
}

pub fn part2(input: &str) -> u64 {
    solve(input, true)
}

pub fn solve(input: &str, interpret_j_as_joker: bool) -> u64 {
    let mut hands = parse(input, interpret_j_as_joker)
        .map(|(hand, bid)| (hand, hand.strength(), bid))
        .collect::<Vec<_>>();
    hands.sort_by(|(hand_a, strength_a, _), (hand_b, strength_b, _)| {
        match strength_a.cmp(strength_b) {
            Ordering::Equal => hand_a
                .cards
                .iter()
                .zip(hand_b.cards.iter())
                .find(|(a, b)| a != b)
                .map(|(a, b)| a.cmp(b))
                .unwrap_or(Ordering::Equal), // Technically unreachable, as totally equal hands would have had the same strength to begin with.
            ordering => ordering,
        }
    });
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_hand, _strength, bid))| bid * (rank as u64 + 1))
        .sum()
}

fn parse(input: &str, interpret_j_as_joker: bool) -> impl Iterator<Item = (Hand, u64)> + '_ {
    input
        .lines()
        .map(|line| line.split_once(' ').expect("line with at least one space"))
        .map(move |(card_identifiers, num)| {
            (
                {
                    let mut it = card_identifiers
                        .chars()
                        .map(|c| Card::try_from((c, interpret_j_as_joker)).expect("valid card identifier"));
                    Hand {
                        cards: [
                            it.next().expect("first card"),
                            it.next().expect("second card"),
                            it.next().expect("third card"),
                            it.next().expect("fourth card"),
                            it.next().expect("fifth card"),
                        ],
                    }
                },
                num.parse::<u64>().expect("valid number"),
            )
        })
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    /// Order must be preserved!
    cards: [Card; 5],
}

impl Hand {
    fn strength(&self) -> HandStrength {
        let mut jokers = 0;
        let mut counts = [0u8; 13];

        for card in self.cards {
            match card {
                Card::Ace => counts[12] += 1,
                Card::King => counts[11] += 1,
                Card::Queen => counts[10] += 1,
                Card::Jack => counts[9] += 1,
                Card::Ten => counts[8] += 1,
                Card::Nine => counts[7] += 1,
                Card::Eight => counts[6] += 1,
                Card::Seven => counts[5] += 1,
                Card::Six => counts[4] += 1,
                Card::Five => counts[3] += 1,
                Card::Four => counts[2] += 1,
                Card::Three => counts[1] += 1,
                Card::Two => counts[0] += 1,
                Card::Joker => jokers += 1,
            }
        }
        let (mut most, second_most) = find_highest_and_second_highest(counts.into_iter());
        most += jokers;

        assert!(most >= second_most);
        match (most, second_most) {
            (5, _) => HandStrength::FiveOfAKind,
            (4, _) => HandStrength::FourOfAKind,
            (3, 2) => HandStrength::FullHouse,
            (2, 3) => HandStrength::FullHouse,
            (3, _) => HandStrength::ThreeOfAKind,
            (2, 2) => HandStrength::TwoPairs,
            (2, _) => HandStrength::OnePair,
            (1, _) => HandStrength::HighCard,
            _ => unreachable!("All possible 'most' cases are handled!"),
        }
    }
}

fn find_highest_and_second_highest(iter: impl Iterator<Item = u8>) -> (u8, u8) {
    let mut most = 0;
    let mut second_most = 0;
    for count in iter {
        if count >= most {
            second_most = most;
            most = count;
        } else if count > second_most {
            second_most = count;
        }
    }
    (most, second_most)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl TryFrom<(char, bool)> for Card {
    type Error = String;

    fn try_from(value: (char, bool)) -> Result<Self, Self::Error> {
        match value.0 {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => match value.1 {
                true => Ok(Self::Joker),
                false => Ok(Self::Jack),
            },
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            c => Err(format!("Character '{c}' is not a valid Card identifier.")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Card;
    use super::Hand;
    use super::HandStrength;

    #[test]
    fn strength() {
        assert_eq!(
            Hand {
                cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]
            }
            .strength(),
            HandStrength::FiveOfAKind
        );
        assert_eq!(
            Hand {
                cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]
            }
            .strength(),
            HandStrength::FourOfAKind
        );
        assert_eq!(
            Hand {
                cards: [Card::Two, Card::Ace, Card::Two, Card::Ace, Card::Ace]
            }
            .strength(),
            HandStrength::FullHouse
        );
        assert_eq!(
            Hand {
                cards: [Card::Two, Card::Ace, Card::Two, Card::Two, Card::Ten]
            }
            .strength(),
            HandStrength::ThreeOfAKind
        );
        assert_eq!(
            Hand {
                cards: [Card::Ten, Card::Ace, Card::Two, Card::Two, Card::Ten]
            }
            .strength(),
            HandStrength::TwoPairs
        );
        assert_eq!(
            Hand {
                cards: [Card::Two, Card::Two, Card::Two, Card::Four, Card::Four]
            }
            .strength(),
            HandStrength::FullHouse
        );
    }

    #[test]
    fn hand_strength_ordering() {
        assert!(HandStrength::FiveOfAKind > HandStrength::FourOfAKind);
    }

    #[test]
    fn card_ordering() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
    }
}
