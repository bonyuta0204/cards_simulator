use cards_simulator::{Deck, babanuki::discard_pairs, Hand};

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let hands = deck.deal(5);

    for hand in hands {
        let remaining_cards  = discard_pairs(&hand) ;
        println!("Remaining cards: {}", remaining_cards);
    }
}
