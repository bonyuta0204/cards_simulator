use crate::Hand;
use std::collections::HashMap;

// remove pairs from hand and return remaining cards

pub fn discard_pairs(hand: &Hand) -> Hand {
    let mut counts = HashMap::new();
    
    let cards = &hand.0;

    // Count the occurrences of each rank
    for card in cards {
        if let Some(rank) = card.rank() {
            *counts.entry(rank).or_insert(0) += 1;
        }
    }

    // Discard cards that are part of a pair
    Hand(cards.iter()
        .filter(|&card| match card.rank() {
            Some(rank) => counts[&rank] % 2 == 1,
            None => true,
        } )
        .cloned()
        .collect())
}