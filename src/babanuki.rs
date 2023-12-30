use crate::Hand;
use std::collections::HashMap;

// remove pairs from hand and return remaining cards

pub fn discard_pairs(hand: &Hand) -> Hand {
    let mut counts = HashMap::new();

    let cards = &hand.0;

    // Count the occurrences of each rank
    for card in cards {
        counts.entry(card.rank()).or_insert(Vec::new()).push(card);
    }

    let mut remaining_cards = Vec::new();

    for (key, cards) in counts {
        if cards.len() % 2 == 0 {
        } else {
            remaining_cards.push(*cards[0]);
        }
    }

    Hand(remaining_cards)
}
