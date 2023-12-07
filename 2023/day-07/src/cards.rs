use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Card {
    Jack = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

#[derive(Debug, Eq, Ord)]
pub struct Hand {
    pub hand: Vec<Card>,
    pub bid: usize,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .hand
            .iter()
            .map(|card| match card {
                Card::Two => "2",
                Card::Three => "3",
                Card::Four => "4",
                Card::Five => "5",
                Card::Six => "6",
                Card::Seven => "7",
                Card::Eight => "8",
                Card::Nine => "9",
                Card::Ten => "t",
                Card::Jack => "j",
                Card::Queen => "q",
                Card::King => "k",
                Card::Ace => "a",
            })
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", cards)
    }
}

impl Hand {
    pub fn new(hand: Vec<Card>, bid: usize) -> Self {
        Self { hand, bid }
    }

    pub fn rank(&self) -> usize {
        let mut j_count = 0;
        let map = self.hand.iter().fold(HashMap::new(), |mut acc, card| {
            if *card == Card::Jack {
                j_count += 1;
            }
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        let mut counts = map.values().collect::<Vec<_>>();
        counts.sort();

        match counts[..] {
            [5] => 6,
            [1, 4] => {
                if j_count != 0 {
                    6
                } else {
                    5
                }
            }
            [2, 3] => {
                if j_count != 0 {
                    6
                } else {
                    4
                }
            }
            [1, 1, 3] => {
                if j_count != 0 {
                    5
                } else {
                    3
                }
            }
            [1, 2, 2] => match j_count {
                _ if j_count == 2 => 5,
                _ if j_count == 1 => 4,
                _ => 2,
            },
            [1, 1, 1, 2] => {
                if j_count != 0 {
                    3
                } else {
                    1
                }
            }
            _ => j_count,
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let parts = s.split_once(' ').expect("bad hand format");
        let hand = parts
            .0
            .chars()
            .map(|c| match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                't' => Card::Ten,
                'j' => Card::Jack,
                'q' => Card::Queen,
                'k' => Card::King,
                'a' => Card::Ace,
                _ => panic!("Invalid card: {}", c),
            })
            .collect::<Vec<_>>();
        let bid = parts.1.parse::<usize>().expect("invalid bid");
        Self::new(hand, bid)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand.iter().zip(other.hand.iter()).all(|(a, b)| a == b)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank().cmp(&other.rank()) {
            std::cmp::Ordering::Equal => {
                self.hand
                    .iter()
                    .zip(other.hand.iter())
                    .find_map(|(a, b)| match a.cmp(b) {
                        std::cmp::Ordering::Equal => None,
                        o => Some(o),
                    })
            }
            o => Some(o),
        }
    }
}
