use std::{cmp::Ordering, fmt};

pub mod babanuki;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug, Hash)]
enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}
#[derive(Clone, Copy, Debug)]
enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub enum Card {
    Standard { rank: Rank, suit: Suite },
    Joker,
}

pub struct Hand(Vec<Card>);

impl Card {
    fn compare_rank(&self, other: &Card) -> Ordering {
        match (self, other) {
            (Card::Standard { rank: rank1, .. }, Card::Standard { rank: rank2, .. }) => {
                rank1.cmp(rank2)
            }
            (Card::Joker, Card::Joker) => Ordering::Equal,
            (Card::Joker, _) => Ordering::Greater,
            (_, Card::Joker) => Ordering::Less,
        }
    }

    fn rank(&self) -> Rank {
        match self {
            Card::Standard { rank, .. } => *rank,
            Card::Joker => Rank::Joker,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_symbol = match self {
            Suite::Hearts => "♥️",
            Suite::Diamonds => "♦️",
            Suite::Clubs => "♣️",
            Suite::Spades => "♠️",
        };
        write!(f, "{}", suit_symbol)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_symbol = match self {
            Rank::One => "1",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Joker => "",
        };
        write!(f, "{}", rank_symbol)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Standard { rank, suit } => write!(f, "{}  {} ", suit, rank),
            Card::Joker => write!(f, "🃏 "),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.0 {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}

impl Hand {
    fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for &suit in &[Suite::Hearts, Suite::Diamonds, Suite::Clubs, Suite::Spades] {
            for &rank in &[
                Rank::One,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card::Standard {
                    rank: rank,
                    suit: suit,
                })
            }
        }
        cards.push(Card::Joker);

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // Dplit the deck into a vector of hands
    pub fn deal(&self, num_hands: u32) -> Vec<Hand> {
        let mut hands: Vec<Hand> = Vec::with_capacity(num_hands as usize);

        for _ in 0..num_hands {
            hands.push(Hand(Vec::new()));
        }

        for (i, &card) in self.cards.iter().enumerate() {
            hands[i % num_hands as usize].push(card);
        }

        hands
    }
}
