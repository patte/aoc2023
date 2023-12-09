// https://www.reddit.com/r/adventofcode/comments/18cnzbm/comment/kcg85zm
// https://www.reddit.com/user/aleks31414/

#![feature(slice_group_by)]
use core::cmp::PartialEq;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            'J' => J,
            '2' => _2,
            '3' => _3,
            '4' => _4,
            '5' => _5,
            '6' => _6,
            '7' => _7,
            '8' => _8,
            '9' => _9,
            'T' => T,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    type_: HandType,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let mut sorted: [Card; 5] = [Card::_2; 5];
        sorted.copy_from_slice(&cards[..]);
        sorted.sort();

        let (not_jokers, jokers): (Vec<_>, Vec<_>) =
            sorted.into_iter().partition(|&c| c != Card::J);

        let groups = if !not_jokers.is_empty() {
            let mut groups: Vec<usize> = not_jokers
                .group_by(PartialEq::eq)
                .map(|g| g.len())
                .collect();
            groups.sort_by(|a, b| b.cmp(a));
            groups[0] += jokers.len();
            groups
        } else {
            vec![jokers.len()]
        };

        let type_ = match groups[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        };
        Self { cards, type_ }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = self.type_.cmp(&other.type_);
        if order.is_eq() {
            self.cards.cmp(&other.cards)
        } else {
            order
        }
    }
}

fn main() {
    let mut hands: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (card, bid) = l.split_once(' ').unwrap();
            (
                Hand::new(
                    card.chars()
                        .map(|c| c.into())
                        .collect::<Vec<Card>>()
                        .try_into()
                        .unwrap(),
                ),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    let winnings: u32 = hands
        .into_iter()
        .zip(1u32..)
        .map(|((_, bid), rank)| rank * bid)
        .sum();
    println!("{winnings}");
}
