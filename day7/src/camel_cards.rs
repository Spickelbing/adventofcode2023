use eyre::{anyhow, Error};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(anyhow!("invalid digit")),
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.chars().count() != 5 {
            return Err(anyhow!("invalid string length"));
        }

        let mut cards = Vec::new();
        for char in string.chars() {
            let card: Card = char.try_into()?;
            cards.push(card);
        }

        Ok(Hand {
            cards: cards.try_into().unwrap(),
        })
    }
}

impl From<&Hand> for HandCategory {
    fn from(hand: &Hand) -> Self {
        if hand.cards.iter().all_equal() {
            return Self::FiveOfAKind;
        }

        let n_occurences_per_present_card = hand.cards.iter().counts();

        let len = n_occurences_per_present_card.len();
        let four_equal = n_occurences_per_present_card
            .iter()
            .any(|(_, &n_occurences)| n_occurences == 4);
        let three_equal = n_occurences_per_present_card
            .iter()
            .any(|(_, &n_occurences)| n_occurences == 3);

        match (len, four_equal, three_equal) {
            (_, true, _) => Self::FourOfAKind,
            (2, _, true) => Self::FullHouse,
            (3, _, true) => Self::ThreeOfAKind,
            (3, _, false) => Self::TwoPair,
            (4, _, _) => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandCategory {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn category(&self) -> HandCategory {
        self.into()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.category() != other.category() {
            return self.category().cmp(&other.category());
        }

        for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }

        Ordering::Equal
    }
}
