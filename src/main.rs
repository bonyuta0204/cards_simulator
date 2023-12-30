use std::collections::HashMap;

use cards_simulator::{babanuki::discard_pairs, Deck};

fn main() {
    const TRIAL: u32 = 10000;
    const NUM_HANDS: u32 = 2;

    let mut deck = Deck::new();
    let mut distribution: HashMap<usize, u32> = HashMap::new();

    for i in 0..TRIAL {
        deck.shuffle();

        let hands = deck.deal(NUM_HANDS);

        for hand in hands {
            let remaining_cards = discard_pairs(&hand);
            *distribution.entry(remaining_cards.len()).or_insert(0) += 1;
        }
    }
    // Collect keys and sort them
    let mut sorted_keys: Vec<_> = distribution.keys().cloned().collect();
    sorted_keys.sort();

    // Print the distribution using the sorted keys
    println!("Distribution of remaining card counts:");
    for key in sorted_keys {
        if let Some(&occurrences) = distribution.get(&key) {
            println!("{:2} cards: {:4} occurrences", key, occurrences);
        }
    }

    let expectation = distribution.iter().fold(0.0, |acc, (&key, &value)| {
        acc + (key as f64) * (value as f64) / (TRIAL as f64 * NUM_HANDS as f64)
    });
    println!("Expectation: {}", expectation)
}
