use crate::game::cards::card::{Card, Suit};
use std::collections::HashMap;

/// Cards that are visible to at least one player
/// i.e. A player's hand, the game's river
#[derive(Debug)]
pub struct ActiveCards {
    pub cards: HashMap<Suit, Vec<Card>>,
}

impl ActiveCards {
    pub fn new() -> ActiveCards {
        ActiveCards {
            cards: HashMap::new(),
        }
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.add_card(card);
        }
    }

    pub fn add_card(&mut self, card: Card) {
        if !self.cards.contains_key(&card.suit) {
            self.cards.insert(card.suit, Vec::new());
        }

        self.cards
            .get_mut(&card.suit)
            .expect("Vector of cards")
            .push(card);
    }

    pub fn check_match(&self, suit: Suit) -> bool {
        self.cards.contains_key(&suit)
    }
}
