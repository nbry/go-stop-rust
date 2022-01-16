use std::collections::VecDeque;

use crate::cards::{active::ActiveCards, deck::Deck};

use super::player::Player;

pub struct Round<'a> {
    deck: Deck,
    river: ActiveCards,
    players: &'a mut VecDeque<Player>,
}

impl Round<'_> {
    pub fn run(players: &mut VecDeque<Player>) {
        let mut round = Round {
            deck: Deck::new(true),
            river: ActiveCards::new(),
            players,
        };

        round.deal_cards(false)
    }

    fn deal_cards(&mut self, tap: bool) {
        /*
        TODO: Deal cards in traditional way
        TODO: Make configurable, based on number of players
        */
        const STARTING_NUM_CARDS_IN_HAND: i8 = 10;
        const STARTING_NUM_CARDS_IN_RIVER: i8 = 10;

        for player in self.players.iter_mut() {
            for _i in 0..STARTING_NUM_CARDS_IN_HAND {
                player.hand.add_card(self.deck.draw());
            }
        }

        for _i in 0..STARTING_NUM_CARDS_IN_RIVER {
            self.river.add_card(self.deck.draw());
        }
    }

    fn draw_cards_and_move_to_active(
        num_cards_to_draw: i8,
        deck: &mut Deck,
        active: &mut ActiveCards,
    ) {
        for _i in 0..num_cards_to_draw {
            active.add_card(deck.draw())
        }
    }

    fn next_turn(&mut self) {
        let previous_player = self
            .players
            .pop_front()
            .expect("Players in round empty... for some reason");

        self.players.push_back(previous_player);
    }
}
