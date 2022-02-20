use crate::game::cards::{active::ActiveCards, deck::Deck};

use super::player::Players;

struct PrePlay {
    deck: Deck,
    players: Players,
    river: ActiveCards,
}

pub fn run_pre_play(players: Players) -> ActivePlay {
    let mut pre_play = PrePlay {
        deck: Deck::new(true),
        players,
        river: ActiveCards::new(),
    };

    pre_play
        .players
        .deal_cards(false, &mut pre_play.deck, &mut pre_play.river); //TODO: Implement tap

    // pre_play.determine_active_players()

    ActivePlay {
        deck: pre_play.deck,
        players: pre_play.players,
        river: pre_play.river,
    }
}

pub struct ActivePlay {
    deck: Deck,
    river: ActiveCards,
    players: Players,
}

impl ActivePlay {
    pub fn run_round(self) -> Players {
        self.players
    }
}

// fn draw_cards_and_move_to_active(num_cards_to_draw: i8, deck: &mut Deck, active: &mut ActiveCards) {
//     for _i in 0..num_cards_to_draw {
//         active.add_card(deck.draw())
//     }
// }
